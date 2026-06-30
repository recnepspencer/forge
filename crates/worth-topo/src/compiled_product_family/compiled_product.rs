use schema::facade::platform::authority::compiled_product_semantic_graph::{
    admit_compiled_product_authority_truth_identity_with_coordinates,
    admit_compiled_product_equivalence_policy_identity, admit_compiled_product_identity,
    admit_compiled_product_rebuild_denial_identity, admit_compiled_product_reuse_decision_identity,
    CompiledProductAuthorityInstanceCoordinate, CompiledProductAuthorityTruthIdentity,
    CompiledProductEquivalencePolicyIdentity, CompiledProductIdentity,
    CompiledProductLocalityFootprintIdentity, CompiledProductRebuildDenialIdentity,
    CompiledProductReuseDecisionIdentity,
};
use serde::{Deserialize, Serialize};
use worth_primitives::{truth_digest_parts, TruthDigestScope};

use crate::derived_topology::materialized_graph::MaterializedTopologyView;
use crate::derived_topology::traversal_views::InterpretedTopologyView;
use crate::validation::DerivedTopologyValidationReport;

use super::admitted_input::TopologyCompiledProductFamilyAdmittedInput;
use super::declaration::TopologyCompiledProductFamilyDeclaration;
use super::error::{TopologyCompiledProductFamilyError, TopologyCompiledProductFamilyErrorKind};
use super::family_identity::TopologyCompiledProductFamilyIdentity;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeterministicDigest {
    pub algorithm: String,
    pub digest_hex: String,
    pub row_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct TopologyCompiledProductLoweredIdentity {
    family_identity: TopologyCompiledProductFamilyIdentity,
    family_digest: String,
    authority_truth_identity: CompiledProductAuthorityTruthIdentity,
    compiled_product_identity: CompiledProductIdentity,
    equivalence_policy_identity: CompiledProductEquivalencePolicyIdentity,
    reuse_decision_identity: CompiledProductReuseDecisionIdentity,
}

impl TopologyCompiledProductLoweredIdentity {
    pub fn family_identity(&self) -> TopologyCompiledProductFamilyIdentity {
        self.family_identity
    }

    pub fn family_digest(&self) -> &str {
        &self.family_digest
    }

    pub fn authority_truth_identity(&self) -> &CompiledProductAuthorityTruthIdentity {
        &self.authority_truth_identity
    }

    pub fn compiled_product_identity(&self) -> &CompiledProductIdentity {
        &self.compiled_product_identity
    }

    pub fn equivalence_policy_identity(&self) -> &CompiledProductEquivalencePolicyIdentity {
        &self.equivalence_policy_identity
    }

    pub fn reuse_decision_identity(&self) -> &CompiledProductReuseDecisionIdentity {
        &self.reuse_decision_identity
    }

    pub fn rebuild_required_identity(
        &self,
        denial_reason: &str,
    ) -> Result<CompiledProductRebuildDenialIdentity, TopologyCompiledProductFamilyError> {
        admit_compiled_product_rebuild_denial_identity(
            &self.compiled_product_identity,
            denial_reason,
        )
        .map_err(|error| {
            TopologyCompiledProductFamilyError::new(
                TopologyCompiledProductFamilyErrorKind::SchemaVocabularyAdmissionFailed,
                format!("compiled-product rebuild denial admission failed: {error:?}"),
            )
        })
    }
}

pub fn lower_topology_compiled_product_identity(
    declaration: &TopologyCompiledProductFamilyDeclaration,
    admitted_input: &TopologyCompiledProductFamilyAdmittedInput,
    _materialized: &MaterializedTopologyView,
    _interpreted: &InterpretedTopologyView,
    _validation: &DerivedTopologyValidationReport,
) -> Result<TopologyCompiledProductLoweredIdentity, TopologyCompiledProductFamilyError> {
    let authority_truth_identity =
        admit_compiled_product_authority_truth_identity_with_coordinates(
            "worth-topo",
            admitted_input.truth_basis_digest_hex(),
            "derived-topology-truth",
            [
                CompiledProductAuthorityInstanceCoordinate::snapshot_identity(
                    admitted_input.authority_snapshot_id().to_string(),
                )
                .expect("derived topology snapshot authority coordinate"),
                CompiledProductAuthorityInstanceCoordinate::branch_identity(
                    admitted_input.authority_branch_id().to_string(),
                )
                .expect("derived topology branch authority coordinate"),
            ],
        )
        .map_err(|error| {
            TopologyCompiledProductFamilyError::new(
                TopologyCompiledProductFamilyErrorKind::SchemaVocabularyAdmissionFailed,
                format!("compiled-product authority truth admission failed: {error:?}"),
            )
        })?;
    let locality_identity = CompiledProductLocalityFootprintIdentity::invalidation_closure(
        admitted_input.locality_digest().to_string(),
    )
    .map_err(|error| {
        TopologyCompiledProductFamilyError::new(
            TopologyCompiledProductFamilyErrorKind::SchemaVocabularyAdmissionFailed,
            format!("compiled-product locality admission failed: {error:?}"),
        )
    })?;
    let compiled_product_identity = admit_compiled_product_identity(
        authority_truth_identity.clone(),
        locality_identity,
        None,
        None,
    );
    let equivalence_policy_identity = admit_compiled_product_equivalence_policy_identity(
        declaration.equivalence_policy_name(),
        declaration.equivalence_dimensions().iter().copied(),
    )
    .map_err(|error| {
        TopologyCompiledProductFamilyError::new(
            TopologyCompiledProductFamilyErrorKind::SchemaVocabularyAdmissionFailed,
            format!("compiled-product equivalence policy admission failed: {error:?}"),
        )
    })?;
    let reuse_decision_identity = admit_compiled_product_reuse_decision_identity(
        &compiled_product_identity,
        &equivalence_policy_identity,
        "ordinary-reuse-admitted",
    )
    .map_err(|error| {
        TopologyCompiledProductFamilyError::new(
            TopologyCompiledProductFamilyErrorKind::SchemaVocabularyAdmissionFailed,
            format!("compiled-product reuse decision admission failed: {error:?}"),
        )
    })?;

    Ok(TopologyCompiledProductLoweredIdentity {
        family_identity: declaration.identity(),
        family_digest: declaration.family_digest().to_string(),
        authority_truth_identity,
        compiled_product_identity,
        equivalence_policy_identity,
        reuse_decision_identity,
    })
}

pub fn topology_invalidation_closure_digest(
    authority_snapshot_id: u64,
    authority_branch_id: &str,
    touched_aspect_count: usize,
    triggered_invalidation_targets: &[schema::facade::platform::authority::DerivedInvalidationTarget],
) -> String {
    let mut target_rows = triggered_invalidation_targets
        .iter()
        .map(|target| format!("{target:?}"))
        .collect::<Vec<_>>();
    target_rows.sort();
    truth_digest_parts(
        TruthDigestScope::ArtifactIdentity,
        &[
            "worth-topo:derived-invalidation-closure:v1".to_string(),
            format!("snapshot:{authority_snapshot_id}"),
            format!("branch:{authority_branch_id}"),
            format!("touched-aspects:{touched_aspect_count}"),
            format!("targets:{}", target_rows.join("|")),
        ],
    )
}

pub fn digest_materialized_topology_view(
    materialized: &MaterializedTopologyView,
) -> DeterministicDigest {
    digest_structured_value(materialized)
}

pub fn digest_interpreted_topology_view(
    interpreted: &InterpretedTopologyView,
) -> DeterministicDigest {
    digest_structured_value(interpreted)
}

pub fn digest_derived_validation_report(
    validation: &DerivedTopologyValidationReport,
) -> DeterministicDigest {
    digest_structured_value(validation)
}

fn digest_structured_value<T: serde::Serialize>(value: &T) -> DeterministicDigest {
    let json =
        serde_json::to_string(value).expect("derived parity serialization should be deterministic");
    let mut state: u64 = 0xcbf29ce484222325;
    let mut row_count = 0usize;
    row_count += 1;
    for byte in json.as_bytes() {
        state ^= u64::from(*byte);
        state = state.wrapping_mul(0x100000001b3);
    }

    DeterministicDigest {
        algorithm: "fnv1a64".to_string(),
        digest_hex: format!("{state:016x}"),
        row_count,
    }
}
