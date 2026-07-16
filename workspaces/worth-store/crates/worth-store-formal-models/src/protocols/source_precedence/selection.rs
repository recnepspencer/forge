use super::SourcePrecedenceDenial;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SourceAuthorityPosture {
    AdmittedAuthority,
    AdvisoryOnly,
    DerivedLocator,
    ReplayHelper,
    Quarantined,
}

pub fn require_selectable_source(
    posture: SourceAuthorityPosture,
) -> Result<(), SourcePrecedenceDenial> {
    match posture {
        SourceAuthorityPosture::AdmittedAuthority => Ok(()),
        SourceAuthorityPosture::Quarantined => {
            Err(SourcePrecedenceDenial::QuarantinedSourceCannotBeSelected)
        }
        SourceAuthorityPosture::DerivedLocator | SourceAuthorityPosture::ReplayHelper => {
            Err(SourcePrecedenceDenial::DerivedSourceCannotBeAuthority)
        }
        SourceAuthorityPosture::AdvisoryOnly => Err(SourcePrecedenceDenial::CandidateNotAdmitted),
    }
}
