use super::super::*;

pub(super) fn freeze_resource_policy_registry_evidence() -> ResourcePolicyRegistryFreezeReport {
    FrozenResourcePolicyRegistry::built_in()
        .freeze_report()
        .clone()
}
