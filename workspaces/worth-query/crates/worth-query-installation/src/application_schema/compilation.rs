use worth_foundational::facade::CanonicalDigestDerivationDenial;

use super::WorthQueryInstalledApplicationSchema;
use crate::application_capability::WorthQueryApplicationCapabilityInstallationDenial;

pub(crate) enum ApplicationSchemaCompilationDenial {
    Capability(WorthQueryApplicationCapabilityInstallationDenial),
    Canonical(CanonicalDigestDerivationDenial),
}

impl<Schema> std::fmt::Debug for WorthQueryInstalledApplicationSchema<Schema> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("WorthQueryInstalledApplicationSchema")
            .field("owner", &self.package_authority.owner())
            .field("schema_name", &self.schema_name)
            .field("schema_identity", &self.schema_identity)
            .finish_non_exhaustive()
    }
}
