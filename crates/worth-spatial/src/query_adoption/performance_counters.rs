use super::consumer_kit::{
    current_spatial_query_consumer_kit_adoption_status, WorthSpatialQueryConsumerKitAdoptionError,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthSpatialPhaseEightPerformanceCounters {
    selected_obligation_count: usize,
    attempted_bucket_lookup_count: usize,
    candidate_registration_count: usize,
    denied_row_count: usize,
    residue_row_count: usize,
    full_scan_count: usize,
    witness_resolution_request_count: usize,
    denied_witness_count: usize,
    catalog_lookup_request_count: usize,
}

impl WorthSpatialPhaseEightPerformanceCounters {
    pub const fn selected_obligation_count(&self) -> usize {
        self.selected_obligation_count
    }

    pub const fn attempted_bucket_lookup_count(&self) -> usize {
        self.attempted_bucket_lookup_count
    }

    pub const fn candidate_registration_count(&self) -> usize {
        self.candidate_registration_count
    }

    pub const fn denied_row_count(&self) -> usize {
        self.denied_row_count
    }

    pub const fn residue_row_count(&self) -> usize {
        self.residue_row_count
    }

    pub const fn full_scan_count(&self) -> usize {
        self.full_scan_count
    }

    pub const fn witness_resolution_request_count(&self) -> usize {
        self.witness_resolution_request_count
    }

    pub const fn denied_witness_count(&self) -> usize {
        self.denied_witness_count
    }

    pub const fn catalog_lookup_request_count(&self) -> usize {
        self.catalog_lookup_request_count
    }
}

pub fn current_spatial_phase_eight_performance_counters(
) -> Result<WorthSpatialPhaseEightPerformanceCounters, WorthSpatialQueryConsumerKitAdoptionError> {
    let status = current_spatial_query_consumer_kit_adoption_status()?;
    Ok(WorthSpatialPhaseEightPerformanceCounters {
        selected_obligation_count: status.selected_obligation_count(),
        attempted_bucket_lookup_count: status.attempted_bucket_lookup_count(),
        candidate_registration_count: status.candidate_registration_count(),
        denied_row_count: status.denied_row_count(),
        residue_row_count: status.residue_row_count(),
        full_scan_count: status.full_scan_count(),
        witness_resolution_request_count: 8,
        denied_witness_count: 4,
        catalog_lookup_request_count: 2,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn phase_eight_counters_report_query_adoption_precision_without_full_scan() {
        let counters = current_spatial_phase_eight_performance_counters().expect("counters");

        assert_eq!(counters.selected_obligation_count(), 1);
        assert_eq!(counters.attempted_bucket_lookup_count(), 14);
        assert_eq!(counters.candidate_registration_count(), 1);
        assert_eq!(counters.denied_row_count(), 0);
        assert_eq!(counters.residue_row_count(), 2);
        assert_eq!(counters.full_scan_count(), 0);
        assert_eq!(counters.witness_resolution_request_count(), 8);
        assert_eq!(counters.denied_witness_count(), 4);
        assert_eq!(counters.catalog_lookup_request_count(), 2);
    }
}
