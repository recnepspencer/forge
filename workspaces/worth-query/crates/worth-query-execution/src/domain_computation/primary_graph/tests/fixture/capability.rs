#[path = "capability/contract.rs"]
mod contract;
#[path = "capability/declaration.rs"]
mod declaration;

pub(super) use contract::install;
pub use declaration::*;
