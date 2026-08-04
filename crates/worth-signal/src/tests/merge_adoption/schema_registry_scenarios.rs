use crate::facade::{
    AspectMergePolicyBinding, AspectMergePolicyName, ConflictIsolationPolicyName,
    ConflictPolicyName, DeletionPolicyName, IdentityMatcherName, MergeStrategyName, NodeContract,
    SourceOnlyPolicyName,
};
use crate::schema::data::{
    SignalSchemaDescriptor, SignalSchemaId, SignalSchemaName, SignalSchemaRegistration,
    SignalSchemaRegistry, SignalSchemaVersion,
};
use crate::tests::support::ASPECT_A;

pub(super) fn merge_schema_registry(
    default_strategy_name: &str,
    default_conflict_policy_name: Option<&str>,
    default_identity_matcher_name: Option<&str>,
) -> SignalSchemaRegistry {
    SignalSchemaRegistry::from_registrations(vec![SignalSchemaRegistration::new(
        SignalSchemaDescriptor::new_with_merge_semantics(
            SignalSchemaId(41),
            SignalSchemaName::new("signal.demo.merge-owned"),
            SignalSchemaVersion::new(1, 0),
            NodeContract::wildcard(),
            Some(MergeStrategyName::new(default_strategy_name)),
            default_conflict_policy_name.map(ConflictPolicyName::new),
            default_identity_matcher_name.map(IdentityMatcherName::new),
            None,
            None,
        ),
    )
    .expect("valid schema registration")])
    .expect("valid schema registry")
}

pub(super) fn cross_identity_merge_schema_registry(
    default_identity_matcher_name: Option<&str>,
) -> SignalSchemaRegistry {
    SignalSchemaRegistry::from_registrations(vec![SignalSchemaRegistration::new(
        SignalSchemaDescriptor::new_with_merge_semantics(
            SignalSchemaId(42),
            SignalSchemaName::new("signal.demo.cross-identity-merge-owned"),
            SignalSchemaVersion::new(1, 0),
            NodeContract::wildcard().with_cross_identity_persistent_matching(),
            Some(MergeStrategyName::new(
                "signal.merge.rebase-source-onto-target",
            )),
            None,
            default_identity_matcher_name.map(IdentityMatcherName::new),
            None,
            None,
        ),
    )
    .expect("valid schema registration")])
    .expect("valid schema registry")
}

pub(super) fn source_only_merge_schema_registry(
    default_source_only_policy_name: Option<&str>,
) -> SignalSchemaRegistry {
    SignalSchemaRegistry::from_registrations(vec![SignalSchemaRegistration::new(
        SignalSchemaDescriptor::new_with_merge_semantics(
            SignalSchemaId(43),
            SignalSchemaName::new("signal.demo.source-only-merge-owned"),
            SignalSchemaVersion::new(1, 0),
            NodeContract::wildcard(),
            Some(MergeStrategyName::new(
                "signal.merge.rebase-source-onto-target",
            )),
            None,
            None,
            default_source_only_policy_name.map(SourceOnlyPolicyName::new),
            None,
        ),
    )
    .expect("valid schema registration")])
    .expect("valid schema registry")
}

pub(super) fn deletion_merge_schema_registry(
    default_deletion_policy_name: Option<&str>,
) -> SignalSchemaRegistry {
    SignalSchemaRegistry::from_registrations(vec![SignalSchemaRegistration::new(
        SignalSchemaDescriptor::new_with_merge_semantics(
            SignalSchemaId(44),
            SignalSchemaName::new("signal.demo.deletion-merge-owned"),
            SignalSchemaVersion::new(1, 0),
            NodeContract::wildcard(),
            Some(MergeStrategyName::new(
                "signal.merge.rebase-source-onto-target",
            )),
            None,
            None,
            None,
            default_deletion_policy_name.map(DeletionPolicyName::new),
        ),
    )
    .expect("valid schema registration")])
    .expect("valid schema registry")
}

pub(super) fn aspect_policy_merge_schema_registry(
    default_aspect_policy_name: Option<&str>,
) -> SignalSchemaRegistry {
    SignalSchemaRegistry::from_registrations(vec![SignalSchemaRegistration::new(
        SignalSchemaDescriptor::new_with_merge_semantics_and_aspects(
            SignalSchemaId(44),
            SignalSchemaName::new("signal.demo.aspect-merge-owned"),
            SignalSchemaVersion::new(1, 0),
            NodeContract::wildcard(),
            Some(MergeStrategyName::new(
                "signal.merge.rebase-source-onto-target",
            )),
            None,
            None,
            None,
            None,
            default_aspect_policy_name
                .map(|name| {
                    vec![AspectMergePolicyBinding::new(
                        ASPECT_A,
                        AspectMergePolicyName::new(name),
                    )]
                })
                .unwrap_or_default(),
        ),
    )
    .expect("valid schema registration")])
    .expect("valid schema registry")
}

pub(super) fn conflict_isolation_merge_schema_registry(
    default_conflict_isolation_policy_name: Option<&str>,
) -> SignalSchemaRegistry {
    SignalSchemaRegistry::from_registrations(vec![SignalSchemaRegistration::new(
        SignalSchemaDescriptor::new_with_merge_semantics_and_isolation(
            SignalSchemaId(45),
            SignalSchemaName::new("signal.demo.conflict-isolation-owned"),
            SignalSchemaVersion::new(1, 0),
            NodeContract::wildcard(),
            Some(MergeStrategyName::new(
                "signal.merge.rebase-source-onto-target",
            )),
            Some(ConflictPolicyName::new(
                "signal.conflict.resolve-source-when-structure-matches",
            )),
            None,
            None,
            None,
            default_conflict_isolation_policy_name.map(ConflictIsolationPolicyName::new),
        ),
    )
    .expect("valid schema registration")])
    .expect("valid schema registry")
}
