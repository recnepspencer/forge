use worth_store_authority::{
    ControlStoreFencingPort, ControlStoreFencingProviderDenial, ControlStoreSelectionCoordinates,
    StoreCurrentAuthorityIdentity, StoreCurrentAuthorityWitness,
};

#[derive(Debug)]
pub(crate) struct ExactControlSelection {
    authority: StoreCurrentAuthorityIdentity,
    coordinates: ControlStoreSelectionCoordinates,
}

impl ExactControlSelection {
    pub(crate) fn current(
        authority: &StoreCurrentAuthorityWitness,
        control: &crate::OperationalControlStore,
    ) -> Self {
        Self {
            authority: authority.authority_identity(),
            coordinates: control
                .observe_selection_coordinates()
                .expect("control selection observation")
                .expect("control history is nonempty"),
        }
    }
}

impl ControlStoreFencingPort for ExactControlSelection {
    fn selected_control_store(
        &self,
        current_authority: StoreCurrentAuthorityIdentity,
    ) -> Result<ControlStoreSelectionCoordinates, ControlStoreFencingProviderDenial> {
        if current_authority == self.authority {
            Ok(self.coordinates)
        } else {
            Err(ControlStoreFencingProviderDenial::Unavailable)
        }
    }
}

pub(crate) fn selected_staging_kind(
    authority: &StoreCurrentAuthorityWitness,
    control: &crate::OperationalControlStore,
) -> crate::RecoveryStagingOperationKind {
    let selection = ExactControlSelection::current(authority, control);
    let fencing = worth_store_authority::ControlStoreFencingAuthority::for_current_store(
        authority, &selection,
    );
    let crate::ControlStoreTrustPosture::Selected(selected) = control.inspect_generations(&fencing)
    else {
        panic!("control history must remain selectable");
    };
    let [handle] = selected.indeterminate_recovery_staging_handles() else {
        panic!("one completed staging recovery handle must remain");
    };
    assert!(handle.completed_media_identity().is_some());
    handle.operation_kind()
}
