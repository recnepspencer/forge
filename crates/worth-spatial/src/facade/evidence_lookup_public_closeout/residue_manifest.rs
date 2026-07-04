use std::sync::OnceLock;

use forge_query::facade::consumer_kit::ForgeQuerySupportPinFinding;

use crate::workload_platform::evidence_lookup_query_consumer_kit::{
    audit_evidence_lookup_query_consumer_residue_for_roots,
    current_evidence_lookup_query_consumer_kit, derived_support_requirements,
    evidence_lookup_query_consumer_kit_residue_roots,
    evidence_lookup_query_support_pinning_contract, project_evidence_lookup_query_support_snapshot,
    residue_rows_from_report, EvidenceLookupQueryConsumerResidueRow,
    EvidenceLookupQuerySupportRequirementRow,
};
use crate::workload_platform::evidence_lookup_query_surface_matrix::current_evidence_lookup_query_surface_matrix;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EvidenceLookupPublicCloseoutResidueOwner {
    WorthSpatial,
    WorthTopo,
    ForgeQuery,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EvidenceLookupPublicCloseoutResidueDisposition {
    ExplicitResidue,
    QueryGap,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EvidenceLookupPublicCloseoutQueryGapKind {
    MissingArtifact,
    NotAdmittedOnSupportedPath,
    NotExposedAtBoundary,
    IdentitySemanticsInsufficient,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EvidenceLookupPublicCloseoutResidueRow {
    source_path: String,
    current_surface: String,
    owner: EvidenceLookupPublicCloseoutResidueOwner,
    disposition: EvidenceLookupPublicCloseoutResidueDisposition,
    query_gap_kind: Option<EvidenceLookupPublicCloseoutQueryGapKind>,
    blocker: String,
    removal_trigger: String,
}

impl EvidenceLookupPublicCloseoutResidueRow {
    fn new(
        source_path: impl Into<String>,
        current_surface: impl Into<String>,
        owner: EvidenceLookupPublicCloseoutResidueOwner,
        disposition: EvidenceLookupPublicCloseoutResidueDisposition,
        query_gap_kind: Option<EvidenceLookupPublicCloseoutQueryGapKind>,
        blocker: impl Into<String>,
        removal_trigger: impl Into<String>,
    ) -> Self {
        Self {
            source_path: source_path.into(),
            current_surface: current_surface.into(),
            owner,
            disposition,
            query_gap_kind,
            blocker: blocker.into(),
            removal_trigger: removal_trigger.into(),
        }
    }

    pub fn source_path(&self) -> &str {
        &self.source_path
    }

    pub fn current_surface(&self) -> &str {
        &self.current_surface
    }

    pub const fn owner(&self) -> EvidenceLookupPublicCloseoutResidueOwner {
        self.owner
    }

    pub const fn disposition(&self) -> EvidenceLookupPublicCloseoutResidueDisposition {
        self.disposition
    }

    pub const fn query_gap_kind(&self) -> Option<EvidenceLookupPublicCloseoutQueryGapKind> {
        self.query_gap_kind
    }

    pub fn blocker(&self) -> &str {
        &self.blocker
    }

    pub fn removal_trigger(&self) -> &str {
        &self.removal_trigger
    }
}

pub fn current_evidence_lookup_public_closeout_residue_manifest(
) -> &'static [EvidenceLookupPublicCloseoutResidueRow] {
    static CACHE: OnceLock<Vec<EvidenceLookupPublicCloseoutResidueRow>> = OnceLock::new();
    CACHE.get_or_init(build_live_evidence_lookup_public_closeout_residue_manifest)
}

fn build_live_evidence_lookup_public_closeout_residue_manifest(
) -> Vec<EvidenceLookupPublicCloseoutResidueRow> {
    let mut rows = current_evidence_lookup_query_consumer_kit()
        .map(|closeout| {
            closeout
                .query_residue_rows()
                .iter()
                .map(EvidenceLookupPublicCloseoutResidueRow::from_query_consumer_residue_row)
                .collect::<Vec<_>>()
        })
        .unwrap_or_else(|_| derive_live_query_consumer_residue_rows());
    rows.extend(derive_live_query_gap_rows());
    rows.sort_by(|left, right| {
        left.source_path
            .cmp(&right.source_path)
            .then(left.current_surface.cmp(&right.current_surface))
    });
    rows
}

fn derive_live_query_consumer_residue_rows() -> Vec<EvidenceLookupPublicCloseoutResidueRow> {
    let report = audit_evidence_lookup_query_consumer_residue_for_roots(
        evidence_lookup_query_consumer_kit_residue_roots(),
    )
    .expect("spatial residue ledger should derive live Query consumer residue rows");
    residue_rows_from_report(&report)
        .expect("spatial residue ledger should lower Query consumer residue rows")
        .iter()
        .map(EvidenceLookupPublicCloseoutResidueRow::from_query_consumer_residue_row)
        .collect()
}

fn derive_live_query_gap_rows() -> Vec<EvidenceLookupPublicCloseoutResidueRow> {
    let matrix = current_evidence_lookup_query_surface_matrix()
        .expect("spatial residue ledger should derive from the live query surface matrix");
    let snapshot = project_evidence_lookup_query_support_snapshot(&matrix)
        .expect("spatial residue ledger should project the live Query support snapshot");
    let requirements = derived_support_requirements(&matrix);
    let contract = evidence_lookup_query_support_pinning_contract(&snapshot, &matrix)
        .expect("spatial residue ledger should derive the live Query support contract");
    let report = contract
        .evaluate_snapshot(&snapshot)
        .expect("spatial residue ledger should evaluate the live Query support contract");

    requirements
        .iter()
        .filter_map(|requirement: &EvidenceLookupQuerySupportRequirementRow| {
            report
                .findings()
                .iter()
                .find(|finding: &&ForgeQuerySupportPinFinding| {
                    finding.family() == Some(requirement.runtime_family())
                        && finding.surface() == requirement.runtime_family().as_str()
                })
                .map(|finding| {
                    EvidenceLookupPublicCloseoutResidueRow::from_support_pin_finding(
                        finding,
                        report.report_digest(),
                    )
                })
        })
        .collect()
}

impl EvidenceLookupPublicCloseoutResidueRow {
    fn from_query_consumer_residue_row(row: &EvidenceLookupQueryConsumerResidueRow) -> Self {
        Self::new(
            row.source_path(),
            format!(
                "ForgeQueryConsumerResidueClass::{}@{}:{}",
                row.residue_class().as_str(),
                row.line(),
                row.column()
            ),
            EvidenceLookupPublicCloseoutResidueOwner::WorthSpatial,
            EvidenceLookupPublicCloseoutResidueDisposition::ExplicitResidue,
            None,
            format!(
                "evidence-lookup Query consumer residue audit still detects `{}` in the cut-over public closeout lane",
                row.residue_class().as_str()
            ),
            format!(
                "remove once the worth-spatial evidence-lookup Query consumer residue audit is clean for `{}`",
                row.residue_class().as_str()
            ),
        )
    }

    fn from_support_pin_finding(
        finding: &ForgeQuerySupportPinFinding,
        support_contract_digest: &str,
    ) -> Self {
        let family = finding
            .family()
            .expect("support pin finding should carry the required runtime family");
        let current_surface = format!(
            "workspace.admit_public_api_family({})@{}",
            family.as_str(),
            finding.kind().as_str()
        );
        let kind = finding.kind().as_str();
        let blocker = match kind {
            "required-row-missing" => format!(
                "Forge Query does not currently publish the required `{}` public support row on the ordinary supported path for spatial evidence lookup",
                family.as_str()
            ),
            "status-mismatch" => format!(
                "Forge Query does not currently admit `{}` as `supported` for the ordinary spatial evidence-lookup path",
                family.as_str()
            ),
            "teaching-posture-mismatch" => format!(
                "Forge Query exposes `{}` but not with `ordinary-runtime-dx` teaching posture required by the ordinary spatial evidence-lookup path",
                family.as_str()
            ),
            other => panic!(
                "unexpected Forge Query support pin finding kind `{other}` for `{}`; phase 15 requires exact blocker classification",
                family.as_str()
            ),
        };
        let query_gap_kind = match kind {
            "required-row-missing" => {
                EvidenceLookupPublicCloseoutQueryGapKind::MissingArtifact
            }
            "status-mismatch" => {
                EvidenceLookupPublicCloseoutQueryGapKind::NotAdmittedOnSupportedPath
            }
            "teaching-posture-mismatch" => {
                EvidenceLookupPublicCloseoutQueryGapKind::NotExposedAtBoundary
            }
            other => panic!(
                "unexpected Forge Query support pin finding kind `{other}` for `{}`; phase 15 requires exact blocker classification",
                family.as_str()
            ),
        };
        let removal_trigger = format!(
            "remove once Forge Query support admission satisfies `{}` on the ordinary supported path and the support pin report `{support_contract_digest}` becomes clean",
            family.as_str()
        );
        Self::new(
            "crates/worth-spatial/src/facade/evidence_lookup_public_closeout/residue_manifest.rs",
            current_surface,
            EvidenceLookupPublicCloseoutResidueOwner::ForgeQuery,
            EvidenceLookupPublicCloseoutResidueDisposition::QueryGap,
            Some(query_gap_kind),
            blocker,
            removal_trigger,
        )
    }
}

impl EvidenceLookupPublicCloseoutQueryGapKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MissingArtifact => "missing",
            Self::NotAdmittedOnSupportedPath => "not-admitted",
            Self::NotExposedAtBoundary => "not-exposed",
            Self::IdentitySemanticsInsufficient => "identity-semantics-insufficient",
        }
    }
}
