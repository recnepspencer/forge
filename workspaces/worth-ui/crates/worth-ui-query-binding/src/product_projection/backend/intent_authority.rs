use worth_foundational::facade::{AspectKey, AspectValue, CanonicalFieldPath, FieldKey};
use worth_query::facade::{foundation, runtime};
use worth_runtime_bridge::facade::{RuntimeBridge, TruthCommitIdentity};

use super::super::source_record::platform_pulse_entity_identity;
use super::mutation_authority::WorthUiScalarProjectionMutationAuthority;
use super::snapshot::projection_snapshot_identity;
use super::state::SharedSourceState;

pub(super) struct WorthUiScalarProjectionIntentAuthority {
    state: SharedSourceState,
}

impl WorthUiScalarProjectionIntentAuthority {
    pub(super) fn new(state: SharedSourceState) -> Self {
        Self { state }
    }
}

impl runtime::WorthQueryIntentAuthorityAdapter for WorthUiScalarProjectionIntentAuthority {
    fn execute_intent(
        &mut self,
        bridge: &RuntimeBridge,
        _relational_runtime: Option<&mut worth_relational::facade::runtime::RelationalRuntime>,
        declaration: &runtime::WorthQueryIntentDeclaration,
    ) -> Result<runtime::WorthQueryIntentExecution, foundation::WorthQueryWorkspaceError> {
        validate_declaration(declaration)?;
        let revision = declaration
            .input_string_field("source_revision")
            .ok_or_else(|| workspace_error("product action omitted `source_revision`"))?
            .parse::<u64>()
            .map_err(|_| workspace_error("product action `source_revision` was not a u64"))?;
        let status = declaration
            .input_string_field("status")
            .ok_or_else(|| workspace_error("product action omitted `status`"))?;
        let next_positions = {
            let state = self.state.borrow();
            let record = state
                .record()
                .ok_or_else(|| workspace_error("product action has no installed source record"))?;
            if record.revision() != revision {
                return Err(workspace_error(
                    "product action source revision no longer matches current Query truth",
                ));
            }
            state.next_authoritative_positions()
        };
        let next_record = super::super::WorthUiScalarProjectionSourceRecord::new(status, revision)
            .map_err(workspace_error)?;
        let mutation = admitted_status_mutation(status)?;
        let entity_identity = platform_pulse_entity_identity();
        let snapshot_identity = projection_snapshot_identity(next_positions.1);
        let mutation_authority = WorthUiScalarProjectionMutationAuthority;
        use runtime::WorthQueryRuntimeWriteAuthorityAdapter;
        let bridge_authority = mutation_authority.build_bridge_mutation_authority_bundle(
            bridge,
            &snapshot_identity,
            &mutation,
            "WorthUiProjectionText",
            &entity_identity,
            foundation::WorthQueryMutationKind::Updated,
        )?;
        let mutation_receipt =
            foundation::WorthQueryMutationReceipt::from_bridge_authoritative_parts(
                foundation::WorthQueryCommitIdentity::from_bridge_commit_projection(
                    TruthCommitIdentity::from_relational_commit_id(next_positions.0),
                ),
                snapshot_identity,
                vec![foundation::WorthQueryMutationDelta::from_touched_aspects(
                    "WorthUiProjectionText",
                    entity_identity,
                    foundation::WorthQueryMutationKind::Updated,
                    mutation.declared_aspect_touches(),
                )],
                bridge_authority,
            );
        self.state
            .borrow_mut()
            .commit_action(revision, next_positions, next_record)
            .map_err(workspace_error)?;

        Ok(runtime::WorthQueryIntentExecution::admitted(
            declaration.strategy_name(),
            declaration.strategy_version(),
            "worth-ui-platform-pulse-status-rebind-v1",
            declaration.input_digest(),
            format!(
                "worth-ui-platform-pulse-action:{}:{}:{}",
                revision, next_positions.0, next_positions.1
            ),
            [
                "worth-ui-source-revision-match",
                "worth-ui-native-contract-admission",
            ],
            mutation_receipt,
        ))
    }
}

fn validate_declaration(
    declaration: &runtime::WorthQueryIntentDeclaration,
) -> Result<(), foundation::WorthQueryWorkspaceError> {
    if declaration.name() != super::super::action_contract::PRODUCT_ACTION_NAME
        || declaration.strategy_name() != super::super::action_contract::PRODUCT_ACTION_STRATEGY
        || declaration.strategy_version()
            != super::super::action_contract::PRODUCT_ACTION_STRATEGY_VERSION
        || declaration.input_contract()
            != super::super::action_contract::PRODUCT_ACTION_INPUT_CONTRACT
    {
        return Err(workspace_error(
            "product intent declaration does not match the installed action contract",
        ));
    }
    Ok(())
}

fn admitted_status_mutation(
    status: &str,
) -> Result<runtime::WorthQueryBackendAdmissibleMutation, foundation::WorthQueryWorkspaceError> {
    runtime::WorthQueryBackendAdmissibleMutation::admit_native_field_update(
        platform_pulse_entity_identity(),
        AspectKey::new("query_text").expect("static product aspect must admit"),
        CanonicalFieldPath::single(
            FieldKey::new("status").expect("static product field must admit"),
        ),
        AspectValue::String(status.into()),
        crate::worth_ui_native_aspect_contracts(),
    )
    .map_err(|error| workspace_error(format!("product action contract denied: {error:?}")))
}

fn workspace_error(detail: impl Into<String>) -> foundation::WorthQueryWorkspaceError {
    foundation::WorthQueryWorkspaceError::new(detail)
}
