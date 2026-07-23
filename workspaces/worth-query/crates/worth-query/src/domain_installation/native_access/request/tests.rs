use worth_foundational::facade::{
    AbsenceLaw, AspectContract, AspectContractRevision, AspectEvolutionPolicy, AspectIdentity,
    AspectKey, CanonicalFieldPath, FieldDeclaration, FieldKey, FieldRequirement, ScalarAspectType,
    StructAspectShape,
};

use super::builder::map_contract_denial;
use crate::domain_installation::WorthQueryNativeProjectionRequestDenialKind as Kind;
use crate::projection_consumption::{
    DeclaredNativeAspectContractBasis, DeclaredNativeFactContract,
};

#[test]
fn request_contract_denials_map_to_exact_public_grammar() {
    let field = FieldKey::new("visible").unwrap();
    let other = FieldKey::new("other").unwrap();
    let structured = struct_contract(field.clone());
    let scalar = AspectContract::scalar(
        AspectKey::new("native.scalar").unwrap(),
        AspectIdentity(0x9150_00e1),
        AspectContractRevision(4),
        ScalarAspectType::String,
    );
    let opaque = AspectContract::opaque_token(
        AspectKey::new("native.opaque").unwrap(),
        AspectIdentity(0x9150_00e2),
        AspectContractRevision(5),
    );

    let cases = [
        (
            DeclaredNativeFactContract::whole(
                DeclaredNativeAspectContractBasis::new(structured.clone()),
                false,
            )
            .unwrap_err(),
            structured.clone(),
            None,
            Kind::WholeAspectNotProjected,
        ),
        (
            DeclaredNativeFactContract::field(
                DeclaredNativeAspectContractBasis::new(scalar.clone()),
                &[],
                true,
                &field,
            )
            .unwrap_err(),
            scalar,
            Some(field.clone()),
            Kind::FieldRequiresStruct,
        ),
        (
            DeclaredNativeFactContract::field(
                DeclaredNativeAspectContractBasis::new(structured.clone()),
                &[CanonicalFieldPath::single(other)],
                false,
                &field,
            )
            .unwrap_err(),
            structured,
            Some(field),
            Kind::FieldNotProjected,
        ),
        (
            DeclaredNativeFactContract::whole(
                DeclaredNativeAspectContractBasis::new(opaque.clone()),
                true,
            )
            .unwrap_err(),
            opaque,
            None,
            Kind::UnsupportedAspectShape,
        ),
    ];

    for (internal, contract, requested, expected) in cases {
        let public = map_contract_denial(internal, &contract, requested.clone());
        assert_eq!(public.kind(), expected);
        assert_eq!(public.contract_key(), contract.key());
        assert_eq!(public.contract_revision(), contract.revision());
        assert_eq!(public.requested_field(), requested.as_ref());
    }
}

fn struct_contract(field: FieldKey) -> AspectContract {
    AspectContract::struct_aspect(
        AspectKey::new("native.struct").unwrap(),
        AspectIdentity(0x9150_00e0),
        AspectContractRevision(3),
        StructAspectShape::new([FieldDeclaration::new(
            field,
            ScalarAspectType::String,
            FieldRequirement::Required,
            AbsenceLaw::Required,
            AspectEvolutionPolicy::ExplicitBreakRequired,
        )
        .unwrap()])
        .unwrap(),
    )
}
