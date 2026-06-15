#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ShellCommandDescriptor {
    id: &'static str,
    label: &'static str,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ShellCommandRegistry {
    commands: &'static [ShellCommandDescriptor],
}

impl ShellCommandDescriptor {
    pub const fn new(id: &'static str, label: &'static str) -> Self {
        Self { id, label }
    }

    pub fn id(self) -> &'static str {
        self.id
    }

    pub fn label(self) -> &'static str {
        self.label
    }
}

impl ShellCommandRegistry {
    pub const DEFAULT: Self = Self {
        commands: &[
            ShellCommandDescriptor::new("validation.command.open-palette", "Command palette"),
            ShellCommandDescriptor::new("validation.command.run-scenario", "Run scenario"),
            ShellCommandDescriptor::new("validation.command.inspect-evidence", "Inspect evidence"),
        ],
    };

    pub fn commands(self) -> &'static [ShellCommandDescriptor] {
        self.commands
    }
}
