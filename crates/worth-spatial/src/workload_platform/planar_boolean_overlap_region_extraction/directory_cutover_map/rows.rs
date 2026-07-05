use super::scan::{
    certify_deleted_legacy_surface_contracts, certify_overlap_lane_relative_path,
    certify_phase_two_artifact_contracts, scan_live_phase_two_family,
    PlanarBooleanOverlapRegionExtractionPathDenial,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PlanarBooleanOverlapRegionExtractionArtifactOwnerRow {
    artifact_name: &'static str,
    owning_folder: &'static str,
    owning_source_path: &'static str,
    consuming_phase: u8,
    consuming_source_path: &'static str,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PlanarBooleanOverlapRegionExtractionLegacySurfaceRow {
    surface_name: &'static str,
    source_path: &'static str,
    disposition: &'static str,
    must_be_absent: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanOverlapRegionExtractionDirectoryCutoverMap {
    artifact_rows: Vec<PlanarBooleanOverlapRegionExtractionArtifactOwnerRow>,
    legacy_rows: Vec<PlanarBooleanOverlapRegionExtractionLegacySurfaceRow>,
}

impl PlanarBooleanOverlapRegionExtractionDirectoryCutoverMap {
    pub fn phase_2() -> Self {
        Self {
            artifact_rows: artifact_rows(),
            legacy_rows: legacy_rows(),
        }
    }

    pub fn artifact_rows(&self) -> &[PlanarBooleanOverlapRegionExtractionArtifactOwnerRow] {
        &self.artifact_rows
    }

    pub fn legacy_rows(&self) -> &[PlanarBooleanOverlapRegionExtractionLegacySurfaceRow] {
        &self.legacy_rows
    }

    pub fn certifies_one_owner_per_artifact(&self) -> bool {
        let mut seen = std::collections::BTreeSet::new();
        self.artifact_rows
            .iter()
            .all(|row| seen.insert(row.artifact_name()))
    }

    pub fn certifies_one_consuming_phase_per_artifact(&self) -> bool {
        self.artifact_rows
            .iter()
            .all(|row| (1..=15).contains(&row.consuming_phase()))
    }

    pub fn certifies_live_artifact_contracts(
        &self,
    ) -> Result<(), PlanarBooleanOverlapRegionExtractionPathDenial> {
        certify_phase_two_artifact_contracts(&self.artifact_rows)
    }

    pub fn certifies_legacy_surface_contracts(
        &self,
    ) -> Result<(), PlanarBooleanOverlapRegionExtractionPathDenial> {
        certify_deleted_legacy_surface_contracts(&self.legacy_rows)
    }

    pub fn certify_overlap_lane_path(
        &self,
        relative_path: &str,
    ) -> Result<(), PlanarBooleanOverlapRegionExtractionPathDenial> {
        certify_overlap_lane_relative_path(relative_path)
    }

    pub fn certify_live_overlap_lane(
        &self,
    ) -> Result<(), PlanarBooleanOverlapRegionExtractionPathDenial> {
        scan_live_phase_two_family()
    }
}

impl PlanarBooleanOverlapRegionExtractionArtifactOwnerRow {
    const fn new(
        artifact_name: &'static str,
        owning_folder: &'static str,
        owning_source_path: &'static str,
        consuming_phase: u8,
        consuming_source_path: &'static str,
    ) -> Self {
        Self {
            artifact_name,
            owning_folder,
            owning_source_path,
            consuming_phase,
            consuming_source_path,
        }
    }

    pub fn artifact_name(&self) -> &'static str {
        self.artifact_name
    }

    pub fn owning_folder(&self) -> &'static str {
        self.owning_folder
    }

    pub fn consuming_phase(&self) -> u8 {
        self.consuming_phase
    }

    pub fn owning_source_path(&self) -> &'static str {
        self.owning_source_path
    }

    pub fn consuming_source_path(&self) -> &'static str {
        self.consuming_source_path
    }
}

impl PlanarBooleanOverlapRegionExtractionLegacySurfaceRow {
    const fn new(
        surface_name: &'static str,
        source_path: &'static str,
        disposition: &'static str,
        must_be_absent: bool,
    ) -> Self {
        Self {
            surface_name,
            source_path,
            disposition,
            must_be_absent,
        }
    }

    pub fn surface_name(&self) -> &'static str {
        self.surface_name
    }

    pub fn source_path(&self) -> &'static str {
        self.source_path
    }

    pub fn disposition(&self) -> &'static str {
        self.disposition
    }

    pub fn must_be_absent(&self) -> bool {
        self.must_be_absent
    }
}

fn artifact_rows() -> Vec<PlanarBooleanOverlapRegionExtractionArtifactOwnerRow> {
    vec![
        PlanarBooleanOverlapRegionExtractionArtifactOwnerRow::new(
            "CoplanarOverlapWorkloadOperator",
            "legacy_operator_surface",
            "crates/worth-spatial/src/workload_platform/planar_boolean_overlap_region_extraction/legacy_operator_surface/operator.rs",
            2,
            "crates/worth-kernel/src/workload_composition/operator_harness/coplanar_overlap_execution.rs",
        ),
        PlanarBooleanOverlapRegionExtractionArtifactOwnerRow::new(
            "CoplanarOverlapOperatorReceipt",
            "legacy_operator_surface",
            "crates/worth-spatial/src/workload_platform/planar_boolean_overlap_region_extraction/legacy_operator_surface/receipt.rs",
            2,
            "crates/worth-spatial/src/certification/public_facade_contracts/contracts/planar_overlap/metaboss/platform_storm_subject.rs",
        ),
        PlanarBooleanOverlapRegionExtractionArtifactOwnerRow::new(
            "PlanarBooleanOverlapRegionExtractionDirectoryCutoverMap",
            "directory_cutover_map",
            "crates/worth-spatial/src/workload_platform/planar_boolean_overlap_region_extraction/directory_cutover_map/rows.rs",
            2,
            "crates/worth-kernel/src/workload_composition/planar_boolean_overlap_region_extraction/registration_contract.rs",
        ),
        PlanarBooleanOverlapRegionExtractionArtifactOwnerRow::new(
            "PlanarBooleanOverlapBlueprintRegistry",
            "topology_operators/overlap_region_blueprint",
            "crates/worth-topo/src/topology_operators/overlap_region_blueprint/registry.rs",
            2,
            "crates/worth-topo/tests/overlap_region_blueprint_contract.rs",
        ),
        PlanarBooleanOverlapRegionExtractionArtifactOwnerRow::new(
            "PlanarBooleanOverlapOperatorClassificationMatrix",
            "topology_operators/overlap_region_blueprint",
            "crates/worth-topo/src/topology_operators/overlap_region_blueprint/registry.rs",
            2,
            "crates/worth-kernel/src/workload_composition/planar_boolean_overlap_region_extraction/registration_contract.rs",
        ),
        PlanarBooleanOverlapRegionExtractionArtifactOwnerRow::new(
            "PlanarBooleanOverlapValidatorRegistrationPlan",
            "topology_operators/overlap_region_blueprint",
            "crates/worth-topo/src/topology_operators/overlap_region_blueprint/registry.rs",
            2,
            "crates/worth-kernel/src/workload_composition/planar_boolean_overlap_region_extraction/registration_contract.rs",
        ),
        PlanarBooleanOverlapRegionExtractionArtifactOwnerRow::new(
            "PlanarBooleanOverlapRegistrationContract",
            "workload_composition/planar_boolean_overlap_region_extraction",
            "crates/worth-kernel/src/workload_composition/planar_boolean_overlap_region_extraction/registration_contract.rs",
            2,
            "crates/worth-kernel/src/workload_composition/planar_boolean_overlap_region_extraction/mod.rs",
        ),
    ]
}

fn legacy_rows() -> Vec<PlanarBooleanOverlapRegionExtractionLegacySurfaceRow> {
    vec![
        PlanarBooleanOverlapRegionExtractionLegacySurfaceRow::new(
            "CoplanarOverlapWorkloadOperator wrapper",
            "crates/worth-spatial/src/workload_platform/workload_operators/coplanar_overlap.rs",
            "deleted internal wrapper; ordinary callers must use the overlap-region owner directly",
            true,
        ),
        PlanarBooleanOverlapRegionExtractionLegacySurfaceRow::new(
            "CoplanarOverlapOperatorReceipt wrapper",
            "crates/worth-spatial/src/workload_platform/workload_operators/coplanar_overlap_receipt.rs",
            "deleted internal wrapper; ordinary callers must use the overlap-region owner directly",
            true,
        ),
        PlanarBooleanOverlapRegionExtractionLegacySurfaceRow::new(
            "CoplanarOverlapWorkloadOperator facade wrapper",
            "crates/worth-spatial/src/facade/workload_operators/mod.rs",
            "deleted public wrapper; ordinary callers must use the overlap-region facade",
            true,
        ),
        PlanarBooleanOverlapRegionExtractionLegacySurfaceRow::new(
            "CoplanarOverlapStorm",
            "crates/worth-spatial/src/workload_platform/coplanar_overlap_storm",
            "displaced storm surface outside the 7.5 owning lane",
            false,
        ),
    ]
}
