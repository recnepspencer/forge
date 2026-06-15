use forge_query::facade::{
    ForgeQueryAdmittedConfiguredDomainHandle, ForgeQueryDomainOperatingContext,
    ForgeQueryOrdinaryNextStep, ForgeQueryOrdinaryOutcome, ForgeQueryOrdinaryPosture,
    ForgeQueryOrdinaryPostureKind,
};

use crate::bindings::query_native_planar_projection::authoring::ProjectPointToCertifiedPlane2DEntry;
use crate::bindings::query_native_planar_projection::domain::ProjectPointToCertifiedPlane2DQueryDomain;
use crate::planar_contracts::projection_2d::{
    ProjectPointToCertifiedPlane2DMutationEvidence,
    ProjectPointToCertifiedPlane2DPerformanceCounters, ProjectPointToCertifiedPlane2DReceipt,
};

#[derive(Clone, Debug, PartialEq)]
pub enum ProjectPointToCertifiedPlane2DFactError {
    OutcomeNotBound {
        kind: ForgeQueryOrdinaryPostureKind,
        reason: String,
        next_step: ForgeQueryOrdinaryNextStep,
    },
}

impl ProjectPointToCertifiedPlane2DFactError {
    fn outcome_not_bound(posture: &ForgeQueryOrdinaryPosture) -> Self {
        Self::OutcomeNotBound {
            kind: posture.kind(),
            reason: posture.reason().to_string(),
            next_step: posture.next_step(),
        }
    }
}

pub fn project_point_to_certified_plane_2d_facts<C>(
    entry: &ProjectPointToCertifiedPlane2DEntry,
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<ProjectPointToCertifiedPlane2DQueryDomain, C>,
) -> Result<ProjectPointToCertifiedPlane2DReceipt, ProjectPointToCertifiedPlane2DFactError>
where
    C: ForgeQueryDomainOperatingContext<ProjectPointToCertifiedPlane2DQueryDomain>,
{
    match handle.orchestrate_declaration_entry_outcome(entry.clone()) {
        ForgeQueryOrdinaryOutcome::Bound(envelope) => {
            let basis = entry.case().basis().clone();
            let envelope_digest = format!("{:?}", envelope.envelope_digest());
            let declaration_digest = envelope.declaration_digest().to_string();
            let basis_part_count = ProjectPointToCertifiedPlane2DReceipt::digest_parts(
                &basis,
                &declaration_digest,
                &envelope_digest,
            )
            .len();
            let fact_digest = ProjectPointToCertifiedPlane2DReceipt::fact_digest_for(
                &basis,
                &declaration_digest,
                &envelope_digest,
            );
            let mutation_evidence =
                ProjectPointToCertifiedPlane2DMutationEvidence::from_projection_fact(
                    &basis,
                    &declaration_digest,
                    &envelope_digest,
                    &fact_digest,
                );
            Ok(ProjectPointToCertifiedPlane2DReceipt::new(
                basis,
                declaration_digest,
                envelope_digest,
                fact_digest,
                mutation_evidence,
                ProjectPointToCertifiedPlane2DPerformanceCounters::certified(basis_part_count),
            ))
        }
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
        | ForgeQueryOrdinaryOutcome::WrongWorld(posture) => Err(
            ProjectPointToCertifiedPlane2DFactError::outcome_not_bound(&posture),
        ),
    }
}
