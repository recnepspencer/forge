use forge_store_contracts::{DurableArtifactFamilyId, WalRecordFamily};
use forge_store_security::StoreCurrentSecurityScopeWitnessSet;
use forge_store_wal::StoreWalRecordIdentity;

use super::denial::{map_key_domain_denial, LsmMaintenanceAdmissionDenied};

pub(super) fn admit_wal_operation_context(
    security: &StoreCurrentSecurityScopeWitnessSet,
    record_family: WalRecordFamily,
    record_identity: StoreWalRecordIdentity,
) -> Result<
    (
        crate::AdmittedPhysicalArtifactFamily,
        crate::AdmittedConcretePhysicalKey,
    ),
    LsmMaintenanceAdmissionDenied,
> {
    let declarations = crate::layout_declarations();
    let declaration = declarations
        .declaration(DurableArtifactFamilyId::PublicationWalIntent)
        .map_err(|_| LsmMaintenanceAdmissionDenied::ArtifactFamily)?;
    let family = declarations
        .admit_physical_artifact_family(declaration, security)
        .into_result()
        .map_err(|_| LsmMaintenanceAdmissionDenied::ArtifactFamily)?;
    let key_domain = declarations
        .admit_physical_key_domain(family, security)
        .into_result()
        .map_err(map_key_domain_denial)?;
    let concrete_key = declarations
        .admit_wal_key(key_domain, record_family, record_identity)
        .map_err(|_| LsmMaintenanceAdmissionDenied::ConcreteKey)?;
    Ok((family, concrete_key))
}
