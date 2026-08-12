use bank_domain::schema::{
    RequestEstateEmergencyAccessCapability, RequestEstateEmergencyAccessOperation,
    ViewEstateAdministrationCapability, ViewRestrictedEstateOperation,
};
use worth_query_host::facade::domain::{
    WorthQueryInstallationRuntimeIdentity, WorthQueryInstalledApplicationCapabilityIdentity,
    WorthQueryInstalledGraphAuthorizationRequirement,
    WorthQueryInstalledGraphObligationEffectPosture, WorthQueryInstalledGraphObligationInspection,
    WorthQueryInstalledGraphObligationResourcePosture,
    WorthQueryInstalledGraphObligationSelectionBasis,
    WorthQueryInstalledGraphObligationTerminalRequirement,
};

fn assert_capability_only_authority(
    obligations: WorthQueryInstalledGraphObligationInspection<'_>,
    capability_identity: &WorthQueryInstalledApplicationCapabilityIdentity,
) {
    let [authorization] = obligations.rows() else {
        panic!("capability graph authority must contain exactly its authorization observation")
    };

    assert_eq!(
        authorization.effect_posture(),
        WorthQueryInstalledGraphObligationEffectPosture::Policy
    );
    assert!(matches!(
        authorization.selection_basis(),
        WorthQueryInstalledGraphObligationSelectionBasis::AuthenticatedAccessContext
    ));
    assert_eq!(
        authorization.terminal_requirement(),
        WorthQueryInstalledGraphObligationTerminalRequirement::AuthorizationDecisionFact
    );
    assert_eq!(
        authorization.resource_posture(),
        &WorthQueryInstalledGraphObligationResourcePosture::ApplicationQuery {
            maximum_traversal_depth: 0,
            maximum_result_count: 0,
            maximum_authorization_facts: 2,
        }
    );

    let Some(WorthQueryInstalledGraphAuthorizationRequirement::Capabilities(requirements)) =
        authorization.authorization_requirement()
    else {
        panic!("capability graph authority must retain the installed capability requirement")
    };
    let [requirement] = requirements else {
        panic!("capability graph authority must retain exactly one capability requirement")
    };
    assert_eq!(requirement.identity(), capability_identity);
}

#[test]
fn executable_metadata_cannot_select_capability_graph_authority() {
    let (_index, bank) = super::installed_bank(WorthQueryInstallationRuntimeIdentity::fresh());

    let executable_capability = bank
        .capability(
            RequestEstateEmergencyAccessCapability::reference(),
            RequestEstateEmergencyAccessOperation::reference(),
        )
        .unwrap();
    let executable_authority = bank
        .installed_operation_for_capability(&executable_capability)
        .unwrap();
    assert_capability_only_authority(
        executable_authority.graph_obligations(),
        executable_capability.identity(),
    );

    let descriptive_capability = bank
        .capability(
            ViewEstateAdministrationCapability::reference(),
            ViewRestrictedEstateOperation::reference(),
        )
        .unwrap();
    let descriptive_authority = bank
        .installed_operation_for_capability(&descriptive_capability)
        .unwrap();
    assert_capability_only_authority(
        descriptive_authority.graph_obligations(),
        descriptive_capability.identity(),
    );
}
