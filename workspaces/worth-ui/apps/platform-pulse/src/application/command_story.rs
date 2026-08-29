use worth_ui::facade::declaration::{
    CommandDescriptor, CommandId, UiCommandKeyCode, UiCommandModifierSet,
    UiCommandRouteDeclaration, UiCommandRouteDestination, UiCommandRouteScopeIdentity,
    UiCommandShortcutSequence, UiCommandShortcutStroke, UiCommandTextInputPolicy,
};

use super::PlatformPulsePreparationDenial;

const APPLICATION_COMMAND: &str = "platform.pulse.command.run.application";
const PORTAL_COMMAND: &str = "platform.pulse.command.run.portal";

pub(super) fn register(
    builder: worth_ui::facade::app::WorthUiApplicationBuilder<
        worth_ui::facade::app::UiChangeProfileInstalled,
        worth_ui::facade::app::UiIntentWiringSatisfied,
    >,
) -> Result<
    worth_ui::facade::app::WorthUiApplicationBuilder<
        worth_ui::facade::app::UiChangeProfileInstalled,
        worth_ui::facade::app::UiIntentWiringSatisfied,
    >,
    PlatformPulsePreparationDenial,
> {
    let shortcut = UiCommandShortcutSequence::single(UiCommandShortcutStroke::logical(
        UiCommandKeyCode::P,
        UiCommandModifierSet::none().with_primary().with_shift(),
    ));
    let destination = UiCommandRouteDestination::for_intent::<
        worth_ui_platform_pulse::intent::PlatformPulseAction,
    >();
    let destination_route = || {
        UiCommandRouteDeclaration::new(destination)
            .with_text_input_policy(UiCommandTextInputPolicy::SuppressDuringComposition)
    };
    let application = CommandDescriptor::new(command_id(APPLICATION_COMMAND), "Run live action")
        .with_description("Run the Pulse action from the application context")
        .with_default_shortcut(shortcut)
        .with_route(destination_route());
    let portal = CommandDescriptor::new(command_id(PORTAL_COMMAND), "Run from details")
        .with_description("Run the Pulse action from the active details Portal")
        .with_default_shortcut(shortcut)
        .with_route(destination_route().for_active_portal(
            UiCommandRouteScopeIdentity::for_authored_component(
                "platform.pulse.component.portal_target",
            ),
        ));
    Ok(builder
        .register_command(application)
        .register_command(portal))
}

fn command_id(identity: &str) -> CommandId {
    CommandId::new(identity).expect("Pulse command identities are valid")
}
