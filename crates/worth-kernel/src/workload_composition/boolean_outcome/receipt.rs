use worth_spatial::facade::blocker_provenance::{
    WorkloadBlockerBoundaryKind, WorkloadBlockerProvenance, WorkloadBlockerProvenanceReceipt,
    WorkloadBlockerSourceKind,
};
use worth_spatial::facade::user_response::{
    PlanarBooleanUserResponseClass, PlanarBooleanUserResponseSource, WorthUserOutcome,
    WorthUserOutcomeCauseKind, WorthUserOutcomeKind, WorthUserResponseSource,
    WorthUserResponseWorkload,
};

use crate::workload_composition::{
    PlanarBooleanDeclaration, PlanarBooleanDeclarationReceipt, PlanarBooleanEntryError,
    PlanarBooleanSupportPosture, PlanarBooleanSupportReceipt,
};

use super::{PlanarBooleanBlockerContext, PlanarBooleanOutcomeKind};

#[derive(Clone, Debug, PartialEq)]
pub struct PlanarBooleanOutcomeReceipt {
    kind: PlanarBooleanOutcomeKind,
    declaration: PlanarBooleanDeclarationReceipt,
    support: PlanarBooleanSupportReceipt,
    user_outcome: WorthUserOutcome,
    blocker_provenance: Option<WorkloadBlockerProvenanceReceipt>,
}

impl PlanarBooleanOutcomeReceipt {
    pub fn classify(
        declaration: &PlanarBooleanDeclaration,
    ) -> Result<Self, PlanarBooleanEntryError> {
        Self::from_declaration_receipt(&declaration.clone().bind()?)
    }

    pub fn from_declaration_receipt(
        declaration: &PlanarBooleanDeclarationReceipt,
    ) -> Result<Self, PlanarBooleanEntryError> {
        let support = PlanarBooleanSupportReceipt::for_declaration(declaration)?;
        match support.posture() {
            PlanarBooleanSupportPosture::Admitted => Ok(Self::admitted(
                declaration.clone(),
                support,
                format!(
                    "{} is admitted for {} on the B-rep execution lane.",
                    declaration.family().human_name(),
                    declaration.operation().human_name()
                ),
            )?),
            PlanarBooleanSupportPosture::VisibleNotAdmitted => Self::policy_required(
                declaration.clone(),
                support,
                "EMBER stays visible on the declaration boundary but is not admitted in milestone 7.0.",
                WorkloadBlockerSourceKind::PlanarBooleanDeclaration,
                WorkloadBlockerBoundaryKind::BooleanLanePolicy,
                declaration.query_declaration_digest(),
                declaration.query_handle_digest(),
            ),
        }
    }

    pub fn admitted(
        declaration: PlanarBooleanDeclarationReceipt,
        support: PlanarBooleanSupportReceipt,
        human_reason: impl Into<String>,
    ) -> Result<Self, PlanarBooleanEntryError> {
        let declaration_digest = declaration.query_declaration_digest().to_string();
        let support_digest = support.query_support_digest().to_string();
        Self::from_response_source(
            PlanarBooleanOutcomeKind::Admitted,
            declaration,
            support,
            PlanarBooleanUserResponseSource::admitted(
                human_reason,
                support_digest,
                declaration_digest,
            ),
            None,
        )
    }

    pub fn unsupported(
        declaration: PlanarBooleanDeclarationReceipt,
        support: PlanarBooleanSupportReceipt,
        human_reason: impl Into<String>,
        source_kind: WorkloadBlockerSourceKind,
        boundary_kind: WorkloadBlockerBoundaryKind,
        source_identity: impl Into<String>,
        boundary_identity: impl Into<String>,
    ) -> Result<Self, PlanarBooleanEntryError> {
        let source_identity = source_identity.into();
        let support_digest = support.query_support_digest().to_string();
        Self::from_non_admitted(
            PlanarBooleanOutcomeKind::Unsupported,
            declaration,
            support,
            PlanarBooleanUserResponseSource::unsupported(
                human_reason,
                support_digest,
                source_identity.clone(),
            ),
            source_kind,
            boundary_kind,
            source_identity,
            boundary_identity,
        )
    }

    pub fn blocked(
        declaration: PlanarBooleanDeclarationReceipt,
        support: PlanarBooleanSupportReceipt,
        human_reason: impl Into<String>,
        source_kind: WorkloadBlockerSourceKind,
        boundary_kind: WorkloadBlockerBoundaryKind,
        source_identity: impl Into<String>,
        boundary_identity: impl Into<String>,
    ) -> Result<Self, PlanarBooleanEntryError> {
        let source_identity = source_identity.into();
        let support_digest = support.query_support_digest().to_string();
        Self::from_non_admitted(
            PlanarBooleanOutcomeKind::Blocked,
            declaration,
            support,
            PlanarBooleanUserResponseSource::blocked(
                human_reason,
                support_digest,
                source_identity.clone(),
            ),
            source_kind,
            boundary_kind,
            source_identity,
            boundary_identity,
        )
    }

    pub fn denied(
        declaration: PlanarBooleanDeclarationReceipt,
        support: PlanarBooleanSupportReceipt,
        human_reason: impl Into<String>,
        source_kind: WorkloadBlockerSourceKind,
        boundary_kind: WorkloadBlockerBoundaryKind,
        source_identity: impl Into<String>,
        boundary_identity: impl Into<String>,
    ) -> Result<Self, PlanarBooleanEntryError> {
        let source_identity = source_identity.into();
        let support_digest = support.query_support_digest().to_string();
        Self::from_non_admitted(
            PlanarBooleanOutcomeKind::Denied,
            declaration,
            support,
            PlanarBooleanUserResponseSource::denied(
                human_reason,
                support_digest,
                source_identity.clone(),
            ),
            source_kind,
            boundary_kind,
            source_identity,
            boundary_identity,
        )
    }

    pub fn policy_required(
        declaration: PlanarBooleanDeclarationReceipt,
        support: PlanarBooleanSupportReceipt,
        human_reason: impl Into<String>,
        source_kind: WorkloadBlockerSourceKind,
        boundary_kind: WorkloadBlockerBoundaryKind,
        source_identity: impl Into<String>,
        boundary_identity: impl Into<String>,
    ) -> Result<Self, PlanarBooleanEntryError> {
        let source_identity = source_identity.into();
        let support_digest = support.query_support_digest().to_string();
        Self::from_non_admitted(
            PlanarBooleanOutcomeKind::PolicyRequired,
            declaration,
            support,
            PlanarBooleanUserResponseSource::policy_required(
                human_reason,
                support_digest,
                source_identity.clone(),
            ),
            source_kind,
            boundary_kind,
            source_identity,
            boundary_identity,
        )
    }

    pub fn integrity_mismatch(
        declaration: PlanarBooleanDeclarationReceipt,
        support: PlanarBooleanSupportReceipt,
        human_reason: impl Into<String>,
        source_kind: WorkloadBlockerSourceKind,
        boundary_kind: WorkloadBlockerBoundaryKind,
        source_identity: impl Into<String>,
        boundary_identity: impl Into<String>,
    ) -> Result<Self, PlanarBooleanEntryError> {
        let source_identity = source_identity.into();
        let support_digest = support.query_support_digest().to_string();
        Self::from_non_admitted(
            PlanarBooleanOutcomeKind::IntegrityMismatch,
            declaration,
            support,
            PlanarBooleanUserResponseSource::integrity_mismatch(
                human_reason,
                support_digest,
                source_identity.clone(),
            ),
            source_kind,
            boundary_kind,
            source_identity,
            boundary_identity,
        )
    }

    pub fn no_options(
        declaration: PlanarBooleanDeclarationReceipt,
        support: PlanarBooleanSupportReceipt,
        human_reason: impl Into<String>,
        source_kind: WorkloadBlockerSourceKind,
        boundary_kind: WorkloadBlockerBoundaryKind,
        source_identity: impl Into<String>,
        boundary_identity: impl Into<String>,
    ) -> Result<Self, PlanarBooleanEntryError> {
        let source_identity = source_identity.into();
        let support_digest = support.query_support_digest().to_string();
        Self::from_non_admitted(
            PlanarBooleanOutcomeKind::NoOptions,
            declaration,
            support,
            PlanarBooleanUserResponseSource::no_options(
                human_reason,
                support_digest,
                source_identity.clone(),
            ),
            source_kind,
            boundary_kind,
            source_identity,
            boundary_identity,
        )
    }

    pub fn kind(&self) -> PlanarBooleanOutcomeKind {
        self.kind
    }

    pub fn declaration(&self) -> &PlanarBooleanDeclarationReceipt {
        &self.declaration
    }

    pub fn support(&self) -> &PlanarBooleanSupportReceipt {
        &self.support
    }

    pub fn user_outcome(&self) -> &WorthUserOutcome {
        &self.user_outcome
    }

    pub fn blocker_provenance(&self) -> Option<&WorkloadBlockerProvenanceReceipt> {
        self.blocker_provenance.as_ref()
    }

    pub fn human_reason(&self) -> &str {
        self.user_outcome.human_response().summary()
    }

    fn from_non_admitted(
        kind: PlanarBooleanOutcomeKind,
        declaration: PlanarBooleanDeclarationReceipt,
        support: PlanarBooleanSupportReceipt,
        source: PlanarBooleanUserResponseSource,
        source_kind: WorkloadBlockerSourceKind,
        boundary_kind: WorkloadBlockerBoundaryKind,
        source_identity: impl Into<String>,
        boundary_identity: impl Into<String>,
    ) -> Result<Self, PlanarBooleanEntryError> {
        let blocker_context = PlanarBooleanBlockerContext::new(
            source_kind,
            boundary_kind,
            source_identity,
            boundary_identity,
            source.message(),
        );
        Self::from_response_source(kind, declaration, support, source, Some(blocker_context))
    }

    fn from_response_source(
        kind: PlanarBooleanOutcomeKind,
        declaration: PlanarBooleanDeclarationReceipt,
        support: PlanarBooleanSupportReceipt,
        source: PlanarBooleanUserResponseSource,
        blocker_context: Option<PlanarBooleanBlockerContext>,
    ) -> Result<Self, PlanarBooleanEntryError> {
        let user_outcome = WorthUserResponseWorkload::from_source(
            WorthUserResponseSource::from_planar_boolean_outcome(&source),
        )
        .declared("respond to planar boolean outcome")
        .respond()
        .map_err(|error| PlanarBooleanEntryError::QueryAdmissionFailed(format!("{error:?}")))?
        .outcome()
        .clone();
        ensure_user_outcome_matches_machine_kind(kind, &source, &user_outcome)?;
        let blocker_provenance = if let Some(blocker_context) = blocker_context {
            Some(
                WorkloadBlockerProvenance::from_planar_boolean_outcome(
                    blocker_context.provenance(),
                )
                .certify_non_admitted(&user_outcome)
                .map_err(|error| {
                    PlanarBooleanEntryError::QueryAdmissionFailed(error.human_reason().to_string())
                })?,
            )
        } else {
            None
        };
        Ok(Self {
            kind,
            declaration,
            support,
            user_outcome,
            blocker_provenance,
        })
    }
}

fn ensure_user_outcome_matches_machine_kind(
    machine_kind: PlanarBooleanOutcomeKind,
    source: &PlanarBooleanUserResponseSource,
    user_outcome: &WorthUserOutcome,
) -> Result<(), PlanarBooleanEntryError> {
    let expected_user_kind = expected_user_kind(machine_kind);
    if user_outcome.kind() != expected_user_kind {
        return Err(PlanarBooleanEntryError::OutcomeProjectionMismatch(format!(
            "planar boolean outcome {:?} must project to {:?}, but projected to {:?}",
            machine_kind,
            expected_user_kind,
            user_outcome.kind()
        )));
    }
    let expected_cause = expected_cause_kind(source.class());
    if user_outcome.cause().map(|cause| cause.kind()) != expected_cause {
        return Err(PlanarBooleanEntryError::OutcomeProjectionMismatch(format!(
            "planar boolean outcome {:?} must project cause {:?}, but projected {:?}",
            machine_kind,
            expected_cause,
            user_outcome.cause().map(|cause| cause.kind())
        )));
    }
    Ok(())
}

fn expected_user_kind(machine_kind: PlanarBooleanOutcomeKind) -> WorthUserOutcomeKind {
    match machine_kind {
        PlanarBooleanOutcomeKind::Admitted => WorthUserOutcomeKind::Admitted,
        PlanarBooleanOutcomeKind::Unsupported => WorthUserOutcomeKind::Unsupported,
        PlanarBooleanOutcomeKind::Blocked | PlanarBooleanOutcomeKind::NoOptions => {
            WorthUserOutcomeKind::NoOptions
        }
        PlanarBooleanOutcomeKind::Denied => WorthUserOutcomeKind::Denied,
        PlanarBooleanOutcomeKind::PolicyRequired => WorthUserOutcomeKind::PolicyRequired,
        PlanarBooleanOutcomeKind::IntegrityMismatch => WorthUserOutcomeKind::IntegrityMismatch,
    }
}

fn expected_cause_kind(
    source_class: PlanarBooleanUserResponseClass,
) -> Option<WorthUserOutcomeCauseKind> {
    match source_class {
        PlanarBooleanUserResponseClass::Admitted => None,
        PlanarBooleanUserResponseClass::Unsupported => {
            Some(WorthUserOutcomeCauseKind::UnsupportedInput)
        }
        PlanarBooleanUserResponseClass::Blocked | PlanarBooleanUserResponseClass::NoOptions => {
            Some(WorthUserOutcomeCauseKind::MissingEvidence)
        }
        PlanarBooleanUserResponseClass::Denied => Some(WorthUserOutcomeCauseKind::OverlapDenied),
        PlanarBooleanUserResponseClass::PolicyRequired => {
            Some(WorthUserOutcomeCauseKind::PolicyRequired)
        }
        PlanarBooleanUserResponseClass::IntegrityMismatch => {
            Some(WorthUserOutcomeCauseKind::IntegrityMismatch)
        }
    }
}
