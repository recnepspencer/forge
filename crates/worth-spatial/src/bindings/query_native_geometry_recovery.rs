use forge_query::facade::{
    ForgeQueryAdmittedConfiguredDomainHandle, ForgeQueryDeclarationCanonicalEntry,
    ForgeQueryDeclarationInput, ForgeQueryDomainOperatingContext, ForgeQueryOrdinaryNextStep,
    ForgeQueryOrdinaryOutcome, ForgeQueryOrdinaryPostureKind,
};
use worth_primitives::{truth_digest_parts, TruthDigestScope};

use crate::bindings::query_native_rebinding::PrimitiveRebindingQueryDomain;
use crate::bindings::query_native_retained_geometry::{
    retained_source_digest, GeometryRecoveryActionDeclarationFamily,
};
use crate::bindings::rebinding::{
    NeighborhoodBindingFamily, PrimitiveRebindingRetainedFactSource, RebindingOutcomeClass,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GeometryRecoveryAction {
    CheckSupport,
    CorrectHandle,
    CorrectWorld,
    EscalateFailure,
    GatherAvailability,
    InspectCheckedLane,
    InspectProofLane,
    NarrowInput,
    RebindContext,
    RefreshBasis,
    RepairDeclarationMeaning,
    RetryLater,
    ReviewContributionIntent,
    UseExplicitHandoff,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GeometryRecoverySourcePosture {
    Ambiguous,
    AspectConflict,
    AuthorityMismatch,
    BasisMismatch,
    ContributionDenied,
    DeclarationDenied,
    Deferred,
    Failed,
    MissingRequiredAspect,
    RebindRequired,
    Refused,
    Stale,
    Unavailable,
    Unsupported,
    WrongHandle,
    WrongWorld,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GeometryRecoveryTargetScope {
    AvailabilityDiscovery,
    ContributionIntent,
    DeclarationMeaning,
    FailureEscalation,
    HandleIdentity,
    InputNarrowing,
    InspectionLane,
    SupportReadiness,
    TruthContinuationContext,
    UseExplicitHandoff,
    WorldIdentity,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GeometryRecoveryActionFactReceipt {
    recovery_action_kind: GeometryRecoveryAction,
    source_posture: GeometryRecoverySourcePosture,
    source_family: NeighborhoodBindingFamily,
    recovery_target_scope: GeometryRecoveryTargetScope,
    fact_digest: String,
    resulting_binding_identity: Option<String>,
    resulting_target_identity: Option<String>,
}

impl GeometryRecoveryActionFactReceipt {
    pub fn recovery_action_kind(&self) -> GeometryRecoveryAction {
        self.recovery_action_kind
    }

    pub fn source_posture(&self) -> GeometryRecoverySourcePosture {
        self.source_posture
    }

    pub fn source_family(&self) -> NeighborhoodBindingFamily {
        self.source_family
    }

    pub fn recovery_target_scope(&self) -> GeometryRecoveryTargetScope {
        self.recovery_target_scope
    }

    pub fn fact_digest(&self) -> &str {
        &self.fact_digest
    }

    pub fn resulting_binding_identity(&self) -> Option<&str> {
        self.resulting_binding_identity.as_deref()
    }

    pub fn resulting_target_identity(&self) -> Option<&str> {
        self.resulting_target_identity.as_deref()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct GeometryRecoveryActionEntry {
    source: PrimitiveRebindingRetainedFactSource,
    seed: Option<GeometryRecoveryActionSeed>,
}

impl GeometryRecoveryActionEntry {
    pub fn source(&self) -> &PrimitiveRebindingRetainedFactSource {
        &self.source
    }

    fn seed(&self) -> Option<&GeometryRecoveryActionSeed> {
        self.seed.as_ref()
    }
}

impl ForgeQueryDeclarationInput<PrimitiveRebindingQueryDomain> for GeometryRecoveryActionEntry {
    type Family = GeometryRecoveryActionDeclarationFamily;

    fn canonical_declaration_entries(&self) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
        let entries = vec![
            ForgeQueryDeclarationCanonicalEntry::text(
                "geometry.recovery.kind",
                "geometry_recovery_action",
            ),
            ForgeQueryDeclarationCanonicalEntry::text(
                "geometry.recovery.source_family",
                self.source
                    .receipt()
                    .neighborhood_family()
                    .rebinding_kind_label(),
            ),
            ForgeQueryDeclarationCanonicalEntry::text(
                "geometry.recovery.source_receipt_digest",
                retained_source_digest(&self.source),
            ),
        ];
        entries
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum GeometryRecoveryActionError {
    EntryOutcomeNotBound {
        kind: ForgeQueryOrdinaryPostureKind,
        reason: String,
        next_step: ForgeQueryOrdinaryNextStep,
    },
    RecoveryNotAvailable {
        reason: &'static str,
    },
}

pub fn geometry_recovery_action_entry(
    source: PrimitiveRebindingRetainedFactSource,
) -> GeometryRecoveryActionEntry {
    let seed = geometry_recovery_seed_from_source(&source);
    GeometryRecoveryActionEntry { source, seed }
}

pub fn primitive_rebinding_geometry_recovery_action<C>(
    entry: &GeometryRecoveryActionEntry,
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<PrimitiveRebindingQueryDomain, C>,
) -> Result<GeometryRecoveryActionFactReceipt, GeometryRecoveryActionError>
where
    C: ForgeQueryDomainOperatingContext<PrimitiveRebindingQueryDomain>,
{
    let entry_envelope = match handle.orchestrate_declaration_entry_outcome(entry.clone()) {
        ForgeQueryOrdinaryOutcome::Bound(envelope) => envelope,
        ForgeQueryOrdinaryOutcome::Ambiguous(posture)
        | ForgeQueryOrdinaryOutcome::AspectConflict(posture)
        | ForgeQueryOrdinaryOutcome::AuthorityMismatch(posture)
        | ForgeQueryOrdinaryOutcome::BasisMismatch(posture)
        | ForgeQueryOrdinaryOutcome::Deferred(posture)
        | ForgeQueryOrdinaryOutcome::Denied(posture)
        | ForgeQueryOrdinaryOutcome::ExplicitNarrowingRequired(posture)
        | ForgeQueryOrdinaryOutcome::Failed(posture)
        | ForgeQueryOrdinaryOutcome::MissingRequiredAspect(posture)
        | ForgeQueryOrdinaryOutcome::RebindRequired(posture)
        | ForgeQueryOrdinaryOutcome::Refused(posture)
        | ForgeQueryOrdinaryOutcome::Stale(posture)
        | ForgeQueryOrdinaryOutcome::Unavailable(posture)
        | ForgeQueryOrdinaryOutcome::Unsupported(posture)
        | ForgeQueryOrdinaryOutcome::WrongHandle(posture)
        | ForgeQueryOrdinaryOutcome::WrongWorld(posture) => {
            return Err(GeometryRecoveryActionError::EntryOutcomeNotBound {
                kind: posture.kind(),
                reason: posture.reason().to_string(),
                next_step: posture.next_step(),
            })
        }
    };

    let seed = entry
        .seed()
        .ok_or(GeometryRecoveryActionError::RecoveryNotAvailable {
        reason:
            "geometry recovery is only defined for denied or non-authoritative rebinding outcomes",
    })?;
    let fact_digest = recovery_fact_digest(&[
        format!("{:?}", seed.action),
        format!("{:?}", seed.source_posture),
        format!("{:?}", seed.source_family),
        format!("{:?}", seed.recovery_target_scope),
        entry_envelope.declaration_digest().to_string(),
        format!("{:?}", entry_envelope.envelope_digest()),
        retained_source_digest(entry.source()),
    ]);

    Ok(GeometryRecoveryActionFactReceipt {
        recovery_action_kind: seed.action,
        source_posture: seed.source_posture,
        source_family: seed.source_family,
        recovery_target_scope: seed.recovery_target_scope,
        fact_digest,
        resulting_binding_identity: None,
        resulting_target_identity: None,
    })
}

fn geometry_recovery_lane_from_outcome_class(
    outcome_class: RebindingOutcomeClass,
) -> Option<(
    GeometryRecoveryAction,
    GeometryRecoverySourcePosture,
    GeometryRecoveryTargetScope,
)> {
    match outcome_class {
        RebindingOutcomeClass::Ambiguous => Some((
            GeometryRecoveryAction::NarrowInput,
            GeometryRecoverySourcePosture::Ambiguous,
            GeometryRecoveryTargetScope::InputNarrowing,
        )),
        RebindingOutcomeClass::Orphaned => Some((
            GeometryRecoveryAction::RebindContext,
            GeometryRecoverySourcePosture::RebindRequired,
            GeometryRecoveryTargetScope::TruthContinuationContext,
        )),
        RebindingOutcomeClass::Unsupported => Some((
            GeometryRecoveryAction::CheckSupport,
            GeometryRecoverySourcePosture::Unsupported,
            GeometryRecoveryTargetScope::SupportReadiness,
        )),
        RebindingOutcomeClass::Preserved
        | RebindingOutcomeClass::ExactReattachment
        | RebindingOutcomeClass::ContinuityJustifiedReattachment
        | RebindingOutcomeClass::CorrespondenceOnly => None,
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct GeometryRecoveryActionSeed {
    action: GeometryRecoveryAction,
    source_posture: GeometryRecoverySourcePosture,
    source_family: NeighborhoodBindingFamily,
    recovery_target_scope: GeometryRecoveryTargetScope,
}

fn geometry_recovery_seed_from_source(
    source: &PrimitiveRebindingRetainedFactSource,
) -> Option<GeometryRecoveryActionSeed> {
    let (action, source_posture, recovery_target_scope) =
        geometry_recovery_lane_from_outcome_class(source.receipt().outcome_class())?;
    Some(GeometryRecoveryActionSeed {
        action,
        source_posture,
        source_family: source.receipt().neighborhood_family(),
        recovery_target_scope,
    })
}

fn recovery_fact_digest(parts: &[String]) -> String {
    truth_digest_parts(TruthDigestScope::ArtifactIdentity, parts)
}
