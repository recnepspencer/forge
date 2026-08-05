use crate::harness::certification::HostileExpectation;
use crate::harness::certification::ParityAnchor;
use crate::harness::milestone_nine_certification::bundles::MilestoneNineCertificationRow;
use crate::harness::milestone_nine_certification::classifications::MilestoneNinePerturbationClass;
use crate::harness::milestone_nine_certification::fixtures::admitted_bundle;
use crate::harness::milestone_nine_certification::fixtures::base_policy;
use crate::harness::milestone_nine_certification::fixtures::canonical_query_with_manager_traversal;
use crate::harness::milestone_nine_certification::fixtures::canonical_query_with_secret_projection;
use crate::harness::milestone_nine_certification::fixtures::phase_two_bundle;
use crate::harness::milestone_nine_certification::fixtures::schema;
use crate::harness::milestone_nine_certification::fixtures::secret_salary_key;
use crate::harness::milestone_nine_certification::fixtures::tenant;
use crate::policy_basis::admit_policy_tenant_context;
use crate::policy_basis::BranchAccessGrant;
use crate::policy_basis::PolicyExecutionModeRequest;
use crate::relationship_proof::RelationshipProofBudget;
use crate::relationship_proof::RelationshipProofDescriptor;
use crate::relationship_proof::RelationshipProofDescriptorSet;

pub(super) fn canonical_narrowing_rows() -> Vec<MilestoneNineCertificationRow> {
    let phase_two_canonical = canonical_query_with_secret_projection();
    let policy = base_policy(true);
    let phase_two_admitted = admit_policy_tenant_context(
        phase_two_canonical.query(),
        policy.clone(),
        tenant(),
        BranchAccessGrant::synthetic_granted("branch-a", &policy),
        schema(),
        PolicyExecutionModeRequest::CurrentRead,
    )
    .unwrap();
    let phase_two_mask = crate::authorized_projection::PolicyAspectMask::allow_all()
        .with_masked(secret_salary_key());
    let phase_two_no_proof = phase_two_bundle(
        phase_two_canonical.clone(),
        phase_two_mask.clone(),
        RelationshipProofDescriptorSet::none(),
    );
    let non_disclosing_use = phase_two_bundle(
        phase_two_canonical.clone(),
        crate::authorized_projection::PolicyAspectMask::allow_all()
            .with_non_disclosing_use_only(secret_salary_key()),
        RelationshipProofDescriptorSet::none(),
    );
    let phase_two_direct_proof = phase_two_bundle(
        canonical_query_with_manager_traversal(),
        phase_two_mask.clone(),
        RelationshipProofDescriptorSet::new(
            vec![RelationshipProofDescriptor::direct_edge(
                "manager",
                phase_two_admitted.bundle().policy_digest(),
            )],
            RelationshipProofBudget::bounded(1, 1),
        ),
    );
    let phase_two_tenant_membership = phase_two_bundle(
        phase_two_canonical.clone(),
        phase_two_mask.clone(),
        RelationshipProofDescriptorSet::new(
            vec![RelationshipProofDescriptor::tenant_membership(
                phase_two_admitted.bundle().tenant_schema_basis_digest(),
            )],
            RelationshipProofBudget::bounded(1, 1),
        ),
    );
    let saved_exact_bundle = phase_two_no_proof.clone();
    vec![
        MilestoneNineCertificationRow {
            row_name: "authorized-projection-removes-masked-aspect",
            perturbation_class:
                MilestoneNinePerturbationClass::AuthorizedProjectionRemovesMaskedAspect,
            hostile_expectation: HostileExpectation::DistinctFromControl,
            parity_anchor: ParityAnchor::Hostile,
            control_lane: admitted_bundle(PolicyExecutionModeRequest::CurrentRead, false),
            hostile_lane: phase_two_no_proof.clone(),
            parity_lane: phase_two_no_proof.clone(),
        },
        MilestoneNineCertificationRow {
            row_name: "non-disclosing-use-is-not-delivered",
            perturbation_class: MilestoneNinePerturbationClass::NonDisclosingUseIsNotDelivered,
            hostile_expectation: HostileExpectation::DistinctFromControl,
            parity_anchor: ParityAnchor::Hostile,
            control_lane: phase_two_no_proof.clone(),
            hostile_lane: non_disclosing_use.clone(),
            parity_lane: non_disclosing_use,
        },
        MilestoneNineCertificationRow {
            row_name: "relationship-proof-direct-edge-admission",
            perturbation_class:
                MilestoneNinePerturbationClass::RelationshipProofDirectEdgeAdmission,
            hostile_expectation: HostileExpectation::DistinctFromControl,
            parity_anchor: ParityAnchor::Hostile,
            control_lane: phase_two_no_proof.clone(),
            hostile_lane: phase_two_direct_proof.clone(),
            parity_lane: phase_two_direct_proof.clone(),
        },
        MilestoneNineCertificationRow {
            row_name: "relationship-proof-tenant-membership-admission",
            perturbation_class:
                MilestoneNinePerturbationClass::RelationshipProofTenantMembershipAdmission,
            hostile_expectation: HostileExpectation::DistinctFromControl,
            parity_anchor: ParityAnchor::Hostile,
            control_lane: phase_two_no_proof.clone(),
            hostile_lane: phase_two_tenant_membership.clone(),
            parity_lane: phase_two_tenant_membership,
        },
        MilestoneNineCertificationRow {
            row_name: "narrowed-artifact-binds-policy-tenant-schema",
            perturbation_class:
                MilestoneNinePerturbationClass::NarrowedArtifactBindsPolicyTenantSchema,
            hostile_expectation: HostileExpectation::EquivalentToControl,
            parity_anchor: ParityAnchor::Control,
            control_lane: phase_two_no_proof.clone(),
            hostile_lane: phase_two_no_proof.clone(),
            parity_lane: phase_two_no_proof.clone(),
        },
        MilestoneNineCertificationRow {
            row_name: "saved-query-exact-reuse-narrows-identically",
            perturbation_class:
                MilestoneNinePerturbationClass::SavedQueryExactReuseNarrowsIdentically,
            hostile_expectation: HostileExpectation::EquivalentToControl,
            parity_anchor: ParityAnchor::Control,
            control_lane: saved_exact_bundle.clone(),
            hostile_lane: saved_exact_bundle.clone(),
            parity_lane: saved_exact_bundle,
        },
        MilestoneNineCertificationRow {
            row_name: "optimizer-input-excludes-masked-fields",
            perturbation_class: MilestoneNinePerturbationClass::OptimizerInputExcludesMaskedFields,
            hostile_expectation: HostileExpectation::EquivalentToControl,
            parity_anchor: ParityAnchor::Control,
            control_lane: phase_two_no_proof.clone(),
            hostile_lane: phase_two_no_proof.clone(),
            parity_lane: phase_two_no_proof.clone(),
        },
        MilestoneNineCertificationRow {
            row_name: "phase-two-support-profile-honesty",
            perturbation_class: MilestoneNinePerturbationClass::PhaseTwoSupportProfileHonesty,
            hostile_expectation: HostileExpectation::EquivalentToControl,
            parity_anchor: ParityAnchor::Control,
            control_lane: phase_two_no_proof.clone(),
            hostile_lane: phase_two_no_proof.clone(),
            parity_lane: phase_two_no_proof,
        },
    ]
}
