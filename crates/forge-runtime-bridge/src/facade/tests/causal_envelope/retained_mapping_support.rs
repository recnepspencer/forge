use super::*;
use crate::facade::{
    BridgeCausalEvidenceBinding, BridgeCausalEvidenceFamily, BridgeCausalEvidenceOwner,
    BridgeCausalEvidenceReference, BridgeMergeAuthorityBasis, BridgeMergeAuthorityBasisKind,
    BridgeMergeConsumptionClass, BridgeMergeOntologyMappingSurface, BridgeMergeParentOrderProof,
    BridgeMergeStructuralAdvisoryDisposition, MergeHistoryDeclaration,
    MergeHistoryDeclarationIdentity,
};
use std::sync::Arc;

#[derive(Clone)]
struct CausalLineageSource;

impl crate::adapter::ContinuityLineageSource for CausalLineageSource {
    fn historical_lineage(
        &self,
        request: crate::adapter::BridgeHistoricalLineageRequest,
    ) -> Result<
        crate::adapter::BridgeHistoricalLineageAuthority,
        crate::error::BridgeLineageSourceError,
    > {
        crate::adapter::BridgeHistoricalLineageAuthority::try_new(
            request.authority_basis().clone(),
            vec![Arc::from("lineage:causal-successor")],
            vec![Arc::from("record:causal-successor")],
            vec![1],
        )
    }
}

pub(super) fn bridge_reference(
    family: BridgeCausalEvidenceFamily,
    identity: &str,
) -> BridgeCausalEvidenceReference {
    BridgeCausalEvidenceReference::new(BridgeCausalEvidenceOwner::RuntimeBridge, family, identity)
        .expect("bridge reference should be valid")
}

pub(super) fn query_observation_reference(identity: &str) -> BridgeCausalEvidenceReference {
    BridgeCausalEvidenceReference::new(
        BridgeCausalEvidenceOwner::Query,
        BridgeCausalEvidenceFamily::QueryObservation,
        identity,
    )
    .expect("query observation reference should be valid")
}

pub(super) fn binding_for<'a>(
    bindings: &'a [BridgeCausalEvidenceBinding],
    family: BridgeCausalEvidenceFamily,
    reference_identity: &str,
) -> &'a BridgeCausalEvidenceBinding {
    bindings
        .iter()
        .find(|binding| {
            binding.owner() == BridgeCausalEvidenceOwner::RuntimeBridge
                && binding.family() == family
                && binding.reference_identity() == reference_identity
        })
        .expect("expected retained bridge binding should be present")
}

pub(super) fn digest(label: &str, parts: &[&str]) -> String {
    use sha2::{Digest, Sha256};

    let mut canonical = String::from(label);
    for part in parts {
        canonical.push('|');
        canonical.push_str(part);
    }
    let digest = Sha256::digest(canonical.as_bytes());
    format!("{label}:sha256:{digest:x}")
}

pub(super) fn registered_causal_merge(id: &str) -> MergeHistoryDeclaration {
    MergeHistoryDeclaration::new(
        MergeHistoryDeclarationIdentity::new(id),
        BridgeMergeConsumptionClass::AspectReconciliationMerge,
        BridgeMergeOntologyMappingSurface::direct_phase_m9_0("rel-merge-v1"),
        BridgeMergeAuthorityBasis::new(
            BridgeMergeAuthorityBasisKind::OrderedMergeCommit,
            format!("merge-artifact:{id}"),
            "rel-merge-v1",
            "schema-policy-v1",
            BridgeMergeParentOrderProof::new(vec![
                TruthCommitIdentity::new("parent-a"),
                TruthCommitIdentity::new("parent-b"),
            ]),
        ),
    )
    .with_structural_advisory(BridgeMergeStructuralAdvisoryDisposition::AdvisoryConsistent)
}

pub(super) fn branch_comparison_declaration(id: &str) -> StructuralIdentityDeclaration {
    StructuralIdentityDeclaration::branch_comparison(
        StructuralIdentityDeclarationIdentity::new(id),
        StructuralSchemaIdentity::new("schema:geometry"),
        StructuralFingerprintEquivalenceContract::new(
            StructuralSchemaIdentity::new("schema:geometry"),
            StructuralFingerprintFamily::BranchComparisonFingerprint,
            "geometry-branch-v1",
            StructuralFingerprintNormalizationRule::SchemaDeclaredCanonicalForm,
            StructuralFingerprintOrderingRule::SchemaDeclaredCanonicalOrder,
            StructuralFingerprintOmissionPolicy::SchemaDeclaredOmissionPolicy,
        ),
        StructuralTruthViewBasis::explicit_branch_pair(
            BridgeTruthViewSelector::branch_snapshot(
                TruthBranchIdentity::new("left"),
                TruthSnapshotIdentity::new("snapshot-a"),
            ),
            BridgeTruthViewSelector::branch_snapshot(
                TruthBranchIdentity::new("right"),
                TruthSnapshotIdentity::new("snapshot-a"),
            ),
        ),
    )
}

pub(super) fn retained_runtime(
    merge_declaration: MergeHistoryDeclaration,
    branch_declaration: StructuralIdentityDeclaration,
) -> RuntimeBridge {
    RuntimeBridgeBuilder::new()
        .with_policy(BridgeRuntimePolicy::default())
        .with_relational_source(StaticSource)
        .with_source_adapter(StaticSourceAdapter)
        .with_truth_branch_head_source(StaticSource)
        .with_continuity_lineage_source(CausalLineageSource)
        .with_signal_sink(StaticSink)
        .register_source(registered_source(
            "source:analysis-snapshot",
            BridgeTruthViewSelector::branch_snapshot(
                TruthBranchIdentity::new("analysis"),
                TruthSnapshotIdentity::new("snapshot-a"),
            ),
            vec![
                BridgeSourceCapability::SnapshotRead,
                BridgeSourceCapability::BranchRead,
            ],
        ))
        .register_source(registered_source(
            "source:analysis-history",
            BridgeTruthViewSelector::historical_commit(
                TruthBranchIdentity::new("analysis"),
                TruthCommitIdentity::new("commit-a"),
            ),
            vec![
                BridgeSourceCapability::SnapshotRead,
                BridgeSourceCapability::HistoricalRead,
                BridgeSourceCapability::BranchRead,
                BridgeSourceCapability::ReplayCompatibleRead,
            ],
        ))
        .register_structural(registered_structural(
            "structural:analysis-snapshot",
            StructuralFingerprintFamily::TopologyFingerprint,
            StructuralTruthViewBasis::explicit_snapshot(BridgeTruthViewSelector::branch_snapshot(
                TruthBranchIdentity::new("analysis"),
                TruthSnapshotIdentity::new("snapshot-a"),
            )),
        ))
        .register_structural(branch_declaration)
        .register_merge(merge_declaration)
        .register_mapping(BridgeMappingRegistration::new(
            BridgeMappingId::new("mapping"),
            TruthPatchScope::new(
                MappingSelector::exact("entity-1"),
                MappingSelector::exact("profile"),
                MappingSelector::exact("name"),
            ),
            SignalInvalidationScope::new("signal:profile"),
            CoarseRoutingMode::Direct,
        ))
        .build()
        .expect("retained mapping runtime should build")
}
