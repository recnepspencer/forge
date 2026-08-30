mod dense;
#[cfg(test)]
mod frontier;

pub use dense::DenseBitset;
#[cfg(test)]
pub(crate) use frontier::BitsetFrontier;
