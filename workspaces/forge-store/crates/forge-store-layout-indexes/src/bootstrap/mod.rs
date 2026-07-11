mod bootstrap_only_path;
mod catalog;
mod catalog_read_admission;
mod catalog_read_outcome;
mod denial;
mod facade;
mod root_discovery;
pub(crate) mod test_support;
#[cfg(test)]
mod tests;

pub use bootstrap_only_path::S8BootstrapOnlyAccessPath;
pub use catalog::S8BootstrapLayoutCatalog;
pub use catalog_read_admission::S8BootstrapCatalogReadAdmission;
pub(super) use catalog_read_outcome::issue_catalog_read;
pub use catalog_read_outcome::{S8BootstrapCatalogReadOutcome, S8BootstrapCatalogReadOutcomeView};
pub use denial::S8BootstrapOnlyAccessDenied;
pub use facade::{bootstrap_catalog, BootstrapCatalogFacade};
pub use root_discovery::S8MinimalRootDiscoveryLayout;
