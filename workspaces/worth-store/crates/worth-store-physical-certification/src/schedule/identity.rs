use worth_foundational::canonicalization_api::lower_lane::basis::{
    prepare_canonical_basis_sequence, CanonicalBasisDomain, CanonicalBasisEntry,
    CanonicalBasisEntryKind, CanonicalBasisLocus, CanonicalBasisValue, CanonicalizationRuleVersion,
};
use worth_foundational::canonicalization_api::lower_lane::digest::{
    admit_canonical_sequence_digest_derivation, derive_canonical_digest, CanonicalDerivedDigest,
    CanonicalDigestAlgorithmId, CanonicalSingleSequenceDigestAlgorithmSlot,
};
use worth_foundational::InternedString;
use worth_proof::TransitionOutcome;

use crate::{PhysicalSimulationPlan, PhysicalSimulationProfile};

use super::actor_sequence::PhysicalActorStepSequence;
use super::actor_step::actor_role_token;
use super::authority::AdmittedScheduleOrderingAuthority;
use super::budget::{PartialOrderReductionPosture, ScheduleExplorationCost};
use super::{SchedulePerturbationSeed, ScheduleReplayDenial};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScheduleReplayIdentity {
    digest: CanonicalDerivedDigest,
}

pub(crate) struct ScheduleReplayIdentityParts<'a> {
    pub(crate) plan: &'a PhysicalSimulationPlan,
    pub(crate) seed: SchedulePerturbationSeed,
    pub(crate) ordering_authority: AdmittedScheduleOrderingAuthority,
    pub(crate) actor_steps: &'a PhysicalActorStepSequence,
    pub(crate) exploration_cost: ScheduleExplorationCost,
}

impl ScheduleReplayIdentity {
    pub(crate) fn from_parts(
        parts: ScheduleReplayIdentityParts<'_>,
    ) -> Result<Self, ScheduleReplayDenial> {
        let entries = canonical_schedule_entries(parts);
        let version = schedule_canonicalization_version();
        let ready_basis = match prepare_canonical_basis_sequence(version, SCHEDULE_DOMAIN, entries)
        {
            TransitionOutcome::Success(ready) => ready,
            TransitionOutcome::Denied(denial) => {
                return Err(ScheduleReplayDenial::ScheduleCanonicalBasisDenied(denial));
            }
            TransitionOutcome::Deferred(value) => match value {},
            TransitionOutcome::Stale(value) => match value {},
            TransitionOutcome::RebindRequired(value) => match value {},
            TransitionOutcome::Failed(value) => match value {},
        };
        let slot = CanonicalSingleSequenceDigestAlgorithmSlot::single_sequence(
            CanonicalDigestAlgorithmId::test_stable_fixture(),
            ready_basis.payload().domain(),
            ready_basis.payload().version().clone(),
        );
        match admit_canonical_sequence_digest_derivation(ready_basis, slot) {
            TransitionOutcome::Success(ready) => Ok(Self {
                digest: derive_canonical_digest(ready),
            }),
            TransitionOutcome::Denied(denial) => {
                Err(ScheduleReplayDenial::ScheduleDigestDerivationDenied(denial))
            }
            TransitionOutcome::Deferred(value) => match value {},
            TransitionOutcome::Stale(value) => match value {},
            TransitionOutcome::RebindRequired(value) => match value {},
            TransitionOutcome::Failed(value) => match value {},
        }
    }

    pub fn digest_bytes(&self) -> &[u8; 32] {
        self.digest.value().bytes()
    }

    pub fn canonical_basis_entry_count(&self) -> u32 {
        self.digest.metadata().entry_count()
    }
}

const SCHEDULE_DOMAIN: CanonicalBasisDomain =
    CanonicalBasisDomain::Future("store.physical.interleaving.schedule");
const SCHEDULE_FIELD_KIND: CanonicalBasisEntryKind =
    CanonicalBasisEntryKind::Future("store-physical-interleaving-schedule-field");

fn canonical_schedule_entries(parts: ScheduleReplayIdentityParts<'_>) -> Vec<CanonicalBasisEntry> {
    let mut entries = vec![
        text_entry(
            "schedule.scenario.identity",
            hex_digest(parts.plan.scenario_identity().digest_bytes()),
        ),
        text_entry(
            "schedule.plan.identity",
            hex_digest(parts.plan.identity().digest_bytes()),
        ),
        text_entry("schedule.seed", parts.seed.value().to_string()),
        text_entry("schedule.profile", profile_token(parts.plan.profile())),
        text_entry(
            "schedule.ordering_authority",
            parts.ordering_authority.canonical_token(),
        ),
        text_entry(
            "schedule.yieldpoint.scheduled",
            parts.plan.yieldpoint_binding().scheduled_yieldpoint(),
        ),
        text_entry(
            "schedule.yieldpoint.declared",
            parts.plan.yieldpoint_binding().declared_yieldpoint().name(),
        ),
        text_entry(
            "schedule.budget.max_steps",
            parts.exploration_cost.budget().max_steps().to_string(),
        ),
        text_entry(
            "schedule.cost.explored_steps",
            parts.exploration_cost.explored_steps().to_string(),
        ),
        text_entry(
            "schedule.cost.pruned_steps",
            parts.exploration_cost.pruned_steps().to_string(),
        ),
        text_entry(
            "schedule.cost.partial_order_reduction",
            partial_order_reduction_token(parts.exploration_cost.partial_order_reduction()),
        ),
    ];
    entries.extend(
        parts
            .actor_steps
            .as_slice()
            .iter()
            .enumerate()
            .flat_map(|(index, step)| {
                [
                    text_entry(
                        format!("schedule.actor_step.{index:04}.index"),
                        step.step_index().to_string(),
                    ),
                    text_entry(
                        format!("schedule.actor_step.{index:04}.actor_id"),
                        step.actor_id(),
                    ),
                    text_entry(
                        format!("schedule.actor_step.{index:04}.actor_role"),
                        actor_role_token(step.actor_role()),
                    ),
                    text_entry(
                        format!("schedule.actor_step.{index:04}.yieldpoint"),
                        step.yieldpoint(),
                    ),
                ]
            }),
    );
    entries
}

fn text_entry(
    locus: impl Into<InternedString>,
    value: impl Into<InternedString>,
) -> CanonicalBasisEntry {
    CanonicalBasisEntry::new(
        SCHEDULE_DOMAIN,
        CanonicalBasisLocus::Named(locus.into()),
        SCHEDULE_FIELD_KIND,
        CanonicalBasisValue::ExactText(value.into()),
    )
}

fn schedule_canonicalization_version() -> CanonicalizationRuleVersion {
    CanonicalizationRuleVersion::new("store.physical.interleaving.schedule.v1")
        .expect("schedule canonicalization version is static and valid")
}

fn profile_token(profile: PhysicalSimulationProfile) -> &'static str {
    match profile {
        PhysicalSimulationProfile::DeveloperSmoke => "developer-smoke",
        PhysicalSimulationProfile::CiCertification => "ci-certification",
        PhysicalSimulationProfile::LocalSoak => "local-soak",
        PhysicalSimulationProfile::ReleaseCertification => "release-certification",
        PhysicalSimulationProfile::HardwareQualification => "hardware-qualification",
    }
}

fn partial_order_reduction_token(posture: PartialOrderReductionPosture) -> &'static str {
    match posture {
        PartialOrderReductionPosture::NotApplied => "not-applied",
        PartialOrderReductionPosture::AppliedDeterministically => "applied-deterministically",
    }
}

fn hex_digest(bytes: &[u8; 32]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}
