use std::collections::BTreeMap;

use super::OperationalAuditRecord;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OperationalAuditAssemblyDenial {
    ConflictingDuplicateIdentity,
    ConflictingOperationSequence,
    ConflictingOperationTransition,
    InvalidCausalOrder,
}

pub fn assemble_operational_audit_records(
    deliveries: impl IntoIterator<Item = OperationalAuditRecord>,
) -> Result<Vec<OperationalAuditRecord>, OperationalAuditAssemblyDenial> {
    let mut identities = BTreeMap::<[u8; 32], OperationalAuditRecord>::new();
    for record in deliveries {
        match identities.get(&record.record_identity()) {
            Some(existing) if existing != &record => {
                return Err(OperationalAuditAssemblyDenial::ConflictingDuplicateIdentity)
            }
            Some(_) => continue,
            None => {
                identities.insert(record.record_identity(), record);
            }
        }
    }

    let mut ordered = identities.into_values().collect::<Vec<_>>();
    ordered.sort_by(|left, right| {
        left.operation_id()
            .as_str()
            .cmp(right.operation_id().as_str())
            .then(left.sequence().get().cmp(&right.sequence().get()))
            .then(
                left.transition_id()
                    .as_str()
                    .cmp(right.transition_id().as_str()),
            )
    });
    for pair in ordered.windows(2) {
        if pair[0].operation_id() != pair[1].operation_id() {
            continue;
        }
        if pair[0].sequence() == pair[1].sequence() {
            return Err(OperationalAuditAssemblyDenial::ConflictingOperationSequence);
        }
        if pair[0].transition_id() == pair[1].transition_id() {
            return Err(OperationalAuditAssemblyDenial::ConflictingOperationTransition);
        }
        if pair[1]
            .causal_parent()
            .map(|parent| parent.record_identity())
            != Some(pair[0].record_identity())
        {
            return Err(OperationalAuditAssemblyDenial::InvalidCausalOrder);
        }
    }
    for record in &ordered {
        if record.sequence().get() == 1 && record.causal_parent().is_some() {
            return Err(OperationalAuditAssemblyDenial::InvalidCausalOrder);
        }
    }
    Ok(ordered)
}
