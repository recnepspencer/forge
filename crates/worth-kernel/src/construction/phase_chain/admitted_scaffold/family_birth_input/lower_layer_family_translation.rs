#[cfg(test)]
use super::super::super::request::PrimitiveConstructionFamily;
#[cfg(test)]
use worth_primitives::PrimitiveConstructionFamilyKey;

#[cfg(test)]
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
