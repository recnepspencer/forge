//! Historical compatibility imports retained only as inputs to the compatibility owner.

use super::super::{LegacyAccessPathBypass as Bypass, LegacySurfaceInventoryRow};
use super::row::owner_input as input;

pub(super) const ROWS: &[LegacySurfaceInventoryRow] = &[
    input("CompatibilityRegistry", Bypass::DeepImportPrecedent),
    input("CompatibilityRegistrySnapshot", Bypass::DeepImportPrecedent),
];
