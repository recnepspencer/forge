use crate::identity::hash_parts;

use super::fixtures::{
    certified_admitted_intent_fixture, certified_advisory_intent_fixture,
    certified_violation_intent_fixture,
};
use super::fixtures::{certified_deferred_intent_fixture, certified_unsupported_intent_fixture};

mod comparison;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryIntentAdmissionOracleLane {
    AdmittedControl,
    AdvisoryControl,
    ViolationControl,
    DeferredControl,
    UnsupportedControl,
}

impl ForgeQueryIntentAdmissionOracleLane {
    fn as_str(self) -> &'static str {
        match self {
            Self::AdmittedControl => "admitted_control",
            Self::AdvisoryControl => "advisory_control",
            Self::ViolationControl => "violation_control",
            Self::DeferredControl => "deferred_control",
            Self::UnsupportedControl => "unsupported_control",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryIntentAdmissionOracleManifestRow {
    lane: ForgeQueryIntentAdmissionOracleLane,
    row_digest: String,
}

impl ForgeQueryIntentAdmissionOracleManifestRow {
    pub fn lane(&self) -> ForgeQueryIntentAdmissionOracleLane {
        self.lane
    }

    pub fn row_digest(&self) -> &str {
        &self.row_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryIntentAdmissionOracleComparisonRow {
    lane: ForgeQueryIntentAdmissionOracleLane,
    expected_detail: String,
    expected_digest: String,
    actual_detail: String,
    actual_digest: String,
    row_digest: String,
}

impl ForgeQueryIntentAdmissionOracleComparisonRow {
    pub fn lane(&self) -> ForgeQueryIntentAdmissionOracleLane {
        self.lane
    }

    pub fn expected_digest(&self) -> &str {
        &self.expected_digest
    }

    pub fn expected_detail(&self) -> &str {
        &self.expected_detail
    }

    pub fn actual_detail(&self) -> &str {
        &self.actual_detail
    }

    pub fn actual_digest(&self) -> &str {
        &self.actual_digest
    }

    pub fn row_digest(&self) -> &str {
        &self.row_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryIntentAdmissionOracleReport {
    manifest_rows: Vec<ForgeQueryIntentAdmissionOracleManifestRow>,
    comparison_rows: Vec<ForgeQueryIntentAdmissionOracleComparisonRow>,
    manifest_digest: String,
    oracle_digest: String,
}

impl ForgeQueryIntentAdmissionOracleReport {
    pub fn manifest_rows(&self) -> &[ForgeQueryIntentAdmissionOracleManifestRow] {
        &self.manifest_rows
    }

    pub fn comparison_rows(&self) -> &[ForgeQueryIntentAdmissionOracleComparisonRow] {
        &self.comparison_rows
    }

    pub fn manifest_digest(&self) -> &str {
        &self.manifest_digest
    }

    pub fn oracle_digest(&self) -> &str {
        &self.oracle_digest
    }
}

pub fn forge_query_intent_admission_oracle_report() -> ForgeQueryIntentAdmissionOracleReport {
    let admitted = certified_admitted_intent_fixture();
    let advisory = certified_advisory_intent_fixture();
    let violation = certified_violation_intent_fixture();
    let deferred = certified_deferred_intent_fixture();
    let unsupported = certified_unsupported_intent_fixture();
    let manifest_rows = vec![
        manifest_row(
            ForgeQueryIntentAdmissionOracleLane::AdmittedControl,
            "shared_lattice_admitted_control",
        ),
        manifest_row(
            ForgeQueryIntentAdmissionOracleLane::AdvisoryControl,
            "shared_lattice_advisory_control",
        ),
        manifest_row(
            ForgeQueryIntentAdmissionOracleLane::ViolationControl,
            "shared_lattice_violation_control",
        ),
        manifest_row(
            ForgeQueryIntentAdmissionOracleLane::DeferredControl,
            "shared_lattice_deferred_control",
        ),
        manifest_row(
            ForgeQueryIntentAdmissionOracleLane::UnsupportedControl,
            "shared_lattice_unsupported_control",
        ),
    ];
    let comparison_rows = vec![
        comparison_row(
            ForgeQueryIntentAdmissionOracleLane::AdmittedControl,
            comparison::expected_admitted_detail(),
            comparison::expected_admitted_digest(),
            comparison::actual_admitted_detail(&admitted),
            comparison::actual_admitted_digest(&admitted),
        ),
        comparison_row(
            ForgeQueryIntentAdmissionOracleLane::AdvisoryControl,
            comparison::expected_advisory_detail(),
            comparison::expected_advisory_digest(),
            comparison::actual_advisory_detail(&advisory),
            comparison::actual_advisory_digest(&advisory),
        ),
        comparison_row(
            ForgeQueryIntentAdmissionOracleLane::ViolationControl,
            comparison::expected_violation_detail(),
            comparison::expected_violation_digest(),
            comparison::actual_violation_detail(&violation),
            comparison::actual_violation_digest(&violation),
        ),
        comparison_row(
            ForgeQueryIntentAdmissionOracleLane::DeferredControl,
            comparison::expected_deferred_detail(),
            comparison::expected_deferred_digest(),
            comparison::actual_deferred_detail(&deferred),
            comparison::actual_deferred_digest(&deferred),
        ),
        comparison_row(
            ForgeQueryIntentAdmissionOracleLane::UnsupportedControl,
            comparison::expected_unsupported_detail(),
            comparison::expected_unsupported_digest(),
            comparison::actual_unsupported_detail(&unsupported),
            comparison::actual_unsupported_digest(&unsupported),
        ),
    ];
    let manifest_digest = hash_parts(
        &manifest_rows
            .iter()
            .map(|row| row.row_digest().to_string())
            .collect::<Vec<_>>(),
    );
    let oracle_digest = hash_parts(
        &comparison_rows
            .iter()
            .map(|row| row.row_digest().to_string())
            .chain(std::iter::once(format!("manifest:{manifest_digest}")))
            .collect::<Vec<_>>(),
    );
    ForgeQueryIntentAdmissionOracleReport {
        manifest_rows,
        comparison_rows,
        manifest_digest,
        oracle_digest,
    }
}

fn manifest_row(
    lane: ForgeQueryIntentAdmissionOracleLane,
    lane_name: &'static str,
) -> ForgeQueryIntentAdmissionOracleManifestRow {
    ForgeQueryIntentAdmissionOracleManifestRow {
        lane,
        row_digest: hash_parts(&[
            "forge_query_intent_admission_oracle_manifest_row_v1".to_string(),
            format!("lane:{}", lane.as_str()),
            format!("name:{lane_name}"),
            "owner:intent_admission::certification::oracles".to_string(),
        ]),
    }
}

fn comparison_row(
    lane: ForgeQueryIntentAdmissionOracleLane,
    expected_detail: String,
    expected_digest: String,
    actual_detail: String,
    actual_digest: String,
) -> ForgeQueryIntentAdmissionOracleComparisonRow {
    let row_digest = hash_parts(&[
        "forge_query_intent_admission_oracle_comparison_row_v1".to_string(),
        format!("lane:{}", lane.as_str()),
        format!("expected:{expected_digest}"),
        format!("actual:{actual_digest}"),
        format!("match:{}", expected_digest == actual_digest),
    ]);
    ForgeQueryIntentAdmissionOracleComparisonRow {
        lane,
        expected_detail,
        expected_digest,
        actual_detail,
        actual_digest,
        row_digest,
    }
}
