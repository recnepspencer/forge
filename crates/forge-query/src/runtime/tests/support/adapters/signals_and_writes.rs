use super::*;

fn command_collection(command: &ForgeQueryWriteCommand) -> String {
    command
        .declared_collection()
        .unwrap_or_else(|| match command.mutation_family() {
            ForgeQueryMutationFamily::Insert
            | ForgeQueryMutationFamily::Update
            | ForgeQueryMutationFamily::Delete
            | ForgeQueryMutationFamily::Assertion => "Task".to_string(),
        })
}

pub(in crate::runtime::tests) struct TestWriteAuthority;

impl ForgeQueryRuntimeWriteAuthorityAdapter for TestWriteAuthority {
    fn write(
        &mut self,
        _bridge: &RuntimeBridge,
        _relational_runtime: Option<&mut RelationalRuntime>,
        command: ForgeQueryWriteCommand,
    ) -> Result<ForgeQueryMutationReceipt, ForgeQueryWorkspaceError> {
        let aspect_paths = command.declared_aspect_paths();
        let collection = command_collection(&command);
        Ok(ForgeQueryMutationReceipt {
            commit_identity: "external-commit-1".to_string(),
            snapshot_token: "external-snapshot-1".to_string(),
            deltas: vec![crate::memory_workspace::ForgeQueryMutationDelta {
                collection,
                entity_identity: "external-entity-1".to_string(),
                kind: ForgeQueryMutationKind::Created,
                aspect_paths,
            }],
            bridge_authority: None,
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

pub(in crate::runtime::tests) struct AtomicBatchCountingWriteAuthority {
    pub(in crate::runtime::tests) attempted_writes: std::rc::Rc<std::cell::Cell<usize>>,
    pub(in crate::runtime::tests) attempted_batches: std::rc::Rc<std::cell::Cell<usize>>,
}

impl ForgeQueryRuntimeWriteAuthorityAdapter for AtomicBatchCountingWriteAuthority {
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

    fn write_batch(
        &mut self,
        _bridge: &RuntimeBridge,
        _relational_runtime: Option<&mut RelationalRuntime>,
        commands: Vec<ForgeQueryWriteCommand>,
    ) -> Result<Vec<ForgeQueryMutationReceipt>, ForgeQueryWorkspaceError> {
        self.attempted_batches
            .set(self.attempted_batches.get().saturating_add(1));
        let mut receipts = Vec::with_capacity(commands.len());
        for (index, command) in commands.into_iter().enumerate() {
            let aspect_paths = command.declared_aspect_paths();
            let collection = command_collection(&command);
            receipts.push(ForgeQueryMutationReceipt {
                commit_identity: "external-batch-commit-1".to_string(),
                snapshot_token: "external-batch-snapshot-1".to_string(),
                deltas: vec![crate::memory_workspace::ForgeQueryMutationDelta {
                    collection,
                    entity_identity: format!("external-entity-{}", index + 1),
                    kind: ForgeQueryMutationKind::Created,
                    aspect_paths,
                }],
                bridge_authority: None,
            });
        }
        Ok(receipts)
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
