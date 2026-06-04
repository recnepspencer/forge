use crate::facade::{
    AdmittedStructuralRegistry, BridgePreviewRetainedArtifactSchema, BridgePreviewSessionBasis,
    BridgePreviewSessionDeclaration, BridgePreviewSessionDeclarationIdentity,
    BridgePreviewStructuralBasis, BridgeRequestKind, BridgeSignalBranchIdentity,
    BridgeSourceCapability, BridgeSourceCapabilitySet, BridgeSpeculativeBranchBinding,
    BridgeSpeculativeBranchBindingIdentity, BridgeTruthViewSelector,
    StructuralFingerprintEquivalenceContract, StructuralFingerprintFamily,
    StructuralFingerprintNormalizationRule, StructuralFingerprintOmissionPolicy,
    StructuralFingerprintOrderingRule, StructuralIdentityDeclaration,
    StructuralIdentityDeclarationIdentity, StructuralSchemaIdentity, StructuralTruthViewBasis,
    TruthBranchIdentity, TruthSnapshotIdentity,
};

mod lifecycle;
mod promotion;
mod replay_and_diagnostics;
mod reuse;

fn preview_declaration() -> BridgePreviewSessionDeclaration {
    BridgePreviewSessionDeclaration::new(
        BridgePreviewSessionDeclarationIdentity::new("preview:analysis"),
        BridgeRequestKind::Preview,
        BridgeSpeculativeBranchBinding::new(
            BridgeSpeculativeBranchBindingIdentity::new("binding:analysis"),
            TruthBranchIdentity::new("truth:analysis"),
            BridgeSignalBranchIdentity::new("signal:analysis"),
        ),
        preview_session_basis(PreviewSessionBasisInput {
            truth_branch_identity: TruthBranchIdentity::new("truth:analysis"),
            snapshot_identity: TruthSnapshotIdentity::new("snapshot:analysis"),
        }),
    )
}

struct PreviewSessionBasisInput {
    truth_branch_identity: TruthBranchIdentity,
    snapshot_identity: TruthSnapshotIdentity,
}

fn preview_session_basis(input: PreviewSessionBasisInput) -> BridgePreviewSessionBasis {
    BridgePreviewSessionBasis::new(
        BridgeTruthViewSelector::branch_snapshot(
            input.truth_branch_identity,
            input.snapshot_identity,
        ),
        BridgeSourceCapabilitySet::new(vec![
            BridgeSourceCapability::SnapshotRead,
            BridgeSourceCapability::BranchRead,
        ]),
        BridgePreviewRetainedArtifactSchema::PreviewLifecycleArtifactsV1,
    )
}

struct StructuralBasisInput {
    schema_identity: StructuralSchemaIdentity,
    declaration_identity: StructuralIdentityDeclarationIdentity,
    truth_branch_identity: TruthBranchIdentity,
    snapshot_identity: TruthSnapshotIdentity,
    semantics_version: StructuralSemanticsVersion,
}

enum StructuralSemanticsVersion {
    Drift,
    HostileReentry,
}

impl StructuralSemanticsVersion {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Drift => "topology-drift",
            Self::HostileReentry => "topology-hostile-reentry",
        }
    }
}

fn structural_basis(input: StructuralBasisInput) -> BridgePreviewStructuralBasis {
    let schema = input.schema_identity;
    let declaration = StructuralIdentityDeclaration::advisory_remap(
        input.declaration_identity,
        schema.clone(),
        StructuralFingerprintEquivalenceContract::new(
            schema,
            StructuralFingerprintFamily::TopologyFingerprint,
            input.semantics_version.as_str(),
            StructuralFingerprintNormalizationRule::SchemaDeclaredCanonicalForm,
            StructuralFingerprintOrderingRule::SchemaDeclaredCanonicalOrder,
            StructuralFingerprintOmissionPolicy::SchemaDeclaredOmissionPolicy,
        ),
        StructuralTruthViewBasis::explicit_snapshot(BridgeTruthViewSelector::branch_snapshot(
            input.truth_branch_identity,
            input.snapshot_identity,
        )),
    );
    let registry =
        AdmittedStructuralRegistry::freeze(vec![declaration]).expect("structural basis admits");
    BridgePreviewStructuralBasis::from_admitted_contract(&registry.contracts()[0])
}
