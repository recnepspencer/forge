use std::num::NonZeroU32;

use worth_foundational::facade::{CanonicalDigestDerivationDenial, CanonicalDigestId};
use worth_query_declaration::facade::application_schema::ApplicationSchemaBindingIdentity;

use crate::application_operation::WorthQueryInstalledApplicationOperationAuthorization;
use crate::domain_computation::WorthQueryExecutionResourceContract;
use crate::domain_operation::{
    WorthQueryInvariantEnforcement, WorthQueryInvariantExecutionContract,
    WorthQueryOperationEffectContract, WorthQueryOperationEffectFamily,
    WorthQueryOperationGraphAccess, WorthQueryOperationGraphParticipation,
    WorthQueryOperationGraphReadContract, WorthQueryOperationGraphReadRole,
    WorthQueryOperationInvariantContract, WorthQueryOperationTouchContract,
};

use super::identity::derive_set_identity;
use super::{
    bind_operation_obligations, WorthQueryApplicationOperationObligationSource,
    WorthQueryGraphObligationInstallationDenial, WorthQueryInstalledGraphObligationContract,
    WorthQueryInstalledGraphObligationResourcePosture,
};

#[test]
fn warm_operation_validation_has_no_obligation_reconstruction_surface() {
    let installed_operation_source = include_str!("../application_operation/installed.rs");
    let obligation_sources = [
        include_str!("contract.rs"),
        include_str!("identity.rs"),
        include_str!("installed_set.rs"),
        include_str!("operation_binding.rs"),
        include_str!("query_binding.rs"),
        include_str!("selection_index.rs"),
    ];

    assert!(!installed_operation_source.contains("fn meaning_matches"));
    assert!(!installed_operation_source.contains("fn authority_matches"));
    assert!(!include_str!("../application_query/installed_contract.rs")
        .contains("fn authority_matches"));
    for source in obligation_sources {
        assert!(!source.contains("sha2::"));
        assert!(!source.contains("Sha256"));
    }
}

#[test]
fn obligation_identity_denies_entry_and_encoded_byte_overflow() {
    let binding = binding();
    let resources = WorthQueryInstalledGraphObligationResourcePosture::ApplicationQuery {
        maximum_traversal_depth: 1,
        maximum_result_count: 1,
        maximum_authorization_facts: 0,
    };
    let too_many = vec![WorthQueryInstalledGraphObligationContract::PrincipalAuthorization; 4_096];
    let entry_denial = derive_set_identity(
        &binding,
        "application-operation",
        "entry-overflow",
        Some("Input"),
        &too_many,
        &resources,
    )
    .unwrap_err();
    assert!(matches!(
        entry_denial,
        CanonicalDigestDerivationDenial::EntryLimitExceeded { .. }
    ));

    let huge_name = "x".repeat(2 * 1024 * 1024);
    let byte_denial = derive_set_identity(
        &binding,
        "application-operation",
        &huge_name,
        Some("Input"),
        &[WorthQueryInstalledGraphObligationContract::PrincipalAuthorization],
        &resources,
    )
    .unwrap_err();
    assert!(matches!(
        byte_denial,
        CanonicalDigestDerivationDenial::EncodedByteLimitExceeded { .. }
    ));
}

#[test]
fn mismatched_invariant_owner_contract_is_denied_before_identity_minting() {
    let graph_reads = WorthQueryOperationGraphReadContract::Declared {
        roles: vec![WorthQueryOperationGraphReadRole {
            role: "primary".to_owned(),
            participation: WorthQueryOperationGraphParticipation::PrimaryLogicalGraph,
            access: WorthQueryOperationGraphAccess::Project,
            semantic_reads: Vec::new(),
        }],
    };
    let touches = WorthQueryOperationTouchContract::Declared {
        graph_roles: vec!["primary".to_owned()],
        scopes: vec!["entity:Account".to_owned()],
    };
    let effects = WorthQueryOperationEffectContract::Declared {
        effect_families: vec![WorthQueryOperationEffectFamily::Mutation],
    };
    let invariants = WorthQueryOperationInvariantContract::Declared {
        invariant_slots: vec!["declared".to_owned()],
    };
    let invariant_execution = WorthQueryInvariantExecutionContract::declared([
        crate::domain_operation::WorthQueryInstalledInvariantExecutionRequirement::new(
            "different",
            "test",
            NonZeroU32::new(1).unwrap(),
            WorthQueryInvariantEnforcement::Blocking,
            "primary",
            ["proposed-state"],
            1,
            1,
        )
        .unwrap(),
    ])
    .unwrap();
    let resources = WorthQueryExecutionResourceContract::default();

    let denial = bind_operation_obligations(
        &binding(),
        "Mutate",
        "Input",
        WorthQueryApplicationOperationObligationSource {
            authorization: WorthQueryInstalledApplicationOperationAuthorization::Principal,
            ability_requirements: &[],
            capability_requirements: &[],
            graph_reads: &graph_reads,
            touches: &touches,
            effects: &effects,
            invariants: &invariants,
            invariant_execution: &invariant_execution,
            resources: &resources,
        },
    )
    .unwrap_err();

    assert!(matches!(
        denial,
        WorthQueryGraphObligationInstallationDenial::InvalidContract
    ));
}

fn binding() -> ApplicationSchemaBindingIdentity {
    ApplicationSchemaBindingIdentity::from_installed_parts(
        7,
        3,
        CanonicalDigestId::new([0x11; 32]),
        CanonicalDigestId::new([0x22; 32]),
    )
}
