use std::collections::{BTreeMap, BTreeSet};

const HEADER: &str = "path,match_families,deletion_phase,replacement_owner,absence_gate,status,disposition,disposition_basis";

pub(super) fn parse_removal_ledger(document: &str) -> Result<BTreeMap<String, RemovalRow>, String> {
    let mut lines = document.lines();
    if lines.next() != Some(HEADER) {
        return Err("removal ledger has an invalid schema header".to_owned());
    }

    let mut rows = BTreeMap::new();
    for (index, line) in lines.enumerate() {
        if line.trim().is_empty() {
            continue;
        }
        let columns = line.split(',').map(str::trim).collect::<Vec<_>>();
        if columns.len() != 8 {
            return Err(format!(
                "removal ledger row {} has {} columns, expected 8",
                index + 2,
                columns.len()
            ));
        }
        if columns.iter().any(|column| column.is_empty()) {
            return Err(format!(
                "removal ledger row {} has an empty required field",
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
            deletion_phase: columns[2].to_owned(),
            replacement_owner: columns[3].to_owned(),
            absence_gate: columns[4].to_owned(),
            status: RemovalStatus::parse(columns[5])?,
            disposition: RemovalDisposition::parse(columns[6])?,
            disposition_basis: columns[7].to_owned(),
        };
        if rows.insert(path.clone(), row).is_some() {
            return Err(format!("duplicate removal ledger row for {path}"));
        }
    }
    Ok(rows)
}

pub(super) struct RemovalRow {
    pub(super) path: String,
    pub(super) families: BTreeSet<String>,
    pub(super) deletion_phase: String,
    pub(super) replacement_owner: String,
    pub(super) absence_gate: String,
    pub(super) status: RemovalStatus,
    pub(super) disposition: RemovalDisposition,
    pub(super) disposition_basis: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum RemovalDisposition {
    Preserve,
    Narrow,
    Delete,
}

impl RemovalDisposition {
    fn parse(value: &str) -> Result<Self, String> {
        match value {
            "preserve" => Ok(Self::Preserve),
            "narrow" => Ok(Self::Narrow),
            "delete" => Ok(Self::Delete),
            _ => Err(format!("invalid removal disposition {value}")),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum RemovalStatus {
    InventoryOpen,
    Deleted(String),
}

impl RemovalStatus {
    fn parse(value: &str) -> Result<Self, String> {
        if value == "inventory-open" {
            return Ok(Self::InventoryOpen);
        }
        if let Some(phase) = value.strip_prefix("deleted-") {
            if matches!(
                phase,
                "phase-3" | "phase-5" | "phase-6" | "phase-7" | "phase-8"
            ) {
                return Ok(Self::Deleted(phase.to_owned()));
            }
        }
        Err(format!("invalid removal status {value}"))
    }
}

#[cfg(test)]
mod tests {
    use super::{parse_removal_ledger, RemovalDisposition};

    const VALID: &str = concat!(
        "path,match_families,deletion_phase,replacement_owner,absence_gate,status,",
        "disposition,disposition_basis\n",
        "crates/example.rs,c6-identifier,phase-8,workspace:Cargo.toml,",
        "source-and-metadata-absence,inventory-open,narrow,Retain canonical file\n",
    );

    #[test]
    fn parser_requires_the_disposition_schema_and_basis() {
        assert!(parse_removal_ledger(&VALID.replace(",narrow,", ",unknown,")).is_err());
        assert!(
            parse_removal_ledger(&VALID.replace(",narrow,Retain canonical file", ",narrow,"))
                .is_err()
        );
        assert!(parse_removal_ledger(&VALID.replacen("disposition,", "", 1)).is_err());
    }

    #[test]
    fn parser_types_every_valid_disposition() {
        for expected in [
            ("preserve", RemovalDisposition::Preserve),
            ("narrow", RemovalDisposition::Narrow),
            ("delete", RemovalDisposition::Delete),
        ] {
            let document = VALID.replace(",narrow,", &format!(",{},", expected.0));
            let rows = parse_removal_ledger(&document).expect("parse controlled ledger");
            assert_eq!(rows["crates/example.rs"].disposition, expected.1);
        }
    }
}
