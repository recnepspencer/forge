use worth_ui::facade::app::WorthUiApplicationPreparationDenial;
use worth_ui::facade::intent::{
    UiIntentCatalogPreparationDenial, UiIntentPayload, UiIntentProductOutcome,
    UiSemanticInteractionFamily,
};
use worth_ui_dsl::{WorthUiIntentInteractionFamily, WorthUiIntentInteractionRoute};

use super::support::{
    component_with_routes, declaration, freeze_module, AdvanceOutcome, AdvancePayload, DECLARATION,
    DEFINITION,
};

#[test]
fn unknown_definition_stops_candidate_preparation() {
    let unknown = component_with_routes([]).with_intent_declaration(declaration(
        DECLARATION,
        "platform.pulse.unknown",
        WorthUiIntentInteractionFamily::Activate,
    ));
    assert_catalog_denial(
        unknown,
        UiIntentCatalogPreparationDenial::UnknownDefinition {
            declaration: DECLARATION.into(),
            definition: "platform.pulse.unknown".into(),
        },
    );
}

#[test]
fn payload_and_outcome_schema_mismatches_remain_distinct() {
    let mismatch = declaration(
        DECLARATION,
        DEFINITION,
        WorthUiIntentInteractionFamily::Activate,
    )
    .with_expected_schemas(
        "platform.pulse.wrong_payload",
        1,
        AdvanceOutcome::SCHEMA.stable_identity(),
        AdvanceOutcome::SCHEMA.version(),
    );
    let mismatch = component_with_routes([]).with_intent_declaration(mismatch);
    let error = freeze_error(mismatch);
    match error {
        WorthUiApplicationPreparationDenial::IntentCatalog(
            UiIntentCatalogPreparationDenial::PayloadSchemaMismatch {
                declaration,
                registered,
                ..
            },
        ) => {
            assert_eq!(&*declaration, DECLARATION);
            assert_eq!(registered, AdvancePayload::SCHEMA);
        }
        other => panic!("expected payload-schema denial, got {other:?}"),
    }

    let mismatch = declaration(
        DECLARATION,
        DEFINITION,
        WorthUiIntentInteractionFamily::Activate,
    )
    .with_expected_schemas(
        AdvancePayload::SCHEMA.stable_identity(),
        AdvancePayload::SCHEMA.version(),
        "platform.pulse.wrong_outcome",
        1,
    );
    let mismatch = component_with_routes([]).with_intent_declaration(mismatch);
    let error = freeze_error(mismatch);
    assert!(matches!(
        error,
        WorthUiApplicationPreparationDenial::IntentCatalog(
            UiIntentCatalogPreparationDenial::OutcomeSchemaMismatch { .. }
        )
    ));
}

#[test]
fn definition_family_and_unknown_route_stops_remain_distinct() {
    let family = component_with_routes([]).with_intent_declaration(declaration(
        DECLARATION,
        DEFINITION,
        WorthUiIntentInteractionFamily::Submit,
    ));
    assert_catalog_denial(
        family,
        UiIntentCatalogPreparationDenial::InteractionNotAccepted {
            declaration: DECLARATION.into(),
            interaction: UiSemanticInteractionFamily::Submit,
        },
    );

    let unknown_route = component_with_routes([WorthUiIntentInteractionRoute::product(
        WorthUiIntentInteractionFamily::Activate,
        "missing.declaration",
    )]);
    assert_catalog_denial(
        unknown_route,
        UiIntentCatalogPreparationDenial::UnknownRouteDeclaration {
            declaration: "missing.declaration".into(),
        },
    );
}

#[test]
fn product_ambiguity_and_route_kind_crossover_remain_distinct() {
    let product = WorthUiIntentInteractionRoute::product(
        WorthUiIntentInteractionFamily::Activate,
        DECLARATION,
    );
    let duplicate =
        component_with_routes([product.clone(), product]).with_intent_declaration(declaration(
            DECLARATION,
            DEFINITION,
            WorthUiIntentInteractionFamily::Activate,
        ));
    let duplicate_error = freeze_error(duplicate);
    assert!(matches!(
        duplicate_error,
        WorthUiApplicationPreparationDenial::IntentCatalog(
            UiIntentCatalogPreparationDenial::AmbiguousProductRoute { .. }
        )
    ));

    let crossover = component_with_routes([
        WorthUiIntentInteractionRoute::product(
            WorthUiIntentInteractionFamily::Activate,
            DECLARATION,
        ),
        WorthUiIntentInteractionRoute::confirmation(DECLARATION),
    ])
    .with_intent_declaration(declaration(
        DECLARATION,
        DEFINITION,
        WorthUiIntentInteractionFamily::Activate,
    ));
    let crossover_error = freeze_error(crossover);
    assert!(matches!(
        crossover_error,
        WorthUiApplicationPreparationDenial::IntentCatalog(
            UiIntentCatalogPreparationDenial::RouteKindCrossover { .. }
        )
    ));
}

#[test]
fn duplicate_declaration_stops_before_route_binding() {
    let duplicate_declaration = component_with_routes([])
        .with_intent_declaration(declaration(
            DECLARATION,
            DEFINITION,
            WorthUiIntentInteractionFamily::Activate,
        ))
        .with_intent_declaration(declaration(
            DECLARATION,
            DEFINITION,
            WorthUiIntentInteractionFamily::Activate,
        ));
    assert_catalog_denial(
        duplicate_declaration,
        UiIntentCatalogPreparationDenial::DuplicateDeclaration {
            identity: DECLARATION.into(),
        },
    );
}

#[test]
fn product_family_mismatch_and_confirmation_ambiguity_stop_distinctly() {
    let mismatched_route = component_with_routes([WorthUiIntentInteractionRoute::product(
        WorthUiIntentInteractionFamily::Submit,
        DECLARATION,
    )])
    .with_intent_declaration(declaration(
        DECLARATION,
        DEFINITION,
        WorthUiIntentInteractionFamily::Activate,
    ));
    let error = freeze_error(mismatched_route);
    assert!(matches!(
        error,
        WorthUiApplicationPreparationDenial::IntentCatalog(
            UiIntentCatalogPreparationDenial::ProductInteractionMismatch { .. }
        )
    ));

    let confirmation = WorthUiIntentInteractionRoute::confirmation(DECLARATION);
    let duplicate_confirmation = component_with_routes([confirmation.clone(), confirmation])
        .with_intent_declaration(declaration(
            DECLARATION,
            DEFINITION,
            WorthUiIntentInteractionFamily::Activate,
        ));
    let error = freeze_error(duplicate_confirmation);
    assert!(matches!(
        error,
        WorthUiApplicationPreparationDenial::IntentCatalog(
            UiIntentCatalogPreparationDenial::AmbiguousConfirmationRoute { .. }
        )
    ));
}

fn assert_catalog_denial(
    module: worth_ui_dsl::WorthUiRustAuthoredArtifactInputModule,
    expected: UiIntentCatalogPreparationDenial,
) {
    let error = freeze_error(module);
    assert_eq!(
        error,
        WorthUiApplicationPreparationDenial::IntentCatalog(expected)
    );
}

fn freeze_error(
    module: worth_ui_dsl::WorthUiRustAuthoredArtifactInputModule,
) -> WorthUiApplicationPreparationDenial {
    match freeze_module(module) {
        Ok(_) => panic!("catalog defect must stop preparation"),
        Err(error) => error,
    }
}
