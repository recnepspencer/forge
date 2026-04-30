use super::*;
use crate::runtime::ForgeQueryAspectValue;

impl ForgeQueryMemoryApp {
    pub fn insert(
        &mut self,
        collection: &str,
        payload: Value,
    ) -> Result<ForgeQueryMutationReceipt, ForgeQueryWorkspaceError> {
        let collection_runtime = self.collection_mut(collection)?;
        collection_runtime.next_client_key += 1;
        let client_key = InternedString::Raw(format!(
            "{collection}:{}",
            collection_runtime.next_client_key
        ));
        let kind_id = collection_runtime.kind_id;
        let receipt = self.execute_query_writeback(
            collection,
            ForgeQueryMutationKind::Created,
            Vec::new(),
            &payload,
            ForgeQueryPendingOperation::Insert {
                kind_id,
                client_key,
                payload: payload.clone(),
            },
        )?;
        for entity in receipt.deltas.iter().map(|delta| &delta.entity_identity) {
            self.entity_collections
                .insert(entity.clone(), collection.to_string());
        }
        self.deliver_live_patches(&receipt);
        Ok(receipt)
    }

    pub fn insert_aspects(
        &mut self,
        collection: &str,
        aspects: Vec<ForgeQueryAspectValue>,
    ) -> Result<ForgeQueryMutationReceipt, ForgeQueryWorkspaceError> {
        let aspect_paths = aspects
            .iter()
            .map(|aspect| aspect.aspect_path().to_string())
            .collect::<Vec<_>>();
        let payload = crate::runtime::aspect_values_to_payload(&aspects)?;
        let collection_runtime = self.collection_mut(collection)?;
        collection_runtime.next_client_key += 1;
        let client_key = InternedString::Raw(format!(
            "{collection}:{}",
            collection_runtime.next_client_key
        ));
        let kind_id = collection_runtime.kind_id;
        let receipt = self.execute_query_writeback(
            collection,
            ForgeQueryMutationKind::Created,
            aspect_paths,
            &payload,
            ForgeQueryPendingOperation::Insert {
                kind_id,
                client_key,
                payload: payload.clone(),
            },
        )?;
        for entity in receipt.deltas.iter().map(|delta| &delta.entity_identity) {
            self.entity_collections
                .insert(entity.clone(), collection.to_string());
        }
        self.deliver_live_patches(&receipt);
        Ok(receipt)
    }

    pub fn update_aspect(
        &mut self,
        entity_identity: &str,
        aspect_path: &str,
        value: Value,
    ) -> Result<ForgeQueryMutationReceipt, ForgeQueryWorkspaceError> {
        let entity_id = super::helpers::parse_entity_identity(entity_identity)?;
        let current = self
            .latest_entity_payload(entity_id)?
            .ok_or_else(|| ForgeQueryWorkspaceError::new("entity not found"))?;
        let mut next = current;
        super::helpers::set_json_path(&mut next, aspect_path, value)?;
        let collection = self
            .entity_collections
            .get(entity_identity)
            .cloned()
            .ok_or_else(|| ForgeQueryWorkspaceError::new("entity collection not tracked"))?;
        let receipt = self.execute_query_writeback(
            &collection,
            ForgeQueryMutationKind::Updated,
            vec![aspect_path.to_string()],
            &next,
            ForgeQueryPendingOperation::Update {
                entity_id,
                payload: next.clone(),
                existing_truth_binding: None,
            },
        )?;
        self.deliver_live_patches(&receipt);
        Ok(receipt)
    }

    pub fn update_aspects(
        &mut self,
        entity_identity: &str,
        aspects: Vec<ForgeQueryAspectValue>,
    ) -> Result<ForgeQueryMutationReceipt, ForgeQueryWorkspaceError> {
        let entity_id = super::helpers::parse_entity_identity(entity_identity)?;
        let current = self
            .latest_entity_payload(entity_id)?
            .ok_or_else(|| ForgeQueryWorkspaceError::new("entity not found"))?;
        let mut next = current;
        let mut aspect_paths = Vec::with_capacity(aspects.len());
        for aspect in aspects {
            aspect_paths.push(aspect.aspect_path().to_string());
            super::helpers::set_json_path(&mut next, aspect.aspect_path(), aspect.value().clone())?;
        }
        let collection = self
            .entity_collections
            .get(entity_identity)
            .cloned()
            .ok_or_else(|| ForgeQueryWorkspaceError::new("entity collection not tracked"))?;
        let receipt = self.execute_query_writeback(
            &collection,
            ForgeQueryMutationKind::Updated,
            aspect_paths,
            &next,
            ForgeQueryPendingOperation::Update {
                entity_id,
                payload: next.clone(),
                existing_truth_binding: None,
            },
        )?;
        self.deliver_live_patches(&receipt);
        Ok(receipt)
    }

    pub fn update_aspects_existing(
        &mut self,
        binding: crate::runtime::ForgeQueryExistingTruthTargetBinding,
        aspects: Vec<ForgeQueryAspectValue>,
    ) -> Result<ForgeQueryMutationReceipt, ForgeQueryWorkspaceError> {
        let entity_identity = binding.resolved_entity_identity().to_string();
        let entity_id = super::helpers::parse_entity_identity(&entity_identity)?;
        let current = self
            .latest_entity_payload(entity_id)?
            .ok_or_else(|| ForgeQueryWorkspaceError::new("entity not found"))?;
        let mut next = current;
        let mut aspect_paths = Vec::with_capacity(aspects.len());
        for aspect in aspects {
            aspect_paths.push(aspect.aspect_path().to_string());
            super::helpers::set_json_path(&mut next, aspect.aspect_path(), aspect.value().clone())?;
        }
        let collection = self
            .entity_collections
            .get(&entity_identity)
            .cloned()
            .ok_or_else(|| ForgeQueryWorkspaceError::new("entity collection not tracked"))?;
        let receipt = self.execute_query_writeback(
            &collection,
            ForgeQueryMutationKind::Updated,
            aspect_paths,
            &next,
            ForgeQueryPendingOperation::Update {
                entity_id,
                payload: next.clone(),
                existing_truth_binding: Some(binding),
            },
        )?;
        self.deliver_live_patches(&receipt);
        Ok(receipt)
    }

    pub fn verify_existing_assertion(
        &self,
        binding: &crate::runtime::ForgeQueryExistingTruthTargetBinding,
        aspects: &[ForgeQueryAspectValue],
    ) -> Result<crate::runtime::ForgeQueryVerifiedExistingTruthAssertion, ForgeQueryWorkspaceError>
    {
        let entity_identity = binding.resolved_entity_identity().to_string();
        let entity_id = super::helpers::parse_entity_identity(&entity_identity)?;
        let current = self
            .latest_entity_payload(entity_id)?
            .ok_or_else(|| ForgeQueryWorkspaceError::new("entity not found"))?;
        for aspect in aspects {
            if aspect.clears_existing_value() {
                return Err(ForgeQueryWorkspaceError::new(format!(
                    "backend-verified existing-truth assertion does not admit clear assertion for `{}`",
                    aspect.aspect_path()
                )));
            }
            let Some(found) = super::helpers::get_json_path(&current, aspect.aspect_path()) else {
                return Err(ForgeQueryWorkspaceError::new(format!(
                    "asserted aspect `{}` is not present on the current authoritative payload",
                    aspect.aspect_path()
                )));
            };
            if found != aspect.value() {
                return Err(ForgeQueryWorkspaceError::new(format!(
                    "asserted aspect `{}` had authoritative value `{}`, expected `{}`",
                    aspect.aspect_path(),
                    serde_json::to_string(found).unwrap_or_else(|_| found.to_string()),
                    serde_json::to_string(aspect.value())
                        .unwrap_or_else(|_| aspect.value().to_string())
                )));
            }
        }
        crate::runtime::ForgeQueryVerifiedExistingTruthAssertion::new(binding, aspects)
    }

    pub fn probe_existing_truth(
        &self,
        request: &crate::runtime::ForgeQueryExistingTruthProbeRequest,
    ) -> Result<crate::runtime::ForgeQueryExistingTruthProbe, ForgeQueryWorkspaceError> {
        let entity_identity = request.binding().resolved_entity_identity().to_string();
        let entity_id = super::helpers::parse_entity_identity(&entity_identity)?;
        let current = self
            .latest_entity_payload(entity_id)?
            .ok_or_else(|| ForgeQueryWorkspaceError::new("entity not found"))?;
        let mut fields = Vec::with_capacity(request.aspect_paths().len());
        for aspect_path in request.aspect_paths() {
            let value = super::helpers::get_json_path(&current, aspect_path)
                .cloned()
                .ok_or_else(|| {
                    ForgeQueryWorkspaceError::new(format!(
                        "probed aspect `{aspect_path}` is not present on the current authoritative payload"
                    ))
                })?;
            fields.push((aspect_path.clone(), value));
        }
        Ok(crate::runtime::ForgeQueryExistingTruthProbe::backend_verified(request, fields))
    }

    pub fn delete(
        &mut self,
        entity_identity: &str,
    ) -> Result<ForgeQueryMutationReceipt, ForgeQueryWorkspaceError> {
        self.delete_with(entity_identity, Vec::new())
    }

    pub fn delete_with(
        &mut self,
        entity_identity: &str,
        touched_aspect_paths: Vec<String>,
    ) -> Result<ForgeQueryMutationReceipt, ForgeQueryWorkspaceError> {
        let entity_id = super::helpers::parse_entity_identity(entity_identity)?;
        let collection = self
            .entity_collections
            .get(entity_identity)
            .cloned()
            .ok_or_else(|| ForgeQueryWorkspaceError::new("entity collection not tracked"))?;
        let mut receipt = self.execute_query_writeback(
            &collection,
            ForgeQueryMutationKind::Deleted,
            touched_aspect_paths.clone(),
            &Value::String(entity_identity.to_string()),
            ForgeQueryPendingOperation::Delete {
                entity_id,
                existing_truth_binding: None,
            },
        )?;
        if receipt.deltas.is_empty() {
            receipt.deltas.push(ForgeQueryMutationDelta {
                collection: collection.clone(),
                entity_identity: entity_identity.to_string(),
                kind: ForgeQueryMutationKind::Deleted,
                aspect_paths: touched_aspect_paths,
            });
        }
        self.entity_collections.remove(entity_identity);
        self.deliver_live_patches(&receipt);
        Ok(receipt)
    }

    pub fn delete_existing_with(
        &mut self,
        binding: crate::runtime::ForgeQueryExistingTruthTargetBinding,
        touched_aspect_paths: Vec<String>,
    ) -> Result<ForgeQueryMutationReceipt, ForgeQueryWorkspaceError> {
        let entity_identity = binding.resolved_entity_identity().to_string();
        let entity_id = super::helpers::parse_entity_identity(&entity_identity)?;
        let collection = self
            .entity_collections
            .get(&entity_identity)
            .cloned()
            .ok_or_else(|| ForgeQueryWorkspaceError::new("entity collection not tracked"))?;
        let mut receipt = self.execute_query_writeback(
            &collection,
            ForgeQueryMutationKind::Deleted,
            touched_aspect_paths.clone(),
            &Value::String(entity_identity.clone()),
            ForgeQueryPendingOperation::Delete {
                entity_id,
                existing_truth_binding: Some(binding),
            },
        )?;
        if receipt.deltas.is_empty() {
            receipt.deltas.push(ForgeQueryMutationDelta {
                collection: collection.clone(),
                entity_identity: entity_identity.clone(),
                kind: ForgeQueryMutationKind::Deleted,
                aspect_paths: touched_aspect_paths,
            });
        }
        self.entity_collections.remove(&entity_identity);
        self.deliver_live_patches(&receipt);
        Ok(receipt)
    }
}
