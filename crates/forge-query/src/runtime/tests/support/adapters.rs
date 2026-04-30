use super::*;
use crate::memory_workspace::{ForgeQueryLivePatch, ForgeQueryLiveViewHandle};

impl ForgeQueryRuntimeSchemaAdapter for TestSchemaAdapter {
    fn admit_live_view(
        &self,
        _name: &str,
        _request: &DeclarativeLiveQueryRequest,
        _schema_view: &QuerySchemaView,
    ) -> Result<(), ForgeQueryWorkspaceError> {
        Ok(())
    }
}

pub(in crate::runtime::tests) struct TestSchemaAdapter;

pub(in crate::runtime::tests) struct DenyingSchemaAdapter;

impl ForgeQueryRuntimeSchemaAdapter for DenyingSchemaAdapter {
    fn admit_live_view(
        &self,
        _name: &str,
        _request: &DeclarativeLiveQueryRequest,
        _schema_view: &QuerySchemaView,
    ) -> Result<(), ForgeQueryWorkspaceError> {
        Err(ForgeQueryWorkspaceError::new(
            "schema admission denied by test adapter",
        ))
    }
}

#[derive(Default)]
pub(in crate::runtime::tests) struct TestSourceAdapter {
    live_views: BTreeMap<String, String>,
    fail_declare: bool,
}

impl TestSourceAdapter {
    pub(in crate::runtime::tests) fn fail_declare() -> Self {
        Self {
            live_views: BTreeMap::new(),
            fail_declare: true,
        }
    }
}

impl ForgeQueryRuntimeSourceAdapter for TestSourceAdapter {
    fn declare_live_view(
        &mut self,
        name: String,
        request: DeclarativeLiveQueryRequest,
        _schema_view: QuerySchemaView,
    ) -> Result<ForgeQueryLiveViewHandle, ForgeQueryWorkspaceError> {
        if self.fail_declare {
            return Err(ForgeQueryWorkspaceError::new(
                "source declaration denied by test adapter",
            ));
        }
        self.live_views
            .insert(name.clone(), request.target().to_string());
        Ok(ForgeQueryLiveViewHandle::new(name))
    }

    fn live_entities(&self, _view_name: &str) -> Vec<ForgeQueryEntity> {
        Vec::new()
    }

    fn drain_live_patches(&mut self, _view_name: &str) -> Vec<ForgeQueryLivePatch> {
        Vec::new()
    }

    fn affected_live_view_ids(&self, receipt: &ForgeQueryMutationReceipt) -> Vec<String> {
        let mut affected = receipt
            .deltas
            .iter()
            .flat_map(|delta| {
                self.live_views
                    .iter()
                    .filter(move |(_, collection)| *collection == &delta.collection)
                    .map(|(name, _)| name.clone())
            })
            .collect::<Vec<_>>();
        affected.sort();
        affected.dedup();
        affected
    }

    fn snapshot_token(&self) -> String {
        "external-snapshot".to_string()
    }
}

#[derive(Default)]
pub(in crate::runtime::tests) struct DriftingSnapshotSourceAdapter {
    snapshot_sequence: std::cell::Cell<u64>,
}

impl ForgeQueryRuntimeSourceAdapter for DriftingSnapshotSourceAdapter {
    fn declare_live_view(
        &mut self,
        name: String,
        _request: DeclarativeLiveQueryRequest,
        _schema_view: QuerySchemaView,
    ) -> Result<ForgeQueryLiveViewHandle, ForgeQueryWorkspaceError> {
        Ok(ForgeQueryLiveViewHandle::new(name))
    }

    fn live_entities(&self, _view_name: &str) -> Vec<ForgeQueryEntity> {
        Vec::new()
    }

    fn drain_live_patches(&mut self, _view_name: &str) -> Vec<ForgeQueryLivePatch> {
        Vec::new()
    }

    fn affected_live_view_ids(&self, _receipt: &ForgeQueryMutationReceipt) -> Vec<String> {
        Vec::new()
    }

    fn snapshot_token(&self) -> String {
        let next = self.snapshot_sequence.get() + 1;
        self.snapshot_sequence.set(next);
        format!("drifting-snapshot-{next}")
    }
}

pub(in crate::runtime::tests) struct TestWriteAuthority;

impl ForgeQueryRuntimeWriteAuthorityAdapter for TestWriteAuthority {
    #[allow(deprecated)]
    fn write(
        &mut self,
        _bridge: &RuntimeBridge,
        _relational_runtime: Option<&mut RelationalRuntime>,
        command: ForgeQueryWriteCommand,
    ) -> Result<ForgeQueryMutationReceipt, ForgeQueryWorkspaceError> {
        let aspect_paths = command.declared_aspect_paths();
        let collection = match command {
            ForgeQueryWriteCommand::Insert { collection, .. } => collection,
            ForgeQueryWriteCommand::InsertAspects { collection, .. } => collection,
            ForgeQueryWriteCommand::UpdateAspect { .. } => "Task".to_string(),
            ForgeQueryWriteCommand::UpdateAspects { .. } => "Task".to_string(),
            ForgeQueryWriteCommand::Delete { .. } => "Task".to_string(),
        };
        Ok(ForgeQueryMutationReceipt {
            commit_identity: "external-commit-1".to_string(),
            snapshot_token: "external-snapshot-1".to_string(),
            deltas: vec![crate::memory_workspace::ForgeQueryMutationDelta {
                collection,
                entity_identity: "external-entity-1".to_string(),
                kind: ForgeQueryMutationKind::Created,
                aspect_paths,
            }],
        })
    }
}

pub(in crate::runtime::tests) struct DenyingWriteAuthority;

impl ForgeQueryRuntimeWriteAuthorityAdapter for DenyingWriteAuthority {
    fn write(
        &mut self,
        _bridge: &RuntimeBridge,
        _relational_runtime: Option<&mut RelationalRuntime>,
        _command: ForgeQueryWriteCommand,
    ) -> Result<ForgeQueryMutationReceipt, ForgeQueryWorkspaceError> {
        Err(ForgeQueryWorkspaceError::new(
            "write authority denied by test",
        ))
    }
}

pub(in crate::runtime::tests) struct CountingWriteAuthority {
    pub(in crate::runtime::tests) attempted_writes: std::rc::Rc<std::cell::Cell<usize>>,
}

impl ForgeQueryRuntimeWriteAuthorityAdapter for CountingWriteAuthority {
    fn write(
        &mut self,
        _bridge: &RuntimeBridge,
        _relational_runtime: Option<&mut RelationalRuntime>,
        command: ForgeQueryWriteCommand,
    ) -> Result<ForgeQueryMutationReceipt, ForgeQueryWorkspaceError> {
        self.attempted_writes
            .set(self.attempted_writes.get().saturating_add(1));
        let mut authority = TestWriteAuthority;
        authority.write(_bridge, _relational_runtime, command)
    }
}

pub(in crate::runtime::tests) struct TestIntentAuthority;

impl ForgeQueryIntentAuthorityAdapter for TestIntentAuthority {
    fn execute_intent(
        &mut self,
        _bridge: &RuntimeBridge,
        _relational_runtime: Option<&mut RelationalRuntime>,
        declaration: &ForgeQueryIntentDeclaration,
    ) -> Result<ForgeQueryIntentExecution, ForgeQueryWorkspaceError> {
        let mutation_receipt = ForgeQueryMutationReceipt {
            commit_identity: "external-intent-commit-1".to_string(),
            snapshot_token: "external-intent-snapshot-1".to_string(),
            deltas: vec![crate::memory_workspace::ForgeQueryMutationDelta {
                collection: "Task".to_string(),
                entity_identity: "intent-task-1".to_string(),
                kind: ForgeQueryMutationKind::Updated,
                aspect_paths: vec!["title.value".to_string()],
            }],
        };
        Ok(ForgeQueryIntentExecution::admitted(
            declaration.strategy_name(),
            declaration.strategy_version(),
            "test-strategy-descriptor-digest",
            declaration.input_digest(),
            crate::identity::hash_parts(&[
                "test-intent-produced-mutation".to_string(),
                mutation_receipt.commit_identity.clone(),
                mutation_receipt.snapshot_token.clone(),
            ]),
            ["test-invariant-authority"],
            mutation_receipt,
        ))
    }
}

pub(in crate::runtime::tests) struct CountingIntentAuthority {
    pub(in crate::runtime::tests) attempted: std::rc::Rc<std::cell::Cell<usize>>,
}

impl ForgeQueryIntentAuthorityAdapter for CountingIntentAuthority {
    fn execute_intent(
        &mut self,
        _bridge: &RuntimeBridge,
        _relational_runtime: Option<&mut RelationalRuntime>,
        declaration: &ForgeQueryIntentDeclaration,
    ) -> Result<ForgeQueryIntentExecution, ForgeQueryWorkspaceError> {
        self.attempted.set(self.attempted.get().saturating_add(1));
        let mut authority = TestIntentAuthority;
        authority.execute_intent(_bridge, _relational_runtime, declaration)
    }
}

pub(in crate::runtime::tests) struct NoopIntentAuthority;

impl ForgeQueryIntentAuthorityAdapter for NoopIntentAuthority {
    fn execute_intent(
        &mut self,
        _bridge: &RuntimeBridge,
        _relational_runtime: Option<&mut RelationalRuntime>,
        declaration: &ForgeQueryIntentDeclaration,
    ) -> Result<ForgeQueryIntentExecution, ForgeQueryWorkspaceError> {
        Ok(ForgeQueryIntentExecution::idempotent_noop(
            declaration.strategy_name(),
            declaration.strategy_version(),
            "test-noop-strategy-descriptor-digest",
            declaration.input_digest(),
            crate::identity::hash_parts(&[
                "test-intent-idempotent-noop".to_string(),
                declaration.input_digest().to_string(),
            ]),
            ["test-invariant-authority", "idempotent-noop"],
            "external-intent-noop-commit-1",
            "external-intent-noop-snapshot-1",
        ))
    }
}

pub(in crate::runtime::tests) struct EmptyMutatingIntentAuthority;

impl ForgeQueryIntentAuthorityAdapter for EmptyMutatingIntentAuthority {
    fn execute_intent(
        &mut self,
        _bridge: &RuntimeBridge,
        _relational_runtime: Option<&mut RelationalRuntime>,
        declaration: &ForgeQueryIntentDeclaration,
    ) -> Result<ForgeQueryIntentExecution, ForgeQueryWorkspaceError> {
        let mutation_receipt = ForgeQueryMutationReceipt {
            commit_identity: "external-intent-empty-mutating-commit-1".to_string(),
            snapshot_token: "external-intent-empty-mutating-snapshot-1".to_string(),
            deltas: Vec::new(),
        };
        Ok(ForgeQueryIntentExecution::admitted(
            declaration.strategy_name(),
            declaration.strategy_version(),
            "test-empty-mutating-strategy-descriptor-digest",
            declaration.input_digest(),
            crate::identity::hash_parts(&[
                "test-intent-empty-mutating".to_string(),
                mutation_receipt.commit_identity.clone(),
                mutation_receipt.snapshot_token.clone(),
            ]),
            ["test-invariant-authority"],
            mutation_receipt,
        ))
    }
}

pub(in crate::runtime::tests) struct InvariantViolationIntentAuthority;

impl ForgeQueryIntentAuthorityAdapter for InvariantViolationIntentAuthority {
    fn execute_intent(
        &mut self,
        _bridge: &RuntimeBridge,
        _relational_runtime: Option<&mut RelationalRuntime>,
        declaration: &ForgeQueryIntentDeclaration,
    ) -> Result<ForgeQueryIntentExecution, ForgeQueryWorkspaceError> {
        Ok(ForgeQueryIntentExecution::invariant_violation(
            declaration.strategy_name(),
            declaration.strategy_version(),
            "test-invariant-violation-strategy-descriptor-digest",
            declaration.input_digest(),
            crate::identity::hash_parts(&[
                "test-intent-invariant-violation".to_string(),
                declaration.input_digest().to_string(),
            ]),
            [
                "relational-invariant:constraint-a:false",
                "relational-invariant:constraint-b:false",
            ],
            "external-intent-invariant-denial-snapshot-1",
        ))
    }
}

pub(in crate::runtime::tests) struct DriftingIntentAuthority;

impl ForgeQueryIntentAuthorityAdapter for DriftingIntentAuthority {
    fn execute_intent(
        &mut self,
        _bridge: &RuntimeBridge,
        _relational_runtime: Option<&mut RelationalRuntime>,
        declaration: &ForgeQueryIntentDeclaration,
    ) -> Result<ForgeQueryIntentExecution, ForgeQueryWorkspaceError> {
        let mutation_receipt = ForgeQueryMutationReceipt {
            commit_identity: "external-intent-commit-1".to_string(),
            snapshot_token: "external-intent-snapshot-1".to_string(),
            deltas: vec![crate::memory_workspace::ForgeQueryMutationDelta {
                collection: "Task".to_string(),
                entity_identity: "intent-task-1".to_string(),
                kind: ForgeQueryMutationKind::Updated,
                aspect_paths: vec!["title.value".to_string()],
            }],
        };
        Ok(ForgeQueryIntentExecution::admitted(
            "strategy.intent.other",
            declaration.strategy_version(),
            "test-strategy-descriptor-digest",
            declaration.input_digest(),
            crate::identity::hash_parts(&[
                "test-intent-produced-mutation".to_string(),
                mutation_receipt.commit_identity.clone(),
                mutation_receipt.snapshot_token.clone(),
            ]),
            ["test-invariant-authority"],
            mutation_receipt,
        ))
    }
}

pub(in crate::runtime::tests) struct TestSignalSink;

impl ForgeQueryRuntimeSignalSinkAdapter for TestSignalSink {
    fn route_write_receipt(
        &mut self,
        _receipt: &ForgeQueryMutationReceipt,
    ) -> Result<(), ForgeQueryWorkspaceError> {
        Ok(())
    }
}

pub(in crate::runtime::tests) struct CountingSignalSink {
    pub(in crate::runtime::tests) routed: std::rc::Rc<std::cell::Cell<usize>>,
}

impl ForgeQueryRuntimeSignalSinkAdapter for CountingSignalSink {
    fn route_write_receipt(
        &mut self,
        _receipt: &ForgeQueryMutationReceipt,
    ) -> Result<(), ForgeQueryWorkspaceError> {
        self.routed.set(self.routed.get().saturating_add(1));
        Ok(())
    }
}

pub(in crate::runtime::tests) struct TestSubscriptionActivation;

impl ForgeQueryRuntimeSubscriptionActivationAdapter for TestSubscriptionActivation {
    fn support_evidence(&self) -> String {
        "test-subscription-activation".to_string()
    }

    fn admit_activation(
        &mut self,
        view_name: &str,
        activation: &crate::subscription::SubscriptionActivationInput,
    ) -> Result<String, ForgeQueryWorkspaceError> {
        Ok(format!(
            "test-subscription-activation:{view_name}:{}",
            activation.activation_digest()
        ))
    }
}

pub(in crate::runtime::tests) struct DenyingSubscriptionActivation;

impl ForgeQueryRuntimeSubscriptionActivationAdapter for DenyingSubscriptionActivation {
    fn support_evidence(&self) -> String {
        "denying-subscription-activation".to_string()
    }

    fn admit_activation(
        &mut self,
        _view_name: &str,
        _activation: &crate::subscription::SubscriptionActivationInput,
    ) -> Result<String, ForgeQueryWorkspaceError> {
        Err(ForgeQueryWorkspaceError::new(
            "activation denied by test adapter",
        ))
    }
}

pub(in crate::runtime::tests) struct TestPreviewBasis;

impl ForgeQueryRuntimePreviewBasisAdapter for TestPreviewBasis {
    fn admit_preview_basis(
        &self,
        label: &str,
        effect_policy: ForgeQueryEffectPolicy,
        authority: &ForgeQueryRuntimeEvidenceAuthority,
    ) -> Result<ForgeQueryPreviewBasisAdmission, ForgeQueryWorkspaceError> {
        Ok(ForgeQueryPreviewBasisAdmission::new(
            authority,
            label,
            effect_policy,
            ["test-preview-basis"],
        ))
    }
}

pub(in crate::runtime::tests) struct TestInspectorEvidence;

impl ForgeQueryRuntimeInspectorEvidenceAdapter for TestInspectorEvidence {
    fn inspect_write_receipt(
        &self,
        receipt: &ForgeQueryWriteReceipt,
        authority: &ForgeQueryRuntimeEvidenceAuthority,
    ) -> Result<ForgeQueryRuntimeInspectionEvidence, ForgeQueryWorkspaceError> {
        Ok(ForgeQueryRuntimeInspectionEvidence::new(
            authority,
            "test-write-receipt",
            receipt.authority_lane(),
            ["test-inspector-evidence"],
        ))
    }
}
