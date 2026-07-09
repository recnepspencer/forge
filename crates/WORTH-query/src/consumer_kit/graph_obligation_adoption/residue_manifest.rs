use std::collections::{BTreeMap, BTreeSet};

use super::error::{
    WorthQueryGraphObligationConsumerKitError, WorthQueryGraphObligationConsumerKitErrorKind,
};
use super::kit_digest;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryGraphObligationResidueRow {
    class: String,
    owner: String,
    introduced_in: String,
    current_count: usize,
    must_not_exceed_count: usize,
    blocker: String,
    removal_trigger: String,
    decision: String,
    row_digest: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryGraphObligationResidueManifest {
    rows: Vec<WorthQueryGraphObligationResidueRow>,
    manifest_digest: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryGraphObligationResidueCertification {
    previous_manifest_digest: String,
    candidate_manifest_digest: String,
    certified_row_count: usize,
    certification_digest: String,
}

impl WorthQueryGraphObligationResidueManifest {
    pub fn capped(
        rows: impl IntoIterator<Item = WorthQueryGraphObligationResidueRow>,
    ) -> Result<Self, WorthQueryGraphObligationConsumerKitError> {
        let mut rows = rows.into_iter().collect::<Vec<_>>();
        reject_duplicate_or_over_cap_rows(&rows)?;
        rows.sort_by(|left, right| left.row_digest.cmp(&right.row_digest));
        let manifest_digest = kit_digest(
            "graph-obligation-residue-manifest",
            rows.iter().map(|row| row.row_digest.as_str()),
        );
        Ok(Self {
            rows,
            manifest_digest,
        })
    }

    pub fn empty() -> Self {
        Self {
            rows: Vec::new(),
            manifest_digest: kit_digest("graph-obligation-residue-manifest", ["empty"]),
        }
    }

    pub fn rows(&self) -> &[WorthQueryGraphObligationResidueRow] {
        &self.rows
    }

    pub fn manifest_digest(&self) -> &str {
        &self.manifest_digest
    }

    pub fn certify_candidate_against_previous(
        previous: &Self,
        candidate: &Self,
    ) -> Result<
        WorthQueryGraphObligationResidueCertification,
        WorthQueryGraphObligationConsumerKitError,
    > {
        let previous_by_class = previous
            .rows
            .iter()
            .map(|row| (row.class.as_str(), row))
            .collect::<BTreeMap<_, _>>();
        for candidate_row in &candidate.rows {
            let Some(previous_row) = previous_by_class.get(candidate_row.class.as_str()) else {
                continue;
            };
            if candidate_row.current_count > previous_row.current_count {
                return Err(WorthQueryGraphObligationConsumerKitError::new(
                    WorthQueryGraphObligationConsumerKitErrorKind::ResidueGrowthAfterIntroduction,
                    format!(
                        "graph obligation residue class `{}` grew from {} to {}",
                        candidate_row.class,
                        previous_row.current_count,
                        candidate_row.current_count
                    ),
                ));
            }
            if !candidate_row.has_same_contract(previous_row) {
                return Err(WorthQueryGraphObligationConsumerKitError::new(
                    WorthQueryGraphObligationConsumerKitErrorKind::ResidueContractDrift,
                    format!(
                        "graph obligation residue class `{}` changed owner, introduction, cap, blocker, trigger, or decision",
                        candidate_row.class
                    ),
                ));
            }
        }
        let certified_row_count = candidate.rows.len();
        let certified_row_count_text = certified_row_count.to_string();
        let certification_digest = kit_digest(
            "graph-obligation-residue-certification",
            [
                previous.manifest_digest.as_str(),
                candidate.manifest_digest.as_str(),
                certified_row_count_text.as_str(),
            ],
        );
        Ok(WorthQueryGraphObligationResidueCertification {
            previous_manifest_digest: previous.manifest_digest.clone(),
            candidate_manifest_digest: candidate.manifest_digest.clone(),
            certified_row_count,
            certification_digest,
        })
    }
}

impl WorthQueryGraphObligationResidueRow {
    pub fn explicit(
        class: impl Into<String>,
        owner: impl Into<String>,
        introduced_in: impl Into<String>,
        current_count: usize,
        must_not_exceed_count: usize,
        blocker: impl Into<String>,
        removal_trigger: impl Into<String>,
        decision: impl Into<String>,
    ) -> Result<Self, WorthQueryGraphObligationConsumerKitError> {
        let class = required_residue_text(class.into(), "class")?;
        let owner = required_residue_text(owner.into(), "owner")?;
        let introduced_in = required_residue_text(introduced_in.into(), "introduced_in")?;
        let blocker = required_residue_text(blocker.into(), "blocker")?;
        let removal_trigger = required_residue_text(removal_trigger.into(), "removal_trigger")?;
        let decision = required_residue_text(decision.into(), "decision")?;
        let current_count_text = current_count.to_string();
        let cap_text = must_not_exceed_count.to_string();
        let row_digest = kit_digest(
            "graph-obligation-residue-row",
            [
                class.as_str(),
                owner.as_str(),
                introduced_in.as_str(),
                current_count_text.as_str(),
                cap_text.as_str(),
                blocker.as_str(),
                removal_trigger.as_str(),
                decision.as_str(),
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
            decision,
            row_digest,
        })
    }

    pub fn class(&self) -> &str {
        &self.class
    }

    pub fn current_count(&self) -> usize {
        self.current_count
    }

    pub fn owner(&self) -> &str {
        &self.owner
    }

    pub fn introduced_in(&self) -> &str {
        &self.introduced_in
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

    pub fn decision(&self) -> &str {
        &self.decision
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
            && self.decision == previous.decision
    }
}

impl WorthQueryGraphObligationResidueCertification {
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

fn reject_duplicate_or_over_cap_rows(
    rows: &[WorthQueryGraphObligationResidueRow],
) -> Result<(), WorthQueryGraphObligationConsumerKitError> {
    let mut classes = BTreeSet::new();
    for row in rows {
        if !classes.insert(row.class.as_str()) {
            return Err(WorthQueryGraphObligationConsumerKitError::new(
                WorthQueryGraphObligationConsumerKitErrorKind::DuplicateResidueClass,
                format!(
                    "graph obligation residue class `{}` is declared twice",
                    row.class
                ),
            ));
        }
        if row.current_count > row.must_not_exceed_count {
            return Err(WorthQueryGraphObligationConsumerKitError::new(
                WorthQueryGraphObligationConsumerKitErrorKind::ResidueCapExceeded,
                format!(
                    "graph obligation residue class `{}` has {} rows over cap {}",
                    row.class, row.current_count, row.must_not_exceed_count
                ),
            ));
        }
    }
    Ok(())
}

fn required_residue_text(
    value: String,
    field: &'static str,
) -> Result<String, WorthQueryGraphObligationConsumerKitError> {
    let value = value.trim().to_string();
    if value.is_empty() {
        return Err(WorthQueryGraphObligationConsumerKitError::new(
            WorthQueryGraphObligationConsumerKitErrorKind::IncompleteResidueRow,
            format!("graph obligation residue field `{field}` must not be blank"),
        ));
    }
    Ok(value)
}
