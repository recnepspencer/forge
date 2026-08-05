use std::num::NonZeroU32;

use worth_foundational::facade::{CanonicalDigestDerivationDenial, CanonicalDigestId};
use worth_query_declaration::facade::application_schema::ApplicationSchemaBindingIdentity;

use crate::application_operation::WorthQueryInstalledApplicationOperationAuthorization;
use crate::domain_computation::WorthQueryExecutionResourceContract;
use crate::domain_operation::{
    WorthQueryInstalledInvariantExecutionRequirement, WorthQueryInvariantEnforcement,
    WorthQueryInvariantExecutionContract, WorthQueryOperationEffectContract,
    WorthQueryOperationEffectFamily, WorthQueryOperationGraphAccess,
    WorthQueryOperationGraphParticipation, WorthQueryOperationGraphReadContract,
    WorthQueryOperationGraphReadRole, WorthQueryOperationInvariantContract,
    WorthQueryOperationTouchContract,
};

use super::identity::derive_set_identity;
use super::{
    bind_operation_obligations, WorthQueryApplicationOperationObligationSource,
    WorthQueryGraphObligationInstallationDenial, WorthQueryInstalledGraphObligationContract,
    WorthQueryInstalledGraphObligationKind, WorthQueryInstalledGraphObligationOwner,
    WorthQueryInstalledGraphObligationResourcePosture,
};

#[test]
fn installed_mutation_matrix_names_only_real_semantic_owners() {
    let graph_reads = graph_reads();
    let touches = WorthQueryOperationTouchContract::Declared {
        graph_roles: vec!["primary".to_owned()],
        scopes: vec!["entity:Account".to_owned()],
    };
    let effects = WorthQueryOperationEffectContract::Declared {
        effect_families: vec![WorthQueryOperationEffectFamily::Mutation],
    };
    let invariants = WorthQueryOperationInvariantContract::Declared {
        invariant_slots: vec!["account-balance".to_owned()],
    };
    let invariant_execution =
        WorthQueryInvariantExecutionContract::declared([invariant()]).unwrap();
    let resources = WorthQueryExecutionResourceContract::default();
    let installed = bind_operation_obligations(
        &binding(),
        "Transfer",
        "TransferInput",
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
    .unwrap();

    assert_eq!(installed.rows().len(), 5);
    assert_eq!(
        installed
            .inspect_kind(WorthQueryInstalledGraphObligationKind::GraphRead)
            .rows()[0]
            .required_owners(),
        &[WorthQueryInstalledGraphObligationOwner::RelationalGraph]
    );
    assert_eq!(
        installed
            .inspect_kind(WorthQueryInstalledGraphObligationKind::MutationTouch)
            .rows()[0]
            .required_owners(),
        &[WorthQueryInstalledGraphObligationOwner::QueryApplicationProgram]
    );
    assert_eq!(
        installed
            .inspect_kind(WorthQueryInstalledGraphObligationKind::InvariantExecution)
            .rows()[0]
            .required_owners(),
        &[
            WorthQueryInstalledGraphObligationOwner::RelationalGraph,
            WorthQueryInstalledGraphObligationOwner::QueryInstalledInvariantProvider,
        ]
    );
}

#[test]
fn mismatched_invariant_contract_is_denied_before_identity_minting() {
    let graph_reads = graph_reads();
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
    let invariant_execution =
        WorthQueryInvariantExecutionContract::declared([invariant()]).unwrap();
    let resources = WorthQueryExecutionResourceContract::default();
    let denial = bind_operation_obligations(
        &binding(),
        "Transfer",
        "TransferInput",
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

#[test]
fn installed_identity_enforces_entry_and_encoded_byte_budgets() {
    let resources = WorthQueryInstalledGraphObligationResourcePosture::ApplicationQuery {
        maximum_traversal_depth: 1,
        maximum_result_count: 1,
        maximum_authorization_facts: 0,
    };
    let too_many = vec![WorthQueryInstalledGraphObligationContract::PrincipalAuthorization; 4_096];
    assert!(matches!(
        derive_set_identity(
            &binding(),
            "application-operation",
            "entry-overflow",
            Some("Input"),
            &too_many,
            &resources,
        )
        .unwrap_err(),
        CanonicalDigestDerivationDenial::EntryLimitExceeded { .. }
    ));

    let huge_name = "x".repeat(2 * 1024 * 1024);
    assert!(matches!(
        derive_set_identity(
            &binding(),
            "application-operation",
            &huge_name,
            Some("Input"),
            &[WorthQueryInstalledGraphObligationContract::PrincipalAuthorization],
            &resources,
        )
        .unwrap_err(),
        CanonicalDigestDerivationDenial::EncodedByteLimitExceeded { .. }
    ));
}

fn graph_reads() -> WorthQueryOperationGraphReadContract {
    WorthQueryOperationGraphReadContract::Declared {
        roles: vec![WorthQueryOperationGraphReadRole {
            role: "primary".to_owned(),
            participation: WorthQueryOperationGraphParticipation::PrimaryLogicalGraph,
            access: WorthQueryOperationGraphAccess::Project,
            semantic_reads: Vec::new(),
        }],
    }
}

fn invariant() -> WorthQueryInstalledInvariantExecutionRequirement {
    WorthQueryInstalledInvariantExecutionRequirement::new(
        "account-balance",
        "bank",
        NonZeroU32::new(1).unwrap(),
        WorthQueryInvariantEnforcement::Blocking,
        "primary",
        ["proposed-state"],
        1,
        8,
    )
    .unwrap()
}

fn binding() -> ApplicationSchemaBindingIdentity {
    ApplicationSchemaBindingIdentity::from_installed_parts(
        7,
        3,
        CanonicalDigestId::new([0x11; 32]),
        CanonicalDigestId::new([0x22; 32]),
    )
}
