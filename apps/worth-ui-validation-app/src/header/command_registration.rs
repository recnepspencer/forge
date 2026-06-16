use worth_ui::facade::{
    CommandCategory, CommandDescriptor, CommandId, CommandProjectionCommandReference,
    CommandProjectionDescriptor, CommandProjectionId, CommandProjectionSurface, WorthUiAppBuilder,
};

pub(crate) const HEADER_MENU_PROJECTIONS: &[(&str, &str)] = &[
    ("File", "validation.header.menu.file"),
    ("Edit", "validation.header.menu.edit"),
    ("Terminal", "validation.header.menu.terminal"),
    ("Help", "validation.header.menu.help"),
];

const HEADER_COMMANDS: &[HeaderCommandDescriptor] = &[
    HeaderCommandDescriptor::new("validation.command.file.new", "New File", "Ctrl+N", "file"),
    HeaderCommandDescriptor::new(
        "validation.command.file.open",
        "Open File",
        "Ctrl+O",
        "file",
    ),
    HeaderCommandDescriptor::new("validation.command.file.save", "Save", "Ctrl+S", "file"),
    HeaderCommandDescriptor::new("validation.command.file.exit", "Exit", "Alt+F4", "file"),
    HeaderCommandDescriptor::new("validation.command.edit.undo", "Undo", "Ctrl+Z", "edit"),
    HeaderCommandDescriptor::new("validation.command.edit.redo", "Redo", "Ctrl+Y", "edit"),
    HeaderCommandDescriptor::new("validation.command.edit.cut", "Cut", "Ctrl+X", "edit"),
    HeaderCommandDescriptor::new("validation.command.edit.copy", "Copy", "Ctrl+C", "edit"),
    HeaderCommandDescriptor::new("validation.command.edit.paste", "Paste", "Ctrl+V", "edit"),
    HeaderCommandDescriptor::new(
        "validation.command.terminal.new",
        "New Terminal",
        "Ctrl+`",
        "terminal",
    ),
    HeaderCommandDescriptor::new(
        "validation.command.terminal.split",
        "Split Terminal",
        "Ctrl+Shift+5",
        "terminal",
    ),
    HeaderCommandDescriptor::new(
        "validation.command.terminal.clear",
        "Clear Terminal",
        "Ctrl+K",
        "terminal",
    ),
    HeaderCommandDescriptor::new(
        "validation.command.help.palette",
        "Command Palette",
        "Ctrl+Shift+P",
        "help",
    ),
    HeaderCommandDescriptor::new(
        "validation.command.help.docs",
        "Worth UI Docs",
        "F1",
        "help",
    ),
    HeaderCommandDescriptor::new(
        "validation.command.help.about",
        "About Worth UI",
        "",
        "help",
    ),
];

pub(crate) fn register_header_command_capabilities(
    mut builder: WorthUiAppBuilder,
) -> WorthUiAppBuilder {
    for command in HEADER_COMMANDS {
        builder = builder.register_command(command.to_worth_ui_descriptor());
    }

    for (title, projection_id) in HEADER_MENU_PROJECTIONS {
        builder = builder.register_command_projection(header_projection(title, projection_id));
    }

    builder
}

fn header_projection(title: &str, projection_id: &str) -> CommandProjectionDescriptor {
    let mut descriptor = CommandProjectionDescriptor::new(
        CommandProjectionId::new(projection_id).expect("valid header projection id"),
        CommandProjectionSurface::menu_bar(),
    );

    for command in HEADER_COMMANDS
        .iter()
        .filter(|command| command.menu_title == title.to_ascii_lowercase())
    {
        descriptor = descriptor.with_command_reference(CommandProjectionCommandReference::command(
            CommandId::new(command.id).expect("valid header command id"),
        ));
    }

    descriptor.show_shortcuts()
}

#[derive(Clone, Copy)]
struct HeaderCommandDescriptor {
    id: &'static str,
    label: &'static str,
    shortcut: &'static str,
    menu_title: &'static str,
}

impl HeaderCommandDescriptor {
    const fn new(
        id: &'static str,
        label: &'static str,
        shortcut: &'static str,
        menu_title: &'static str,
    ) -> Self {
        Self {
            id,
            label,
            shortcut,
            menu_title,
        }
    }

    fn to_worth_ui_descriptor(self) -> CommandDescriptor {
        let mut descriptor = CommandDescriptor::new(
            CommandId::new(self.id).expect("valid header command id"),
            self.label,
        )
        .with_category(command_category(self.menu_title));

        if !self.shortcut.is_empty() {
            descriptor = descriptor.with_default_shortcut_reference(self.shortcut);
        }

        descriptor
    }
}

fn command_category(menu_title: &str) -> CommandCategory {
    match menu_title {
        "file" => CommandCategory::File,
        "edit" => CommandCategory::Edit,
        "help" => CommandCategory::Help,
        _ => CommandCategory::Tools,
    }
}
