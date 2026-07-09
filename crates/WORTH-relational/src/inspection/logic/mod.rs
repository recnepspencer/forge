mod access;
mod commit;
mod connectivity;
mod graph;
mod historical;
#[path = "merge_support/mod.rs"]
mod merge_support;
mod retention;
mod structural_identity;

#[cfg(test)]
mod tests;

pub use access::InspectionAccess;
#[cfg(test)]
pub(crate) use merge_support::support_inspection_witness;
