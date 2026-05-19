use crate::identity::hash_parts;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryIntentAdmissionSlopeLane {
    AdmissionClassification,
    DecisionTraceAssembly,
    DecisionSupportLookup,
    CoveredEntrypointInventory,
    ExecutionProvenanceAssembly,
    LegacyDelegationParity,
    DecisionCertificationCoverage,
}

impl ForgeQueryIntentAdmissionSlopeLane {
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            Self::AdmissionClassification => "admission_classification",
            Self::DecisionTraceAssembly => "decision_trace_assembly",
            Self::DecisionSupportLookup => "decision_support_lookup",
            Self::CoveredEntrypointInventory => "covered_entrypoint_inventory",
            Self::ExecutionProvenanceAssembly => "execution_provenance_assembly",
            Self::LegacyDelegationParity => "legacy_delegation_parity",
            Self::DecisionCertificationCoverage => "decision_certification_coverage",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryIntentAdmissionWidthRunScale {
    Small,
    Medium,
    Large,
}

impl ForgeQueryIntentAdmissionWidthRunScale {
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            Self::Small => "small",
            Self::Medium => "medium",
            Self::Large => "large",
        }
    }

    pub(crate) fn multiplier(self) -> usize {
        match self {
            Self::Small => 1,
            Self::Medium => 2,
            Self::Large => 3,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryIntentAdmissionWidthRunRow {
    lane: ForgeQueryIntentAdmissionSlopeLane,
    scale: ForgeQueryIntentAdmissionWidthRunScale,
    width: usize,
    row_digest: String,
}

impl ForgeQueryIntentAdmissionWidthRunRow {
    pub fn lane(&self) -> ForgeQueryIntentAdmissionSlopeLane {
        self.lane
    }

    pub fn scale(&self) -> ForgeQueryIntentAdmissionWidthRunScale {
        self.scale
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn row_digest(&self) -> &str {
        &self.row_digest
    }
}

pub(crate) fn lane_width_runs(
    lane: ForgeQueryIntentAdmissionSlopeLane,
    tokens: Vec<String>,
) -> Vec<ForgeQueryIntentAdmissionWidthRunRow> {
    [
        ForgeQueryIntentAdmissionWidthRunScale::Small,
        ForgeQueryIntentAdmissionWidthRunScale::Medium,
        ForgeQueryIntentAdmissionWidthRunScale::Large,
    ]
    .into_iter()
    .map(|scale| width_run_row(lane, scale, &tokens))
    .collect()
}

pub(crate) fn slope_digest(
    width_runs: &[ForgeQueryIntentAdmissionWidthRunRow],
    lane: ForgeQueryIntentAdmissionSlopeLane,
) -> String {
    hash_parts(
        &width_runs
            .iter()
            .filter(|row| row.lane() == lane)
            .map(|row| row.row_digest().to_string())
            .collect::<Vec<_>>(),
    )
}

fn width_run_row(
    lane: ForgeQueryIntentAdmissionSlopeLane,
    scale: ForgeQueryIntentAdmissionWidthRunScale,
    tokens: &[String],
) -> ForgeQueryIntentAdmissionWidthRunRow {
    let width = tokens.len() * scale.multiplier();
    let expanded_digest = hash_parts(
        &(0..scale.multiplier())
            .flat_map(|iteration| {
                tokens
                    .iter()
                    .map(move |token| format!("iter:{iteration}:{token}"))
            })
            .collect::<Vec<_>>(),
    );
    ForgeQueryIntentAdmissionWidthRunRow {
        lane,
        scale,
        width,
        row_digest: hash_parts(&[
            "forge_query_intent_admission_width_run_row_v1".to_string(),
            format!("lane:{}", lane.as_str()),
            format!("scale:{}", scale.as_str()),
            format!("width:{width}"),
            expanded_digest,
        ]),
    }
}
