use std::collections::{BTreeMap, BTreeSet};
use std::rc::Rc;

use super::record::WorthUiPlanRegionRecord;
use super::{
    WorthUiPlanRegionExecutable, WorthUiPlanRegionHandle, WorthUiPlanRegionIdentity,
    WorthUiPlanRegionSchema, WorthUiPlanRegionStorageCounters, WorthUiPlanRegionStore,
    WorthUiPlanRegionStoreDenial, WorthUiPlanRegionTransition, WorthUiPlanRegionTransitionEvidence,
};

impl WorthUiPlanRegionStore {
    pub(super) fn apply_schema_batch(
        &mut self,
        schemas: Vec<WorthUiPlanRegionSchema>,
        evidence: &mut Vec<WorthUiPlanRegionTransitionEvidence>,
        counters: &mut WorthUiPlanRegionStorageCounters,
    ) -> Result<(), WorthUiPlanRegionStoreDenial> {
        let mut pending = BTreeMap::new();
        for schema in schemas {
            if pending.insert(schema.identity().clone(), schema).is_some() {
                return Err(WorthUiPlanRegionStoreDenial::DuplicateRegionIdentity);
            }
        }
        for schema in pending.values() {
            for dependency in schema.input().dependency_identity_bases() {
                let identity = WorthUiPlanRegionIdentity::from_exact_basis(dependency);
                if !pending.contains_key(&identity) && self.handle_for(&identity).is_none() {
                    return Err(WorthUiPlanRegionStoreDenial::MissingLinkedRegion);
                }
            }
        }
        let mut final_handles =
            BTreeMap::<WorthUiPlanRegionIdentity, WorthUiPlanRegionHandle>::new();
        let mut sealed = Vec::with_capacity(pending.len());

        while !pending.is_empty() {
            let ready = pending
                .iter()
                .find(|(_, schema)| {
                    schema
                        .input()
                        .dependency_identity_bases()
                        .iter()
                        .all(|identity| {
                            let identity = WorthUiPlanRegionIdentity::from_exact_basis(*identity);
                            final_handles.contains_key(&identity)
                                || (!pending.contains_key(&identity)
                                    && self.handle_for(&identity).is_some())
                        })
                })
                .map(|(identity, _)| identity.clone())
                .ok_or(WorthUiPlanRegionStoreDenial::CyclicRegionDependency)?;
            let schema = pending
                .remove(&ready)
                .expect("ready regional schema remains pending");
            let executable = WorthUiPlanRegionExecutable::lower(schema.input(), |identity| {
                let identity = WorthUiPlanRegionIdentity::from_exact_basis(identity);
                final_handles
                    .get(&identity)
                    .cloned()
                    .or_else(|| self.handle_for(&identity).cloned())
            })?;
            let existing = self.record_for_identity(schema.identity()).cloned();
            if existing.is_some() {
                counters.record_exact_comparison();
            }
            let (handle, transition) = match existing {
                Some(record)
                    if record.schema.exactly_matches(&schema)
                        && record.executable == executable =>
                {
                    counters.record_reuse();
                    (record.handle.clone(), WorthUiPlanRegionTransition::Reused)
                }
                Some(record) => {
                    counters.record_retirement();
                    (
                        record.handle.replacement_successor()?,
                        WorthUiPlanRegionTransition::Replaced,
                    )
                }
                None => {
                    let handle = WorthUiPlanRegionHandle::initial(
                        schema.identity().clone(),
                        self.next_stable_slot_value(),
                    );
                    self.advance_stable_slot()?;
                    self.increment_region_count();
                    (handle, WorthUiPlanRegionTransition::Inserted)
                }
            };
            final_handles.insert(schema.identity().clone(), handle.clone());
            sealed.push((schema, handle, executable, transition));
        }

        for (schema, handle, executable, transition) in sealed {
            let identity = schema.identity().clone();
            if transition != WorthUiPlanRegionTransition::Reused {
                counters.record_region_construction();
                self.insert_sealed_record(
                    Rc::new(WorthUiPlanRegionRecord::new(schema, handle, executable)),
                    counters,
                );
            }
            evidence.push(WorthUiPlanRegionTransitionEvidence::new(
                identity, transition,
            ));
        }
        Ok(())
    }

    pub(super) fn reconcile_owner_bundle(
        &mut self,
        root_identity: WorthUiPlanRegionIdentity,
        schemas: Vec<WorthUiPlanRegionSchema>,
        evidence: &mut Vec<WorthUiPlanRegionTransitionEvidence>,
        counters: &mut WorthUiPlanRegionStorageCounters,
    ) -> Result<(), WorthUiPlanRegionStoreDenial> {
        validate_owner_manifest(&root_identity, &schemas)?;
        let old_members = self
            .record_for_identity(&root_identity)
            .map(|record| record.executable.owned_region_identities().to_vec())
            .unwrap_or_default();
        let successor = schemas
            .iter()
            .map(|schema| schema.identity().clone())
            .collect::<BTreeSet<_>>();
        self.apply_schema_batch(schemas, evidence, counters)?;
        for identity in old_members {
            if !successor.contains(&identity) {
                self.retire(identity, evidence, counters);
            }
        }
        Ok(())
    }

    pub(super) fn retire_owner_bundle(
        &mut self,
        root_identity: WorthUiPlanRegionIdentity,
        evidence: &mut Vec<WorthUiPlanRegionTransitionEvidence>,
        counters: &mut WorthUiPlanRegionStorageCounters,
    ) {
        let members = self
            .record_for_identity(&root_identity)
            .map(|record| record.executable.owned_region_identities().to_vec())
            .unwrap_or_default();
        for identity in members {
            self.retire(identity, evidence, counters);
        }
        self.retire(root_identity, evidence, counters);
    }
}

fn validate_owner_manifest(
    root_identity: &WorthUiPlanRegionIdentity,
    schemas: &[WorthUiPlanRegionSchema],
) -> Result<(), WorthUiPlanRegionStoreDenial> {
    let references = schemas.iter().collect::<Vec<_>>();
    validate_owner_manifest_references(root_identity, &references)
}

pub(super) fn validate_launch_owner_bundles(
    schemas: &[WorthUiPlanRegionSchema],
) -> Result<(), WorthUiPlanRegionStoreDenial> {
    let mut identities = BTreeMap::new();
    let mut owner_roots = BTreeSet::new();
    for schema in schemas {
        if identities
            .insert(schema.identity().exact_basis(), schema)
            .is_some()
        {
            return Err(WorthUiPlanRegionStoreDenial::DuplicateRegionIdentity);
        }
        if let Some(owner) = schema.input().owner_identity_basis() {
            owner_roots.insert(owner);
        }
        if !schema.input().owned_region_identity_bases().is_empty() {
            owner_roots.insert(schema.identity().exact_basis());
        }
    }
    for owner in owner_roots {
        let root_identity = WorthUiPlanRegionIdentity::from_exact_basis(owner);
        let root = identities
            .get(owner)
            .copied()
            .ok_or(WorthUiPlanRegionStoreDenial::OwnerManifestMismatch)?;
        let mut bundle = vec![root];
        bundle.extend(
            schemas
                .iter()
                .filter(|schema| schema.input().owner_identity_basis() == Some(owner)),
        );
        validate_owner_manifest_references(&root_identity, &bundle)?;
    }
    Ok(())
}

fn validate_owner_manifest_references(
    root_identity: &WorthUiPlanRegionIdentity,
    schemas: &[&WorthUiPlanRegionSchema],
) -> Result<(), WorthUiPlanRegionStoreDenial> {
    let root = schemas
        .iter()
        .copied()
        .find(|schema| schema.identity() == root_identity)
        .ok_or(WorthUiPlanRegionStoreDenial::OwnerManifestMismatch)?;
    if root.input().owner_identity_basis().is_some() {
        return Err(WorthUiPlanRegionStoreDenial::OwnerManifestMismatch);
    }
    let bundle_identities = schemas
        .iter()
        .map(|schema| schema.identity().clone())
        .collect::<BTreeSet<_>>();
    let expected = schemas
        .iter()
        .filter(|schema| schema.identity() != root_identity)
        .map(|schema| schema.identity().clone())
        .collect::<BTreeSet<_>>();
    if schemas
        .iter()
        .filter(|schema| schema.identity() != root_identity)
        .any(|schema| schema.input().owner_identity_basis() != Some(root_identity.exact_basis()))
    {
        return Err(WorthUiPlanRegionStoreDenial::OwnerManifestMismatch);
    }
    if schemas.iter().any(|schema| {
        schema
            .input()
            .dependency_identity_bases()
            .into_iter()
            .map(WorthUiPlanRegionIdentity::from_exact_basis)
            .any(|dependency| !bundle_identities.contains(&dependency))
    }) {
        return Err(WorthUiPlanRegionStoreDenial::OwnerManifestMismatch);
    }
    let mut incoming = BTreeMap::<WorthUiPlanRegionIdentity, usize>::new();
    for dependency in schemas
        .iter()
        .flat_map(|schema| schema.input().dependency_identity_bases())
        .map(WorthUiPlanRegionIdentity::from_exact_basis)
        .filter(|dependency| dependency != root_identity)
    {
        let count = incoming.entry(dependency).or_default();
        *count += 1;
        if *count > 1 {
            return Err(WorthUiPlanRegionStoreDenial::OverlappingChildTarget);
        }
    }
    if expected
        .iter()
        .any(|identity| incoming.get(identity).copied() != Some(1))
    {
        return Err(WorthUiPlanRegionStoreDenial::OwnerManifestMismatch);
    }
    let declared = root
        .input()
        .owned_region_identity_bases()
        .iter()
        .map(WorthUiPlanRegionIdentity::from_exact_basis)
        .collect::<BTreeSet<_>>();
    if expected != declared {
        return Err(WorthUiPlanRegionStoreDenial::OwnerManifestMismatch);
    }
    Ok(())
}
