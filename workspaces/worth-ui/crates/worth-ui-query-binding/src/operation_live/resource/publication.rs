use super::WorthUiOperationLiveResource;
use crate::WorthUiCollectionChangeConsequence;

impl WorthUiOperationLiveResource {
    pub(crate) fn admit_collection_change(
        &mut self,
        consequence: WorthUiCollectionChangeConsequence,
    ) -> Result<
        crate::WorthUiCollectionChangeStagingReceipt,
        crate::WorthUiCollectionChangeAdmissionStop,
    > {
        let receipt = match validate_or_stop(self, &consequence) {
            Ok(receipt) => receipt,
            Err(denial) => {
                return Err(crate::WorthUiCollectionChangeAdmissionStop::new(
                    denial,
                    consequence,
                ));
            }
        };
        self.staged_change_admitted = true;
        Ok(receipt)
    }

    pub(crate) fn admit_collection_change_for_publication(
        &mut self,
        consequence: WorthUiCollectionChangeConsequence,
    ) -> Result<
        crate::WorthUiAdmittedCollectionChangePublication,
        crate::WorthUiCollectionChangeAdmissionStop,
    > {
        let receipt = match validate_or_stop(self, &consequence) {
            Ok(receipt) => receipt,
            Err(denial) => {
                return Err(crate::WorthUiCollectionChangeAdmissionStop::new(
                    denial,
                    consequence,
                ));
            }
        };
        self.staged_change_admitted = true;
        Ok(crate::WorthUiAdmittedCollectionChangePublication::seal(
            consequence,
            receipt,
        ))
    }

    pub(crate) fn validate_collection_change_observation(
        &self,
        consequence: WorthUiCollectionChangeConsequence,
    ) -> Result<
        crate::WorthUiValidatedCollectionChangeObservation,
        crate::WorthUiCollectionChangeAdmissionStop,
    > {
        let receipt = match validate_or_stop(self, &consequence) {
            Ok(receipt) => receipt,
            Err(denial) => {
                return Err(crate::WorthUiCollectionChangeAdmissionStop::new(
                    denial,
                    consequence,
                ));
            }
        };
        Ok(crate::WorthUiValidatedCollectionChangeObservation::seal(
            consequence,
            receipt,
        ))
    }

    pub(crate) fn publish_staged_collection_change(&mut self) -> bool {
        if !self.staged_change_admitted {
            return false;
        }
        let Some(consequence) = self.staged_change.take() else {
            return false;
        };
        self.staged_change_admitted = false;
        self.admitted_changes.push_back(consequence);
        true
    }

    pub(crate) fn publish_admitted_collection_change(
        &mut self,
        admission: crate::WorthUiAdmittedCollectionChangePublication,
    ) -> Result<(), crate::WorthUiCollectionChangePublicationStop> {
        if let Err(denial) = validate_publication_admission(self, &admission) {
            return Err(crate::WorthUiCollectionChangePublicationStop::new(
                denial, admission,
            ));
        }
        let consequence = self
            .staged_change
            .take()
            .expect("validated publication admission retains one staged change");
        self.staged_change_admitted = false;
        self.admitted_changes.push_back(consequence);
        Ok(())
    }

    pub(crate) fn withdraw_admitted_collection_change(
        &mut self,
        admission: crate::WorthUiAdmittedCollectionChangePublication,
    ) -> Result<WorthUiCollectionChangeConsequence, crate::WorthUiCollectionChangePublicationStop>
    {
        if let Err(denial) = validate_publication_admission(self, &admission) {
            return Err(crate::WorthUiCollectionChangePublicationStop::new(
                denial, admission,
            ));
        }
        self.staged_change_admitted = false;
        Ok(admission.into_consequence())
    }

    pub(crate) fn retry_collection_change_handoff(
        &self,
    ) -> Result<WorthUiCollectionChangeConsequence, crate::WorthUiCollectionChangeHandoffRetryDenial>
    {
        if self.staged_change_admitted {
            return Err(
                crate::WorthUiCollectionChangeHandoffRetryDenial::AlreadyAdmittedToFrameworkTurn,
            );
        }
        self.staged_change
            .as_ref()
            .map(crate::collection_delivery::WorthUiRetainedCollectionChangeConsequence::handoff)
            .ok_or(crate::WorthUiCollectionChangeHandoffRetryDenial::NoUnpublishedChange)
    }

    pub fn admitted_collection_change_count(&self) -> usize {
        self.admitted_changes.len()
    }

    pub fn staged_collection_change_count(&self) -> usize {
        usize::from(self.staged_change.is_some())
    }

    pub(crate) fn collection_change_observation(
        &self,
    ) -> crate::WorthUiOperationLiveChangeObservation {
        crate::WorthUiOperationLiveChangeObservation::new(
            self.staged_collection_change_count(),
            self.admitted_collection_change_count(),
            self.next_change_order,
        )
    }
}

fn validate_or_stop(
    resource: &WorthUiOperationLiveResource,
    consequence: &WorthUiCollectionChangeConsequence,
) -> Result<
    crate::WorthUiCollectionChangeStagingReceipt,
    crate::WorthUiCollectionChangeAdmissionDenial,
> {
    let belongs_to_resource = consequence.installed_reference() == &resource.installed_reference
        && resource
            .collection_source
            .as_ref()
            .is_some_and(|source| source == consequence.source())
        && consequence.change_order() == resource.next_change_order
        && resource
            .staged_change
            .as_ref()
            .is_some_and(|staged| staged.matches(consequence));
    if !belongs_to_resource {
        return Err(crate::WorthUiCollectionChangeAdmissionDenial::StaleOrForeignConsequence);
    }
    if resource.staged_change_admitted {
        return Err(crate::WorthUiCollectionChangeAdmissionDenial::AlreadyAdmitted);
    }
    Ok(crate::WorthUiCollectionChangeStagingReceipt::from_consequence(consequence))
}

fn validate_publication_admission(
    resource: &WorthUiOperationLiveResource,
    admission: &crate::WorthUiAdmittedCollectionChangePublication,
) -> Result<(), crate::WorthUiCollectionChangePublicationDenial> {
    if admission.installed_reference() != &resource.installed_reference
        || !resource
            .staged_change
            .as_ref()
            .is_some_and(|staged| staged.matches(admission.consequence()))
    {
        return Err(crate::WorthUiCollectionChangePublicationDenial::StaleOrForeignAdmission);
    }
    if !resource.staged_change_admitted {
        return Err(crate::WorthUiCollectionChangePublicationDenial::AdmissionNotActive);
    }
    Ok(())
}
