use forge_query::facade::{
    readmit_lower_runtime_evidence, BasisFamily, DeniedBasisCapability,
    ForgeQueryAdmittedConfiguredDomainHandle, ForgeQueryDeclarationAdmissionError,
    ForgeQueryDeclarationEntryInspection, ForgeQueryDeclarationEntryInspectionError,
    ForgeQueryDeclarationEntryInspectionInput, ForgeQueryDomainOperatingContext,
    LowerRuntimeBasisEvidence, ScopedInspectionBasis,
};
use worth_primitives::{truth_digest_parts, TruthDigestScope};
use worth_spatial::facade::bindings::AdmittedRebindingDecision;

use crate::binding::rebinding::{
    PrimitiveRebindingAuthoringError, PrimitiveRebindingDeclarationEntry,
    PrimitiveRebindingQueryDomain,
};

#[cfg_attr(not(test), allow(dead_code))]
pub(crate) struct PrimitiveRebindingBranchLocalInspection {
    inspection: ForgeQueryDeclarationEntryInspection<
        PrimitiveRebindingQueryDomain,
        PrimitiveRebindingDeclarationEntry,
    >,
    decision: AdmittedRebindingDecision,
    branch_basis_digest: String,
    branch_binding_digest: String,
    branch_local_digest: String,
}

#[cfg_attr(not(test), allow(dead_code))]
impl PrimitiveRebindingBranchLocalInspection {
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

    pub(crate) fn branch_basis_digest(&self) -> &str {
        &self.branch_basis_digest
    }

    pub(crate) fn branch_binding_digest(&self) -> &str {
        &self.branch_binding_digest
    }

    pub(crate) fn branch_local_digest(&self) -> &str {
        &self.branch_local_digest
    }
}

#[allow(dead_code)]
pub(crate) enum PrimitiveRebindingBranchLocalInspectionError {
    UnsupportedBranchBasisFamily {
        family: BasisFamily,
    },
    LowerRuntimeBasis(DeniedBasisCapability),
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

impl PrimitiveRebindingBranchLocalInspectionError {
    pub(crate) fn reason(&self) -> &'static str {
        match self {
            Self::UnsupportedBranchBasisFamily { .. } => {
                "branch-local rebinding inspection requires a branch-head or branch-snapshot inspection basis"
            }
            Self::LowerRuntimeBasis(_) => {
                "branch-local rebinding inspection requires readmitted branch-scoped lower-runtime evidence from the same admitted branch basis"
            }
            Self::Declaration(_) => {
                "branch-local rebinding inspection requires a canonical retained rebinding declaration under the admitted handle"
            }
            Self::Inspection(error) => error.reason(),
            Self::Spatial(_) => {
                "branch-local rebinding inspection requires branch-local rebinding truth that still admits spatially"
            }
            Self::RetainedBasisMismatch { reason } | Self::TruncatedRetainedBasis { reason } => {
                reason
            }
        }
    }
}

impl std::fmt::Debug for PrimitiveRebindingBranchLocalInspectionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PrimitiveRebindingBranchLocalInspectionError")
            .field("reason", &self.reason())
            .finish()
    }
}

#[cfg_attr(not(test), allow(dead_code))]
pub(crate) fn primitive_rebinding_branch_local_inspection<C>(
    entry: &PrimitiveRebindingDeclarationEntry,
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<PrimitiveRebindingQueryDomain, C>,
    scoped_basis: &ScopedInspectionBasis,
    branch_basis_evidence: LowerRuntimeBasisEvidence,
    subject: ForgeQueryDeclarationEntryInspectionInput<
        PrimitiveRebindingQueryDomain,
        PrimitiveRebindingDeclarationEntry,
    >,
) -> Result<PrimitiveRebindingBranchLocalInspection, PrimitiveRebindingBranchLocalInspectionError>
where
    C: ForgeQueryDomainOperatingContext<PrimitiveRebindingQueryDomain>,
{
    let bound_basis = readmit_lower_runtime_evidence(scoped_basis.clone(), branch_basis_evidence)
        .map_err(PrimitiveRebindingBranchLocalInspectionError::LowerRuntimeBasis)?;
    if !matches!(
        bound_basis.scoped_basis().family(),
        BasisFamily::BranchHead | BasisFamily::BranchSnapshot
    ) {
        return Err(
            PrimitiveRebindingBranchLocalInspectionError::UnsupportedBranchBasisFamily {
                family: bound_basis.scoped_basis().family(),
            },
        );
    }
    let branch_binding_digest = bound_basis.lower_runtime_binding_digest();

    let inspection = handle
        .inspect_declaration_entry(subject)
        .map_err(PrimitiveRebindingBranchLocalInspectionError::Inspection)?;
    if inspection.progression_digest().is_none()
        || inspection.route_plan_digest().is_none()
        || inspection.receipt_digest().is_none()
    {
        return Err(
            PrimitiveRebindingBranchLocalInspectionError::TruncatedRetainedBasis {
                reason: "branch-local rebinding inspection requires retained progression, route-plan, and receipt truth before interpretation",
            },
        );
    }

    let declaration = handle
        .declare(entry.clone())
        .map_err(PrimitiveRebindingBranchLocalInspectionError::Declaration)?;
    if format!("{:?}", declaration.declaration_digest()) != inspection.declaration_digest() {
        return Err(
            PrimitiveRebindingBranchLocalInspectionError::RetainedBasisMismatch {
                reason: "branch-local rebinding inspection requires retained declaration truth that matches the retained rebinding declaration",
            },
        );
    }

    let decision = entry
        .clone()
        .admit()
        .map_err(PrimitiveRebindingBranchLocalInspectionError::Spatial)?;
    let branch_local_digest = derive_branch_local_digest(
        &inspection,
        &decision,
        bound_basis.scoped_basis(),
        branch_binding_digest,
    );

    Ok(PrimitiveRebindingBranchLocalInspection {
        inspection,
        decision,
        branch_basis_digest: bound_basis.scoped_basis().scoped_basis_digest().to_string(),
        branch_binding_digest: branch_binding_digest.to_string(),
        branch_local_digest,
    })
}

fn derive_branch_local_digest(
    inspection: &ForgeQueryDeclarationEntryInspection<
        PrimitiveRebindingQueryDomain,
        PrimitiveRebindingDeclarationEntry,
    >,
    decision: &AdmittedRebindingDecision,
    scoped_basis: &ScopedInspectionBasis,
    _branch_binding_digest: &str,
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
            format!("branch_family:{}", scoped_basis.family().as_str()),
            format!("branch_basis:{}", scoped_basis.scoped_basis_digest()),
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
