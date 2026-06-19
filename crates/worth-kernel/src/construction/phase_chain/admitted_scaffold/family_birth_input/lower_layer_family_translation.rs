use super::super::super::request::PrimitiveConstructionFamily;
use worth_primitives::PrimitiveConstructionFamilyKey;

pub(super) fn to_lower_layer_birth_family(
    family: PrimitiveConstructionFamily,
) -> PrimitiveConstructionFamilyKey {
    match family {
        PrimitiveConstructionFamily::SimplexSolid => PrimitiveConstructionFamilyKey::SimplexSolid,
        PrimitiveConstructionFamily::Orthotope => PrimitiveConstructionFamilyKey::Orthotope,
        PrimitiveConstructionFamily::RegularPrism => PrimitiveConstructionFamilyKey::RegularPrism,
        PrimitiveConstructionFamily::RegularPyramid => {
            PrimitiveConstructionFamilyKey::RegularPyramid
        }
        PrimitiveConstructionFamily::WireBody => PrimitiveConstructionFamilyKey::WireBody,
        PrimitiveConstructionFamily::ShellWithHole => PrimitiveConstructionFamilyKey::ShellWithHole,
    }
}
