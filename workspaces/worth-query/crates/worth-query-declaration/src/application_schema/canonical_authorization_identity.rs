use super::application_authorization_path_canonical_components;
use super::canonical_basis::ApplicationSchemaCanonicalBasis;
use super::ApplicationAuthorizationPath;

pub(super) fn append_authorization_path(
    basis: &mut ApplicationSchemaCanonicalBasis,
    prefix: &str,
    path: &ApplicationAuthorizationPath,
) {
    for component in application_authorization_path_canonical_components(path) {
        basis.value(
            format!("{prefix}.{}", component.locus()),
            component.value().clone(),
        );
    }
}
