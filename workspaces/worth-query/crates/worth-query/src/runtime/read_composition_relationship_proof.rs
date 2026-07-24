use crate::canonicalization::CanonicalQueryArtifact;
use crate::identity::SchemaBasisDigest;
use crate::policy_basis::{
    admit_policy_tenant_context, BranchAccessGrant, PolicyEpoch, PolicyExecutionModeRequest,
    PolicyRuleSnapshot, PolicyTenantAdmissionError,
};
use crate::relationship_proof::{
    admit_relationship_proofs, RelationshipProofAdmission, RelationshipProofBudget,
    RelationshipProofDescriptor, RelationshipProofDescriptorSet, RelationshipProofError,
    RelationshipProofSupportProfile, RelationshipProofSupportStatus, RelationshipProofSurface,
    RelationshipProofTopologyClass,
};
use crate::runtime::{
    WorthQueryReadDenial, WorthQueryReadDenialKind, WorthQueryReadRelationshipProofDenial,
};
use crate::tenant_basis::{SchemaVariantSnapshot, TenantBasisEpoch, TenantBindingSnapshot};
use crate::validation::ValidatedTraversalEntry;

const RUNTIME_READ_POLICY_LABEL: &str = "runtime-read-relationship-proof";
const RUNTIME_READ_BRANCH_IDENTITY: &str = "runtime-current-branch";

pub(super) fn admit_read_relationship_proof(
    canonical: &CanonicalQueryArtifact,
    schema_basis: &SchemaBasisDigest,
    traversals: &[ValidatedTraversalEntry],
    built_in_operators: &[crate::runtime::WorthQueryReadBuiltInOperator],
) -> Result<Option<RelationshipProofAdmission>, WorthQueryReadDenial> {
    if traversals.is_empty() {
        return Ok(None);
    }
    let admitted = synthetic_runtime_read_context(canonical, schema_basis)?;
    let descriptors =
        descriptor_set_for_read_traversals(traversals, built_in_operators, &admitted)?;
    let (admission, _) = admit_relationship_proofs(canonical, &admitted, &descriptors)
        .map_err(relationship_proof_admission_denial)?;
    Ok(Some(admission))
}

pub(super) fn support_profile_for_relationship_proof(
    admission: &RelationshipProofAdmission,
) -> RelationshipProofSupportProfile {
    let mut surfaces = vec![(
        RelationshipProofSurface::DescriptorAdmission,
        RelationshipProofSupportStatus::Verified,
    )];
    if admission
        .topology_classes()
        .contains(&RelationshipProofTopologyClass::DirectEdge)
    {
        surfaces.push((
            RelationshipProofSurface::DirectEdgeTopology,
            RelationshipProofSupportStatus::Verified,
        ));
    }
    if admission
        .topology_classes()
        .contains(&RelationshipProofTopologyClass::BoundedAncestor)
    {
        surfaces.push((
            RelationshipProofSurface::BoundedAncestorTopology,
            RelationshipProofSupportStatus::Verified,
        ));
    }
    if admission
        .topology_classes()
        .contains(&RelationshipProofTopologyClass::BoundedDescendant)
    {
        surfaces.push((
            RelationshipProofSurface::BoundedDescendantTopology,
            RelationshipProofSupportStatus::Verified,
        ));
    }
    if admission
        .topology_classes()
        .contains(&RelationshipProofTopologyClass::TenantMembership)
    {
        surfaces.push((
            RelationshipProofSurface::TenantMembershipTopology,
            RelationshipProofSupportStatus::Verified,
        ));
    }
    surfaces.push((
        RelationshipProofSurface::RuntimeProofEvaluation,
        RelationshipProofSupportStatus::Deferred,
    ));
    surfaces.push((
        RelationshipProofSurface::HostCallbackProofs,
        RelationshipProofSupportStatus::Forbidden,
    ));
    RelationshipProofSupportProfile::new(surfaces)
}

fn synthetic_runtime_read_context(
    canonical: &CanonicalQueryArtifact,
    schema_basis: &SchemaBasisDigest,
) -> Result<crate::policy_basis::AdmittedPolicyTenantContext, WorthQueryReadDenial> {
    let schema_identity = schema_basis.as_str().to_string();
    let tenant_identity = format!("runtime-read-tenant:{schema_identity}");
    let policy = PolicyRuleSnapshot::synthetic_authority(
        RUNTIME_READ_POLICY_LABEL,
        &schema_identity,
        PolicyEpoch::Synthetic(1),
    );
    let tenant = TenantBindingSnapshot::synthetic_direct(
        &tenant_identity,
        RUNTIME_READ_BRANCH_IDENTITY,
        &schema_identity,
        TenantBasisEpoch::Synthetic(1),
    );
    let branch = BranchAccessGrant::synthetic_granted(RUNTIME_READ_BRANCH_IDENTITY, &policy);
    let schema = SchemaVariantSnapshot::synthetic_authority(
        &tenant_identity,
        &schema_identity,
        "runtime-read-compatible",
    );
    admit_policy_tenant_context(
        canonical,
        policy,
        tenant,
        branch,
        schema,
        PolicyExecutionModeRequest::CurrentRead,
    )
    .map_err(policy_tenant_admission_denial)
}

fn descriptor_set_for_read_traversals(
    traversals: &[ValidatedTraversalEntry],
    built_in_operators: &[crate::runtime::WorthQueryReadBuiltInOperator],
    admitted: &crate::policy_basis::AdmittedPolicyTenantContext,
) -> Result<RelationshipProofDescriptorSet, WorthQueryReadDenial> {
    let mut descriptors = Vec::with_capacity(traversals.len() + 1);
    let mut topology_width = 1usize;
    let use_descendant_topology = built_in_operators
        .contains(&crate::runtime::WorthQueryReadBuiltInOperator::BoundedDescendant);
    for traversal in traversals {
        let relation = traversal.relation_name().clone();
        if traversal.depth() <= 1 {
            descriptors.push(RelationshipProofDescriptor::direct_edge_relation_name(
                relation,
                admitted.bundle().policy_digest(),
            ));
            topology_width += 1;
        } else {
            descriptors.push(
                if use_descendant_topology {
                    RelationshipProofDescriptor::bounded_descendant_relation_name(
                        relation,
                        traversal.depth(),
                        admitted.bundle().policy_digest(),
                    )
                } else {
                    RelationshipProofDescriptor::bounded_ancestor_relation_name(
                        relation,
                        traversal.depth(),
                        admitted.bundle().policy_digest(),
                    )
                }
                .map_err(|error| {
                    WorthQueryReadDenial::new(
                        WorthQueryReadDenialKind::ValidationDenied,
                        format!("{error:?}"),
                    )
                })?,
            );
            topology_width += usize::from(traversal.depth());
        }
    }
    descriptors.push(RelationshipProofDescriptor::tenant_membership(
        admitted.bundle().tenant_schema_basis_digest(),
    ));
    Ok(RelationshipProofDescriptorSet::new(
        descriptors,
        RelationshipProofBudget::bounded(traversals.len() + 1, topology_width),
    ))
}

fn policy_tenant_admission_denial(error: PolicyTenantAdmissionError) -> WorthQueryReadDenial {
    WorthQueryReadDenial::new_relationship_proof_admission_denied(
        WorthQueryReadRelationshipProofDenial::for_policy_failure(error.failure_class()),
        format!("{error:?}"),
    )
}

fn relationship_proof_admission_denial(error: RelationshipProofError) -> WorthQueryReadDenial {
    WorthQueryReadDenial::new_relationship_proof_admission_denied(
        WorthQueryReadRelationshipProofDenial::for_relationship_proof_failure(
            error.failure_class(),
        ),
        format!("{error:?}"),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::authoring::{
        AspectFieldSelector, AuthoredResultShapeField, GuidedAuthoringPath, RawAuthoredQuery,
        RawAuthoredResultShape, RootEntityKey,
    };
    use crate::policy_basis::PolicyTenantAdmissionFailureClass;
    use crate::relationship_proof::RelationshipProofFailureClass;
    use crate::runtime::{WorthQueryReadDenialKind, WorthQueryReadRelationshipProofDenialStage};

    fn canonical_query() -> crate::canonicalization::CanonicalQueryBundle {
        let query = RawAuthoredQuery::detail_builder(RootEntityKey::new("user").unwrap())
            .project(AspectFieldSelector::new("identity", "id").unwrap())
            .build()
            .unwrap();
        let result_shape = RawAuthoredResultShape::detail_builder()
            .field(AuthoredResultShapeField::new("identity", "id", "id").unwrap())
            .build()
            .unwrap();
        GuidedAuthoringPath::canonicalize_detail(query, result_shape).unwrap()
    }

    #[test]
    fn policy_tenant_failures_map_to_structured_relationship_proof_denial() {
        let canonical = canonical_query();
        let policy = PolicyRuleSnapshot::synthetic_authority_with_budget(
            RUNTIME_READ_POLICY_LABEL,
            "schema-a",
            PolicyEpoch::Synthetic(1),
            true,
            crate::policy_basis::PolicyCostPosture::UnknownCost,
            Some(crate::policy_basis::PolicyWorkBudget::bounded(1, 1, 1)),
        );
        let tenant = TenantBindingSnapshot::synthetic_direct(
            "tenant-a",
            RUNTIME_READ_BRANCH_IDENTITY,
            "schema-a",
            TenantBasisEpoch::Synthetic(1),
        );
        let branch = BranchAccessGrant::synthetic_granted(RUNTIME_READ_BRANCH_IDENTITY, &policy);
        let schema = SchemaVariantSnapshot::synthetic_authority("tenant-a", "schema-a", "compat");

        let denial = admit_policy_tenant_context(
            canonical.query(),
            policy,
            tenant,
            branch,
            schema,
            PolicyExecutionModeRequest::CurrentRead,
        )
        .map_err(policy_tenant_admission_denial)
        .expect_err("unknown-cost synthetic runtime context should deny");

        assert_eq!(
            denial.kind(),
            &WorthQueryReadDenialKind::RelationshipProofAdmissionDenied
        );
        let structured = denial
            .relationship_proof_denial()
            .expect("relationship-proof denials should expose structured payload");
        assert_eq!(
            structured.stage(),
            &WorthQueryReadRelationshipProofDenialStage::SyntheticRuntimeContext
        );
        assert_eq!(
            structured.policy_failure_class(),
            Some(PolicyTenantAdmissionFailureClass::PolicyWorkBudgetDenied)
        );
        assert_eq!(structured.relationship_proof_failure_class(), None);
    }

    #[test]
    fn relationship_proof_failures_map_to_structured_read_denial() {
        let denial = relationship_proof_admission_denial(
            RelationshipProofError::unbounded_recursive_walk("proof failure"),
        );

        assert_eq!(
            denial.kind(),
            &WorthQueryReadDenialKind::RelationshipProofAdmissionDenied
        );
        let structured = denial
            .relationship_proof_denial()
            .expect("relationship-proof denials should expose structured payload");
        assert_eq!(
            structured.stage(),
            &WorthQueryReadRelationshipProofDenialStage::DescriptorAdmission
        );
        assert_eq!(structured.policy_failure_class(), None);
        assert_eq!(
            structured.relationship_proof_failure_class(),
            Some(RelationshipProofFailureClass::UnboundedRecursiveWalk)
        );
    }
}
