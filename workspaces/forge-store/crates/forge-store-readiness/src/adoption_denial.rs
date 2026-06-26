use crate::FoundationalAdoptionFamily;
use forge_foundational::canonicalization_api::lower_lane::basis::CanonicalBasisConstructionDenial;
use forge_foundational::canonicalization_api::lower_lane::digest::CanonicalDigestDerivationDenial;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FoundationalAdoptionDenial {
    MissingRequiredFamily(FoundationalAdoptionFamily),
    DuplicateFamily(FoundationalAdoptionFamily),
    WrongRoadmapScope,
    CanonicalBasisDenied(CanonicalBasisConstructionDenial),
    CanonicalDigestDenied(CanonicalDigestDerivationDenial),
}
