use std::collections::BTreeMap;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum ExactOwnerChangeKey {
    Semantic {
        entity: std::sync::Arc<str>,
        record: Option<worth_runtime_bridge::facade::RelationalBridgeRecordIdentityParts>,
        aspect: worth_foundational::facade::AspectKey,
        aspect_identity: worth_foundational::facade::AspectIdentity,
        contract_revision: worth_foundational::facade::AspectContractRevision,
        binding: worth_foundational::facade::AspectBinding,
        kind: worth_foundational::facade::AuthoritativeAspectChangeKind,
        field: Option<worth_foundational::facade::CanonicalFieldPath>,
        precision: worth_runtime_bridge::facade::BridgeAspectChangePrecision,
        widening_cause: Option<worth_runtime_bridge::facade::BridgeAspectChangeWideningCause>,
    },
    Structural {
        record: worth_runtime_bridge::facade::RelationalBridgeRecordIdentityParts,
        kind: ExactStructuralKind,
    },
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum ExactStructuralKind {
    Created,
    Updated,
    Deleted,
    RetainedForAudit,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) struct WorthQueryStagedOwnerDeliveryAdmission {
    staged_changes_inspected: usize,
    owner_changes_inspected: usize,
    causal_keys_materialized: usize,
    causal_key_lookups: usize,
}

impl WorthQueryStagedOwnerDeliveryAdmission {
    pub(crate) const fn staged_changes_inspected(self) -> usize {
        self.staged_changes_inspected
    }
    pub(crate) const fn owner_changes_inspected(self) -> usize {
        self.owner_changes_inspected
    }
    pub(crate) const fn causal_keys_materialized(self) -> usize {
        self.causal_keys_materialized
    }
    pub(crate) const fn causal_key_lookups(self) -> usize {
        self.causal_key_lookups
    }
}

pub(crate) struct WorthQueryStagedOwnerDeliveryAdmissionError {
    denial: crate::domain_installation::WorthQueryImpactAdmissionDenial,
    work: WorthQueryStagedOwnerDeliveryAdmission,
}

impl WorthQueryStagedOwnerDeliveryAdmissionError {
    pub(crate) fn causal_mismatch(work: WorthQueryStagedOwnerDeliveryAdmission) -> Self {
        Self {
            denial: crate::domain_installation::WorthQueryImpactAdmissionDenial::new(
                crate::domain_installation::WorthQueryImpactAdmissionDenialKind::CausalDeliveryMismatch,
                impact_counters(work, 0),
            ),
            work,
        }
    }
    pub(crate) fn missing_route() -> Self {
        Self::causal_mismatch(WorthQueryStagedOwnerDeliveryAdmission {
            staged_changes_inspected: 0,
            owner_changes_inspected: 0,
            causal_keys_materialized: 0,
            causal_key_lookups: 1,
        })
    }

    pub(crate) fn out_of_order(work: WorthQueryStagedOwnerDeliveryAdmission) -> Self {
        Self {
            denial: crate::domain_installation::WorthQueryImpactAdmissionDenial::new(
                crate::domain_installation::WorthQueryImpactAdmissionDenialKind::OwnerDeliveryOutOfOrder,
                impact_counters(work, 1),
            ),
            work,
        }
    }
    pub(crate) const fn denial(
        &self,
    ) -> crate::domain_installation::WorthQueryImpactAdmissionDenial {
        self.denial
    }
    pub(crate) const fn work(&self) -> WorthQueryStagedOwnerDeliveryAdmission {
        self.work
    }
}

fn impact_counters(
    work: WorthQueryStagedOwnerDeliveryAdmission,
    owner_order_checks: usize,
) -> crate::domain_installation::WorthQueryImpactCounters {
    crate::domain_installation::WorthQueryImpactCounters {
        staged_changes_inspected: work.staged_changes_inspected(),
        owner_changes_inspected: work.owner_changes_inspected(),
        causal_keys_materialized: work.causal_keys_materialized(),
        causal_key_lookups: work.causal_key_lookups(),
        owner_order_checks,
        ..crate::domain_installation::WorthQueryImpactCounters::default()
    }
}

pub(super) fn compare_owner_delivery(
    staged: &worth_runtime_bridge::facade::BridgeCorrespondenceDeliveryReceipt,
    owner: &worth_runtime_bridge::facade::BridgeCorrespondenceDeliveryReceipt,
) -> (bool, WorthQueryStagedOwnerDeliveryAdmission) {
    let staged_set = staged.change_set();
    let owner_set = owner.change_set();
    if staged_set.basis() != owner_set.basis()
        || !staged_set
            .dependency()
            .retains_same_source_authority_as(owner_set.dependency())
    {
        return (false, WorthQueryStagedOwnerDeliveryAdmission::default());
    }
    let mut work = WorthQueryStagedOwnerDeliveryAdmission::default();
    let staged_changes = exact_change_counts(staged_set.changes(), true, &mut work);
    let owner_changes = exact_change_counts(owner_set.changes(), false, &mut work);
    let matches = exact_change_multisets_match(&staged_changes, &owner_changes, &mut work);
    (matches, work)
}

pub(super) fn owner_only_work(
    owner: &worth_runtime_bridge::facade::BridgeCorrespondenceDeliveryReceipt,
) -> WorthQueryStagedOwnerDeliveryAdmission {
    let mut work = WorthQueryStagedOwnerDeliveryAdmission::default();
    let _ = exact_change_counts(owner.change_set().changes(), false, &mut work);
    work.causal_key_lookups = 1;
    work
}

fn exact_change_counts(
    changes: &[worth_runtime_bridge::facade::BridgeDeliveredCorrespondenceChange],
    staged: bool,
    work: &mut WorthQueryStagedOwnerDeliveryAdmission,
) -> BTreeMap<ExactOwnerChangeKey, usize> {
    let mut counts = BTreeMap::new();
    for change in changes {
        retain_change(&mut counts, exact_change_key(change), staged, work);
    }
    counts
}

fn retain_change(
    counts: &mut BTreeMap<ExactOwnerChangeKey, usize>,
    key: ExactOwnerChangeKey,
    staged: bool,
    work: &mut WorthQueryStagedOwnerDeliveryAdmission,
) {
    if staged {
        work.staged_changes_inspected += 1;
    } else {
        work.owner_changes_inspected += 1;
    }
    work.causal_keys_materialized += 1;
    *counts.entry(key).or_default() += 1;
}

fn exact_change_multisets_match(
    staged: &BTreeMap<ExactOwnerChangeKey, usize>,
    owner: &BTreeMap<ExactOwnerChangeKey, usize>,
    work: &mut WorthQueryStagedOwnerDeliveryAdmission,
) -> bool {
    staged.len() == owner.len()
        && owner.iter().all(|(key, count)| {
            work.causal_key_lookups += 1;
            staged.get(key) == Some(count)
        })
}

fn exact_change_key(
    change: &worth_runtime_bridge::facade::BridgeDeliveredCorrespondenceChange,
) -> ExactOwnerChangeKey {
    use worth_runtime_bridge::facade::BridgeCommittedRecordChangeKind as Kind;
    if let Some(semantic) = change.semantic_change() {
        return ExactOwnerChangeKey::Semantic {
            entity: change
                .entity_identity()
                .expect("Bridge semantic change retains its source projection")
                .into(),
            record: change.relational_record_identity(),
            aspect: semantic.aspect_key().clone(),
            aspect_identity: semantic.aspect_identity(),
            contract_revision: semantic.contract_revision(),
            binding: semantic.binding().clone(),
            kind: semantic.kind(),
            field: semantic.field_path().cloned(),
            precision: semantic.precision(),
            widening_cause: semantic.widening_cause(),
        };
    }
    let structural = change
        .structural_change()
        .expect("Bridge delivered change retains semantic or structural meaning");
    ExactOwnerChangeKey::Structural {
        record: structural.record_identity(),
        kind: match structural.kind() {
            Kind::Created => ExactStructuralKind::Created,
            Kind::Updated => ExactStructuralKind::Updated,
            Kind::Deleted => ExactStructuralKind::Deleted,
            Kind::RetainedForAudit => ExactStructuralKind::RetainedForAudit,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn structural(slot: u64) -> ExactOwnerChangeKey {
        ExactOwnerChangeKey::Structural {
            record: worth_runtime_bridge::facade::RelationalBridgeRecordIdentityParts::entity(
                1, slot, 1,
            ),
            kind: ExactStructuralKind::Updated,
        }
    }

    #[test]
    fn exact_join_work_scales_with_unique_keys_and_multiplicity() {
        const UNIQUE_KEYS: u64 = 64;
        const MULTIPLICITY: usize = 3;
        let mut staged = BTreeMap::new();
        let mut owner = BTreeMap::new();
        let mut work = WorthQueryStagedOwnerDeliveryAdmission::default();
        for slot in 0..UNIQUE_KEYS {
            for _ in 0..MULTIPLICITY {
                retain_change(&mut staged, structural(slot), true, &mut work);
                retain_change(&mut owner, structural(slot), false, &mut work);
            }
        }

        assert!(exact_change_multisets_match(&staged, &owner, &mut work));
        assert_eq!(
            work.staged_changes_inspected(),
            UNIQUE_KEYS as usize * MULTIPLICITY
        );
        assert_eq!(
            work.owner_changes_inspected(),
            UNIQUE_KEYS as usize * MULTIPLICITY
        );
        assert_eq!(
            work.causal_keys_materialized(),
            UNIQUE_KEYS as usize * MULTIPLICITY * 2
        );
        assert_eq!(work.causal_key_lookups(), UNIQUE_KEYS as usize);
    }
}
