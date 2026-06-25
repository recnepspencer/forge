use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WireViewOldAuthorityResidueRow {
    caller: String,
    owner: String,
    blocker: String,
    removal_trigger: String,
    row_digest: String,
}

impl WireViewOldAuthorityResidueRow {
    pub fn caller(&self) -> &str {
        &self.caller
    }

    pub fn owner(&self) -> &str {
        &self.owner
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
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WireViewOldAuthorityResidue {
    capped_direct_interpreter_count: usize,
    capped_rows: Vec<WireViewOldAuthorityResidueRow>,
    residue_digest: String,
}

impl WireViewOldAuthorityResidue {
    pub fn current_source_scan() -> Self {
        Self::new(Vec::new())
    }

    pub fn required_capped_callers() -> &'static [&'static str] {
        &[]
    }

    #[cfg(test)]
    pub(crate) fn uncapped_for_tests() -> Self {
        Self::new(Vec::new())
    }

    fn new(capped_rows: Vec<WireViewOldAuthorityResidueRow>) -> Self {
        let capped_direct_interpreter_count = capped_rows.len();
        let mut parts = vec![
            "worth-topo:wire-view-old-authority-residue:v1".to_string(),
            format!("capped-count:{capped_direct_interpreter_count}"),
        ];
        parts.extend(
            capped_rows
                .iter()
                .map(|row| format!("row:{}", row.row_digest())),
        );
        let residue_digest = super::super::super::catalog::catalog_digest(parts);
        Self {
            capped_direct_interpreter_count,
            capped_rows,
            residue_digest,
        }
    }

    pub const fn capped_direct_interpreter_count(&self) -> usize {
        self.capped_direct_interpreter_count
    }

    pub fn capped_rows(&self) -> &[WireViewOldAuthorityResidueRow] {
        &self.capped_rows
    }

    pub fn contains_required_caps(&self) -> bool {
        Self::required_capped_callers()
            .iter()
            .all(|required| self.capped_rows.iter().any(|row| row.caller() == *required))
    }

    pub fn residue_digest(&self) -> &str {
        &self.residue_digest
    }
}
