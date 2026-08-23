use crate::basis_lifecycle::BasisOperationLane;
use crate::domain_installation::WorthQuerySharedLiveProjectionLease;

/// Query-owned authority for publishing one shared maintenance result to one
/// current consumer.
///
/// Sharing owns computation only. This product keeps the consumer's purpose,
/// disclosure/basis, tenant/domain, branch, continuation, resource, lease, and
/// lifecycle axes separate and is reconstructed from the current lease owner
/// immediately before publication.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQuerySharedConsumerDeliveryAuthority {
    consumer_identity: String,
    purpose_identity: String,
    disclosure_identity: String,
    tenant_identity: String,
    branch_identity: String,
    continuation_identity: String,
    backpressure_posture: String,
    backpressure_policy: crate::subscription::DeliveryBackpressurePolicy,
    policy_generation: u64,
    authority_identity: String,
}

impl WorthQuerySharedConsumerDeliveryAuthority {
    pub fn consumer_identity(&self) -> &str {
        &self.consumer_identity
    }

    pub fn purpose_identity(&self) -> &str {
        &self.purpose_identity
    }

    pub fn disclosure_identity(&self) -> &str {
        &self.disclosure_identity
    }

    pub fn tenant_identity(&self) -> &str {
        &self.tenant_identity
    }

    pub fn branch_identity(&self) -> &str {
        &self.branch_identity
    }

    pub fn continuation_identity(&self) -> &str {
        &self.continuation_identity
    }

    pub fn backpressure_posture(&self) -> &str {
        &self.backpressure_posture
    }

    pub(crate) const fn backpressure_policy(
        &self,
    ) -> crate::subscription::DeliveryBackpressurePolicy {
        self.backpressure_policy
    }

    pub const fn policy_generation(&self) -> u64 {
        self.policy_generation
    }

    pub fn authority_identity(&self) -> &str {
        &self.authority_identity
    }
}

pub(super) fn current_shared_consumer_delivery_authority<D, O, F, L: BasisOperationLane>(
    consumer: &WorthQuerySharedLiveProjectionLease<D, O, F, L>,
    workspace: &crate::runtime::WorthQueryWorkspace,
) -> Option<WorthQuerySharedConsumerDeliveryAuthority> {
    let readmission = consumer.readmission();
    let admitted = workspace
        .current_shared_consumer_delivery_policy(consumer.workspace_capability(), readmission)?;
    let affinity = &readmission.closure.affinity;
    let consumer_identity = crate::identity::hash_parts(&[
        "worth_query_shared_consumer_v1".into(),
        format!("source:{}", readmission.source_identity),
        format!("owner:{}", readmission.owner.slot()),
        format!("owner-generation:{}", readmission.owner.generation()),
        format!("lease:{}", readmission.lease.slot()),
        format!("lease-generation:{}", readmission.lease.generation()),
    ]);
    let purpose_identity = admitted.policy.purpose().to_owned();
    let disclosure_identity = admitted.policy.disclosure().to_owned();
    let tenant_identity = affinity.domain_authority_identity.clone();
    let branch_identity = crate::identity::hash_parts(
        &std::iter::once("worth_query_shared_consumer_branch_v1".to_owned())
            .chain(affinity.graph_authority_identities.iter().cloned())
            .collect::<Vec<_>>(),
    );
    let continuation_identity = admitted.policy.continuation().to_owned();
    let backpressure_policy = admitted.policy.backpressure();
    let backpressure_posture = backpressure_policy.as_str().to_owned();
    let authority_identity = crate::identity::hash_parts(&[
        "worth_query_shared_consumer_delivery_authority_v1".into(),
        format!("consumer:{consumer_identity}"),
        format!("purpose:{purpose_identity}"),
        format!("disclosure:{disclosure_identity}"),
        format!("tenant:{tenant_identity}"),
        format!("branch:{branch_identity}"),
        format!("continuation:{continuation_identity}"),
        format!("backpressure:{backpressure_posture}"),
        format!("policy-generation:{}", admitted.generation),
        format!("binding:{}", readmission.binding_identity),
        format!("capability:{}", readmission.capability_identity),
        format!(
            "capability-generation:{}",
            readmission.capability_generation.ordinal()
        ),
    ]);
    Some(WorthQuerySharedConsumerDeliveryAuthority {
        consumer_identity,
        purpose_identity,
        disclosure_identity,
        tenant_identity,
        branch_identity,
        continuation_identity,
        backpressure_posture,
        backpressure_policy,
        policy_generation: admitted.generation,
        authority_identity,
    })
}
