use forge_foundational::{
    AbsenceLaw, AspectContract, AspectMask, CanonicalFieldPath, FieldDeclaration, FieldRequirement,
    MaskAdmissibilityDenial, MutationMask, ProjectionMask, ScalarAspectType, StructAspectShape,
};

use crate::support::{field, identity, key, revision};

#[test]
fn masks_are_mode_typed_and_shape_admitted() {
    let title = field("title");
    let shape = StructAspectShape::new([FieldDeclaration::new(
        title.clone(),
        ScalarAspectType::String,
        FieldRequirement::Required,
        AbsenceLaw::Required,
        forge_foundational::AspectEvolutionPolicy::ExplicitBreakRequired,
    )])
    .expect("unique fields");
    let struct_contract =
        AspectContract::struct_aspect(key("task.summary"), identity(2), revision(1), shape);
    let field_mask = AspectMask::<ProjectionMask>::new([CanonicalFieldPath::single(title.clone())]);

    assert_eq!(struct_contract.admits_projection_mask(&field_mask), Ok(()));

    let scalar_contract = AspectContract::scalar(
        key("task.title"),
        identity(3),
        revision(1),
        ScalarAspectType::String,
    );
    assert_eq!(
        scalar_contract.admits_projection_mask(&field_mask),
        Err(MaskAdmissibilityDenial::FieldMaskRequiresStruct)
    );

    let mutation_whole = AspectMask::<MutationMask>::whole_aspect();
    assert_eq!(
        scalar_contract.admits_mutation_mask(&mutation_whole),
        Ok(())
    );
}
