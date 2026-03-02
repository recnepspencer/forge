//! Handler for `Command::AddBlock`.
//!
//! DOMAIN: Translates the schema's `AddBlock` command into a
//! `NativeFeature::primitive`. Zero construction logic — the feature
//! owns its own parameter normalization.

use super::super::native_feature::NativeFeature;
use crate::operations::primitives::MakePrimitiveFeature;

/// Create a block feature from origin + dimensions.
///
/// The origin→center math is delegated to `MakePrimitiveFeature::block_from_origin`.
pub fn add_block(origin: [f64; 3], dimensions: [f64; 3]) -> NativeFeature {
    let feature = MakePrimitiveFeature::block_from_origin("block", origin, dimensions);
    NativeFeature::primitive("block", feature)
}
