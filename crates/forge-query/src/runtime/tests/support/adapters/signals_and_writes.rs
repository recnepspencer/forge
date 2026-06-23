use super::*;
use crate::runtime::backend::build_bridge_authority_bundle;
use forge_runtime_bridge::facade::RelationalBridgeSnapshotIdentityParts;

fn mutation_collection(mutation: &ForgeQueryBackendAdmissibleMutation) -> String {
    mutation
        .declared_collection_identity()
        .map(|collection| collection.as_str().to_string())
        .unwrap_or_else(|| match mutation.mutation_family() {
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
        mutation: ForgeQueryBackendAdmissibleMutation,
    ) -> Result<WriteAuthorityExecutionReceipt, ForgeQueryWorkspaceError> {
        let aspect_touches = mutation.declared_aspect_touches();
        let collection = mutation_collection(&mutation);
        let entity_identity =
            crate::memory_workspace::admit_authored_entity_label("external-entity-1");
        let snapshot_identity =
            crate::memory_workspace::ForgeQuerySnapshotIdentity::from_relational_snapshot(
                RelationalBridgeSnapshotIdentityParts::new(1, 1),
            );
        let bridge_authority = build_bridge_authority_bundle(
            _bridge,
            &snapshot_identity,
            &mutation,
            &collection,
            &entity_identity,
            ForgeQueryMutationKind::Created,
        )?;
        let receipt = test_mutation_receipt_with_bridge_authority(
            crate::memory_workspace::ForgeQueryCommitIdentity::from_relational_commit_id(1),
            snapshot_identity,
            collection,
            entity_identity,
            ForgeQueryMutationKind::Created,
            aspect_touches,
            bridge_authority,
        );
        Ok(self.build_write_authority_execution_receipt(&mutation, receipt))
    }
}

pub(in crate::runtime::tests) struct DenyingWriteAuthority;

impl ForgeQueryRuntimeWriteAuthorityAdapter for DenyingWriteAuthority {
    fn write(
        &mut self,
        _bridge: &RuntimeBridge,
        _relational_runtime: Option<&mut RelationalRuntime>,
        _mutation: ForgeQueryBackendAdmissibleMutation,
    ) -> Result<WriteAuthorityExecutionReceipt, ForgeQueryWorkspaceError> {
        Err(ForgeQueryWorkspaceError::new(
            "write authority denied by test",
        ))
    }
}

pub(in crate::runtime::tests) struct AuthorityLessWriteAuthority;

impl ForgeQueryRuntimeWriteAuthorityAdapter for AuthorityLessWriteAuthority {
    fn write(
        &mut self,
        _bridge: &RuntimeBridge,
        _relational_runtime: Option<&mut RelationalRuntime>,
        mutation: ForgeQueryBackendAdmissibleMutation,
    ) -> Result<WriteAuthorityExecutionReceipt, ForgeQueryWorkspaceError> {
        let collection = mutation_collection(&mutation);
        let receipt = test_mutation_receipt(
            crate::memory_workspace::admit_external_commit_label("authority-less-commit"),
            crate::memory_workspace::admit_external_snapshot_label("authority-less-snapshot"),
            collection,
            crate::memory_workspace::admit_authored_entity_label("authority-less-entity"),
            ForgeQueryMutationKind::Created,
            mutation.declared_aspect_touches(),
        );
        Ok(self.build_write_authority_execution_receipt(&mutation, receipt))
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
        mutation: ForgeQueryBackendAdmissibleMutation,
    ) -> Result<WriteAuthorityExecutionReceipt, ForgeQueryWorkspaceError> {
        self.attempted_writes
            .set(self.attempted_writes.get().saturating_add(1));
        let mut authority = TestWriteAuthority;
        authority.write(_bridge, _relational_runtime, mutation)
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
        mutation: ForgeQueryBackendAdmissibleMutation,
    ) -> Result<WriteAuthorityExecutionReceipt, ForgeQueryWorkspaceError> {
        self.attempted_writes
            .set(self.attempted_writes.get().saturating_add(1));
        let mut authority = TestWriteAuthority;
        authority.write(_bridge, _relational_runtime, mutation)
    }

    fn write_batch(
        &mut self,
        _bridge: &RuntimeBridge,
        _relational_runtime: Option<&mut RelationalRuntime>,
        mutations: Vec<ForgeQueryBackendAdmissibleMutation>,
    ) -> Result<Vec<WriteAuthorityExecutionReceipt>, ForgeQueryWorkspaceError> {
        self.attempted_batches
            .set(self.attempted_batches.get().saturating_add(1));
        let mut receipts = Vec::with_capacity(mutations.len());
        for (index, mutation) in mutations.into_iter().enumerate() {
            let aspect_touches = mutation.declared_aspect_touches();
            let collection = mutation_collection(&mutation);
            let entity_identity_text = format!("external-entity-{}", index + 1);
            let entity_identity =
                crate::memory_workspace::admit_authored_entity_label(&entity_identity_text);
            let snapshot_identity =
                crate::memory_workspace::ForgeQuerySnapshotIdentity::from_relational_snapshot(
                    RelationalBridgeSnapshotIdentityParts::new(10, index as u64 + 1),
                );
            let bridge_authority = build_bridge_authority_bundle(
                _bridge,
                &snapshot_identity,
                &mutation,
                &collection,
                &entity_identity,
                ForgeQueryMutationKind::Created,
            )?;
            let receipt = test_mutation_receipt_with_bridge_authority(
                crate::memory_workspace::ForgeQueryCommitIdentity::from_relational_commit_id(
                    index as u64 + 1,
                ),
                snapshot_identity,
                collection,
                entity_identity,
                ForgeQueryMutationKind::Created,
                aspect_touches,
                bridge_authority,
            );
            receipts.push(self.build_write_authority_execution_receipt(&mutation, receipt));
        }
        Ok(receipts)
    }
}

pub(in crate::runtime::tests) struct TestSignalSink;

impl ForgeQueryRuntimeSignalSinkAdapter for TestSignalSink {
    fn route_write_receipt(
        &mut self,
        receipt: &ForgeQueryMutationReceipt,
    ) -> Result<SignalInvalidationBoundaryReceipt, ForgeQueryWorkspaceError> {
        let routed = self.build_signal_invalidation_routing_receipt(receipt)?;
        self.build_signal_invalidation_boundary_receipt(receipt, routed)
    }
}

pub(in crate::runtime::tests) struct CountingSignalSink {
    pub(in crate::runtime::tests) routed: std::rc::Rc<std::cell::Cell<usize>>,
}

impl ForgeQueryRuntimeSignalSinkAdapter for CountingSignalSink {
    fn route_write_receipt(
        &mut self,
        receipt: &ForgeQueryMutationReceipt,
    ) -> Result<SignalInvalidationBoundaryReceipt, ForgeQueryWorkspaceError> {
        self.routed.set(self.routed.get().saturating_add(1));
        let routed = self.build_signal_invalidation_routing_receipt(receipt)?;
        self.build_signal_invalidation_boundary_receipt(receipt, routed)
    }
}

pub(in crate::runtime::tests) struct DriftingSignalSink;

impl ForgeQueryRuntimeSignalSinkAdapter for DriftingSignalSink {
    fn route_write_receipt(
        &mut self,
        receipt: &ForgeQueryMutationReceipt,
    ) -> Result<SignalInvalidationBoundaryReceipt, ForgeQueryWorkspaceError> {
        let mut drifted = receipt.clone();
        drifted.commit_identity =
            crate::memory_workspace::admit_external_commit_label("drifted-signal-routing-commit");
        let routed = self.build_signal_invalidation_routing_receipt(&drifted)?;
        self.build_signal_invalidation_boundary_receipt(&drifted, routed)
    }
}

pub(in crate::runtime::tests) struct TruncatingBatchSignalSink;

impl ForgeQueryRuntimeSignalSinkAdapter for TruncatingBatchSignalSink {
    fn route_write_receipt(
        &mut self,
        receipt: &ForgeQueryMutationReceipt,
    ) -> Result<SignalInvalidationBoundaryReceipt, ForgeQueryWorkspaceError> {
        let routed = self.build_signal_invalidation_routing_receipt(receipt)?;
        self.build_signal_invalidation_boundary_receipt(receipt, routed)
    }

    fn route_write_batch(
        &mut self,
        receipts: &[ForgeQueryMutationReceipt],
    ) -> Result<Vec<SignalInvalidationBoundaryReceipt>, ForgeQueryWorkspaceError> {
        receipts
            .iter()
            .take(receipts.len().saturating_sub(1))
            .map(|receipt| {
                let routed = self.build_signal_invalidation_routing_receipt(receipt)?;
                self.build_signal_invalidation_boundary_receipt(receipt, routed)
            })
            .collect::<Result<Vec<_>, _>>()
    }
}
