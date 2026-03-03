//! Transactional overlay for PropertyLayer.

use std::collections::{HashMap, HashSet};
use std::hash::Hash;

use super::property_layer::PropertyLayer;

/// Transactional overlay for a `PropertyLayer`.
///
/// Accumulates inserts and removes. On `commit()`, flushes mutations
/// into the base layer. On `drop` (without commit), mutations are
/// silently discarded (rollback).
pub struct PropertyPatch<K: Eq + Hash, V> {
    pub(crate) base: PropertyLayer<K, V>,
    pub(crate) inserts: HashMap<K, V>,
    pub(crate) removes: HashSet<K>,
}
