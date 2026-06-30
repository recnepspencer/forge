use super::catalog_kernel::kernel_rows;
use super::catalog_spatial::spatial_rows;
use super::catalog_topo::topo_rows;
use super::error::CompiledProductReuseInventoryError;
use super::report::CompiledProductReuseInventoryReport;

pub fn current_compiled_product_reuse_inventory(
) -> Result<CompiledProductReuseInventoryReport, CompiledProductReuseInventoryError> {
    let mut rows = Vec::new();
    rows.extend(topo_rows());
    rows.extend(spatial_rows());
    rows.extend(kernel_rows());
    Ok(CompiledProductReuseInventoryReport::new(rows))
}
