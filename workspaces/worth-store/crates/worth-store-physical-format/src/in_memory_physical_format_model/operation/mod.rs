mod append;
mod physical_access;
mod rebuild_source;
pub(crate) mod restore;
mod root_publication;
mod scan;
mod shortcut_rejection;

use super::contract::InMemoryPhysicalFormatModelDenialKind;
use super::observation::{
    PlatformPhysicalDegradedExactScanReady, PlatformPhysicalDegradedExactScanReceipt,
    PlatformPhysicalDegradedExecutionObservation, PlatformPhysicalHiddenScanDenialReceipt,
    PlatformPhysicalLayoutAccessIntent, PlatformPhysicalLayoutAccessRequest,
};
use super::reports::PlatformPhysicalRootPublicationReport;
use super::storage::InMemoryPhysicalFormatModel;

mod denials {
    pub(super) use super::super::contract::{
        InMemoryPhysicalFormatModelDenial, InMemoryPhysicalFormatModelDenialKind,
    };
}

mod storage {
    pub(super) use super::super::storage::InMemoryPhysicalFormatModelStorage;
}
