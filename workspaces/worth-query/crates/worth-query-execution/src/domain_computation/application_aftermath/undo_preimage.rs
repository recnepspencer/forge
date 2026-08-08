//! Retained inverse pre-image (R8.2 / Resolution A).
//!
//! Query retains the exact pre-image slice demanded by the installed inverse
//! from the decision read-set already observed at admission. Undo consumes this
//! carrier — it must never live-re-read the graph and call the result original.

use worth_foundational::facade::{AspectFieldLocator, AspectValue, CanonicalFieldPath, FieldKey};
use worth_query_installation::facade::InstalledPreImageDemand;
use worth_relational::facade::identity::{EntityId, KindId};
use worth_relational::facade::transactions::RecordRef;

/// One retained field pre-image bound into the strengthened commit receipt.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryRetainedPreImageField {
    field_slot: String,
    value: AspectValue,
    encoded_bytes: usize,
    target_record: RecordRef,
    entity_kind: KindId,
    locator: AspectFieldLocator,
}

impl WorthQueryRetainedPreImageField {
    pub fn field_slot(&self) -> &str {
        &self.field_slot
    }

    pub const fn value(&self) -> &AspectValue {
        &self.value
    }

    pub const fn encoded_bytes(&self) -> usize {
        self.encoded_bytes
    }

    /// Exact record from which this historical value was observed.
    pub const fn target_record(&self) -> &RecordRef {
        &self.target_record
    }

    pub const fn entity_kind(&self) -> KindId {
        self.entity_kind
    }

    pub const fn locator(&self) -> &AspectFieldLocator {
        &self.locator
    }
}

/// Exact pre-image slice retained for a recorded inverse (R8.2).
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryRetainedPreImage {
    fields: Vec<WorthQueryRetainedPreImageField>,
    total_encoded_bytes: usize,
    _private: (),
}

impl WorthQueryRetainedPreImage {
    pub fn fields(&self) -> &[WorthQueryRetainedPreImageField] {
        &self.fields
    }

    pub const fn total_encoded_bytes(&self) -> usize {
        self.total_encoded_bytes
    }

    pub fn field(&self, slot: &str) -> Option<&WorthQueryRetainedPreImageField> {
        self.fields.iter().find(|field| field.field_slot == slot)
    }

    /// One exact target when every demanded field came from the same record.
    pub fn target_record(&self) -> Option<&RecordRef> {
        let first = self.fields.first()?.target_record();
        self.fields
            .iter()
            .all(|field| field.target_record() == first)
            .then_some(first)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryPreImageRetentionDenial {
    MissingDemandedField,
    ExceedsByteBound,
    EmptyDemand,
    /// The demanded slot was observed on more than one mutated record, so no
    /// single prior truth is named (Q8.26-C7).
    AmbiguousDemandedField,
    /// The operation mutates nothing, so there is no prior truth to invert.
    NoMutatedRecord,
}

/// The demanded field slot an observed path names, if any (Q8.26-C3).
///
/// A demand names a single field slot and has no vocabulary for a nested path,
/// so only an exactly-one-segment path names a demanded field. Reducing a longer
/// path to its first segment — as this did before slice 10 — lets a nested
/// `Account.Status` observation satisfy a demand for `Status` and bind the wrong
/// value as the prior truth.
pub fn demanded_field_slot(path: &CanonicalFieldPath) -> Option<&FieldKey> {
    match path.fields() {
        [field] => Some(field),
        _ => None,
    }
}

/// Observed field fact available for retention at commit.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryObservedPreImageCandidate {
    field_slot: String,
    value: AspectValue,
    target_record: RecordRef,
    entity_kind: KindId,
    locator: AspectFieldLocator,
}

impl WorthQueryObservedPreImageCandidate {
    pub(crate) fn from_observed_field(
        locator: AspectFieldLocator,
        value: AspectValue,
        entity_id: EntityId,
        entity_kind: KindId,
    ) -> Self {
        let field_slot = demanded_field_slot(locator.field_path())
            .expect("retention candidates require one exact field")
            .as_str()
            .to_owned();
        Self {
            field_slot,
            value,
            target_record: RecordRef::Entity(entity_id),
            entity_kind,
            locator,
        }
    }

    pub fn field_slot(&self) -> &str {
        &self.field_slot
    }

    pub const fn value(&self) -> &AspectValue {
        &self.value
    }

    pub const fn target_record(&self) -> &RecordRef {
        &self.target_record
    }

    pub const fn entity_kind(&self) -> KindId {
        self.entity_kind
    }

    pub const fn locator(&self) -> &AspectFieldLocator {
        &self.locator
    }
}

/// Retain the exact demand slice from already-observed decision facts.
///
/// Candidates must come from the admitted decision read-set. This function does
/// not read the live graph.
///
/// `mutation` is the exact record-and-field footprint projected from
/// Relational's invariant-validated merged plan. A demanded slot is satisfied
/// only by an admitted observation of the exact field this operation changes
/// (Q8.26-C7). Matching only the slot name, record, or pre-validation worker
/// intent can substitute unrelated prior truth into the receipt.
pub(in crate::domain_computation) fn retain_preimage_from_observed_facts(
    demand: &InstalledPreImageDemand,
    candidates: &[WorthQueryObservedPreImageCandidate],
    mutation: &worth_relational::facade::transactions::ValidatedMutationFootprint,
) -> Result<WorthQueryRetainedPreImage, WorthQueryPreImageRetentionDenial> {
    retain_preimage_matching(demand, candidates, mutation.is_empty(), |candidate| {
        mutation.mutates_field(candidate.target_record(), candidate.locator())
    })
}

#[cfg(test)]
pub(crate) fn retain_preimage_from_test_footprint(
    demand: &InstalledPreImageDemand,
    candidates: &[WorthQueryObservedPreImageCandidate],
    mutation: &[(RecordRef, AspectFieldLocator)],
) -> Result<WorthQueryRetainedPreImage, WorthQueryPreImageRetentionDenial> {
    retain_preimage_matching(demand, candidates, mutation.is_empty(), |candidate| {
        mutation.iter().any(|(record, locator)| {
            record == candidate.target_record() && locator == candidate.locator()
        })
    })
}

fn retain_preimage_matching(
    demand: &InstalledPreImageDemand,
    candidates: &[WorthQueryObservedPreImageCandidate],
    mutation_is_empty: bool,
    mutates: impl Fn(&WorthQueryObservedPreImageCandidate) -> bool,
) -> Result<WorthQueryRetainedPreImage, WorthQueryPreImageRetentionDenial> {
    if demand.field_slots().is_empty() {
        return Err(WorthQueryPreImageRetentionDenial::EmptyDemand);
    }
    if mutation_is_empty {
        return Err(WorthQueryPreImageRetentionDenial::NoMutatedRecord);
    }
    let mut fields = Vec::with_capacity(demand.field_slots().len());
    let mut total = 0usize;
    for slot in demand.field_slots() {
        let mut matching = candidates
            .iter()
            .filter(|candidate| candidate.field_slot() == slot && mutates(candidate));
        let Some(candidate) = matching.next() else {
            return Err(WorthQueryPreImageRetentionDenial::MissingDemandedField);
        };
        if matching.next().is_some() {
            return Err(WorthQueryPreImageRetentionDenial::AmbiguousDemandedField);
        }
        let encoded_bytes = candidate.value().semantic_byte_width();
        total = total.saturating_add(encoded_bytes);
        if total > demand.maximum_encoded_bytes() {
            return Err(WorthQueryPreImageRetentionDenial::ExceedsByteBound);
        }
        fields.push(WorthQueryRetainedPreImageField {
            field_slot: slot.clone(),
            value: candidate.value().clone(),
            encoded_bytes,
            target_record: candidate.target_record().clone(),
            entity_kind: candidate.entity_kind(),
            locator: candidate.locator().clone(),
        });
    }
    Ok(WorthQueryRetainedPreImage {
        fields,
        total_encoded_bytes: total,
        _private: (),
    })
}
