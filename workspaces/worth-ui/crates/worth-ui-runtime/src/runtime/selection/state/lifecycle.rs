impl super::UiSelectionRuntimeState {
    pub(crate) fn has_projection_catalog_owners(&self) -> bool {
        !self.family_owners.is_empty()
    }

    pub(crate) fn catalog_is_current(
        &self,
        owner: crate::runtime::selection::UiSelectionOwnerIdentity,
        incarnation: crate::runtime::selection::UiSelectionOwnerIncarnation,
        catalog_revision: u64,
    ) -> bool {
        self.owners.get(&owner).is_some_and(|record| {
            record.incarnation == incarnation
                && record.catalog_available
                && record.catalog_revision == catalog_revision
        })
    }

    pub(crate) fn projection_families(&self) -> Box<[crate::runtime::UiApplicationItemKeyFamily]> {
        self.family_owners
            .keys()
            .copied()
            .filter(|family| family.projection_input_slot().is_some())
            .collect()
    }

    pub(crate) fn family_requires_catalog_reconciliation(
        &self,
        family: crate::runtime::UiApplicationItemKeyFamily,
        catalog_revision: u64,
    ) -> bool {
        self.family_owners.get(&family).is_some_and(|owners| {
            owners.iter().any(|owner| {
                self.owners.get(owner).is_some_and(|record| {
                    !record.catalog_available || record.catalog_revision != catalog_revision
                })
            })
        })
    }

    pub(crate) fn reconcile_projection_catalog(
        &mut self,
        family: crate::runtime::UiApplicationItemKeyFamily,
        catalog_revision: u64,
        application_keys: &[core::num::NonZeroU64],
        posture: crate::runtime::selection::UiSelectionCatalogPosture,
    ) -> Result<usize, crate::runtime::selection::UiSelectionRequestDenial> {
        let owners = self.family_owners.get(&family).cloned().unwrap_or_default();
        let catalog = application_keys
            .iter()
            .copied()
            .map(|value| {
                crate::runtime::selection::UiSelectionStableKey::new(
                    crate::runtime::UiApplicationItemKey::from_projection_mapping(family, value),
                )
            })
            .collect::<Vec<_>>();
        let mut reconciled = 0;
        for owner in owners {
            let Some(record) = self.owners.get(&owner) else {
                continue;
            };
            if record.catalog_available && record.catalog_revision == catalog_revision {
                continue;
            }
            let registration = crate::runtime::selection::UiSelectionRegistration::new(
                owner,
                record.incarnation,
                record.policy,
                catalog.clone(),
                posture,
            )?
            .with_catalog_revision(catalog_revision);
            self.synchronize(registration)?;
            reconciled += 1;
        }
        Ok(reconciled)
    }

    pub(crate) fn suspend_projection_catalogs(&mut self) -> usize {
        let families = self.projection_families();
        let mut suspended = 0;
        for family in families.iter().copied() {
            let owners = self.family_owners.get(&family).cloned().unwrap_or_default();
            for owner in owners {
                let Some(record) = self.owners.get_mut(&owner) else {
                    continue;
                };
                record.catalog_available = false;
                record.catalog = std::sync::Arc::from([]);
                record.catalog_positions = std::sync::Arc::new(std::collections::BTreeMap::new());
                suspended += 1;
            }
        }
        suspended
    }

    pub(crate) fn retire_mounted_owner(
        &mut self,
        surface: worth_ui_host_contract::UiSemanticSurfaceIdentity,
        graph_node: crate::graph::UiGraphNodeIdentity,
        incarnation: crate::runtime::selection::UiSelectionOwnerIncarnation,
    ) -> usize {
        let owners = self
            .mounted_owners
            .remove(&(surface, graph_node, incarnation))
            .unwrap_or_default();
        let mut released = 0;
        for owner in owners {
            if self
                .owners
                .get(&owner)
                .is_some_and(|record| record.incarnation == incarnation)
            {
                self.owners.remove(&owner);
                let family = owner.key_family();
                let remove_family = if let Some(family_owners) = self.family_owners.get_mut(&family)
                {
                    family_owners.remove(&owner);
                    family_owners.is_empty()
                } else {
                    false
                };
                if remove_family {
                    self.family_owners.remove(&family);
                }
                released += 1;
            }
        }
        released
    }

    pub(crate) fn retire_family(
        &mut self,
        family: crate::runtime::UiApplicationItemKeyFamily,
    ) -> usize {
        let owners = self.family_owners.remove(&family).unwrap_or_default();
        let mut released = 0;
        for owner in owners {
            let Some(record) = self.owners.remove(&owner) else {
                continue;
            };
            let mounted_key = (
                owner.semantic_surface(),
                owner.graph_node(),
                record.incarnation,
            );
            let remove_mounted = if let Some(mounted) = self.mounted_owners.get_mut(&mounted_key) {
                mounted.remove(&owner);
                mounted.is_empty()
            } else {
                false
            };
            if remove_mounted {
                self.mounted_owners.remove(&mounted_key);
            }
            released += 1;
        }
        released
    }
}
