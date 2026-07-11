use crate::{
    courtroom::foundational::store_json_residue_prelude_scan::certify_store_test_preludes_do_not_export_json,
    StoreJsonResidueDenial, StoreJsonResidueInventory,
};

pub fn certify_store_json_residue_inventory(
) -> Result<StoreJsonResidueInventory, StoreJsonResidueDenial> {
    let inventory = StoreJsonResidueInventory::from_current_sources()?;
    certify_store_test_preludes_do_not_export_json()?;
    Ok(inventory)
}
