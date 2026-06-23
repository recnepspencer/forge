use super::*;

pub(in crate::runtime::tests) struct TestIntentAuthority;

impl ForgeQueryIntentAuthorityAdapter for TestIntentAuthority {
    fn execute_intent(
        &mut self,
        _bridge: &RuntimeBridge,
        _relational_runtime: Option<&mut RelationalRuntime>,
        declaration: &ForgeQueryIntentDeclaration,
    ) -> Result<ForgeQueryIntentExecution, ForgeQueryWorkspaceError> {
        let mutation_receipt = test_mutation_receipt(
            crate::memory_workspace::admit_external_commit_label("external-intent-commit-1"),
            crate::memory_workspace::admit_external_snapshot_label("external-intent-snapshot-1"),
            "Task",
            crate::memory_workspace::admit_authored_entity_label("intent-task-1"),
            ForgeQueryMutationKind::Updated,
            test_aspect_touches(["title.value"]).to_vec(),
        );
        Ok(ForgeQueryIntentExecution::admitted(
            declaration.strategy_name(),
            declaration.strategy_version(),
            "test-strategy-descriptor-digest",
            declaration.input_digest(),
            crate::identity::hash_parts(&[
                "test-intent-produced-mutation".to_string(),
                mutation_receipt
                    .commit_identity
                    .evidence_identity()
                    .as_str()
                    .to_string(),
                mutation_receipt
                    .snapshot_identity
                    .evidence_identity()
                    .as_str()
                    .to_string(),
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
            crate::memory_workspace::admit_external_commit_label("external-intent-noop-commit-1"),
            crate::memory_workspace::admit_external_snapshot_label(
                "external-intent-noop-snapshot-1",
            ),
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
        let mutation_receipt = test_empty_mutation_receipt(
            crate::memory_workspace::admit_external_commit_label(
                "external-intent-empty-mutating-commit-1",
            ),
            crate::memory_workspace::admit_external_snapshot_label(
                "external-intent-empty-mutating-snapshot-1",
            ),
        );
        Ok(ForgeQueryIntentExecution::admitted(
            declaration.strategy_name(),
            declaration.strategy_version(),
            "test-empty-mutating-strategy-descriptor-digest",
            declaration.input_digest(),
            crate::identity::hash_parts(&[
                "test-intent-empty-mutating".to_string(),
                mutation_receipt
                    .commit_identity
                    .evidence_identity()
                    .as_str()
                    .to_string(),
                mutation_receipt
                    .snapshot_identity
                    .evidence_identity()
                    .as_str()
                    .to_string(),
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
            crate::memory_workspace::admit_external_snapshot_label(
                "external-intent-invariant-denial-snapshot-1",
            ),
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
        let mutation_receipt = test_mutation_receipt(
            crate::memory_workspace::admit_external_commit_label("external-intent-commit-1"),
            crate::memory_workspace::admit_external_snapshot_label("external-intent-snapshot-1"),
            "Task",
            crate::memory_workspace::admit_authored_entity_label("intent-task-1"),
            ForgeQueryMutationKind::Updated,
            test_aspect_touches(["title.value"]).to_vec(),
        );
        Ok(ForgeQueryIntentExecution::admitted(
            "strategy.intent.other",
            declaration.strategy_version(),
            "test-strategy-descriptor-digest",
            declaration.input_digest(),
            crate::identity::hash_parts(&[
                "test-intent-produced-mutation".to_string(),
                mutation_receipt
                    .commit_identity
                    .evidence_identity()
                    .as_str()
                    .to_string(),
                mutation_receipt
                    .snapshot_identity
                    .evidence_identity()
                    .as_str()
                    .to_string(),
            ]),
            ["test-invariant-authority"],
            mutation_receipt,
        ))
    }
}
