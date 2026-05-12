use super::*;

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
            bridge_authority: None,
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
            bridge_authority: None,
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
            bridge_authority: None,
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
