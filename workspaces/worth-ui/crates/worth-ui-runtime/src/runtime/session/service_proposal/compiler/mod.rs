#[cfg(test)]
mod conflict_tests;
#[cfg(test)]
mod coordination_tests;
mod dependency;
#[cfg(test)]
mod evidence;
mod family_proposal;
mod preflight;
mod proposal;
mod receipt;
mod reservation;
mod settlement;
mod settlement_compiler;
#[cfg(test)]
mod shutdown_tests;
mod staged_reference;
mod staging;
mod terminal;

#[cfg(test)]
mod lifecycle {
    #[path = "tests.rs"]
    mod tests;
}

pub(in crate::runtime) use dependency::{UiServiceProposalDependencyEdge, UiServiceProposalStage};
pub(in crate::runtime) use family_proposal::UiServiceFamilyProposal;
pub(in crate::runtime) use preflight::{
    UiPreflightedServiceProposal, UiServiceProposalPreflightDenial,
};
pub(in crate::runtime) use proposal::{
    UiServiceProposalCandidate, UiServiceProposalDemand, UiServiceProposalDemandConstructionDenial,
    UiServiceProposalIdentity,
};
#[cfg(test)]
pub(in crate::runtime) use receipt::{
    UiRecordedServiceProposalOwnerPort, UiRecordedServiceProposalPublicationPort,
};
pub(in crate::runtime) use receipt::{
    UiServiceProposalOwnerAcknowledgement, UiServiceProposalPublicationDisposition,
    UiServiceProposalPublicationReceipt, UiServiceProposalTerminalOwnerOutcome,
};
pub(in crate::runtime) use reservation::{
    UiReservedServiceProposal, UiServiceProposalBeforeEffectCancellationReceipt,
    UiServiceProposalReservationDenial, UiServiceProposalReservationOutcome,
};
pub(in crate::runtime) use settlement::{
    UiServiceProposalPublicationDenial, UiServiceProposalSettlement,
    UiServiceProposalSettlementDenial,
};
pub(in crate::runtime) use staged_reference::{
    UiServiceMountedWorkReference, UiServiceProducedFactReference,
};
pub(in crate::runtime) use staging::{
    UiServiceProposalStageIssuer, UiServiceProposalStageReceipt, UiServiceProposalStagedBatch,
    UiServiceProposalStaging, UiServiceProposalStagingDenial,
};
pub(in crate::runtime) use terminal::{
    UiServiceProposalCompilerShutdownReceipt, UiServiceProposalTeardown,
    UiServiceProposalTeardownDenial, UiServiceProposalTerminalReason,
    UiServiceProposalTerminalReceipt,
};

#[derive(Debug)]
pub(in crate::runtime) struct UiServiceProposalCompiler {
    census: super::UiServiceProposalCensus,
    occupancy: super::occupancy::UiServiceProposalOccupancyTable,
    cancellations: super::cancellation::UiServiceProposalCancellationRegistry,
}

impl UiServiceProposalCompiler {
    pub(in crate::runtime) fn new() -> Self {
        Self {
            census: super::UiServiceProposalCensus::zero(),
            occupancy: super::occupancy::UiServiceProposalOccupancyTable::new(),
            cancellations: super::cancellation::UiServiceProposalCancellationRegistry::new(),
        }
    }

    pub(in crate::runtime) fn preflight(
        &mut self,
        candidate: UiServiceProposalCandidate,
        current: &super::UiServiceRequestCoherence,
        support: crate::capability::UiRuntimeServiceSupport,
    ) -> Result<UiPreflightedServiceProposal, UiServiceProposalPreflightDenial> {
        preflight::preflight(candidate, current, support)
    }

    pub(in crate::runtime) const fn census(&self) -> super::UiServiceProposalCensus {
        self.census
    }

    pub(in crate::runtime) fn reserve(
        &mut self,
        preflighted: UiPreflightedServiceProposal,
    ) -> Result<UiServiceProposalReservationOutcome, UiServiceProposalReservationDenial> {
        let plan = self
            .occupancy
            .plan(preflighted.candidate())
            .map_err(UiServiceProposalReservationDenial::Occupancy)?;
        if let Some(incumbent) = super::occupancy::UiServiceProposalOccupancyTable::coalesced(&plan)
        {
            return Ok(UiServiceProposalReservationOutcome::Coalesced { incumbent });
        }
        let displaced = plan.displacement();
        self.cancellations
            .can_reserve(
                preflighted.candidate().identity(),
                displaced.map(super::UiServiceProposalDisplacement::proposal),
            )
            .map_err(UiServiceProposalReservationDenial::Cancellation)?;
        let next_census = self
            .census
            .with_reservation(
                preflighted.candidate().family_proposals().len() as u16,
                displaced.map_or(0, super::UiServiceProposalDisplacement::released_leases),
                displaced.is_some(),
            )
            .map_err(UiServiceProposalReservationDenial::Census)?;
        let candidate = preflighted.into_candidate();
        let proposal = candidate.identity();
        let cancellation = candidate.cancellation();
        let (leases, displacement) = self.occupancy.commit(&candidate, plan);
        self.cancellations.reserve(
            proposal,
            cancellation,
            displacement.map(super::UiServiceProposalDisplacement::proposal),
        );
        self.census = next_census;
        Ok(UiServiceProposalReservationOutcome::Reserved(
            UiReservedServiceProposal::from_parts(candidate, leases, displacement),
        ))
    }

    pub(in crate::runtime) fn cancel_before_effect(
        &mut self,
        reservation: UiReservedServiceProposal,
    ) -> Result<UiServiceProposalBeforeEffectCancellationReceipt, UiServiceProposalReservationDenial>
    {
        let proposal = reservation.identity();
        self.occupancy
            .can_release(proposal, reservation.leases())
            .map_err(UiServiceProposalReservationDenial::Occupancy)?;
        self.cancellations
            .can_release(proposal, reservation.candidate().cancellation())
            .map_err(UiServiceProposalReservationDenial::Cancellation)?;
        let next_census = self
            .census
            .with_terminal_release(reservation.leases().len() as u16)
            .map_err(UiServiceProposalReservationDenial::Census)?;
        let released_leases = self.occupancy.release(proposal, reservation.leases());
        self.cancellations.release(proposal);
        self.census = next_census;
        Ok(UiServiceProposalBeforeEffectCancellationReceipt::new(
            proposal,
            released_leases,
        ))
    }

    pub(in crate::runtime) fn begin_staging(
        &mut self,
        reservation: UiReservedServiceProposal,
    ) -> Result<UiServiceProposalStaging, UiServiceProposalStagingDenial> {
        self.occupancy
            .can_release(reservation.identity(), reservation.leases())
            .map_err(UiServiceProposalStagingDenial::Occupancy)?;
        Ok(UiServiceProposalStaging::new(reservation))
    }

    pub(in crate::runtime) fn advance_staging(
        &mut self,
        staging: &mut UiServiceProposalStaging,
        receipt: UiServiceProposalStageReceipt,
    ) -> Result<(), UiServiceProposalStagingDenial> {
        let closes_before_effect = staging.is_before_first_effect();
        if closes_before_effect {
            self.occupancy
                .can_release(staging.identity(), staging.leases())
                .map_err(UiServiceProposalStagingDenial::Occupancy)?;
        }
        let mut next_census = self.census;
        next_census
            .record_stage_receipt()
            .map_err(UiServiceProposalStagingDenial::Census)?;
        staging.accept_stage_receipt(receipt)?;
        if closes_before_effect {
            self.occupancy
                .close_before_effect_window(staging.identity(), staging.leases())
                .expect("first effect follows exact occupancy prevalidation");
        }
        self.census = next_census;
        Ok(())
    }

    pub(in crate::runtime) fn finish_staging(
        &self,
        staging: UiServiceProposalStaging,
    ) -> Result<
        UiServiceProposalStagedBatch,
        (UiServiceProposalStaging, UiServiceProposalStagingDenial),
    > {
        staging.finish()
    }

    pub(in crate::runtime) fn live_occupancy_count(&self) -> usize {
        self.occupancy.live_count()
    }

    pub(in crate::runtime) fn live_cancellation_count(&self) -> usize {
        self.cancellations.live_count()
    }

    pub(in crate::runtime) const fn occupancy_work_counters(
        &self,
    ) -> super::UiServiceProposalOccupancyWorkCounters {
        self.occupancy.work_counters()
    }
}

#[cfg(test)]
mod tests {
    use super::{
        UiReservedServiceProposal, UiServiceProposalCompiler, UiServiceProposalReservationOutcome,
        UiServiceProposalStage,
    };

    #[test]
    fn compiler_begins_empty_and_exposes_fixed_semantic_stages() {
        let compiler = UiServiceProposalCompiler::new();
        assert!(compiler.census().is_zero());
        assert_eq!(UiServiceProposalStage::ORDER.len(), 7);
    }

    #[test]
    fn reservation_and_before_effect_cancellation_are_census_atomic() {
        let mut compiler = UiServiceProposalCompiler::new();
        let coherence = super::super::fixture_service_request_coherence(11);
        let reserved = reserve(
            &mut compiler,
            &coherence,
            11,
            super::super::UiServiceProposalConflictPolicy::RejectOccupied,
        );
        assert_eq!(
            compiler.census().entries(),
            [
                ("proposals", 1),
                ("occupancy_leases", 1),
                ("cancellation_records", 1),
                ("stage_receipts", 0),
            ]
        );
        assert_eq!(compiler.live_occupancy_count(), 1);
        assert_eq!(compiler.live_cancellation_count(), 1);

        let receipt = compiler.cancel_before_effect(reserved).unwrap();
        assert_eq!(receipt.released_leases(), 1);
        assert!(compiler.census().is_zero());
        assert_eq!(compiler.live_occupancy_count(), 0);
        assert_eq!(compiler.live_cancellation_count(), 0);
    }

    #[test]
    fn occupied_denial_changes_no_resource_and_supersession_is_aba_safe() {
        let mut compiler = UiServiceProposalCompiler::new();
        let coherence = super::super::fixture_service_request_coherence(12);
        let incumbent = reserve(
            &mut compiler,
            &coherence,
            12,
            super::super::UiServiceProposalConflictPolicy::RejectOccupied,
        );
        let before = compiler.census();
        let occupied = preflight(
            &mut compiler,
            &coherence,
            13,
            super::super::UiServiceProposalConflictPolicy::RejectOccupied,
        );
        assert!(matches!(
            compiler.reserve(occupied),
            Err(super::UiServiceProposalReservationDenial::Occupancy(
                super::super::UiServiceProposalOccupancyDenial::Occupied(_)
            ))
        ));
        assert_eq!(compiler.census(), before);

        let successor = reserve(
            &mut compiler,
            &coherence,
            14,
            super::super::UiServiceProposalConflictPolicy::SupersedeBeforeEffect,
        );
        assert_eq!(
            successor.displacement().unwrap().disposition(),
            super::super::UiServiceProposalConflictDisposition::Superseded
        );
        assert!(successor.leases()[0].slot_generation() > incumbent.leases()[0].slot_generation());
        assert_eq!(compiler.census(), before);
        assert!(compiler.cancel_before_effect(incumbent).is_err());
        assert_eq!(compiler.census(), before);
        compiler.cancel_before_effect(successor).unwrap();
        assert!(compiler.census().is_zero());
    }

    #[test]
    fn exact_coalescing_reuses_only_an_equivalent_complete_occupancy_set() {
        let mut compiler = UiServiceProposalCompiler::new();
        let coherence = super::super::fixture_service_request_coherence(15);
        let incumbent = reserve(
            &mut compiler,
            &coherence,
            15,
            super::super::UiServiceProposalConflictPolicy::RejectOccupied,
        );
        let before = compiler.census();
        let coalescing = preflight(
            &mut compiler,
            &coherence,
            16,
            super::super::UiServiceProposalConflictPolicy::CoalesceExact,
        );
        assert!(matches!(
            compiler.reserve(coalescing),
            Ok(UiServiceProposalReservationOutcome::Coalesced { incumbent: found })
                if found == incumbent.identity()
        ));
        assert_eq!(compiler.census(), before);
        compiler.cancel_before_effect(incumbent).unwrap();
        assert!(compiler.census().is_zero());
    }

    pub(super) fn reserve(
        compiler: &mut UiServiceProposalCompiler,
        coherence: &super::super::UiServiceRequestCoherence,
        identity: u64,
        policy: super::super::UiServiceProposalConflictPolicy,
    ) -> UiReservedServiceProposal {
        let preflighted = preflight(compiler, coherence, identity, policy);
        match compiler.reserve(preflighted).unwrap() {
            UiServiceProposalReservationOutcome::Reserved(reserved) => reserved,
            UiServiceProposalReservationOutcome::Coalesced { .. } => {
                panic!("fixture expected a new reservation")
            }
        }
    }

    fn preflight(
        compiler: &mut UiServiceProposalCompiler,
        coherence: &super::super::UiServiceRequestCoherence,
        identity: u64,
        policy: super::super::UiServiceProposalConflictPolicy,
    ) -> super::UiPreflightedServiceProposal {
        let participation = super::super::fixture_service_family_participation(1);
        let family = super::UiServiceFamilyProposal::recorded_fixture(
            crate::capability::UiRuntimeServiceFamily::Portal,
            1,
            1,
            1,
            1,
        )
        .with_conflict_policy(policy);
        let candidate = super::UiServiceProposalCandidate::for_test(
            identity,
            super::UiServiceProposalDemand::recorded_fixture(participation, 1, 1, 1),
            coherence.clone(),
            vec![family],
        );
        compiler
            .preflight(
                candidate,
                coherence,
                crate::capability::UiRuntimeServiceSupport::none_installed()
                    .with_installed(crate::capability::UiRuntimeServiceFamily::Portal),
            )
            .unwrap()
    }
}
