use worth_ui::facade::{
    CommandCategory, CommandDescriptor, CommandId, CommandProjectionCommandReference,
    CommandProjectionDescriptor, CommandProjectionId, CommandProjectionSurface, IconDescriptor,
    IconFamily, IconId, IconSourceDescriptor, WorthUiAppBuilder,
};

pub(crate) const HEADER_MENU_PROJECTIONS: &[(&str, &str)] = &[
    ("File", "validation.header.menu.file"),
    ("Edit", "validation.header.menu.edit"),
    ("Terminal", "validation.header.menu.terminal"),
    ("Help", "validation.header.menu.help"),
];

const HEADER_COMMANDS: &[HeaderCommandDescriptor] = &[
    HeaderCommandDescriptor::new(
        "validation.command.file.new",
        "New File",
        "Ctrl+N",
        "file",
        "worth.icon.header.file.new",
    ),
    HeaderCommandDescriptor::new(
        "validation.command.file.open",
        "Open File",
        "Ctrl+O",
        "file",
        "worth.icon.header.file.open",
    ),
    HeaderCommandDescriptor::new(
        "validation.command.file.save",
        "Save",
        "Ctrl+S",
        "file",
        "worth.icon.header.file.save",
    ),
    HeaderCommandDescriptor::new(
        "validation.command.file.exit",
        "Exit",
        "Alt+F4",
        "file",
        "worth.icon.header.file.exit",
    ),
    HeaderCommandDescriptor::new(
        "validation.command.edit.undo",
        "Undo",
        "Ctrl+Z",
        "edit",
        "worth.icon.header.edit.undo",
    ),
    HeaderCommandDescriptor::new(
        "validation.command.edit.redo",
        "Redo",
        "Ctrl+Y",
        "edit",
        "worth.icon.header.edit.redo",
    ),
    HeaderCommandDescriptor::new(
        "validation.command.edit.cut",
        "Cut",
        "Ctrl+X",
        "edit",
        "worth.icon.header.edit.cut",
    ),
    HeaderCommandDescriptor::new(
        "validation.command.edit.copy",
        "Copy",
        "Ctrl+C",
        "edit",
        "worth.icon.header.edit.copy",
    ),
    HeaderCommandDescriptor::new(
        "validation.command.edit.paste",
        "Paste",
        "Ctrl+V",
        "edit",
        "worth.icon.header.edit.paste",
    ),
    HeaderCommandDescriptor::new(
        "validation.command.terminal.new",
        "New Terminal",
        "Ctrl+`",
        "terminal",
        "worth.icon.header.terminal.new",
    ),
    HeaderCommandDescriptor::new(
        "validation.command.terminal.split",
        "Split Terminal",
        "Ctrl+Shift+5",
        "terminal",
        "worth.icon.header.terminal.split",
    ),
    HeaderCommandDescriptor::new(
        "validation.command.terminal.clear",
        "Clear Terminal",
        "Ctrl+K",
        "terminal",
        "worth.icon.header.terminal.clear",
    ),
    HeaderCommandDescriptor::new(
        "validation.command.help.palette",
        "Command Palette",
        "Ctrl+Shift+P",
        "help",
        "worth.icon.header.help.palette",
    ),
    HeaderCommandDescriptor::new(
        "validation.command.help.docs",
        "Worth UI Docs",
        "F1",
        "help",
        "worth.icon.header.help.docs",
    ),
    HeaderCommandDescriptor::new(
        "validation.command.help.about",
        "About Worth UI",
        "",
        "help",
        "worth.icon.header.help.about",
    ),
];

const HEADER_ICONS: &[(&str, &str)] = &[
    ("worth.icon.header.file.new", "plus"),
    ("worth.icon.header.file.open", "eye"),
    ("worth.icon.header.file.save", "check"),
    ("worth.icon.header.file.exit", "x"),
    ("worth.icon.header.edit.undo", "pen-line"),
    ("worth.icon.header.edit.redo", "pencil-line"),
    ("worth.icon.header.edit.cut", "trash-2"),
    ("worth.icon.header.edit.copy", "layers-3"),
    ("worth.icon.header.edit.paste", "grid-2x2"),
    ("worth.icon.header.terminal.new", "message-square"),
    ("worth.icon.header.terminal.split", "grid-2x2"),
    ("worth.icon.header.terminal.clear", "trash-2"),
    ("worth.icon.header.help.palette", "search"),
    ("worth.icon.header.help.docs", "info"),
    ("worth.icon.header.help.about", "info"),
];

pub(crate) fn register_header_icon_capabilities(
    mut builder: WorthUiAppBuilder,
) -> WorthUiAppBuilder {
    for (id, symbol) in HEADER_ICONS {
        builder = builder.register_icon(IconDescriptor::new(
            IconId::new(*id).expect("valid header icon id"),
            IconFamily::toolbar(),
            IconSourceDescriptor::symbol(*symbol),
        ));
    }
    builder
}

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
    icon_id: &'static str,
}

impl HeaderCommandDescriptor {
    const fn new(
        id: &'static str,
        label: &'static str,
        shortcut: &'static str,
        menu_title: &'static str,
        icon_id: &'static str,
    ) -> Self {
        Self {
            id,
            label,
            shortcut,
            menu_title,
            icon_id,
        }
    }

    fn to_worth_ui_descriptor(self) -> CommandDescriptor {
        let mut descriptor = CommandDescriptor::new(
            CommandId::new(self.id).expect("valid header command id"),
            self.label,
        )
        .with_category(command_category(self.menu_title))
        .with_icon(IconId::new(self.icon_id).expect("valid header icon id"));

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
