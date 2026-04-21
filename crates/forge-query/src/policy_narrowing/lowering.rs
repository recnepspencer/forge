use crate::authorized_projection::{
    derive_authorized_projection, PolicyInfluenceSet, PolicyMaskSnapshot,
};
use crate::canonicalization::CanonicalQueryBundle;
use crate::policy_basis::AdmittedPolicyTenantContext;
use crate::relationship_proof::{admit_relationship_proofs, RelationshipProofDescriptorSet};

use super::{
    validate_narrowing_budget, NarrowedPolicyQueryArtifact, PolicyAwareOptimizerInput,
    PolicyAwareValidationReport, PolicyNarrowingCostPosture, PolicyNarrowingCounters,
    PolicyNarrowingError, PolicyNarrowingFailureClass, PolicyNarrowingWorkBudget,
    SavedPolicyNarrowingReuseDescriptor, SavedPolicyNarrowingReuseDisposition,
};

pub fn narrow_policy_query(
    canonical: &CanonicalQueryBundle,
    admitted: AdmittedPolicyTenantContext,
    mask: PolicyMaskSnapshot,
    influence: PolicyInfluenceSet,
    descriptors: RelationshipProofDescriptorSet,
) -> Result<NarrowedPolicyQueryArtifact, PolicyNarrowingError> {
    let work_budget =
        PolicyNarrowingWorkBudget::from_policy_budget(admitted.bundle().policy_work_budget());
    narrow_policy_query_with_budget(
        canonical,
        admitted,
        mask,
        influence,
        descriptors,
        work_budget,
    )
}

pub fn narrow_policy_query_with_budget(
    canonical: &CanonicalQueryBundle,
    admitted: AdmittedPolicyTenantContext,
    mask: PolicyMaskSnapshot,
    influence: PolicyInfluenceSet,
    descriptors: RelationshipProofDescriptorSet,
    work_budget: PolicyNarrowingWorkBudget,
) -> Result<NarrowedPolicyQueryArtifact, PolicyNarrowingError> {
    if admitted.bundle().canonical_query_digest() != canonical.query().digest().as_str() {
        return Err(PolicyNarrowingError::new(
            PolicyNarrowingFailureClass::CanonicalQueryDigestMismatch,
            "admitted policy/tenant context must match the canonical query being narrowed",
            PolicyNarrowingCounters::default(),
        ));
    }

    if mask.policy_digest() != admitted.bundle().policy_digest() {
        return Err(PolicyNarrowingError::new(
            PolicyNarrowingFailureClass::PolicyMaskAuthorityMismatch,
            "policy mask snapshot must be bound to the admitted policy basis",
            PolicyNarrowingCounters::default(),
        ));
    }

    let cost_posture: Option<PolicyNarrowingCostPosture> =
        admitted.bundle().policy_cost_posture().into();
    let Some(cost_posture) = cost_posture else {
        return Err(PolicyNarrowingError::new(
            PolicyNarrowingFailureClass::UnknownNarrowingCost,
            "policy narrowing requires a known bounded cost posture",
            PolicyNarrowingCounters::denied_unknown_cost(),
        ));
    };

    validate_narrowing_budget(canonical, &influence, &descriptors, &work_budget)?;

    let authorized_projection = derive_authorized_projection(
        canonical.query(),
        canonical.result_shape(),
        admitted.bundle().policy_digest(),
        admitted.bundle().tenant_schema_basis_digest(),
        mask.mask(),
        &influence,
        work_budget.max_projected_fields(),
        work_budget.max_masked_fields(),
    )
    .map_err(|err| {
        PolicyNarrowingError::new(
            PolicyNarrowingFailureClass::AuthorizedProjectionDenied(err.failure_class()),
            err.message(),
            PolicyNarrowingCounters::new(
                err.counters().clone(),
                crate::relationship_proof::RelationshipProofCounters::default(),
            ),
        )
    })?;

    let (relationship_proof, relationship_counters) =
        admit_relationship_proofs(canonical.query(), &admitted, &descriptors).map_err(|err| {
            PolicyNarrowingError::new(
                PolicyNarrowingFailureClass::RelationshipProofDenied(err.failure_class()),
                err.message(),
                PolicyNarrowingCounters::new(
                    authorized_projection.counters().clone(),
                    err.counters().clone(),
                ),
            )
        })?;

    let counters = PolicyNarrowingCounters::new(
        authorized_projection.counters().clone(),
        relationship_counters,
    );
    let validation_report = PolicyAwareValidationReport::success(
        &authorized_projection,
        &relationship_proof,
        &counters,
    );

    Ok(NarrowedPolicyQueryArtifact::new(
        &admitted,
        canonical.result_shape().digest().as_str().to_string(),
        authorized_projection,
        relationship_proof,
        validation_report,
        cost_posture,
        work_budget,
        counters,
    ))
}

pub fn optimizer_input_from_narrowed_policy_query(
    artifact: &NarrowedPolicyQueryArtifact,
) -> PolicyAwareOptimizerInput {
    PolicyAwareOptimizerInput::from_narrowed(artifact)
}

pub fn classify_saved_policy_narrowing_reuse(
    descriptor: &SavedPolicyNarrowingReuseDescriptor,
) -> SavedPolicyNarrowingReuseDisposition {
    if descriptor.exact_narrowing_match() {
        return SavedPolicyNarrowingReuseDisposition::LegalNoSemanticChange;
    }
    if descriptor.same_policy_tenant_basis() {
        return SavedPolicyNarrowingReuseDisposition::LegalRequiresFreshNarrowing;
    }
    SavedPolicyNarrowingReuseDisposition::IllegalSemanticDrift
}
