use crate::evidence_identity::{
    WorthQueryEvidenceIdentity, WorthQueryEvidenceScope, WorthQueryEvidenceTag,
};
use crate::runtime::{
    WorthQueryAspectMutationBuilder, WorthQueryRuntimeError, WorthQueryWriteCommand,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryMutationDeclarationIdentity {
    identity: WorthQueryEvidenceIdentity,
}

impl WorthQueryMutationDeclarationIdentity {
    pub fn as_str(&self) -> &str {
        self.identity.as_str()
    }

    pub fn evidence_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.identity
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct WorthQueryMutationDeclaration {
    identity: WorthQueryMutationDeclarationIdentity,
    command: WorthQueryWriteCommand,
}

impl WorthQueryMutationDeclaration {
    pub fn identity(&self) -> &WorthQueryMutationDeclarationIdentity {
        &self.identity
    }

    pub(crate) fn command(&self) -> &WorthQueryWriteCommand {
        &self.command
    }

    pub(crate) fn into_command(self) -> WorthQueryWriteCommand {
        self.command
    }
}

#[derive(Debug)]
pub struct WorthQueryMutationDeclarationStop {
    error: WorthQueryRuntimeError,
}

impl WorthQueryMutationDeclarationStop {
    pub fn error(&self) -> &WorthQueryRuntimeError {
        &self.error
    }

    pub fn next_action(&self) -> super::WorthQueryMutationNextAction {
        super::WorthQueryMutationNextAction::ReviseDeclaration
    }
}

pub fn declare(
    author: impl FnOnce(
        WorthQueryAspectMutationBuilder,
    ) -> Result<WorthQueryWriteCommand, WorthQueryRuntimeError>,
) -> Result<WorthQueryMutationDeclaration, WorthQueryMutationDeclarationStop> {
    let command = author(WorthQueryAspectMutationBuilder::new())
        .map_err(|error| WorthQueryMutationDeclarationStop { error })?;
    let identity = mutation_declaration_identity(&command);
    Ok(WorthQueryMutationDeclaration {
        identity: WorthQueryMutationDeclarationIdentity { identity },
        command,
    })
}

fn mutation_declaration_identity(command: &WorthQueryWriteCommand) -> WorthQueryEvidenceIdentity {
    let collection = command.declared_collection_identity();
    let entity = command
        .declared_entity_identity_ref()
        .map(|identity| identity.evidence_identity());
    WorthQueryEvidenceIdentity::compose(WorthQueryEvidenceScope::WorkflowMutationLowering)
        .field_shape(
            WorthQueryEvidenceTag::new("role"),
            "ordinary-mutation-declaration",
        )
        .field_shape(
            WorthQueryEvidenceTag::new("family"),
            command.mutation_family().as_str(),
        )
        .optional_evidence_identity(
            WorthQueryEvidenceTag::new("collection"),
            collection
                .as_ref()
                .map(crate::runtime::WorthQueryMutationTargetCollectionIdentity::evidence_identity),
        )
        .optional_evidence_identity(WorthQueryEvidenceTag::new("entity"), entity.as_ref())
        .field_usize(
            WorthQueryEvidenceTag::new("aspect_operation_count"),
            command.declared_aspect_operations().len(),
        )
        .seal()
}
