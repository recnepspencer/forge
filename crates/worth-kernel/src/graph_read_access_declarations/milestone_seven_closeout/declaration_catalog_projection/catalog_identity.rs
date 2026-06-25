use crate::graph_read_access_declarations::WorthGraphReadAdmissionPostureRecord;

use super::super::proof_digest::stable_digest;
use super::read_family_identity::WorthGraphReadDeclarationReadFamilyIdentity;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct WorthGraphReadDeclarationCatalogIdentityProjection {
    catalog_digest: String,
    read_family_identities: Vec<WorthGraphReadDeclarationReadFamilyIdentity>,
}

impl WorthGraphReadDeclarationCatalogIdentityProjection {
    pub(crate) fn from_posture_records(records: &[WorthGraphReadAdmissionPostureRecord]) -> Self {
        let read_family_identities = records
            .iter()
            .map(WorthGraphReadDeclarationReadFamilyIdentity::from_posture_record)
            .collect::<Vec<_>>();
        let catalog_digest = stable_digest(
            &read_family_identities
                .iter()
                .map(|identity| format!("read_family:{}", identity.identity_digest()))
                .collect::<Vec<_>>(),
        );
        Self {
            catalog_digest,
            read_family_identities,
        }
    }

    pub(crate) fn catalog_digest(&self) -> &str {
        &self.catalog_digest
    }

    pub(crate) fn read_family_identities(&self) -> &[WorthGraphReadDeclarationReadFamilyIdentity] {
        &self.read_family_identities
    }
}
