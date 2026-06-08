use forge_query::facade::{
    ForgeQueryAdmittedConfiguredDomainHandle, ForgeQueryDeclarationAdmissionError,
    ForgeQueryDeclarationEntryInspection, ForgeQueryDeclarationEntryInspectionError,
    ForgeQueryDeclarationEntryInspectionInput, ForgeQueryDomainOperatingContext,
};
use worth_primitives::{truth_digest_parts, TruthDigestScope};
use worth_spatial::facade::bindings::AdmittedRebindingDecision;

use crate::binding::rebinding::{
    PrimitiveRebindingAuthoringError, PrimitiveRebindingDeclarationEntry,
    PrimitiveRebindingQueryDomain,
};

#[cfg_attr(not(test), allow(dead_code))]
pub(crate) struct PrimitiveRebindingHistoricalInspection {
    inspection: ForgeQueryDeclarationEntryInspection<
        PrimitiveRebindingQueryDomain,
        PrimitiveRebindingDeclarationEntry,
    >,
    decision: AdmittedRebindingDecision,
    historical_digest: String,
}

#[cfg_attr(not(test), allow(dead_code))]
impl PrimitiveRebindingHistoricalInspection {
    pub(crate) fn inspection(
        &self,
    ) -> &ForgeQueryDeclarationEntryInspection<
        PrimitiveRebindingQueryDomain,
        PrimitiveRebindingDeclarationEntry,
    > {
        &self.inspection
    }

    pub(crate) fn decision(&self) -> &AdmittedRebindingDecision {
        &self.decision
    }

    pub(crate) fn historical_digest(&self) -> &str {
        &self.historical_digest
    }
}

#[allow(dead_code)]
pub(crate) enum PrimitiveRebindingHistoricalInspectionError {
    Declaration(
        ForgeQueryDeclarationAdmissionError<
            PrimitiveRebindingQueryDomain,
            PrimitiveRebindingDeclarationEntry,
        >,
    ),
    Inspection(
        ForgeQueryDeclarationEntryInspectionError<
            PrimitiveRebindingQueryDomain,
            PrimitiveRebindingDeclarationEntry,
        >,
    ),
    Spatial(PrimitiveRebindingAuthoringError),
    RetainedBasisMismatch {
        reason: &'static str,
    },
    TruncatedRetainedBasis {
        reason: &'static str,
    },
}

impl PrimitiveRebindingHistoricalInspectionError {
    pub(crate) fn reason(&self) -> &'static str {
        match self {
            Self::Declaration(_) => {
                "historical rebinding inspection requires a canonical retained rebinding declaration under the admitted handle"
            }
            Self::Inspection(error) => error.reason(),
            Self::Spatial(_) => {
                "historical rebinding inspection requires retained rebinding truth that still admits spatially"
            }
            Self::RetainedBasisMismatch { reason } | Self::TruncatedRetainedBasis { reason } => {
                reason
            }
        }
    }
}

impl std::fmt::Debug for PrimitiveRebindingHistoricalInspectionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PrimitiveRebindingHistoricalInspectionError")
            .field("reason", &self.reason())
            .finish()
    }
}

#[cfg_attr(not(test), allow(dead_code))]
pub(crate) fn primitive_rebinding_historical_inspection<C>(
    entry: &PrimitiveRebindingDeclarationEntry,
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<PrimitiveRebindingQueryDomain, C>,
    subject: ForgeQueryDeclarationEntryInspectionInput<
        PrimitiveRebindingQueryDomain,
        PrimitiveRebindingDeclarationEntry,
    >,
) -> Result<PrimitiveRebindingHistoricalInspection, PrimitiveRebindingHistoricalInspectionError>
where
    C: ForgeQueryDomainOperatingContext<PrimitiveRebindingQueryDomain>,
{
    let inspection = handle
        .inspect_declaration_entry(subject)
        .map_err(PrimitiveRebindingHistoricalInspectionError::Inspection)?;
    if inspection.progression_digest().is_none()
        || inspection.route_plan_digest().is_none()
        || inspection.receipt_digest().is_none()
    {
        return Err(
            PrimitiveRebindingHistoricalInspectionError::TruncatedRetainedBasis {
                reason: "historical rebinding inspection requires retained progression, route-plan, and receipt truth before interpretation",
            },
        );
    }

    let declaration = handle
        .declare(entry.clone())
        .map_err(PrimitiveRebindingHistoricalInspectionError::Declaration)?;
    if format!("{:?}", declaration.declaration_digest()) != inspection.declaration_digest() {
        return Err(
            PrimitiveRebindingHistoricalInspectionError::RetainedBasisMismatch {
                reason: "historical rebinding inspection requires retained declaration truth that matches the retained rebinding declaration",
            },
        );
    }

    let decision = entry
        .clone()
        .admit()
        .map_err(PrimitiveRebindingHistoricalInspectionError::Spatial)?;
    let historical_digest = derive_historical_digest(&inspection, &decision);

    Ok(PrimitiveRebindingHistoricalInspection {
        inspection,
        decision,
        historical_digest,
    })
}

#[cfg_attr(not(test), allow(dead_code))]
fn derive_historical_digest(
    inspection: &ForgeQueryDeclarationEntryInspection<
        PrimitiveRebindingQueryDomain,
        PrimitiveRebindingDeclarationEntry,
    >,
    decision: &AdmittedRebindingDecision,
) -> String {
    let explanation = decision.explanation();
    let mut candidate_identities = explanation.candidate_identities().to_vec();
    let mut candidate_labels = explanation.candidate_labels().to_vec();
    let mut candidate_site_identities = explanation.candidate_site_identities().to_vec();
    candidate_identities.sort();
    candidate_labels.sort();
    candidate_site_identities.sort();

    truth_digest_parts(
        TruthDigestScope::ArtifactIdentity,
        &[
            format!("declaration:{}", inspection.declaration_digest()),
            format!(
                "progression:{}",
                inspection.progression_digest().unwrap_or("none")
            ),
            format!("receipt:{}", inspection.receipt_digest().unwrap_or("none")),
            format!("envelope:{}", inspection.envelope_digest()),
            format!("outcome:{:?}", decision.outcome_class()),
            format!("continuity:{:?}", explanation.continuity_class()),
            format!("motion:{:?}", explanation.motion_posture()),
            format!("family:{:?}", explanation.neighborhood_family()),
            format!("prior:{}", explanation.prior_identity()),
            format!("prior_site:{}", explanation.prior_site_identity()),
            format!(
                "selected_identity:{}",
                explanation.selected_candidate_identity().unwrap_or("none")
            ),
            format!(
                "selected_label:{}",
                explanation.selected_candidate_label().unwrap_or("none")
            ),
            format!("unsupported:{:?}", explanation.unsupported_reason()),
            format!("candidate_identities:{}", candidate_identities.join("|")),
            format!("candidate_labels:{}", candidate_labels.join("|")),
            format!("candidate_sites:{}", candidate_site_identities.join("|")),
        ],
    )
}
