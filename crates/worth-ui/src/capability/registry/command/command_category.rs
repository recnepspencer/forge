/// Domain-agnostic category used to group application commands.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum CommandCategory {
    Application,
    Workspace,
    File,
    Edit,
    View,
    Navigate,
    Tools,
    Help,
}

impl CommandCategory {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Application => "application",
            Self::Workspace => "workspace",
            Self::File => "file",
            Self::Edit => "edit",
            Self::View => "view",
            Self::Navigate => "navigate",
            Self::Tools => "tools",
            Self::Help => "help",
        }
    }
}
