use worth_ui_host_contract::{
    UiHostSurfaceIdentity, UiHostSurfacePresentationDenial, UiHostSurfacePresentationMode,
    UiMountedPaintCommand, UiMountedPaintOrderIdentity, UiMountedPaintOrderIntegrity,
    UiMountedPresentationInitial, UiMountedPresentationInitialInput,
    UiMountedSurfaceBindingRequirement, WorthUiHostCapabilityObservationGeneration,
};

use super::validated_initial_commands;

#[test]
fn initial_rejects_duplicate_command_identity_before_retention() {
    let valid = valid_initial();
    let mut commands = valid.commands().to_vec();
    commands.push(commands[0].clone());
    let malformed = rebuild_initial(
        &valid,
        commands,
        valid.order().to_vec(),
        valid.order_integrity(),
    );

    assert_eq!(
        validated_initial_commands(&malformed),
        Err(UiHostSurfacePresentationDenial::MalformedProjection)
    );
}

#[test]
fn initial_rejects_order_that_omits_a_command() {
    let valid = valid_initial();
    let mut order = valid.order().to_vec();
    order.pop().expect("fixture supplies paint order");
    let malformed = rebuild_initial(
        &valid,
        valid.commands().to_vec(),
        order,
        valid.order_integrity(),
    );

    assert_eq!(
        validated_initial_commands(&malformed),
        Err(UiHostSurfacePresentationDenial::MalformedProjection)
    );
}

#[test]
fn initial_rejects_stale_order_integrity_for_exact_membership() {
    let valid = valid_initial();
    let malformed = rebuild_initial(
        &valid,
        valid.commands().to_vec(),
        valid.order().to_vec(),
        UiMountedPaintOrderIntegrity::for_order(&[]),
    );

    assert_eq!(
        validated_initial_commands(&malformed),
        Err(UiHostSurfacePresentationDenial::MalformedProjection)
    );
}

#[test]
fn initial_rejects_command_identity_that_disagrees_with_its_mechanic() {
    let valid = valid_initial();
    let foreign = valid_initial();
    let mut commands = valid.commands().to_vec();
    commands[0] = command_with_identity(commands[0].clone(), foreign.commands()[0].identity());
    assert_malformed_commands(&valid, commands);
}

#[test]
fn initial_rejects_command_payload_that_disagrees_with_projection_row() {
    let valid = valid_initial();
    let foreign = valid_initial();
    let mut commands = valid.commands().to_vec();
    commands[0] = command_with_payload(commands[0].clone(), foreign.commands()[0].clone());
    assert_malformed_commands(&valid, commands);
}

fn assert_malformed_commands(
    valid: &UiMountedPresentationInitial,
    commands: Vec<UiMountedPaintCommand>,
) {
    let malformed = rebuild_initial(
        valid,
        commands,
        valid.order().to_vec(),
        valid.order_integrity(),
    );
    assert_eq!(
        validated_initial_commands(&malformed),
        Err(UiHostSurfacePresentationDenial::MalformedProjection)
    );
}

fn command_with_identity(
    command: UiMountedPaintCommand,
    identity: worth_ui_host_contract::UiMountedPaintCommandIdentity,
) -> UiMountedPaintCommand {
    match command {
        UiMountedPaintCommand::FilledRect { mechanic, .. } => {
            UiMountedPaintCommand::FilledRect { identity, mechanic }
        }
        UiMountedPaintCommand::SemanticText { mechanic, .. } => {
            UiMountedPaintCommand::SemanticText { identity, mechanic }
        }
    }
}

fn command_with_payload(
    command: UiMountedPaintCommand,
    donor: UiMountedPaintCommand,
) -> UiMountedPaintCommand {
    match (command, donor) {
        (
            UiMountedPaintCommand::FilledRect {
                identity,
                ..
            },
            UiMountedPaintCommand::FilledRect { mechanic, .. },
        ) => UiMountedPaintCommand::FilledRect {
            identity,
            mechanic,
        },
        (
            UiMountedPaintCommand::SemanticText {
                identity,
                ..
            },
            UiMountedPaintCommand::SemanticText { mechanic, .. },
        ) => UiMountedPaintCommand::SemanticText {
            identity,
            mechanic,
        },
        _ => panic!("fixture donor must use the same drawable family"),
    }
}

fn valid_initial() -> UiMountedPresentationInitial {
    let projection = worth_ui_test_support::semantic_text_projection_for_certification(
        worth_ui_test_support::UiSemanticTextProjectionCertificationMutation::Exact,
    );
    let requirement = UiMountedSurfaceBindingRequirement::new(
        projection.surface(),
        UiHostSurfaceIdentity::mint_unbound().expect("host surface identity"),
        projection.binding(),
        WorthUiHostCapabilityObservationGeneration::new(7),
        11,
        UiHostSurfacePresentationMode::RecordOnly,
    );
    worth_ui_test_support::initial_presentation_mechanics_for_certification(
        &projection,
        requirement,
    )
}

fn rebuild_initial(
    valid: &UiMountedPresentationInitial,
    commands: Vec<UiMountedPaintCommand>,
    order: Vec<UiMountedPaintOrderIdentity>,
    order_integrity: UiMountedPaintOrderIntegrity,
) -> UiMountedPresentationInitial {
    let affinity = valid.affinity();
    UiMountedPresentationInitial::from_inert_mechanics(UiMountedPresentationInitialInput {
        successor: affinity.successor(),
        surface: affinity.surface(),
        binding: affinity.binding(),
        content: affinity.content(),
        baseline: affinity.baseline(),
        projection: valid.projection().clone(),
        commands,
        order,
        order_integrity,
        damage: valid.damage().to_vec(),
        production_cost: valid.production_cost(),
    })
}
