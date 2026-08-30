use super::*;

impl WorthUiPresentationAsyncRegistry {
    pub(super) fn semantic_installations(
        &mut self,
        specifications: &[PresentationSemanticInstanceSpecification],
    ) -> Result<
        (
            Vec<([RelationalBridgeRecordIdentityParts; DEPENDENCY_COUNT], u64)>,
            Vec<PresentationSemanticPartition>,
        ),
        WorthUiPresentationRuntimeAdmissionDenial,
    > {
        let mut installations = Vec::with_capacity(specifications.len());
        let mut all_new = Vec::new();
        for specification in specifications {
            let (records, new) = self.semantic_records(&specification.partitions)?;
            self.next_semantic_version =
                self.next_semantic_version.checked_add(1).ok_or_else(|| {
                    WorthUiPresentationRuntimeAdmissionDenial::SemanticInstallation(Box::new(
                        duplicate_installation_denial(),
                    ))
                })?;
            installations.push((records, self.next_semantic_version));
            all_new.extend(new);
        }
        Ok((installations, all_new))
    }

    fn semantic_records(
        &mut self,
        keys: &[PresentationSemanticPartition; DEPENDENCY_COUNT],
    ) -> Result<
        (
            [RelationalBridgeRecordIdentityParts; DEPENDENCY_COUNT],
            Vec<PresentationSemanticPartition>,
        ),
        WorthUiPresentationRuntimeAdmissionDenial,
    > {
        let mut records = Vec::with_capacity(DEPENDENCY_COUNT);
        let mut new_partitions = Vec::new();
        for (ordinal, key) in keys.iter().cloned().enumerate() {
            let record = if let Some(record) = self.partitions.get(&key) {
                *record
            } else {
                self.next_partition_identity =
                    self.next_partition_identity.checked_add(1).ok_or_else(|| {
                        WorthUiPresentationRuntimeAdmissionDenial::SemanticInstallation(Box::new(
                            duplicate_installation_denial(),
                        ))
                    })?;
                let record = RelationalBridgeRecordIdentityParts::entity(
                    u32::try_from(ordinal + 1).expect("dependency ordinal fits u32"),
                    self.next_partition_identity,
                    1,
                );
                self.partitions.insert(key.clone(), record);
                new_partitions.push(key);
                record
            };
            records.push(record);
        }
        Ok((
            records
                .try_into()
                .expect("fixed presentation dependency width"),
            new_partitions,
        ))
    }

    pub(super) fn register_instance(
        &mut self,
        specification: &PresentationSemanticInstanceSpecification,
        query: runtime::WorthQueryInstalledOwnedConditionalInstance,
    ) -> u64 {
        let registration = self.instances.issue_registration();
        for partition in &specification.partitions {
            *self
                .partition_references
                .entry(partition.clone())
                .or_default() += 1;
            self.instances.register(
                partition.clone(),
                registration,
                RegisteredSemanticInstance {
                    subscriber: specification.subscriber,
                    query: query.clone(),
                },
            );
        }
        registration
    }

    pub(super) fn unregister_instance(&mut self, registration: AdmissionSemanticRegistration) {
        for partition in registration.partitions {
            self.instances
                .unregister(&partition, registration.registration);
            let remove = match self.partition_references.get_mut(&partition) {
                Some(references) if *references > 1 => {
                    *references -= 1;
                    false
                }
                Some(_) => true,
                None => false,
            };
            if remove {
                self.partition_references.remove(&partition);
                self.partitions.remove(&partition);
                self.execution_attempts.remove(&partition);
            }
        }
    }
}

fn duplicate_installation_denial() -> runtime::WorthQueryOwnedConditionalInstanceDenial {
    runtime::WorthQueryOwnedConditionalInstanceDenial::Installation(
        worth_query::facade::domain::WorthQueryConditionalNodeInstallationDenial::DuplicateInstallation,
    )
}
