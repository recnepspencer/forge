use worth_foundational::facade::AspectValue;

use crate::authoring::AspectFieldKey;
use crate::workflow::QueryWritebackDeclaration;

#[cfg(test)]
use crate::identity::hash_parts;
#[cfg(test)]
use crate::workflow::{
    admit_query_workflow_declaration, bind_workflow_context, lower_query_writeback_declaration,
    WorkflowAuthorityTargetFamily, WorkflowBindingSource, WorkflowBudgetClass, WorkflowCostClass,
    WorkflowDeclarationFamily, WorkflowDeclarationRequest, WorkflowFreshnessPolicy,
    WritebackLoweringInput,
};
#[cfg(test)]
use worth_foundational::facade::prepare_aspect_value_identity_basis;

#[cfg(test)]
use super::session::DeclarativeLiveQuerySession;
#[cfg(test)]
use super::DeclarativeLiveQueryError;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DeclarativeWritebackValue {
    value: AspectValue,
}

impl DeclarativeWritebackValue {
    pub fn string(value: impl Into<String>) -> Self {
        Self {
            value: crate::runtime::WorthQueryAuthoredAspectMutation::native_string_value(value),
        }
    }

    pub fn integer(value: i64) -> Self {
        Self {
            value: AspectValue::Int64(value),
        }
    }

    pub fn boolean(value: bool) -> Self {
        Self {
            value: AspectValue::Bool(value),
        }
    }

    pub fn aspect_value(&self) -> &AspectValue {
        &self.value
    }

    #[cfg(test)]
    fn digest_part(&self) -> String {
        format!(
            "aspect_value:{}",
            prepare_aspect_value_identity_basis(&self.value).as_str()
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DeclarativeWritebackChange {
    source: AspectFieldKey,
    value: DeclarativeWritebackValue,
}

impl DeclarativeWritebackChange {
    pub fn new(source: AspectFieldKey, value: DeclarativeWritebackValue) -> Self {
        Self { source, value }
    }

    pub fn source_field_key(&self) -> &AspectFieldKey {
        &self.source
    }

    pub fn value(&self) -> &DeclarativeWritebackValue {
        &self.value
    }

    #[cfg(test)]
    fn digest_part(&self) -> String {
        format!(
            "change:{}:{}:{}",
            self.source_field_key().aspect().as_str(),
            self.source_field_key().field().as_str(),
            self.value.digest_part()
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DeclarativeWritebackIntent {
    changes: Vec<DeclarativeWritebackChange>,
}

impl DeclarativeWritebackIntent {
    pub fn new(changes: impl IntoIterator<Item = DeclarativeWritebackChange>) -> Self {
        Self {
            changes: changes.into_iter().collect(),
        }
    }

    pub fn update_aspect(source: AspectFieldKey, value: DeclarativeWritebackValue) -> Self {
        Self::new([DeclarativeWritebackChange::new(source, value)])
    }

    pub fn changes(&self) -> &[DeclarativeWritebackChange] {
        &self.changes
    }

    #[cfg(test)]
    fn digest(&self) -> String {
        let mut parts = vec![format!("change_count:{}", self.changes.len())];
        parts.extend(
            self.changes
                .iter()
                .map(DeclarativeWritebackChange::digest_part),
        );
        hash_parts(&parts)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DeclarativeWritebackArtifact {
    live_view_basis_digest: String,
    intent_digest: String,
    changes: Vec<DeclarativeWritebackChange>,
    declaration: QueryWritebackDeclaration,
    artifact_digest: String,
}

impl DeclarativeWritebackArtifact {
    pub fn live_view_basis_digest(&self) -> &str {
        &self.live_view_basis_digest
    }

    pub fn intent_digest(&self) -> &str {
        &self.intent_digest
    }

    pub fn changes(&self) -> &[DeclarativeWritebackChange] {
        &self.changes
    }

    pub fn declaration(&self) -> &QueryWritebackDeclaration {
        &self.declaration
    }

    pub fn artifact_digest(&self) -> &str {
        &self.artifact_digest
    }
}

#[cfg(test)]
pub fn declare_writeback_from_live_session(
    session: &DeclarativeLiveQuerySession,
    intent: DeclarativeWritebackIntent,
) -> Result<DeclarativeWritebackArtifact, DeclarativeLiveQueryError> {
    if intent.changes().is_empty() {
        return Err(DeclarativeLiveQueryError::EmptyWritebackIntent);
    }

    let binding =
        bind_workflow_context(WorkflowBindingSource::RuntimePreflight(session.preflight()))
            .map_err(|error| DeclarativeLiveQueryError::Writeback(format!("{error:?}")))?;
    let workflow = admit_query_workflow_declaration(
        &binding,
        WorkflowDeclarationRequest::new(
            WorkflowDeclarationFamily::WritebackLoweringNarrow,
            WorkflowAuthorityTargetFamily::BridgeWriteback,
            WorkflowCostClass::WritebackLoweringNarrow,
            WorkflowBudgetClass::AuthorityTargetBounded,
            WorkflowFreshnessPolicy::ExactBasis,
        ),
    )
    .map_err(|error| DeclarativeLiveQueryError::Writeback(format!("{error:?}")))?;
    let declaration = lower_query_writeback_declaration(
        &workflow,
        WritebackLoweringInput::projected_state_diff(),
    )
    .map_err(|error| DeclarativeLiveQueryError::Writeback(format!("{error:?}")))?;

    let live_view_basis_digest = session
        .preflight()
        .basis()
        .proof()
        .digest()
        .as_str()
        .to_string();
    let intent_digest = intent.digest();
    let artifact_digest = hash_parts(&[
        format!("basis:{live_view_basis_digest}"),
        format!("intent:{intent_digest}"),
        format!("writeback:{}", declaration.lowering_for_reporting()),
    ]);

    Ok(DeclarativeWritebackArtifact {
        live_view_basis_digest,
        intent_digest,
        changes: intent.changes,
        declaration,
        artifact_digest,
    })
}
