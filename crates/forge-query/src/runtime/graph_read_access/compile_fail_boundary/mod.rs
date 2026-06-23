use crate::identity::hash_parts;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryGraphReadProofBoundaryEvidenceKind {
    ConstructorPrivate,
    PhaseInputRequired,
    RawValueRejected,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ForgeQueryGraphReadProofTransitionManifestRow {
    phase: &'static str,
    artifact: &'static str,
    evidence_kind: ForgeQueryGraphReadProofBoundaryEvidenceKind,
    compile_fail_target: &'static str,
}

const GRAPH_READ_ACCESS_COMPILE_FAIL_TARGETS: &[&str] = &[
    "tests/ui/graph_read_access/access_shape_constructor_private.rs",
    "tests/ui/graph_read_access/access_shape_new_private.rs",
    "tests/ui/graph_read_access/access_requirement_from_string_forbidden.rs",
    "tests/ui/graph_read_access/access_requirement_row_constructor_private.rs",
    "tests/ui/graph_read_access/access_requirement_set_constructor_private.rs",
    "tests/ui/graph_read_access/admitted_boolean_expression_branch_constructor_private.rs",
    "tests/ui/graph_read_access/admitted_boolean_expression_constructor_private.rs",
    "tests/ui/graph_read_access/admitted_boolean_predicate_leaf_constructor_private.rs",
    "tests/ui/graph_read_access/admitted_ordering_field_constructor_private.rs",
    "tests/ui/graph_read_access/admitted_ordering_field_new_private.rs",
    "tests/ui/graph_read_access/admitted_predicate_field_constructor_private.rs",
    "tests/ui/graph_read_access/admitted_predicate_field_new_private.rs",
    "tests/ui/graph_read_access/admitted_projection_field_constructor_private.rs",
    "tests/ui/graph_read_access/admitted_projection_field_new_private.rs",
    "tests/ui/graph_read_access/admitted_references_constructor_private.rs",
    "tests/ui/graph_read_access/admitted_relation_constructor_private.rs",
    "tests/ui/graph_read_access/admitted_relation_new_private.rs",
    "tests/ui/graph_read_access/authority_context_constructor_private.rs",
    "tests/ui/graph_read_access/authority_context_runtime_current_private.rs",
    "tests/ui/graph_read_access/authority_receipt_runtime_current_private.rs",
    "tests/ui/graph_read_access/basis_binding_constructor_private.rs",
    "tests/ui/graph_read_access/basis_binding_new_private.rs",
    "tests/ui/graph_read_access/boolean_selectivity_branch_constructor_private.rs",
    "tests/ui/graph_read_access/boolean_selectivity_branch_conjunctive_root_private.rs",
    "tests/ui/graph_read_access/boolean_selectivity_shape_constructor_private.rs",
    "tests/ui/graph_read_access/boolean_selectivity_shape_new_private.rs",
    "tests/ui/graph_read_access/domain_operation_declaration_constructor_private.rs",
    "tests/ui/graph_read_access/domain_operation_raw_string_not_query_intent.rs",
    "tests/ui/graph_read_access/domain_registered_operation_constructor_private.rs",
    "tests/ui/graph_read_access/domain_registration_callback_execution_forbidden.rs",
    "tests/ui/graph_read_access/operation_capability_requirement_constructor_private.rs",
    "tests/ui/graph_read_access/operation_capability_requirement_resolved_constructor_private.rs",
    "tests/ui/graph_read_access/operation_registry_constructor_private.rs",
    "tests/ui/graph_read_access/operation_registration_constructor_private.rs",
    "tests/ui/graph_read_access/operation_resolution_constructor_private.rs",
    "tests/ui/graph_read_access/operation_resolution_new_private.rs",
    "tests/ui/graph_read_access/operation_unsupported_denial_constructor_private.rs",
    "tests/ui/graph_read_access/operation_unsupported_denial_resolved_constructor_private.rs",
    "tests/ui/graph_read_access/policy_tenant_proof_constructor_private.rs",
    "tests/ui/graph_read_access/policy_tenant_proof_new_private.rs",
    "tests/ui/graph_read_access/predicate_selectivity_row_constructor_private.rs",
    "tests/ui/graph_read_access/predicate_selectivity_row_new_private.rs",
    "tests/ui/graph_read_access/raw_values_cannot_derive_access_shape.rs",
    "tests/ui/graph_read_access/resolved_operation_constructor_private.rs",
    "tests/ui/graph_read_access/resolved_operation_new_private.rs",
];

const GRAPH_READ_ACCESS_PROOF_TRANSITION_MANIFEST:
    &[ForgeQueryGraphReadProofTransitionManifestRow] = &[
    manifest_row(
        "schema_reference_admission",
        "ForgeQueryAdmittedQuerySchemaReferences",
        ForgeQueryGraphReadProofBoundaryEvidenceKind::ConstructorPrivate,
        "tests/ui/graph_read_access/admitted_references_constructor_private.rs",
    ),
    manifest_row(
        "basis_and_narrowing_admission",
        "ForgeQueryGraphReadBasisBinding",
        ForgeQueryGraphReadProofBoundaryEvidenceKind::ConstructorPrivate,
        "tests/ui/graph_read_access/basis_binding_constructor_private.rs",
    ),
    manifest_row(
        "basis_and_narrowing_admission",
        "ForgeQueryGraphReadPolicyTenantProofBinding",
        ForgeQueryGraphReadProofBoundaryEvidenceKind::ConstructorPrivate,
        "tests/ui/graph_read_access/policy_tenant_proof_constructor_private.rs",
    ),
    manifest_row(
        "operation_resolution",
        "ForgeQueryGraphReadOperationResolution",
        ForgeQueryGraphReadProofBoundaryEvidenceKind::ConstructorPrivate,
        "tests/ui/graph_read_access/operation_resolution_constructor_private.rs",
    ),
    manifest_row(
        "access_shape_derivation",
        "ForgeQueryGraphReadAccessShape",
        ForgeQueryGraphReadProofBoundaryEvidenceKind::RawValueRejected,
        "tests/ui/graph_read_access/raw_values_cannot_derive_access_shape.rs",
    ),
    manifest_row(
        "selectivity_normalization",
        "ForgeQueryBooleanSelectivityShape",
        ForgeQueryGraphReadProofBoundaryEvidenceKind::ConstructorPrivate,
        "tests/ui/graph_read_access/boolean_selectivity_shape_constructor_private.rs",
    ),
    manifest_row(
        "requirement_derivation",
        "ForgeQueryGraphReadAccessRequirementSet",
        ForgeQueryGraphReadProofBoundaryEvidenceKind::ConstructorPrivate,
        "tests/ui/graph_read_access/access_requirement_set_constructor_private.rs",
    ),
    manifest_row(
        "requirement_derivation",
        "ForgeQueryGraphReadAccessRequirementRow",
        ForgeQueryGraphReadProofBoundaryEvidenceKind::ConstructorPrivate,
        "tests/ui/graph_read_access/access_requirement_row_constructor_private.rs",
    ),
    manifest_row(
        "inventory_matching",
        "ForgeQueryGraphReadOperationCapabilityRequirement",
        ForgeQueryGraphReadProofBoundaryEvidenceKind::ConstructorPrivate,
        "tests/ui/graph_read_access/operation_capability_requirement_constructor_private.rs",
    ),
    manifest_row(
        "domain_operation_resolution",
        "ForgeQueryDomainRegisteredGraphReadOperation",
        ForgeQueryGraphReadProofBoundaryEvidenceKind::ConstructorPrivate,
        "tests/ui/graph_read_access/domain_registered_operation_constructor_private.rs",
    ),
    manifest_row(
        "schema_relation_admission",
        "ForgeQueryAdmittedGraphReadRelation",
        ForgeQueryGraphReadProofBoundaryEvidenceKind::ConstructorPrivate,
        "tests/ui/graph_read_access/admitted_relation_constructor_private.rs",
    ),
    manifest_row(
        "schema_field_admission",
        "ForgeQueryAdmittedGraphReadProjectionField",
        ForgeQueryGraphReadProofBoundaryEvidenceKind::ConstructorPrivate,
        "tests/ui/graph_read_access/admitted_projection_field_constructor_private.rs",
    ),
];

const fn manifest_row(
    phase: &'static str,
    artifact: &'static str,
    evidence_kind: ForgeQueryGraphReadProofBoundaryEvidenceKind,
    compile_fail_target: &'static str,
) -> ForgeQueryGraphReadProofTransitionManifestRow {
    ForgeQueryGraphReadProofTransitionManifestRow {
        phase,
        artifact,
        evidence_kind,
        compile_fail_target,
    }
}

impl ForgeQueryGraphReadProofBoundaryEvidenceKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ConstructorPrivate => "constructor_private",
            Self::PhaseInputRequired => "phase_input_required",
            Self::RawValueRejected => "raw_value_rejected",
        }
    }
}

impl ForgeQueryGraphReadProofTransitionManifestRow {
    pub fn phase(&self) -> &'static str {
        self.phase
    }

    pub fn artifact(&self) -> &'static str {
        self.artifact
    }

    pub fn evidence_kind(&self) -> ForgeQueryGraphReadProofBoundaryEvidenceKind {
        self.evidence_kind
    }

    pub fn compile_fail_target(&self) -> &'static str {
        self.compile_fail_target
    }

    fn digest_part(&self) -> String {
        format!(
            "transition:{}:{}:{}:{}",
            self.phase,
            self.artifact,
            self.evidence_kind.as_str(),
            self.compile_fail_target
        )
    }
}

pub fn forge_query_graph_read_access_compile_fail_targets() -> Vec<&'static str> {
    GRAPH_READ_ACCESS_COMPILE_FAIL_TARGETS.to_vec()
}

pub fn forge_query_graph_read_access_compile_fail_target_count() -> usize {
    GRAPH_READ_ACCESS_COMPILE_FAIL_TARGETS.len()
}

pub fn forge_query_graph_read_access_compile_fail_boundary_digest() -> String {
    hash_parts(&compile_fail_boundary_digest_parts())
}

pub fn forge_query_graph_read_proof_transition_manifest(
) -> Vec<ForgeQueryGraphReadProofTransitionManifestRow> {
    GRAPH_READ_ACCESS_PROOF_TRANSITION_MANIFEST.to_vec()
}

pub fn forge_query_graph_read_proof_transition_manifest_count() -> usize {
    GRAPH_READ_ACCESS_PROOF_TRANSITION_MANIFEST.len()
}

pub fn forge_query_graph_read_proof_transition_manifest_digest() -> String {
    hash_parts(
        &GRAPH_READ_ACCESS_PROOF_TRANSITION_MANIFEST
            .iter()
            .map(ForgeQueryGraphReadProofTransitionManifestRow::digest_part)
            .collect::<Vec<_>>(),
    )
}

fn compile_fail_boundary_digest_parts() -> Vec<String> {
    GRAPH_READ_ACCESS_COMPILE_FAIL_TARGETS
        .iter()
        .map(|target| target.to_string())
        .chain(std::iter::once(
            forge_query_graph_read_proof_transition_manifest_digest(),
        ))
        .collect()
}
