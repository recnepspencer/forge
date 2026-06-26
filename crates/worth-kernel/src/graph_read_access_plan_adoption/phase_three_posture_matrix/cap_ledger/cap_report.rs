use super::super::errors::{
    WorthGraphReadAccessPostureMatrixError, WorthGraphReadAccessPostureMatrixErrorKind,
};
use super::super::posture_resolution::WorthGraphReadRequirementPostureMap;
use super::super::stable_digest;
use super::cap_family_counter::{count_posture_families, WorthGraphReadAccessPostureFamilyCount};
use super::cap_ledger::WorthGraphReadAccessPostureCapLedger;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthGraphReadAccessPostureCapReport {
    ledger: WorthGraphReadAccessPostureCapLedger,
    observed_family_counts: Vec<WorthGraphReadAccessPostureFamilyCount>,
    observed_family_count: usize,
    uncapped_posture_family_count: usize,
    cap_exceeded_family_count: usize,
    report_digest: String,
}

impl WorthGraphReadAccessPostureCapReport {
    pub(crate) fn from_posture_map(
        posture_map: &WorthGraphReadRequirementPostureMap,
    ) -> Result<Self, WorthGraphReadAccessPostureMatrixError> {
        Self::from_posture_map_and_ledger(
            posture_map,
            WorthGraphReadAccessPostureCapLedger::current(),
        )
    }

    #[cfg(test)]
    pub(crate) fn from_posture_map_and_ledger_for_tests(
        posture_map: &WorthGraphReadRequirementPostureMap,
        ledger: WorthGraphReadAccessPostureCapLedger,
    ) -> Result<Self, WorthGraphReadAccessPostureMatrixError> {
        Self::from_posture_map_and_ledger(posture_map, ledger)
    }

    fn from_posture_map_and_ledger(
        posture_map: &WorthGraphReadRequirementPostureMap,
        ledger: WorthGraphReadAccessPostureCapLedger,
    ) -> Result<Self, WorthGraphReadAccessPostureMatrixError> {
        let family_counts = count_posture_families(posture_map.resolved_postures());
        let mut observed_family_counts = Vec::new();
        for (family, count) in &family_counts {
            let Some(cap_row) = ledger.row_for_family(family) else {
                return Err(WorthGraphReadAccessPostureMatrixError::for_posture_family(
                    WorthGraphReadAccessPostureMatrixErrorKind::UncappedPostureFamily,
                    family.clone(),
                    *count,
                    None,
                ));
            };
            if *count > cap_row.max_count() {
                return Err(WorthGraphReadAccessPostureMatrixError::for_posture_family(
                    WorthGraphReadAccessPostureMatrixErrorKind::PostureFamilyCapExceeded,
                    family.clone(),
                    *count,
                    Some(cap_row.max_count()),
                ));
            }
            observed_family_counts.push(WorthGraphReadAccessPostureFamilyCount::new(
                family.clone(),
                *count,
                cap_row.max_count(),
            ));
        }

        let mut digest_parts = vec![
            "worth_graph_read_access_posture_cap_report_v1".to_string(),
            format!("ledger:{}", ledger.ledger_digest()),
            format!("observed_family_count:{}", family_counts.len()),
            "uncapped_posture_family_count:0".to_string(),
            "cap_exceeded_family_count:0".to_string(),
        ];
        digest_parts.extend(
            observed_family_counts
                .iter()
                .map(|row| format!("family_count:{}", row.row_digest())),
        );

        Ok(Self {
            ledger,
            observed_family_counts,
            observed_family_count: family_counts.len(),
            uncapped_posture_family_count: 0,
            cap_exceeded_family_count: 0,
            report_digest: stable_digest(&digest_parts),
        })
    }

    pub const fn ledger(&self) -> &WorthGraphReadAccessPostureCapLedger {
        &self.ledger
    }

    pub fn observed_family_counts(&self) -> &[WorthGraphReadAccessPostureFamilyCount] {
        &self.observed_family_counts
    }

    pub const fn observed_family_count(&self) -> usize {
        self.observed_family_count
    }

    pub const fn uncapped_posture_family_count(&self) -> usize {
        self.uncapped_posture_family_count
    }

    pub const fn cap_exceeded_family_count(&self) -> usize {
        self.cap_exceeded_family_count
    }

    pub fn report_digest(&self) -> &str {
        &self.report_digest
    }
}
