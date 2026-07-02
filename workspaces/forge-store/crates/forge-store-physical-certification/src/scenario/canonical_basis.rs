use std::collections::BTreeSet;

use forge_foundational::canonicalization_api::lower_lane::basis::{
    prepare_canonical_basis_sequence, CanonicalBasisDomain, CanonicalBasisEntry,
    CanonicalBasisEntryKind, CanonicalBasisLocus, CanonicalBasisReadyArtifact, CanonicalBasisValue,
    CanonicalizationRuleVersion,
};
use forge_foundational::canonicalization_api::lower_lane::digest::{
    admit_canonical_sequence_digest_derivation, derive_canonical_digest,
    CanonicalDigestAlgorithmId, CanonicalSingleSequenceDigestAlgorithmSlot,
};
use forge_foundational::InternedString;
use forge_proof::TransitionOutcome;
use forge_store_aspect_native::{
    StoreAspectBoundaryFact, StoreCanonicalBasisConstruction, StoreCanonicalBasisFamily,
};

use super::definition::PhysicalSimulationScenarioDefinition;
use super::denial::PhysicalScenarioDefinitionDenial;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct PhysicalScenarioCanonicalBasis {
    ready: CanonicalBasisReadyArtifact,
}

impl PhysicalScenarioCanonicalBasis {
    pub(crate) fn from_definition(
        definition: &PhysicalSimulationScenarioDefinition,
    ) -> Result<Self, PhysicalScenarioDefinitionDenial> {
        let mut entries = vec![
            text_entry("scenario.family", family_token(definition.family())),
            text_entry("scenario.intent", intent_token(definition.intent())),
            text_entry(
                "scenario.schedule.yieldpoint",
                definition.schedule().production_boundary_yieldpoint(),
            ),
            text_entry("scenario.fault", fault_token(definition.fault().kind())),
            text_entry(
                "scenario.expectation",
                expectation_token(definition.expectation().kind()),
            ),
        ];
        entries.extend(canonical_fixture_entries(definition.fixtures())?);
        entries.extend(canonical_actor_entries(definition));
        entries.extend(canonical_non_claim_entries(definition));
        let version = scenario_canonicalization_version();
        match prepare_canonical_basis_sequence(version, SCENARIO_DOMAIN, entries) {
            TransitionOutcome::Success(ready) => Ok(Self { ready }),
            TransitionOutcome::Denied(denial) => {
                Err(PhysicalScenarioDefinitionDenial::ScenarioCanonicalBasisDenied(denial))
            }
            TransitionOutcome::Deferred(value) => match value {},
            TransitionOutcome::Stale(value) => match value {},
            TransitionOutcome::RebindRequired(value) => match value {},
            TransitionOutcome::Failed(value) => match value {},
        }
    }

    pub(crate) fn ready(self) -> CanonicalBasisReadyArtifact {
        self.ready
    }
}

const SCENARIO_DOMAIN: CanonicalBasisDomain =
    CanonicalBasisDomain::Future("store.physical.simulation.scenario");
const SCENARIO_FIELD_KIND: CanonicalBasisEntryKind =
    CanonicalBasisEntryKind::Future("store-physical-scenario-field");
const SCENARIO_FIXTURE_KIND: CanonicalBasisEntryKind =
    CanonicalBasisEntryKind::Future("store-physical-scenario-native-fixture");

fn canonical_fixture_entries(
    fixtures: &[StoreAspectBoundaryFact],
) -> Result<Vec<CanonicalBasisEntry>, PhysicalScenarioDefinitionDenial> {
    let mut entries = Vec::new();
    for (fixture_index, fixture) in fixtures.iter().enumerate() {
        let ready = prepare_fixture_basis(fixture)?;
        for (entry_index, entry) in ready.payload().entries().iter().enumerate() {
            entries.push(CanonicalBasisEntry::new(
                SCENARIO_DOMAIN,
                named_locus(format!(
                    "fixture.{fixture_index:04}.entry.{entry_index:04}.locus"
                )),
                SCENARIO_FIXTURE_KIND,
                CanonicalBasisValue::ExactText(locus_token(entry.locus()).into()),
            ));
            entries.push(CanonicalBasisEntry::new(
                SCENARIO_DOMAIN,
                named_locus(format!(
                    "fixture.{fixture_index:04}.entry.{entry_index:04}.value"
                )),
                entry.kind(),
                entry.value().clone(),
            ));
        }
    }
    Ok(entries)
}

pub(crate) fn fixture_canonical_sort_key(
    fixture: &StoreAspectBoundaryFact,
) -> Result<[u8; 32], PhysicalScenarioDefinitionDenial> {
    let ready = prepare_fixture_basis(fixture)?;
    let slot = CanonicalSingleSequenceDigestAlgorithmSlot::single_sequence(
        CanonicalDigestAlgorithmId::test_stable_fixture(),
        ready.payload().domain(),
        ready.payload().version().clone(),
    );
    match admit_canonical_sequence_digest_derivation(ready, slot) {
        TransitionOutcome::Success(ready) => Ok(*derive_canonical_digest(ready).value().bytes()),
        TransitionOutcome::Denied(denial) => {
            Err(PhysicalScenarioDefinitionDenial::ScenarioDigestDerivationDenied(denial))
        }
        TransitionOutcome::Deferred(value) => match value {},
        TransitionOutcome::Stale(value) => match value {},
        TransitionOutcome::RebindRequired(value) => match value {},
        TransitionOutcome::Failed(value) => match value {},
    }
}

fn prepare_fixture_basis(
    fixture: &StoreAspectBoundaryFact,
) -> Result<CanonicalBasisReadyArtifact, PhysicalScenarioDefinitionDenial> {
    let version = scenario_canonicalization_version();
    match StoreCanonicalBasisConstruction::for_family(StoreCanonicalBasisFamily::AspectBoundaryFact)
        .with_aspect_boundary_fact(fixture)
        .prepare(version)
    {
        TransitionOutcome::Success(ready) => Ok(ready),
        TransitionOutcome::Denied(denial) => {
            Err(PhysicalScenarioDefinitionDenial::FixtureCanonicalBasisDenied(denial))
        }
        TransitionOutcome::Deferred(value) => match value {},
        TransitionOutcome::Stale(value) => match value {},
        TransitionOutcome::RebindRequired(value) => match value {},
        TransitionOutcome::Failed(value) => match value {},
    }
}

fn canonical_actor_entries(
    definition: &PhysicalSimulationScenarioDefinition,
) -> Vec<CanonicalBasisEntry> {
    definition
        .actors()
        .iter()
        .enumerate()
        .flat_map(|(index, actor)| {
            [
                text_entry(
                    format!("actor.{index:04}.role"),
                    actor_role_token(actor.role()),
                ),
                text_entry(format!("actor.{index:04}.id"), actor.id()),
            ]
        })
        .collect()
}

fn canonical_non_claim_entries(
    definition: &PhysicalSimulationScenarioDefinition,
) -> Vec<CanonicalBasisEntry> {
    definition
        .expectation()
        .non_claims()
        .iter()
        .copied()
        .collect::<BTreeSet<_>>()
        .into_iter()
        .enumerate()
        .map(|(index, non_claim)| {
            text_entry(format!("non_claim.{index:04}"), non_claim_token(non_claim))
        })
        .collect()
}

fn text_entry(
    locus: impl Into<InternedString>,
    value: impl Into<InternedString>,
) -> CanonicalBasisEntry {
    CanonicalBasisEntry::new(
        SCENARIO_DOMAIN,
        CanonicalBasisLocus::Named(locus.into()),
        SCENARIO_FIELD_KIND,
        CanonicalBasisValue::ExactText(value.into()),
    )
}

fn named_locus(locus: impl Into<InternedString>) -> CanonicalBasisLocus {
    CanonicalBasisLocus::Named(locus.into())
}

fn scenario_canonicalization_version() -> CanonicalizationRuleVersion {
    CanonicalizationRuleVersion::new("store.physical.scenario.v1")
        .expect("scenario canonicalization version is static and valid")
}

fn locus_token(locus: &CanonicalBasisLocus) -> String {
    match locus {
        CanonicalBasisLocus::Root => "root".to_owned(),
        CanonicalBasisLocus::EntryOrdinal(ordinal) => format!("ordinal.{ordinal}"),
        CanonicalBasisLocus::Aspect(aspect) => format!("aspect.{}", aspect.as_str()),
        CanonicalBasisLocus::AspectField { aspect, path } => format!(
            "aspect-field.{}.{}",
            aspect.as_str(),
            path.fields()
                .iter()
                .map(|field| field.as_str())
                .collect::<Vec<_>>()
                .join(".")
        ),
        CanonicalBasisLocus::Named(name) => interned_token(name),
    }
}

fn interned_token(value: &InternedString) -> String {
    match value {
        InternedString::Raw(raw) => raw.clone(),
        InternedString::Symbol(symbol) => format!("symbol.{}", symbol.0),
    }
}

fn family_token(family: super::vocabulary::PhysicalSimulationScenarioFamily) -> &'static str {
    match family {
        super::vocabulary::PhysicalSimulationScenarioFamily::S4RecoveryDogfood => {
            "s4-recovery-dogfood"
        }
        super::vocabulary::PhysicalSimulationScenarioFamily::S5ReadinessShapeProbe => {
            "s5-readiness-shape-probe"
        }
        super::vocabulary::PhysicalSimulationScenarioFamily::ShortcutRejectionDogfood => {
            "shortcut-rejection-dogfood"
        }
        super::vocabulary::PhysicalSimulationScenarioFamily::FutureExtensionSlot => {
            "future-extension-slot"
        }
    }
}

fn intent_token(intent: super::vocabulary::PhysicalScenarioIntent) -> &'static str {
    match intent {
        super::vocabulary::PhysicalScenarioIntent::RecoveryReplayDogfood => {
            "recovery-replay-dogfood"
        }
        super::vocabulary::PhysicalScenarioIntent::ProtectBeforeObserveShape => {
            "protect-before-observe-shape"
        }
        super::vocabulary::PhysicalScenarioIntent::ForbiddenShortcutRejectionShape => {
            "forbidden-shortcut-rejection-shape"
        }
        super::vocabulary::PhysicalScenarioIntent::FutureExtensionSlot => "future-extension-slot",
    }
}

fn actor_role_token(role: super::vocabulary::PhysicalScenarioActorRole) -> &'static str {
    match role {
        super::vocabulary::PhysicalScenarioActorRole::ForegroundReader => "foreground-reader",
        super::vocabulary::PhysicalScenarioActorRole::ForegroundWriter => "foreground-writer",
        super::vocabulary::PhysicalScenarioActorRole::CheckpointDriver => "checkpoint-driver",
        super::vocabulary::PhysicalScenarioActorRole::CompactionDriver => "compaction-driver",
        super::vocabulary::PhysicalScenarioActorRole::MaintenanceReclaimer => {
            "maintenance-reclaimer"
        }
        super::vocabulary::PhysicalScenarioActorRole::RecoveryDriver => "recovery-driver",
        super::vocabulary::PhysicalScenarioActorRole::ScrubDriver => "scrub-driver",
        super::vocabulary::PhysicalScenarioActorRole::OfflineVerifier => "offline-verifier",
        super::vocabulary::PhysicalScenarioActorRole::ShortcutRejectionProbe => {
            "shortcut-rejection-probe"
        }
        super::vocabulary::PhysicalScenarioActorRole::FutureExtensionSlot => {
            "future-extension-slot"
        }
    }
}

fn fault_token(fault: super::vocabulary::PhysicalScenarioFaultKind) -> &'static str {
    match fault {
        super::vocabulary::PhysicalScenarioFaultKind::NoFault => "no-fault",
        super::vocabulary::PhysicalScenarioFaultKind::FutureExtensionSlot => {
            "future-extension-slot"
        }
    }
}

fn expectation_token(
    expectation: super::vocabulary::PhysicalScenarioExpectationKind,
) -> &'static str {
    match expectation {
        super::vocabulary::PhysicalScenarioExpectationKind::S4RecoveryDogfood => {
            "s4-recovery-dogfood"
        }
        super::vocabulary::PhysicalScenarioExpectationKind::S5ReadinessShapeProbe => {
            "s5-readiness-shape-probe"
        }
        super::vocabulary::PhysicalScenarioExpectationKind::S5ReadinessWithShortcutRejectionProbe => {
            "s5-readiness-with-shortcut-rejection-probe"
        }
        super::vocabulary::PhysicalScenarioExpectationKind::ShortcutRejectionDogfood => {
            "shortcut-rejection-dogfood"
        }
        super::vocabulary::PhysicalScenarioExpectationKind::FutureExtensionSlot => {
            "future-extension-slot"
        }
    }
}

fn non_claim_token(non_claim: super::vocabulary::PhysicalScenarioNonClaim) -> &'static str {
    match non_claim {
        super::vocabulary::PhysicalScenarioNonClaim::NoS5PhysicalIsolationCorrectnessClaim => {
            "no-s5-physical-isolation-correctness-claim"
        }
        super::vocabulary::PhysicalScenarioNonClaim::FutureExtensionSlotDoesNotImplementFutureBehavior => {
            "future-extension-slot-does-not-implement-future-behavior"
        }
    }
}
