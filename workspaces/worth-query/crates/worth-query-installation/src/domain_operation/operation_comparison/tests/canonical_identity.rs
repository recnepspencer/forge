use worth_foundational::facade::{
    AbsenceLaw, AspectContract, AspectContractRevision, AspectEvolutionPolicy, AspectIdentity,
    AspectKey, AspectMask, CanonicalFieldPath, FieldDeclaration, FieldKey, FieldRequirement,
    ProjectionMask, ScalarAspectType, StructAspectShape,
};

use crate::domain_operation::*;

use super::{operation, semantics};

#[test]
fn graph_read_identity_frames_mask_paths_without_delimiter_collisions() {
    let contract = delimiter_contract();
    let single_pipe = WorthQueryOperationNativeProjectionContract::new(
        contract.clone(),
        AspectMask::<ProjectionMask>::new([CanonicalFieldPath::single(
            FieldKey::new("a|b").unwrap(),
        )]),
    )
    .unwrap();
    let separate_paths = WorthQueryOperationNativeProjectionContract::new(
        contract,
        AspectMask::<ProjectionMask>::new([
            CanonicalFieldPath::single(FieldKey::new("a").unwrap()),
            CanonicalFieldPath::single(FieldKey::new("b").unwrap()),
        ]),
    )
    .unwrap();

    let mut left = semantics(1, "Entity");
    left.graph_reads = graph_read(single_pipe);
    let mut right = semantics(1, "Entity");
    right.graph_reads = graph_read(separate_paths);
    let left = operation("inspect", 1, left);
    let right = operation("inspect", 1, right);

    assert_ne!(left.canonical_identity(), right.canonical_identity());
}

fn graph_read(
    projection: WorthQueryOperationNativeProjectionContract,
) -> WorthQueryOperationGraphReadContract {
    WorthQueryOperationGraphReadContract::DeclaredDomain {
        roles: vec![WorthQueryDomainOperationGraphReadRole {
            role: "model".into(),
            participation: WorthQueryOperationGraphParticipation::PrimaryLogicalGraph,
            access: WorthQueryOperationGraphAccess::Project,
            semantic_reads: vec![projection],
        }],
    }
}

fn delimiter_contract() -> AspectContract {
    let fields = ["a|b", "a", "b"].map(|key| {
        FieldDeclaration::new(
            FieldKey::new(key).unwrap(),
            ScalarAspectType::String,
            FieldRequirement::Required,
            AbsenceLaw::Required,
            AspectEvolutionPolicy::ExplicitBreakRequired,
        )
        .unwrap()
    });
    AspectContract::struct_aspect(
        AspectKey::new("delimiter-test").unwrap(),
        AspectIdentity(1603),
        AspectContractRevision(1),
        StructAspectShape::new(fields).unwrap(),
    )
}
