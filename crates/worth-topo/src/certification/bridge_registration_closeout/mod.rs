use crate::certification::error::TopologyCertificationError;
use crate::certification::shared::digest_rows;
use crate::certification::DeterministicDigest;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TopologyBridgeRegistrationArea {
    PublicEntry,
    RuntimeInfrastructure,
    CertificationBridgeProof,
}

impl TopologyBridgeRegistrationArea {
    pub const ALL: [Self; 3] = [
        Self::PublicEntry,
        Self::RuntimeInfrastructure,
        Self::CertificationBridgeProof,
    ];

    pub fn as_str(self) -> &'static str {
        match self {
            Self::PublicEntry => "public_entry",
            Self::RuntimeInfrastructure => "runtime_infrastructure",
            Self::CertificationBridgeProof => "certification_bridge_proof",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TopologyBridgeRegistrationStatus {
    Closed,
    Gap,
}

impl TopologyBridgeRegistrationStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Closed => "closed",
            Self::Gap => "gap",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TopologyBridgeRegistrationRow {
    area: TopologyBridgeRegistrationArea,
    status: TopologyBridgeRegistrationStatus,
    reason: String,
    designated_survivor: String,
    row_digest: DeterministicDigest,
}

impl TopologyBridgeRegistrationRow {
    pub fn area(&self) -> TopologyBridgeRegistrationArea {
        self.area
    }

    pub fn status(&self) -> TopologyBridgeRegistrationStatus {
        self.status
    }

    pub fn reason(&self) -> &str {
        &self.reason
    }

    pub fn designated_survivor(&self) -> &str {
        &self.designated_survivor
    }

    pub fn row_digest(&self) -> &DeterministicDigest {
        &self.row_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TopologyBridgeRegistrationCloseoutReport {
    rows: Vec<TopologyBridgeRegistrationRow>,
    phase_eight_ready: bool,
    closeout_digest: DeterministicDigest,
}

impl TopologyBridgeRegistrationCloseoutReport {
    pub fn rows(&self) -> &[TopologyBridgeRegistrationRow] {
        &self.rows
    }

    pub fn phase_eight_ready(&self) -> bool {
        self.phase_eight_ready
    }

    pub fn status(&self, area: TopologyBridgeRegistrationArea) -> TopologyBridgeRegistrationStatus {
        self.rows
            .iter()
            .find(|row| row.area() == area)
            .map(TopologyBridgeRegistrationRow::status)
            .unwrap_or_else(|| panic!("bridge registration closeout rows should cover every area"))
    }

    pub fn closeout_digest(&self) -> &DeterministicDigest {
        &self.closeout_digest
    }
}

pub fn certify_topology_bridge_registration_closeout(
) -> Result<TopologyBridgeRegistrationCloseoutReport, TopologyCertificationError> {
    let rows = vec![
        certify_public_entry_row()?,
        certify_runtime_infrastructure_row()?,
        certify_certification_bridge_proof_row()?,
    ];
    let phase_eight_ready = rows
        .iter()
        .all(|row| row.status() == TopologyBridgeRegistrationStatus::Closed);
    let closeout_digest = digest_rows(rows.iter().map(|row| {
        format!(
            "{}:{}:{}:{}:{}",
            row.area().as_str(),
            row.status().as_str(),
            row.reason(),
            row.designated_survivor(),
            row.row_digest().digest_hex
        )
    }));
    let report = TopologyBridgeRegistrationCloseoutReport {
        rows,
        phase_eight_ready,
        closeout_digest,
    };
    if !report.phase_eight_ready() {
        return Err(TopologyCertificationError::ReadView(
            "worth-topo bridge registration closeout contains gap rows".to_string(),
        ));
    }
    Ok(report)
}

fn certify_public_entry_row() -> Result<TopologyBridgeRegistrationRow, TopologyCertificationError> {
    let facade = source_text("src/facade.rs")?;
    let runtime_support = source_text("src/runtime_support.rs")?;
    let compile_fail_contracts =
        source_text("src/certification/public_facade_contracts/compile_fail_contracts.rs")?;

    ensure(!facade.contains("build_milestone_one_bridge"))?;
    ensure(!facade.contains("milestone_one_bridge_mapping_registrations"))?;
    ensure(!facade.contains("milestone_one_bridge_aspect_registrations"))?;
    ensure(!runtime_support.contains("build_runtime_bridge"))?;
    ensure(!runtime_support.contains("TopologyRuntimeBinding"))?;
    ensure(compile_fail_contracts.contains("public_bridge_registration_entry_not_exported.rs"))?;

    closed_row(
        TopologyBridgeRegistrationArea::PublicEntry,
        "topology-facing public entry no longer teaches bridge builders or bridge registration packs; bridge wiring is hidden behind query-domain entry and runtime support instead of competing as its own entry story",
        "src/query_domain.rs",
    )
}

fn certify_runtime_infrastructure_row(
) -> Result<TopologyBridgeRegistrationRow, TopologyCertificationError> {
    let bridge_mod = source_text("src/projection/runtime_boundary/bridge/mod.rs")?;
    let bridge_mappings = source_text("src/projection/runtime_boundary/bridge/mappings.rs")?;
    let query_runtime_mod = source_text("src/projection/runtime_boundary/query_runtime/mod.rs")?;
    let query_runtime_adapters =
        source_text("src/projection/runtime_boundary/query_runtime/adapters.rs")?;

    ensure(!bridge_mod.contains("pub fn build_milestone_one_bridge("))?;
    ensure(bridge_mappings.contains("pub(crate) fn milestone_one_bridge_mapping_registrations("))?;
    ensure(bridge_mappings.contains("pub(crate) fn milestone_one_bridge_aspect_registrations("))?;
    ensure(!query_runtime_mod.contains("pub use self::adapters::build_runtime_bridge"))?;
    ensure(!query_runtime_mod.contains("pub(crate) use self::adapters::{build_runtime_bridge"))?;
    ensure(!query_runtime_mod.contains("pub use self::adapters::TopologyRuntimeBinding"))?;
    ensure(!query_runtime_mod.contains("pub(crate) use self::adapters::TopologyRuntimeBinding"))?;
    ensure(
        query_runtime_adapters.contains("milestone_one_bridge_mapping_registrations().into_iter()"),
    )?;
    ensure(query_runtime_adapters.contains("milestone_one_bridge_aspect_registrations()"))?;

    closed_row(
        TopologyBridgeRegistrationArea::RuntimeInfrastructure,
        "the surviving bridge machinery is now internal runtime-adapter infrastructure: crate-local bridge registration packs plus one crate-local runtime bridge builder defined below the public query-runtime module instead of a topology-facing bridge-builder API",
        "src/projection/runtime_boundary/query_runtime/adapters.rs",
    )
}

fn certify_certification_bridge_proof_row(
) -> Result<TopologyBridgeRegistrationRow, TopologyCertificationError> {
    let certification_bridge = source_text("src/certification/bridge.rs")?;
    let bridge_tests = source_text("src/projection/runtime_boundary/bridge/tests.rs")?;

    ensure(
        certification_bridge.contains(
            "use crate::projection::runtime_boundary::bridge::build_milestone_one_bridge;",
        ),
    )?;
    ensure(!certification_bridge.contains("use crate::facade::build_milestone_one_bridge;"))?;
    ensure(
        bridge_tests.contains("build_milestone_one_bridge(Arc::clone(&runtime), RecordingSink)"),
    )?;

    closed_row(
        TopologyBridgeRegistrationArea::CertificationBridgeProof,
        "bridge proof and bridge regression tests now enter through the crate-local bridge boundary instead of re-teaching the deleted public bridge-entry lane",
        "src/certification/bridge.rs",
    )
}

fn source_text(path: &str) -> Result<String, TopologyCertificationError> {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(path);
    std::fs::read_to_string(&path)
        .map_err(|error| TopologyCertificationError::ReadView(format!("{path:?}: {error}")))
}

fn ensure(condition: bool) -> Result<(), TopologyCertificationError> {
    if condition {
        Ok(())
    } else {
        Err(TopologyCertificationError::ReadView(
            "bridge registration closeout assertion failed".to_string(),
        ))
    }
}

fn closed_row(
    area: TopologyBridgeRegistrationArea,
    reason: impl Into<String>,
    designated_survivor: impl Into<String>,
) -> Result<TopologyBridgeRegistrationRow, TopologyCertificationError> {
    let reason = reason.into();
    let designated_survivor = designated_survivor.into();
    let row_digest = digest_rows(
        vec![format!(
            "{}:{}:{}",
            area.as_str(),
            reason,
            designated_survivor
        )]
        .into_iter(),
    );
    Ok(TopologyBridgeRegistrationRow {
        area,
        status: TopologyBridgeRegistrationStatus::Closed,
        reason,
        designated_survivor,
        row_digest,
    })
}

#[cfg(test)]
mod tests;
