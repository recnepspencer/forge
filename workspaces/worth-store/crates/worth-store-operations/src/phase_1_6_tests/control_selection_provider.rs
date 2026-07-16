use worth_store_authority::{
    ControlStoreFencingPort, ControlStoreFencingProviderDenial, ControlStoreGeneration,
    ControlStoreSelectionCoordinates, StoreCurrentAuthorityIdentity, StoreCurrentAuthorityWitness,
};

use crate::OperationalControlStore;

#[derive(Debug)]
pub(super) enum TestControlStoreFencingProvider {
    Selected {
        authority_identity: StoreCurrentAuthorityIdentity,
        coordinates: ControlStoreSelectionCoordinates,
    },
    Unsupported,
    Unavailable,
}

impl TestControlStoreFencingProvider {
    pub(super) fn selected(
        authority: &StoreCurrentAuthorityWitness,
        control: &OperationalControlStore,
        generation: ControlStoreGeneration,
    ) -> Self {
        let coordinates = control
            .observe_selection_coordinates()
            .expect("test fencing observation")
            .expect("test fencing requires a nonempty control history");
        assert_eq!(coordinates.generation(), generation);
        Self::Selected {
            authority_identity: authority.authority_identity(),
            coordinates,
        }
    }
}

impl ControlStoreFencingPort for TestControlStoreFencingProvider {
    fn selected_control_store(
        &self,
        current_authority: StoreCurrentAuthorityIdentity,
    ) -> Result<ControlStoreSelectionCoordinates, ControlStoreFencingProviderDenial> {
        match self {
            Self::Selected {
                authority_identity,
                coordinates,
            } if *authority_identity == current_authority => Ok(*coordinates),
            Self::Selected { .. } | Self::Unavailable => {
                Err(ControlStoreFencingProviderDenial::Unavailable)
            }
            Self::Unsupported => Err(ControlStoreFencingProviderDenial::Unsupported),
        }
    }
}
