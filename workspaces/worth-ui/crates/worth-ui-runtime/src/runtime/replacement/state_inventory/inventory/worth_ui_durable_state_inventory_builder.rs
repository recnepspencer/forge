use std::collections::BTreeSet;

use crate::runtime::{
    WorthUiDurableStateFamily, WorthUiDurableStateFamilyHook, WorthUiDurableStateFamilyId,
    WorthUiDurableStateInventory, WorthUiDurableStateInventoryCounters,
    WorthUiDurableStateInventoryDenial, WorthUiNodeReplacementPlan, WorthUiStateOwnershipClass,
    WorthUiTransientInteractionPolicy, WorthUiTransientInteractionState,
};

#[derive(Clone, Debug, Default)]
pub struct WorthUiDurableStateInventoryBuilder {
    platform_families: Vec<WorthUiDurableStateFamily>,
    hooks: Vec<WorthUiDurableStateFamilyHook>,
}

impl WorthUiDurableStateInventoryBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register_platform_family(mut self, family: WorthUiDurableStateFamily) -> Self {
        self.platform_families.push(family);
        self
    }

    pub fn register_family_hook(mut self, hook: WorthUiDurableStateFamilyHook) -> Self {
        self.hooks.push(hook);
        self
    }

    pub fn build_for_replacement(
        self,
        node_plan: &WorthUiNodeReplacementPlan,
    ) -> Result<WorthUiDurableStateInventory, WorthUiDurableStateInventoryDenial> {
        let mut counters = WorthUiDurableStateInventoryCounters::default();
        counters.record_replacement_classifications(node_plan.classifications().len());
        if !node_plan.is_unambiguous() {
            return Err(
                WorthUiDurableStateInventoryDenial::AmbiguousNodeReplacementPlan { counters },
            );
        }

        let mut families = Vec::new();
        let mut seen = BTreeSet::new();
        for family in self.platform_families {
            counters.record_platform_family();
            register_family(family, &mut families, &mut seen, &mut counters)?;
        }
        require_complete_platform_inventory(&seen, &mut counters)?;

        for hook in self.hooks {
            let family = family_from_hook(hook, &mut counters)?;
            counters.record_hook_family();
            register_family(family, &mut families, &mut seen, &mut counters)?;
        }

        let transient_policies = transient_drop_policies(&mut counters);
        Ok(WorthUiDurableStateInventory::new(
            node_plan.active_artifact_digest(),
            node_plan.candidate_artifact_digest(),
            families,
            transient_policies,
            counters,
        ))
    }
}

fn register_family(
    family: WorthUiDurableStateFamily,
    families: &mut Vec<WorthUiDurableStateFamily>,
    seen: &mut BTreeSet<WorthUiDurableStateFamilyId>,
    counters: &mut WorthUiDurableStateInventoryCounters,
) -> Result<(), WorthUiDurableStateInventoryDenial> {
    let family_id = family.id().clone();
    if !seen.insert(family_id.clone()) {
        counters.record_duplicate_family();
        return Err(WorthUiDurableStateInventoryDenial::DuplicateStateFamily {
            family_id,
            counters: *counters,
        });
    }
    if family.ownership_class() == WorthUiStateOwnershipClass::DomainTruth {
        counters.record_rejected_family();
        return Err(WorthUiDurableStateInventoryDenial::DomainTruthStateFamily {
            family_id,
            owner_identity: family.owner_identity().clone(),
            counters: *counters,
        });
    }
    families.push(family);
    Ok(())
}

fn family_from_hook(
    hook: WorthUiDurableStateFamilyHook,
    counters: &mut WorthUiDurableStateInventoryCounters,
) -> Result<WorthUiDurableStateFamily, WorthUiDurableStateInventoryDenial> {
    let family_id = hook.family_id().clone();
    if !matches!(family_id, WorthUiDurableStateFamilyId::Custom(_)) {
        counters.record_rejected_family();
        return Err(
            WorthUiDurableStateInventoryDenial::ReservedPlatformStateFamily {
                family_id,
                counters: *counters,
            },
        );
    }
    if !family_id.is_explicit_custom_family() {
        counters.record_rejected_family();
        return Err(
            WorthUiDurableStateInventoryDenial::InvalidCustomStateFamilyId {
                family_id,
                counters: *counters,
            },
        );
    }
    let Some(owner_identity) = hook.owner_identity() else {
        counters.record_rejected_family();
        return Err(WorthUiDurableStateInventoryDenial::MissingOwnerIdentity {
            family_id,
            counters: *counters,
        });
    };
    if !owner_identity.has_explicit_identity_basis() {
        counters.record_rejected_family();
        return Err(WorthUiDurableStateInventoryDenial::InvalidOwnerIdentity {
            family_id,
            owner_identity,
            counters: *counters,
        });
    }
    if owner_identity.ownership_class() == WorthUiStateOwnershipClass::PlatformShell {
        counters.record_rejected_family();
        return Err(
            WorthUiDurableStateInventoryDenial::ReservedPlatformOwnerIdentity {
                family_id,
                owner_identity,
                counters: *counters,
            },
        );
    }
    if owner_identity.ownership_class() == WorthUiStateOwnershipClass::DomainTruth {
        counters.record_rejected_family();
        return Err(WorthUiDurableStateInventoryDenial::DomainTruthStateFamily {
            family_id,
            owner_identity,
            counters: *counters,
        });
    }
    if hook.replacement_policy().is_none() {
        counters.record_rejected_family();
        return Err(
            WorthUiDurableStateInventoryDenial::MissingReplacementPolicy {
                family_id,
                counters: *counters,
            },
        );
    }
    if hook.persistence_posture().is_none() {
        counters.record_rejected_family();
        return Err(
            WorthUiDurableStateInventoryDenial::MissingPersistencePosture {
                family_id,
                counters: *counters,
            },
        );
    }
    Ok(WorthUiDurableStateFamily::from_validated_hook(hook))
}

fn require_complete_platform_inventory(
    seen: &BTreeSet<WorthUiDurableStateFamilyId>,
    counters: &mut WorthUiDurableStateInventoryCounters,
) -> Result<(), WorthUiDurableStateInventoryDenial> {
    for family_id in WorthUiDurableStateFamilyId::reserved_platform_families() {
        if !seen.contains(family_id) {
            counters.record_rejected_family();
            return Err(
                WorthUiDurableStateInventoryDenial::MissingPlatformStateFamily {
                    family_id: family_id.clone(),
                    counters: *counters,
                },
            );
        }
    }
    Ok(())
}

fn transient_drop_policies(
    counters: &mut WorthUiDurableStateInventoryCounters,
) -> Vec<(
    WorthUiTransientInteractionState,
    WorthUiTransientInteractionPolicy,
)> {
    WorthUiTransientInteractionState::all()
        .iter()
        .map(|state| {
            counters.record_transient_drop_policy();
            (*state, WorthUiTransientInteractionPolicy::Drop)
        })
        .collect()
}
