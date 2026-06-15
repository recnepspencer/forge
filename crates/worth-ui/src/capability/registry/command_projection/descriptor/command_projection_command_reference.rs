use crate::capability::CommandId;

/// Typed command-spine reference admitted into a command projection.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct CommandProjectionCommandReference {
    command_id: CommandId,
}

impl CommandProjectionCommandReference {
    pub fn command(command_id: CommandId) -> Self {
        Self { command_id }
    }

    pub fn command_id(&self) -> &CommandId {
        &self.command_id
    }

    pub(crate) fn digest_basis(&self) -> String {
        self.command_id.as_str().to_owned()
    }
}
