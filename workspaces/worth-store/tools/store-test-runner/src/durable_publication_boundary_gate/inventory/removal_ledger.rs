use std::collections::{BTreeMap, BTreeSet};

pub(super) const REMOVAL_LEDGER: &str =
    "_docs/worth-store/physical-reconstruction-c7-removal-ledger.csv";

const HEADER: &str = concat!(
    "path,match_families,match_counts,responsibility,destination_owner,disposition,",
    "last_consumer,deletion_phase,absence_gate,status"
);

pub(super) fn parse_removal_ledger(document: &str) -> Result<BTreeMap<String, RemovalRow>, String> {
    let mut lines = document.lines();
    if lines.next() != Some(HEADER) {
        return Err("C.7 removal ledger has an invalid schema header".to_owned());
    }
    let mut rows = BTreeMap::new();
    for (index, line) in lines.enumerate() {
        if line.trim().is_empty() {
            continue;
        }
        let columns = line.split(',').map(str::trim).collect::<Vec<_>>();
        if columns.len() != 10 {
            return Err(format!(
                "C.7 removal row {} has {} columns, expected 10",
                index + 2,
                columns.len()
            ));
        }
        if columns.iter().any(|column| column.is_empty()) {
            return Err(format!(
                "C.7 removal row {} has an empty required field",
                index + 2
            ));
        }
        let path = columns[0].to_owned();
        let row = RemovalRow {
            path: path.clone(),
            families: columns[1]
                .split(';')
                .map(str::to_owned)
                .collect::<BTreeSet<_>>(),
            match_counts: parse_match_counts(columns[2], index + 2)?,
            responsibility: columns[3].to_owned(),
            destination_owner: columns[4].to_owned(),
            disposition: Disposition::parse(columns[5])?,
            last_consumer: columns[6].to_owned(),
            deletion_phase: DeletionPhase::parse(columns[7])?,
            absence_gate: columns[8].to_owned(),
            status: RemovalStatus::parse(columns[9])?,
        };
        if rows.insert(path.clone(), row).is_some() {
            return Err(format!("duplicate C.7 removal row for {path}"));
        }
    }
    Ok(rows)
}

fn parse_match_counts(value: &str, row: usize) -> Result<BTreeMap<String, usize>, String> {
    value
        .split(';')
        .map(|entry| {
            let (family, count) = entry
                .split_once('=')
                .ok_or_else(|| format!("invalid match count at C.7 removal row {row}"))?;
            let count = count
                .parse()
                .map_err(|_| format!("invalid match count at C.7 removal row {row}"))?;
            if family.is_empty() || count == 0 {
                return Err(format!(
                    "empty or zero match count at C.7 removal row {row}"
                ));
            }
            Ok((family.to_owned(), count))
        })
        .collect()
}

pub(super) struct RemovalRow {
    pub(super) path: String,
    pub(super) families: BTreeSet<String>,
    pub(super) match_counts: BTreeMap<String, usize>,
    pub(super) responsibility: String,
    pub(super) destination_owner: String,
    pub(super) disposition: Disposition,
    pub(super) last_consumer: String,
    pub(super) deletion_phase: DeletionPhase,
    pub(super) absence_gate: String,
    pub(super) status: RemovalStatus,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum Disposition {
    Preserve,
    Narrow,
    Move,
    Replace,
    Delete,
}

impl Disposition {
    fn parse(value: &str) -> Result<Self, String> {
        match value {
            "preserve" => Ok(Self::Preserve),
            "narrow" => Ok(Self::Narrow),
            "move" => Ok(Self::Move),
            "replace" => Ok(Self::Replace),
            "delete" => Ok(Self::Delete),
            _ => Err(format!("invalid C.7 removal disposition `{value}`")),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum DeletionPhase {
    Phase3,
    Phase4,
    Phase5,
    Phase6,
    Phase7,
    Phase8,
    Phase9,
    Phase10,
    Preserve,
}

impl DeletionPhase {
    fn parse(value: &str) -> Result<Self, String> {
        match value {
            "phase-3" => Ok(Self::Phase3),
            "phase-4" => Ok(Self::Phase4),
            "phase-5" => Ok(Self::Phase5),
            "phase-6" => Ok(Self::Phase6),
            "phase-7" => Ok(Self::Phase7),
            "phase-8" => Ok(Self::Phase8),
            "phase-9" => Ok(Self::Phase9),
            "phase-10" => Ok(Self::Phase10),
            "preserve" => Ok(Self::Preserve),
            _ => Err(format!("invalid C.7 deletion phase `{value}`")),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) enum RemovalStatus {
    InventoryOpen,
    Deleted(DeletionPhase),
}

impl RemovalStatus {
    fn parse(value: &str) -> Result<Self, String> {
        if value == "inventory-open" {
            return Ok(Self::InventoryOpen);
        }
        value
            .strip_prefix("deleted-")
            .ok_or_else(|| format!("invalid C.7 removal status `{value}`"))
            .and_then(DeletionPhase::parse)
            .map(Self::Deleted)
    }
}

#[cfg(test)]
mod tests {
    use super::{parse_removal_ledger, HEADER};

    #[test]
    fn parser_rejects_missing_fields_duplicate_paths_and_unknown_dispositions() {
        let valid = format!(
            "{HEADER}\ncrates/a.rs,page-lsn,page-lsn=1,recovery-page-ordering,recovery,preserve,a.rs,preserve,source-inventory,inventory-open\n"
        );
        assert!(parse_removal_ledger(&valid).is_ok());
        assert!(parse_removal_ledger(&valid.replace(",preserve,", ",unknown,")).is_err());
        assert!(parse_removal_ledger(&valid.replace(",a.rs,", ",,")).is_err());
        assert!(parse_removal_ledger(&valid.replace("page-lsn=1", "page-lsn=0")).is_err());
        assert!(parse_removal_ledger(&(valid.clone() + valid.lines().nth(1).unwrap())).is_err());
    }
}
