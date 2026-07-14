use super::resident_frame_table::next_lease_epoch;
use crate::{
    EvictionCandidateSet, EvictionCounterSnapshot, EvictionPlan, EvictionPressure,
    EvictionProtectionReason, EvictionProtectionSummary, EvictionReceipt, FrameProtectionReceipt,
    ProtectedFrameDenial, ResidentByteCount, ResidentFrameDenial, ResidentFrameDenialKind,
    ResidentFrameIdentity, ResidentFrameTable, ResidentFrameToken,
};

impl ResidentFrameTable {
    pub const fn eviction_counters(&self) -> EvictionCounterSnapshot {
        self.eviction_counters
    }

    pub fn resident_frame_count(&self) -> u64 {
        self.resident_slots.len() as u64
    }

    pub fn protect_frame_for_verifier(
        &mut self,
        token: ResidentFrameToken,
    ) -> Result<FrameProtectionReceipt, ResidentFrameDenial> {
        self.protect_frame(token, EvictionProtectionReason::VerifierProtected)
    }

    pub fn protect_frame_for_recovery(
        &mut self,
        token: ResidentFrameToken,
    ) -> Result<FrameProtectionReceipt, ResidentFrameDenial> {
        self.protect_frame(token, EvictionProtectionReason::RecoveryProtected)
    }

    pub fn protect_frame_for_streaming(
        &mut self,
        token: ResidentFrameToken,
    ) -> Result<FrameProtectionReceipt, ResidentFrameDenial> {
        self.protect_frame(token, EvictionProtectionReason::StreamingProtected)
    }

    pub fn plan_eviction(
        &mut self,
        pressure: EvictionPressure,
    ) -> Result<EvictionPlan, ResidentFrameDenial> {
        self.eviction_counters = self.eviction_counters.with_plan_attempt();
        let scan = self.scan_eviction_candidates()?;
        self.eviction_counters = scan.counters;
        if scan.resident_frames_scanned == 0 {
            self.eviction_counters = self.eviction_counters.with_no_resident_candidate_denial();
            self.publish_eviction_counters();
            return Err(ResidentFrameDenial::new(
                ResidentFrameDenialKind::NoResidentEvictionCandidates,
            ));
        }
        let Some(selected_identity) = scan.selected_identity else {
            self.eviction_counters = self.eviction_counters.with_all_protected_denial();
            self.publish_eviction_counters();
            return Err(ResidentFrameDenial::protected_frame(
                ResidentFrameDenialKind::AllEvictionCandidatesProtected,
                ProtectedFrameDenial::new(scan.protected_exclusions, self.eviction_counters),
            ));
        };
        self.eviction_counters = self.eviction_counters.with_policy_rank();
        self.publish_eviction_counters();
        let record = self.record_at_slot(selected_identity.slot())?;
        let candidate_set = EvictionCandidateSet::new(
            selected_identity,
            scan.resident_frames_scanned,
            scan.candidate_count,
            scan.protected_exclusions,
            self.eviction_counters.policy_rank_count(),
            self.eviction_counters,
        );
        Ok(EvictionPlan::new(
            pressure,
            candidate_set,
            record.request(),
            self.lease_epochs[selected_identity.slot().index() as usize],
        ))
    }

    pub fn record_eviction(
        &mut self,
        plan: EvictionPlan,
    ) -> Result<EvictionReceipt, ResidentFrameDenial> {
        let identity = plan.selected_identity();
        let protection = {
            let record = self.record_at_slot(identity.slot())?;
            if record.identity() != identity
                || self.lease_epochs[identity.slot().index() as usize] != plan.lease_epoch()
            {
                return self.deny_stale_eviction_plan();
            }
            record.eviction_protection_summary()
        };
        if !protection.is_empty() {
            return self.deny_protected_eviction_plan(protection);
        }
        let released_bytes = plan.frame_size_bytes();
        let current_resident_bytes = self.counters.resident_bytes().as_bytes();
        let record = self.record_at_slot_mut(identity.slot())?;
        let request = record.request();
        self.frames[identity.slot().index() as usize] = None;
        self.resident_source_index.remove(&request.source_key());
        self.untrack_resident_slot(identity.slot());
        self.free_slots.push(identity.slot());
        let next_generation = self.next_generation_for_slot(identity.slot())?;
        self.generations[identity.slot().index() as usize] = next_generation;
        self.lease_epochs[identity.slot().index() as usize] = next_lease_epoch(plan.lease_epoch())?;
        self.counters = self
            .counters
            .with_resident_bytes(current_resident_bytes - released_bytes);
        self.eviction_counters = self.eviction_counters.with_receipt();
        self.publish_eviction_counters();
        Ok(EvictionReceipt::new(
            identity,
            ResidentByteCount::from_observed_bytes(released_bytes),
            self.eviction_counters,
        ))
    }

    fn protect_frame(
        &mut self,
        token: ResidentFrameToken,
        reason: EvictionProtectionReason,
    ) -> Result<FrameProtectionReceipt, ResidentFrameDenial> {
        let identity = self.validate_resident_token_for_lease(token)?;
        self.record_at_slot_mut(identity.slot())?
            .mark_eviction_protected(reason);
        Ok(FrameProtectionReceipt::new(identity, reason))
    }

    fn scan_eviction_candidates(&self) -> Result<EvictionCandidateScan, ResidentFrameDenial> {
        self.resident_slots.iter().copied().try_fold(
            EvictionCandidateScan::new(self.eviction_counters),
            |scan, slot| {
                let record = self.record_at_slot(slot)?;
                Ok(scan.with_record(record.identity(), record.eviction_protection_summary()))
            },
        )
    }

    fn deny_stale_eviction_plan<T>(&mut self) -> Result<T, ResidentFrameDenial> {
        self.eviction_counters = self.eviction_counters.with_stale_plan_denial();
        self.publish_eviction_counters();
        Err(ResidentFrameDenial::new(
            ResidentFrameDenialKind::EvictionPlanStale,
        ))
    }

    fn deny_protected_eviction_plan<T>(
        &mut self,
        protection: EvictionProtectionSummary,
    ) -> Result<T, ResidentFrameDenial> {
        self.eviction_counters = self
            .eviction_counters
            .with_protected_exclusion(protection)
            .with_stale_plan_denial();
        self.publish_eviction_counters();
        Err(ResidentFrameDenial::protected_frame(
            ResidentFrameDenialKind::EvictionPlanStale,
            ProtectedFrameDenial::new(protection, self.eviction_counters),
        ))
    }

    pub(crate) fn publish_eviction_counters(&mut self) {
        self.counters = self.counters.with_eviction(self.eviction_counters);
    }
}

#[derive(Debug, Clone, Copy)]
struct EvictionCandidateScan {
    selected_identity: Option<ResidentFrameIdentity>,
    resident_frames_scanned: u64,
    candidate_count: u64,
    protected_exclusions: EvictionProtectionSummary,
    counters: EvictionCounterSnapshot,
}

impl EvictionCandidateScan {
    const fn new(counters: EvictionCounterSnapshot) -> Self {
        Self {
            selected_identity: None,
            resident_frames_scanned: 0,
            candidate_count: 0,
            protected_exclusions: EvictionProtectionSummary::empty(),
            counters,
        }
    }

    fn with_record(
        self,
        identity: ResidentFrameIdentity,
        protection: EvictionProtectionSummary,
    ) -> Self {
        let scanned = self.counters.with_resident_frame_scanned();
        if protection.is_empty() {
            return Self {
                selected_identity: self.selected_identity.or(Some(identity)),
                resident_frames_scanned: self.resident_frames_scanned + 1,
                candidate_count: self.candidate_count + 1,
                protected_exclusions: self.protected_exclusions,
                counters: scanned.with_candidate(),
            };
        }
        Self {
            selected_identity: self.selected_identity,
            resident_frames_scanned: self.resident_frames_scanned + 1,
            candidate_count: self.candidate_count,
            protected_exclusions: self.protected_exclusions.merge(protection),
            counters: scanned.with_protected_exclusion(protection),
        }
    }
}
