use crate::construction::request::PrimitiveConstructionFamily;
use worth_spatial::facade::PrimitiveConstructionBirthFamily;

pub(super) fn to_spatial_family(
    family: PrimitiveConstructionFamily,
) -> PrimitiveConstructionBirthFamily {
    match family {
        PrimitiveConstructionFamily::SimplexSolid => PrimitiveConstructionBirthFamily::SimplexSolid,
        PrimitiveConstructionFamily::Orthotope => PrimitiveConstructionBirthFamily::Orthotope,
        PrimitiveConstructionFamily::RegularPrism => PrimitiveConstructionBirthFamily::RegularPrism,
        PrimitiveConstructionFamily::RegularPyramid => {
            PrimitiveConstructionBirthFamily::RegularPyramid
        }
        PrimitiveConstructionFamily::WireBody => PrimitiveConstructionBirthFamily::WireBody,
        PrimitiveConstructionFamily::ShellWithHole => {
            PrimitiveConstructionBirthFamily::ShellWithHole
        }
    }
}
