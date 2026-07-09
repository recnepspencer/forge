use crate::structural::{
    StructuralFingerprintEquivalenceContract, StructuralFingerprintFamily,
    StructuralFingerprintNormalizationRule, StructuralFingerprintOmissionPolicy,
    StructuralFingerprintOrderingRule, StructuralIdentityDeclaration,
    StructuralIdentityDeclarationIdentity, StructuralSchemaIdentity, StructuralTruthViewBasis,
};

pub(in crate::facade::tests) fn registered_structural(
    id: &str,
    family: StructuralFingerprintFamily,
    truth_view_basis: StructuralTruthViewBasis,
) -> StructuralIdentityDeclaration {
    StructuralIdentityDeclaration::advisory_remap(
        StructuralIdentityDeclarationIdentity::admit_bridge_owned(id),
        StructuralSchemaIdentity::admit_bridge_owned("schema:geometry"),
        StructuralFingerprintEquivalenceContract::new(
            StructuralSchemaIdentity::admit_bridge_owned("schema:geometry"),
            family,
            "geometry-v1",
            StructuralFingerprintNormalizationRule::SchemaDeclaredCanonicalForm,
            StructuralFingerprintOrderingRule::SchemaDeclaredCanonicalOrder,
            StructuralFingerprintOmissionPolicy::SchemaDeclaredOmissionPolicy,
        ),
        truth_view_basis,
    )
}
