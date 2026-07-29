use crate::facade::prepared_application_authority::WorthUiPreparedApplicationAuthority;
use crate::runtime::observation::{UiAuthoredFactDeclarationSide, UiChangeClassificationDenial};

pub(super) fn resolve(
    authority: &WorthUiPreparedApplicationAuthority,
    provenance_digest: u64,
    side: UiAuthoredFactDeclarationSide,
) -> Result<&str, UiChangeClassificationDenial> {
    let identities = authority.authored_identity_bases_for_provenance(provenance_digest);
    match identities {
        [] => Err(
            UiChangeClassificationDenial::MissingAuthoredFactDeclaration {
                side,
                provenance_digest,
            },
        ),
        [identity] => Ok(identity.as_ref()),
        _ => Err(
            UiChangeClassificationDenial::AmbiguousAuthoredFactDeclaration {
                side,
                provenance_digest,
                matches: identities.len(),
            },
        ),
    }
}

pub(super) fn resolve_matched<'candidate>(
    predecessor: &WorthUiPreparedApplicationAuthority,
    candidate: &'candidate WorthUiPreparedApplicationAuthority,
    predecessor_provenance: u64,
    candidate_provenance: u64,
) -> Result<&'candidate str, UiChangeClassificationDenial> {
    let predecessor_identity = resolve(
        predecessor,
        predecessor_provenance,
        UiAuthoredFactDeclarationSide::Predecessor,
    )?;
    let candidate_identity = resolve(
        candidate,
        candidate_provenance,
        UiAuthoredFactDeclarationSide::Candidate,
    )?;
    if predecessor_identity != candidate_identity {
        return Err(
            UiChangeClassificationDenial::AuthoredFactDeclarationIdentityMismatch {
                predecessor: predecessor_identity.into(),
                candidate: candidate_identity.into(),
            },
        );
    }
    Ok(candidate_identity)
}
