use std::collections::{BTreeMap, BTreeSet};

use super::error::WorthQueryGraphReadBypassResidueError;
use super::graph_read_bypass_digest;
use super::registry::WorthQueryGraphReadBypassClass;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryGraphReadBypassResidueRow {
    class: WorthQueryGraphReadBypassClass,
    owner: String,
    introduced_in: String,
    current_count: usize,
    must_not_exceed_count: usize,
    blocker: String,
    removal_trigger: String,
    row_digest: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryGraphReadBypassResidueManifest {
    rows: Vec<WorthQueryGraphReadBypassResidueRow>,
    manifest_digest: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryGraphReadBypassResidueCertification {
    previous_manifest_digest: String,
    candidate_manifest_digest: String,
    certified_row_count: usize,
    certification_digest: String,
}

impl WorthQueryGraphReadBypassResidueRow {
    pub fn explicit(
        class: WorthQueryGraphReadBypassClass,
        owner: impl Into<String>,
        introduced_in: impl Into<String>,
        current_count: usize,
        must_not_exceed_count: usize,
        blocker: impl Into<String>,
        removal_trigger: impl Into<String>,
    ) -> Result<Self, WorthQueryGraphReadBypassResidueError> {
        let owner = required_text(class, owner.into(), "owner")?;
        let introduced_in = required_text(class, introduced_in.into(), "introduced_in")?;
        let blocker = required_text(class, blocker.into(), "blocker")?;
        let removal_trigger = required_text(class, removal_trigger.into(), "removal_trigger")?;
        if current_count > must_not_exceed_count {
            return Err(WorthQueryGraphReadBypassResidueError::count_exceeds_cap(
                class,
                current_count,
                must_not_exceed_count,
            ));
        }
        let current_count_text = current_count.to_string();
        let cap_text = must_not_exceed_count.to_string();
        let row_digest = graph_read_bypass_digest(
            "residue-row",
            [
                class.as_str(),
                owner.as_str(),
                introduced_in.as_str(),
                current_count_text.as_str(),
                cap_text.as_str(),
                blocker.as_str(),
                removal_trigger.as_str(),
            ],
        );
        Ok(Self {
            class,
            owner,
            introduced_in,
            current_count,
            must_not_exceed_count,
            blocker,
            removal_trigger,
            row_digest,
        })
    }

    pub fn class(&self) -> WorthQueryGraphReadBypassClass {
        self.class
    }

    pub fn owner(&self) -> &str {
        &self.owner
    }

    pub fn introduced_in(&self) -> &str {
        &self.introduced_in
    }

    pub fn current_count(&self) -> usize {
        self.current_count
    }

    pub fn must_not_exceed_count(&self) -> usize {
        self.must_not_exceed_count
    }

    pub fn blocker(&self) -> &str {
        &self.blocker
    }

    pub fn removal_trigger(&self) -> &str {
        &self.removal_trigger
    }

    pub fn row_digest(&self) -> &str {
        &self.row_digest
    }

    fn has_same_contract(&self, previous: &Self) -> bool {
        self.owner == previous.owner
            && self.introduced_in == previous.introduced_in
            && self.must_not_exceed_count == previous.must_not_exceed_count
            && self.blocker == previous.blocker
            && self.removal_trigger == previous.removal_trigger
    }
}

impl WorthQueryGraphReadBypassResidueManifest {
    pub fn capped(
        rows: impl IntoIterator<Item = WorthQueryGraphReadBypassResidueRow>,
    ) -> Result<Self, WorthQueryGraphReadBypassResidueError> {
        let mut rows = rows.into_iter().collect::<Vec<_>>();
        reject_duplicate_rows(&rows)?;
        rows.sort_by(|left, right| left.class.as_str().cmp(right.class.as_str()));
        let manifest_digest =
            graph_read_bypass_digest("residue-manifest", rows.iter().map(|row| row.row_digest()));
        Ok(Self {
            rows,
            manifest_digest,
        })
    }

    pub fn empty() -> Self {
        Self {
            rows: Vec::new(),
            manifest_digest: graph_read_bypass_digest("residue-manifest", ["empty"]),
        }
    }

    pub fn rows(&self) -> &[WorthQueryGraphReadBypassResidueRow] {
        &self.rows
    }

    pub fn manifest_digest(&self) -> &str {
        &self.manifest_digest
    }

    pub fn current_count_for_class(&self, class: WorthQueryGraphReadBypassClass) -> usize {
        self.rows
            .iter()
            .find(|row| row.class == class)
            .map_or(0, WorthQueryGraphReadBypassResidueRow::current_count)
    }

    pub fn certify_candidate_against_previous(
        previous: &Self,
        candidate: &Self,
    ) -> Result<WorthQueryGraphReadBypassResidueCertification, WorthQueryGraphReadBypassResidueError>
    {
        let previous_by_class = previous
            .rows
            .iter()
            .map(|row| (row.class, row))
            .collect::<BTreeMap<_, _>>();
        for candidate_row in &candidate.rows {
            let Some(previous_row) = previous_by_class.get(&candidate_row.class) else {
                continue;
            };
            if candidate_row.current_count > previous_row.current_count {
                return Err(WorthQueryGraphReadBypassResidueError::residue_growth(
                    candidate_row.class,
                    candidate_row.current_count,
                    previous_row.current_count,
                ));
            }
            if !candidate_row.has_same_contract(previous_row) {
                return Err(WorthQueryGraphReadBypassResidueError::contract_changed(
                    candidate_row.class,
                ));
            }
        }
        let certified_row_count = candidate.rows.len();
        let certified_row_count_text = certified_row_count.to_string();
        let certification_digest = graph_read_bypass_digest(
            "residue-certification",
            [
                previous.manifest_digest.as_str(),
                candidate.manifest_digest.as_str(),
                certified_row_count_text.as_str(),
            ],
        );
        Ok(WorthQueryGraphReadBypassResidueCertification {
            previous_manifest_digest: previous.manifest_digest.clone(),
            candidate_manifest_digest: candidate.manifest_digest.clone(),
            certified_row_count,
            certification_digest,
        })
    }
}

impl WorthQueryGraphReadBypassResidueCertification {
    pub fn previous_manifest_digest(&self) -> &str {
        &self.previous_manifest_digest
    }

    pub fn candidate_manifest_digest(&self) -> &str {
        &self.candidate_manifest_digest
    }

    pub fn certified_row_count(&self) -> usize {
        self.certified_row_count
    }

    pub fn certification_digest(&self) -> &str {
        &self.certification_digest
    }
}

fn reject_duplicate_rows(
    rows: &[WorthQueryGraphReadBypassResidueRow],
) -> Result<(), WorthQueryGraphReadBypassResidueError> {
    let mut classes = BTreeSet::new();
    for row in rows {
        if !classes.insert(row.class) {
            return Err(WorthQueryGraphReadBypassResidueError::duplicate_class(
                row.class,
            ));
        }
    }
    Ok(())
}

fn required_text(
    class: WorthQueryGraphReadBypassClass,
    value: String,
    field: &'static str,
) -> Result<String, WorthQueryGraphReadBypassResidueError> {
    if value.trim().is_empty() {
        Err(WorthQueryGraphReadBypassResidueError::missing_required_field(class, field))
    } else {
        Ok(value)
    }
}
