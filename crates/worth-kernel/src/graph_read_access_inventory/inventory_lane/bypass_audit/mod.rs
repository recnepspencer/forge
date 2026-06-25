mod adoption;
mod adoption_report;
mod audit;
mod required_root_coverage;
mod residue_report_row;
mod source_inventory;

#[cfg(test)]
mod tests;

pub(in crate::graph_read_access_inventory::inventory_lane) use adoption::certify_graph_read_bypass_adoption;
pub use adoption_report::WorthGraphReadBypassAdoptionReport;
