use worth_query::facade::{
    SubscriptionActivationInput, WorthQueryBackendAdmissibleMutation, WorthQueryCommitIdentity,
    WorthQueryEntityIdentity, WorthQueryEvidenceIdentity, WorthQueryMutationDelta,
    WorthQueryMutationKind, WorthQueryMutationReceipt,
    WorthQueryRuntimeSubscriptionActivationAdapter, WorthQuerySnapshotIdentity,
    WorthQueryWorkspaceError,
};
use worth_runtime_bridge::facade::{
    RelationalBridgeRecordIdentityParts, RelationalBridgeSnapshotIdentityParts,
};

pub(crate) fn test_mutation_receipt(
    command: &WorthQueryBackendAdmissibleMutation,
    ordinal: usize,
) -> WorthQueryMutationReceipt {
    WorthQueryMutationReceipt::from_authoritative_parts(
        WorthQueryCommitIdentity::from_relational_commit_id(ordinal as u64),
        WorthQuerySnapshotIdentity::from_relational_snapshot(
            RelationalBridgeSnapshotIdentityParts::new(ordinal as u64, 1),
        ),
        vec![WorthQueryMutationDelta::from_touched_aspects(
            mutation_collection(command),
            mutation_entity_identity(command, ordinal),
            mutation_kind(command),
            command.declared_aspect_touches(),
        )],
    )
}

pub(crate) struct TestSubscriptionActivation;

impl WorthQueryRuntimeSubscriptionActivationAdapter for TestSubscriptionActivation {
    fn support_evidence_identity(&self) -> WorthQueryEvidenceIdentity {
        worth_query::facade::runtime_subscription_support_evidence_identity(
            "worth-server-query-handoff-test-support",
        )
    }

    fn admit_activation(
        &mut self,
        view_name: &str,
        activation: &SubscriptionActivationInput,
    ) -> Result<worth_query::facade::SubscriptionActivationBoundaryReceipt, WorthQueryWorkspaceError>
    {
        let receipt = self.build_subscription_activation_receipt(view_name, activation);
        Ok(self.build_subscription_activation_boundary_receipt(view_name, activation, receipt))
    }
}

fn mutation_collection(command: &WorthQueryBackendAdmissibleMutation) -> String {
    command
        .declared_collection_identity()
        .map(|collection| collection.as_str().to_string())
        .unwrap_or_else(|| "Task".to_string())
}

fn mutation_entity_identity(
    command: &WorthQueryBackendAdmissibleMutation,
    ordinal: usize,
) -> WorthQueryEntityIdentity {
    command.declared_entity_identity().unwrap_or_else(|| {
        WorthQueryEntityIdentity::from_relational_record(
            RelationalBridgeRecordIdentityParts::entity(1, ordinal as u64, 0),
        )
    })
}

fn mutation_kind(command: &WorthQueryBackendAdmissibleMutation) -> WorthQueryMutationKind {
    match command.mutation_family() {
        worth_query::facade::WorthQueryMutationFamily::Insert => WorthQueryMutationKind::Created,
        worth_query::facade::WorthQueryMutationFamily::Delete => WorthQueryMutationKind::Deleted,
        _ => WorthQueryMutationKind::Updated,
    }
}
