use worth_ui::facade::{RegistryFamily, RegistryFamilyFacadeExposure};

pub(crate) fn assert_every_family_has_facade_exposure_decision() {
    for registry_family in RegistryFamily::all() {
        match registry_family.facade_exposure() {
            RegistryFamilyFacadeExposure::PublicFacade
            | RegistryFamilyFacadeExposure::InternalOnly => {}
        }
    }
}

pub(crate) fn assert_registry_family_names_round_trip_through_facade_inventory() {
    for registry_family in RegistryFamily::all() {
        assert_eq!(
            RegistryFamily::from_name(registry_family.name()),
            Some(*registry_family)
        );
    }
}
