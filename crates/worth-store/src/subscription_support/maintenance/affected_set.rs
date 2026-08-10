use super::super::{
    classification_error, SubscriptionSupportActionOrigin, SubscriptionSupportFamilyId,
    SubscriptionSupportFamilyKind, SubscriptionSupportOperationalBasis, SubscriptionSupportRole,
    SupportAffectedSetDigest,
};
use super::decision::SubscriptionSupportMaintenanceDecision;
use super::descriptor::SupportMaintenanceDescriptor;
use crate::failure::StoreError;
use serde::Serialize;
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SupportMaintenanceAffectedSet {
    family_id: SubscriptionSupportFamilyId,
    family_kind: SubscriptionSupportFamilyKind,
    support_role: SubscriptionSupportRole,
    affected_set_digest: SupportAffectedSetDigest,
    affected_bases: Vec<SubscriptionSupportOperationalBasis>,
}

impl SupportMaintenanceAffectedSet {
    pub(crate) fn from_maintenance_bases(
        affected_bases: Vec<SubscriptionSupportOperationalBasis>,
    ) -> Result<Self, StoreError> {
        let Some(first) = affected_bases.first() else {
            return Err(classification_error(
                "subscription-support maintenance affected sets must not be empty",
            ));
        };
        if first.action_origin() != SubscriptionSupportActionOrigin::Maintenance {
            return Err(classification_error(
                "subscription-support maintenance affected sets require maintenance-origin bases",
            ));
        }
        for basis in &affected_bases {
            if basis.action_origin() != SubscriptionSupportActionOrigin::Maintenance {
                return Err(classification_error(
                    "subscription-support maintenance affected sets cannot mix action origins",
                ));
            }
            if basis.family_id() != first.family_id()
                || basis.family_kind() != first.family_kind()
                || basis.support_role() != first.support_role()
            {
                return Err(classification_error(
                    "subscription-support maintenance affected sets must be family-local",
                ));
            }
        }
        Ok(Self {
            family_id: first.family_id().clone(),
            family_kind: first.family_kind(),
            support_role: first.support_role(),
            affected_set_digest: SupportAffectedSetDigest::from_bases(&affected_bases)?,
            affected_bases,
        })
    }

    pub fn family_id(&self) -> &SubscriptionSupportFamilyId {
        &self.family_id
    }

    pub fn family_kind(&self) -> SubscriptionSupportFamilyKind {
        self.family_kind
    }

    pub fn support_role(&self) -> SubscriptionSupportRole {
        self.support_role
    }

    pub fn affected_count(&self) -> u64 {
        self.affected_bases.len() as u64
    }

    pub fn affected_set_digest(&self) -> &SupportAffectedSetDigest {
        &self.affected_set_digest
    }

    pub(crate) fn primary_basis(&self) -> &SubscriptionSupportOperationalBasis {
        &self.affected_bases[0]
    }

    pub(crate) fn affected_bases(&self) -> &[SubscriptionSupportOperationalBasis] {
        &self.affected_bases
    }

    pub(crate) fn descriptors_for(
        &self,
        decision: &SubscriptionSupportMaintenanceDecision,
    ) -> Result<(Vec<SupportMaintenanceDescriptor>, u64), StoreError> {
        let mut descriptors_by_key = BTreeMap::new();
        let mut duplicate_count = 0;
        for basis in &self.affected_bases {
            let descriptor = SupportMaintenanceDescriptor::from_basis(basis, decision)?;
            let key = descriptor.maintenance_key().to_string();
            if descriptors_by_key.insert(key, descriptor).is_some() {
                duplicate_count += 1;
            }
        }
        Ok((descriptors_by_key.into_values().collect(), duplicate_count))
    }
}
