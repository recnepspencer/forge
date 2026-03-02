//! Unit tests for PropertyLayer and PropertyPatch.

use super::schema::{PropertyLayer, PropertyPatch};

/// Local test handle — avoids dependency on forge-topo.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
struct TestHandle {
    index: u32,
    generation: u32,
}

impl TestHandle {
    fn new(index: u32, generation: u32) -> Self {
        Self { index, generation }
    }
}

#[test]
fn basic_crud() {
    let mut layer: PropertyLayer<TestHandle, String> = PropertyLayer::new();
    let key = TestHandle::new(0, 0);

    assert!(layer.get(key).is_none());
    layer.set(key, "hello".into());
    assert_eq!(layer.get(key).unwrap(), "hello");
    assert_eq!(layer.len(), 1);

    layer.remove(key);
    assert!(layer.get(key).is_none());
    assert!(layer.is_empty());
}

#[test]
fn patch_overrides_base() {
    let mut base: PropertyLayer<TestHandle, f64> = PropertyLayer::new();
    let k = TestHandle::new(0, 0);
    base.set(k, 1.0);

    let mut patch = PropertyPatch::new(base);
    assert_eq!(*patch.get(k).unwrap(), 1.0);

    patch.set(k, 2.0);
    assert_eq!(*patch.get(k).unwrap(), 2.0);

    let committed = patch.commit();
    assert_eq!(*committed.get(k).unwrap(), 2.0);
}

#[test]
fn patch_remove_hides_base() {
    let mut base: PropertyLayer<TestHandle, String> = PropertyLayer::new();
    let k = TestHandle::new(0, 0);
    base.set(k, "original".into());

    let mut patch = PropertyPatch::new(base);
    patch.remove(k);
    assert!(patch.get(k).is_none());

    let committed = patch.commit();
    assert!(committed.get(k).is_none());
}

#[test]
fn patch_rollback_preserves_base() {
    let mut base: PropertyLayer<TestHandle, String> = PropertyLayer::new();
    let k = TestHandle::new(0, 0);
    base.set(k, "original".into());

    let mut patch = PropertyPatch::new(base);
    patch.set(k, "modified".into());

    let restored = patch.rollback();
    assert_eq!(restored.get(k).unwrap(), "original");
}

#[test]
fn copy_on_write_get_mut() {
    let mut base: PropertyLayer<TestHandle, String> = PropertyLayer::new();
    let k = TestHandle::new(5, 1);
    base.set(k, "original".into());

    let mut patch = PropertyPatch::new(base);
    let val = patch.get_mut(k).unwrap();
    val.push_str("_mutated");
    assert_eq!(patch.get(k).unwrap(), "original_mutated");

    // Base unchanged
    assert_eq!(patch.base().get(k).unwrap(), "original");

    // Mutation persists after commit
    let committed = patch.commit();
    assert_eq!(committed.get(k).unwrap(), "original_mutated");
}

#[test]
fn generational_keys_are_distinct() {
    let mut layer: PropertyLayer<TestHandle, String> = PropertyLayer::new();
    let k_gen0 = TestHandle::new(0, 0);
    let k_gen1 = TestHandle::new(0, 1);

    layer.set(k_gen0, "generation_0".into());
    layer.set(k_gen1, "generation_1".into());

    assert_eq!(layer.get(k_gen0).unwrap(), "generation_0");
    assert_eq!(layer.get(k_gen1).unwrap(), "generation_1");
    assert_eq!(layer.len(), 2);
}

#[test]
fn patch_values_iterator_is_complete() {
    let mut base: PropertyLayer<TestHandle, i32> = PropertyLayer::new();
    let a = TestHandle::new(0, 0);
    let b = TestHandle::new(1, 0);
    let c = TestHandle::new(2, 0);
    base.set(a, 1);
    base.set(b, 2);
    base.set(c, 3);

    let mut patch = PropertyPatch::new(base);
    patch.set(b, 20);      // override
    patch.remove(c);       // remove
    let d = TestHandle::new(3, 0);
    patch.set(d, 40);      // new insert

    let mut values: Vec<i32> = patch.values().copied().collect();
    values.sort();
    assert_eq!(values, vec![1, 20, 40]); // a=1, b=20 (overridden), d=40 (new)
    assert_eq!(patch.len(), 3);
}

#[test]
fn get_mut_on_removed_key_returns_none() {
    let mut base: PropertyLayer<TestHandle, String> = PropertyLayer::new();
    let k = TestHandle::new(0, 0);
    base.set(k, "exists".into());

    let mut patch = PropertyPatch::new(base);
    patch.remove(k);
    assert!(patch.get_mut(k).is_none());
}
