use forge_foundational::canonicalization_api::lower_lane::basis::{
    prepare_canonical_basis_sequence, CanonicalBasisDomain, CanonicalBasisEntry,
    CanonicalBasisEntryKind, CanonicalBasisLocus, CanonicalBasisValue, CanonicalizationRuleVersion,
};
use forge_foundational::canonicalization_api::lower_lane::digest::{
    admit_canonical_sequence_digest_derivation, derive_canonical_digest, CanonicalDerivedDigest,
    CanonicalDigestAlgorithmId, CanonicalSingleSequenceDigestAlgorithmSlot,
};
use forge_foundational::InternedString;
use forge_proof::TransitionOutcome;

use super::capabilities::capability_token;
use super::counter_contracts::{counter_contract_kind_token, counter_expectation_strength_token};
use super::evidence_policy::evidence_policy_token;
use super::forbidden_shortcuts::forbidden_shortcut_token;
use super::plan::PhysicalSimulationPlanParts;
use super::profiles::profile_token;
use super::tokens::{
    actor_role_token, fixture_class_token, observer_token, oracle_family_token,
    physical_driver_token,
};
use super::SimulationPlanDenial;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalSimulationPlanIdentity {
    digest: CanonicalDerivedDigest,
}

impl PhysicalSimulationPlanIdentity {
    pub(crate) fn from_parts(
        parts: &PhysicalSimulationPlanParts,
    ) -> Result<Self, SimulationPlanDenial> {
        let entries = canonical_plan_entries(parts);
        let version = plan_canonicalization_version();
        let ready_basis = match prepare_canonical_basis_sequence(version, PLAN_DOMAIN, entries) {
            TransitionOutcome::Success(ready) => ready,
            TransitionOutcome::Denied(denial) => {
                return Err(SimulationPlanDenial::PlanCanonicalBasisDenied(denial));
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
                Err(SimulationPlanDenial::PlanDigestDerivationDenied(denial))
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

const PLAN_DOMAIN: CanonicalBasisDomain =
    CanonicalBasisDomain::Future("store.physical.simulation.plan");
const PLAN_FIELD_KIND: CanonicalBasisEntryKind =
    CanonicalBasisEntryKind::Future("store-physical-simulation-plan-field");

fn canonical_plan_entries(parts: &PhysicalSimulationPlanParts) -> Vec<CanonicalBasisEntry> {
    let mut entries = vec![
        text_entry(
            "plan.scenario.identity",
            hex_digest(parts.scenario_identity.digest_bytes()),
        ),
        text_entry("plan.profile", profile_token(parts.profile)),
        text_entry(
            "plan.resource_envelope.profile",
            profile_token(parts.resource_envelope.profile()),
        ),
        text_entry(
            "plan.resource_envelope.resident_bytes",
            parts
                .resource_envelope
                .resident_bytes()
                .as_bytes()
                .to_string(),
        ),
        text_entry(
            "plan.resource_envelope.max_pinned_pages",
            parts.resource_envelope.max_pinned_pages().to_string(),
        ),
        text_entry(
            "plan.resource_envelope.max_dirty_pages",
            parts.resource_envelope.max_dirty_pages().to_string(),
        ),
        text_entry(
            "plan.resource_envelope.io_queue_depth",
            parts
                .resource_envelope
                .io_queue()
                .max_queue_depth()
                .to_string(),
        ),
        text_entry(
            "plan.evidence_policy",
            evidence_policy_token(parts.evidence_policy),
        ),
    ];
    entries.extend(
        parts
            .required_capabilities
            .iter()
            .enumerate()
            .map(|(index, item)| {
                text_entry(
                    format!("plan.capability.{index:04}"),
                    capability_token(item),
                )
            }),
    );
    entries.extend(parts.actors.iter().enumerate().flat_map(|(index, actor)| {
        [
            text_entry(format!("plan.actor.{index:04}.id"), actor.id()),
            text_entry(
                format!("plan.actor.{index:04}.role"),
                actor_role_token(actor.role()),
            ),
        ]
    }));
    entries.extend(parts.drivers.iter().enumerate().map(|(index, item)| {
        text_entry(
            format!("plan.driver.{index:04}"),
            physical_driver_token(item),
        )
    }));
    entries.extend(
        parts
            .driver_contracts
            .iter()
            .enumerate()
            .flat_map(|(index, item)| {
                [
                    text_entry(
                        format!("plan.driver_contract.{index:04}.kind"),
                        physical_driver_token(item.kind()),
                    ),
                    text_entry(
                        format!("plan.driver_contract.{index:04}.boundary"),
                        item.profile().boundary().token(),
                    ),
                ]
            }),
    );
    entries.push(text_entry(
        "plan.yieldpoint_binding.scheduled",
        parts.yieldpoint_binding.scheduled_yieldpoint(),
    ));
    entries.push(text_entry(
        "plan.yieldpoint_binding.declared",
        parts.yieldpoint_binding.declared_yieldpoint().name(),
    ));
    entries.extend(parts.observers.iter().enumerate().map(|(index, item)| {
        text_entry(format!("plan.observer.{index:04}"), observer_token(item))
    }));
    entries.extend(
        parts
            .oracle_families
            .iter()
            .enumerate()
            .map(|(index, item)| {
                text_entry(format!("plan.oracle.{index:04}"), oracle_family_token(item))
            }),
    );
    entries.extend(
        parts
            .counter_contracts
            .iter()
            .enumerate()
            .flat_map(|(index, item)| {
                [
                    text_entry(
                        format!("plan.counter.{index:04}.kind"),
                        counter_contract_kind_token(item.kind()),
                    ),
                    text_entry(
                        format!("plan.counter.{index:04}.strength"),
                        counter_expectation_strength_token(item.strength()),
                    ),
                    text_entry(
                        format!("plan.counter.{index:04}.expectation"),
                        item.expectation()
                            .value()
                            .map(|value| value.to_string())
                            .unwrap_or_else(|| "none".to_owned()),
                    ),
                ]
            }),
    );
    entries.extend(
        parts
            .fixture_classes
            .iter()
            .enumerate()
            .map(|(index, item)| {
                text_entry(
                    format!("plan.fixture_class.{index:04}"),
                    fixture_class_token(item),
                )
            }),
    );
    entries.extend(
        parts
            .forbidden_shortcuts
            .iter()
            .enumerate()
            .map(|(index, item)| {
                text_entry(
                    format!("plan.forbidden_shortcut.{index:04}"),
                    forbidden_shortcut_token(item),
                )
            }),
    );
    entries
}

fn text_entry(
    locus: impl Into<InternedString>,
    value: impl Into<InternedString>,
) -> CanonicalBasisEntry {
    CanonicalBasisEntry::new(
        PLAN_DOMAIN,
        CanonicalBasisLocus::Named(locus.into()),
        PLAN_FIELD_KIND,
        CanonicalBasisValue::ExactText(value.into()),
    )
}

fn plan_canonicalization_version() -> CanonicalizationRuleVersion {
    CanonicalizationRuleVersion::new("store.physical.simulation.plan.v1")
        .expect("plan canonicalization version is static and valid")
}

fn hex_digest(bytes: &[u8; 32]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}
