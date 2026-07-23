use worth_ui::facade::support::{RegistryFamily, RegistryFamilyLifecyclePropagation};

pub(crate) fn assert_every_family_requires_builder_initialization() {
    for registry_family in RegistryFamily::all() {
        assert_eq!(
            registry_family.lifecycle_propagation(),
            RegistryFamilyLifecyclePropagation::FullRegistryLifecycle
        );
        assert!(registry_family.requires_builder_initialization());
    }
}

pub(crate) fn assert_every_family_requires_snapshot_freeze() {
    for registry_family in RegistryFamily::all() {
        assert_eq!(
            registry_family.lifecycle_propagation(),
            RegistryFamilyLifecyclePropagation::FullRegistryLifecycle
        );
        assert!(registry_family.requires_snapshot_freeze());
    }
}

pub(crate) fn assert_every_family_requires_diagnostics_aggregation() {
    for registry_family in RegistryFamily::all() {
        assert_eq!(
            registry_family.lifecycle_propagation(),
            RegistryFamilyLifecyclePropagation::FullRegistryLifecycle
        );
        assert!(registry_family.requires_diagnostics_aggregation());
    }
}
