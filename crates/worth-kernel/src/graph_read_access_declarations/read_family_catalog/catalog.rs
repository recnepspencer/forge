use std::collections::BTreeMap;

use crate::graph_read_access_inventory::WorthGraphReadDeclarationCandidate;

use super::catalog_key::{stable_digest, WorthGraphReadDeclarationCatalogKey};
use super::catalog_record::WorthGraphReadDeclarationCatalogRecord;
use super::errors::{
    WorthGraphReadAccessDeclarationPhaseTwoError, WorthGraphReadAccessDeclarationPhaseTwoErrorKind,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthGraphReadDeclarationCatalog {
    records: Vec<WorthGraphReadDeclarationCatalogRecord>,
    catalog_digest: String,
    source_candidate_count: usize,
}

impl WorthGraphReadDeclarationCatalog {
    pub(crate) fn from_candidates(
        candidates: &[WorthGraphReadDeclarationCandidate],
    ) -> Result<Self, WorthGraphReadAccessDeclarationPhaseTwoError> {
        let records = catalog_records_from_declaration_candidates(candidates)?;
        Ok(Self {
            catalog_digest: catalog_digest_from_records(&records),
            records,
            source_candidate_count: candidates.len(),
        })
    }

    pub fn records(&self) -> &[WorthGraphReadDeclarationCatalogRecord] {
        &self.records
    }

    pub fn catalog_digest(&self) -> &str {
        &self.catalog_digest
    }

    pub const fn source_candidate_count(&self) -> usize {
        self.source_candidate_count
    }

    pub fn record_for_key(
        &self,
        key: &WorthGraphReadDeclarationCatalogKey,
    ) -> Option<&WorthGraphReadDeclarationCatalogRecord> {
        self.records.iter().find(|record| record.key() == key)
    }
}

fn catalog_records_from_declaration_candidates(
    candidates: &[WorthGraphReadDeclarationCandidate],
) -> Result<Vec<WorthGraphReadDeclarationCatalogRecord>, WorthGraphReadAccessDeclarationPhaseTwoError>
{
    let mut records_by_key = BTreeMap::new();
    let mut requirement_by_conflict_identity = BTreeMap::new();

    for candidate in candidates {
        let key = WorthGraphReadDeclarationCatalogKey::from_candidate(candidate)?;
        record_requirement_conflict_identity(&mut requirement_by_conflict_identity, &key)?;
        merge_candidate_into_catalog_records(&mut records_by_key, key, candidate);
    }

    Ok(records_by_key.into_values().collect())
}

fn record_requirement_conflict_identity(
    requirement_by_conflict_identity: &mut BTreeMap<String, String>,
    key: &WorthGraphReadDeclarationCatalogKey,
) -> Result<(), WorthGraphReadAccessDeclarationPhaseTwoError> {
    let conflict_identity = key.conflict_identity();
    let requirement_digest = key.requirement_evidence_digest().to_string();
    match requirement_by_conflict_identity.get(&conflict_identity) {
        Some(existing) if existing != &requirement_digest => Err(error(
            WorthGraphReadAccessDeclarationPhaseTwoErrorKind::ConflictingTouchedAuthorityReadShape,
        )),
        Some(_) => Ok(()),
        None => {
            requirement_by_conflict_identity.insert(conflict_identity, requirement_digest);
            Ok(())
        }
    }
}

fn merge_candidate_into_catalog_records(
    records_by_key: &mut BTreeMap<
        WorthGraphReadDeclarationCatalogKey,
        WorthGraphReadDeclarationCatalogRecord,
    >,
    key: WorthGraphReadDeclarationCatalogKey,
    candidate: &WorthGraphReadDeclarationCandidate,
) {
    match records_by_key.get_mut(&key) {
        Some(record) => record.add_source_row_identity(candidate.inventory_row_identity().clone()),
        None => {
            records_by_key.insert(
                key.clone(),
                WorthGraphReadDeclarationCatalogRecord::new(
                    key,
                    candidate.inventory_row_identity().clone(),
                ),
            );
        }
    }
}

fn catalog_digest_from_records(records: &[WorthGraphReadDeclarationCatalogRecord]) -> String {
    stable_digest(
        &records
            .iter()
            .map(|record| format!("record:{}", record.declaration_identity_digest()))
            .collect::<Vec<_>>(),
    )
}

const fn error(
    kind: WorthGraphReadAccessDeclarationPhaseTwoErrorKind,
) -> WorthGraphReadAccessDeclarationPhaseTwoError {
    WorthGraphReadAccessDeclarationPhaseTwoError::new(kind)
}
