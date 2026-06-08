mod binding;
mod capability;
mod common_paths;
mod compatibility;
mod compatibility_debt;
mod eligibility;
mod intent;
mod normalization;
mod projection;
mod scoped;

pub use binding::*;
pub use capability::*;
pub use common_paths::*;
pub use compatibility::*;
pub use compatibility_debt::*;
pub use eligibility::*;
pub use intent::*;
pub use normalization::*;
pub use projection::*;
pub use scoped::*;

#[cfg(test)]
mod tests;
