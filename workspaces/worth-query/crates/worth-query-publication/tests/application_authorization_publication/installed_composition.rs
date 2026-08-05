#[path = "installed_composition/contract.rs"]
mod contract;
#[path = "installed_composition/declaration.rs"]
mod declaration;
#[path = "installed_composition/world.rs"]
mod world;

pub(super) use world::{real_denial, CompositionScenario};
