//! Deprecated Boolean operation implementations.
//!
//! DOMAIN: Legacy code preserved as reference during the boolean rewrite.
//! This module will be deleted once the new parametric pipeline passes
//! all brutality tests.
//!
//! DO NOT add new code here. New boolean work goes in `../parametric/`.

#[allow(dead_code)]
pub mod parametric;
#[allow(dead_code)]
pub mod ember;
#[allow(dead_code)]
pub mod shared;
mod debug;

#[cfg(test)]
pub mod test_helpers;
#[cfg(test)]
mod brutality;
#[cfg(test)]
mod tests;
