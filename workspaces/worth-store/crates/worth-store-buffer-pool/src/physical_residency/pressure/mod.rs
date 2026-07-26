mod admission;
mod runtime_denial;

pub use admission::PhysicalResidencyLimitsBuilder;
pub(in crate::physical_residency) use runtime_denial::PhysicalResidencyPressureDemand;
pub use runtime_denial::PhysicalResidencyPressureDenial;

#[cfg(test)]
mod tests;
