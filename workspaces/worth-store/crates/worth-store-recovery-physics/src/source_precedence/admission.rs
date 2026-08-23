use worth_store_physical_format::{
    store_namespace::StableStoreIdentity, DurablePhysicalRootManifest, DurableRootSelector,
    RootSelectorRole,
};

use super::{
    PhysicalRootCandidateDenial, PhysicalRootSlotObservation, PhysicalRootSourceCandidate,
};

pub fn admit_physical_root_slot(
    store: StableStoreIdentity,
    role: RootSelectorRole,
    selector_bytes: Option<&[u8]>,
    manifest_bytes: Option<&[u8]>,
    maximum_manifest_entries: u16,
) -> PhysicalRootSlotObservation {
    let Some(selector_bytes) = selector_bytes else {
        return PhysicalRootSlotObservation::Absent;
    };
    let selector = match DurableRootSelector::decode(selector_bytes) {
        Ok(selector) => selector,
        Err(denial) => {
            return PhysicalRootSlotObservation::Rejected {
                denial: PhysicalRootCandidateDenial::SelectorFormat(denial),
                selector: None,
            };
        }
    };
    if selector.store_identity() != store {
        return rejected(selector, PhysicalRootCandidateDenial::ForeignStore);
    }
    if selector.role() != role {
        return rejected(selector, PhysicalRootCandidateDenial::WrongRole);
    }
    let Some(manifest_bytes) = manifest_bytes else {
        return rejected(selector, PhysicalRootCandidateDenial::RootMissing);
    };
    let (manifest, format) =
        match DurablePhysicalRootManifest::decode(manifest_bytes, maximum_manifest_entries) {
            Ok(decoded) => decoded,
            Err(denial) => {
                return rejected(selector, PhysicalRootCandidateDenial::RootFormat(denial))
            }
        };
    match PhysicalRootSourceCandidate::admit(selector, manifest, format) {
        Ok(candidate) => PhysicalRootSlotObservation::Admitted(candidate),
        Err(denial) => rejected(selector, denial),
    }
}

fn rejected(
    selector: DurableRootSelector,
    denial: PhysicalRootCandidateDenial,
) -> PhysicalRootSlotObservation {
    PhysicalRootSlotObservation::Rejected {
        denial,
        selector: Some(selector),
    }
}
