use super::*;

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
            ForgeQueryWriteCommand::UpdateExistingAspects { binding, .. } => {
                binding.target_collection().unwrap_or("Task").to_string()
            }
            ForgeQueryWriteCommand::UpdateSymbolicAspects { reference, .. } => {
                reference.target_collection().unwrap_or("Task").to_string()
            }
            ForgeQueryWriteCommand::DeleteAspects { .. } => "Task".to_string(),
            ForgeQueryWriteCommand::DeleteExistingAspects { binding, .. } => {
                binding.target_collection().unwrap_or("Task").to_string()
            }
            ForgeQueryWriteCommand::DeleteSymbolicAspects { reference, .. } => {
                reference.target_collection().unwrap_or("Task").to_string()
            }
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
