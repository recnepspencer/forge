//! Immutable (resting) property layer.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::hash::Hash;

/// Immutable (resting) storage for a single property.
///
/// Maps typed keys directly to values. This is the serialized form that
/// persists between transactions.
///
/// # Key Requirements
/// Keys must be `Eq + Hash + Copy` (topology handles satisfy this).
///
/// # Value Requirements
/// Values should be `Clone` if you intend to use `PropertyPatch::get_mut()`.
/// Values must be `Serialize + Deserialize` if the layer will be persisted.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PropertyLayer<K: Eq + Hash, V> {
    pub(in crate::storage) data: HashMap<K, V>,
}
