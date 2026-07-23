use std::collections::BTreeMap;

use crate::domain_computation::WorthQueryPortableArtifactContract;

use super::{WorthQueryInstalledPackageIndexDenial, WorthQueryInstalledPackageIndexDenialKind};

type ArtifactContractKey = (String, String, u32, u32);
type ArtifactContractSlot = (String, u32, u32);
type ArtifactContractSlotAdmission = (String, String);

pub(super) fn admit_artifact_contract(
    contracts: &mut BTreeMap<ArtifactContractKey, WorthQueryPortableArtifactContract>,
    slots: &mut BTreeMap<ArtifactContractSlot, ArtifactContractSlotAdmission>,
    owner: &str,
    contract: &WorthQueryPortableArtifactContract,
) -> Result<(), WorthQueryInstalledPackageIndexDenial> {
    let family = contract.family().as_str();
    let slot = (
        family.to_string(),
        contract.schema_version().get(),
        contract.protocol_version().get(),
    );
    if let Some((existing_identity, existing_owner)) = slots.get(&slot) {
        if existing_identity != contract.identity().as_str() {
            return Err(WorthQueryInstalledPackageIndexDenial::new(
                WorthQueryInstalledPackageIndexDenialKind::ConflictingArtifactContract,
                format!("{family}:{existing_owner}:{owner}"),
            ));
        }
    } else {
        slots.insert(
            slot,
            (contract.identity().as_str().to_string(), owner.to_string()),
        );
    }
    contracts.insert(
        (
            owner.to_string(),
            family.to_string(),
            contract.schema_version().get(),
            contract.protocol_version().get(),
        ),
        contract.clone(),
    );
    Ok(())
}
