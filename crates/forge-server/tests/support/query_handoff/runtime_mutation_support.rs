use forge_query::facade::{
    ForgeQueryCommitIdentity, ForgeQueryEntityIdentity, ForgeQueryEvidenceIdentity,
    ForgeQueryMutationDelta, ForgeQueryMutationKind, ForgeQueryMutationReceipt,
    ForgeQueryRuntimeSubscriptionActivationAdapter, ForgeQuerySnapshotIdentity,
    ForgeQueryWorkspaceError, ForgeQueryWriteCommand, SubscriptionActivationInput,
};
use forge_runtime_bridge::facade::{
    RelationalBridgeRecordIdentityParts, RelationalBridgeSnapshotIdentityParts,
};

pub(crate) fn test_mutation_receipt(
    command: &ForgeQueryWriteCommand,
    ordinal: usize,
) -> ForgeQueryMutationReceipt {
    ForgeQueryMutationReceipt::from_authoritative_parts(
        ForgeQueryCommitIdentity::from_relational_commit_id(ordinal as u64),
        ForgeQuerySnapshotIdentity::from_relational_snapshot(
            RelationalBridgeSnapshotIdentityParts::new(ordinal as u64, 1),
        ),
        vec![ForgeQueryMutationDelta::new(
            mutation_collection(command),
            mutation_entity_identity(command, ordinal),
            mutation_kind(command),
            command.declared_aspect_paths(),
        )],
    )
}

pub(crate) struct TestSubscriptionActivation;

impl ForgeQueryRuntimeSubscriptionActivationAdapter for TestSubscriptionActivation {
    fn support_evidence_identity(&self) -> ForgeQueryEvidenceIdentity {
        forge_query::facade::runtime_subscription_support_evidence_identity(
            "forge-server-query-handoff-test-support",
        )
    }

    fn admit_activation(
        &mut self,
        view_name: &str,
        activation: &SubscriptionActivationInput,
    ) -> Result<forge_query::facade::SubscriptionActivationBoundaryReceipt, ForgeQueryWorkspaceError>
    {
        let receipt = self.build_subscription_activation_receipt(view_name, activation);
        Ok(self.build_subscription_activation_boundary_receipt(view_name, activation, receipt))
    }
}

fn mutation_collection(command: &ForgeQueryWriteCommand) -> String {
    command
        .declared_collection()
        .unwrap_or_else(|| "Task".to_string())
}

fn mutation_entity_identity(
    command: &ForgeQueryWriteCommand,
    ordinal: usize,
) -> ForgeQueryEntityIdentity {
    command.declared_entity_identity().unwrap_or_else(|| {
        ForgeQueryEntityIdentity::from_relational_record(
            RelationalBridgeRecordIdentityParts::entity(1, ordinal as u64, 0),
        )
    })
}

fn mutation_kind(command: &ForgeQueryWriteCommand) -> ForgeQueryMutationKind {
    match command.mutation_family().as_str() {
        "insert" => ForgeQueryMutationKind::Created,
        "delete" => ForgeQueryMutationKind::Deleted,
        _ => ForgeQueryMutationKind::Updated,
    }
}
