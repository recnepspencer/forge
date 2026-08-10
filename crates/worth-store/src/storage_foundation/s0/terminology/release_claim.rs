use super::risk_report::TerminologyRiskReport;
use std::collections::BTreeSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
pub enum PublicClaimRejection {
    OverclaimedPhysicalPosture,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct ReleaseClaimScanPlan {
    release_surface_paths: Vec<String>,
}

impl ReleaseClaimScanPlan {
    pub fn new(
        release_surface_paths: Vec<String>,
    ) -> Result<Self, super::validation::TerminologyCleanupRejection> {
        if release_surface_paths.is_empty() {
            return Err(super::validation::TerminologyCleanupRejection::MissingReleaseSurface);
        }
        let mut seen = BTreeSet::new();
        if release_surface_paths
            .iter()
            .any(|path| path.trim().is_empty() || !seen.insert(path.as_str()))
        {
            return Err(super::validation::TerminologyCleanupRejection::DuplicateReleaseSurface);
        }
        Ok(Self {
            release_surface_paths,
        })
    }

    pub fn release_surface_paths(&self) -> &[String] {
        &self.release_surface_paths
    }
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct ReleaseClaimReport {
    scanned_surface_count: u64,
    rejection_count: u64,
    unqualified_release_claim_count: u64,
    rejected: Vec<(String, u64, PublicClaimRejection)>,
}

impl ReleaseClaimReport {
    pub fn from_terminology_report(
        plan: &ReleaseClaimScanPlan,
        report: &TerminologyRiskReport,
    ) -> Result<Self, super::validation::TerminologyCleanupRejection> {
        let release_paths = plan
            .release_surface_paths()
            .iter()
            .map(String::as_str)
            .collect::<BTreeSet<_>>();
        let scanned_release_paths = report
            .rows()
            .iter()
            .map(super::phrase_finding::TerminologyPhraseFinding::subject_path_or_symbol)
            .filter(|path| release_paths.contains(path))
            .collect::<BTreeSet<_>>();
        if scanned_release_paths.len() != release_paths.len() {
            return Err(super::validation::TerminologyCleanupRejection::UnscannedReleaseSurface);
        }
        let rejected = report
            .rows()
            .iter()
            .filter(|row| release_paths.contains(row.subject_path_or_symbol()))
            .filter_map(|row| match row.allowed_use() {
                super::phrase_policy::TerminologyAllowedUse::OverclaimedPhysicalPosture => Some((
                    row.subject_path_or_symbol().to_string(),
                    row.line_number(),
                    PublicClaimRejection::OverclaimedPhysicalPosture,
                )),
                _ => None,
            })
            .collect::<Vec<_>>();
        Ok(Self {
            scanned_surface_count: release_paths.len() as u64,
            rejection_count: rejected.len() as u64,
            unqualified_release_claim_count: rejected.len() as u64,
            rejected,
        })
    }

    pub fn rejection_count(&self) -> u64 {
        self.rejection_count
    }

    pub fn unqualified_release_claim_count(&self) -> u64 {
        self.unqualified_release_claim_count
    }

    pub fn scanned_surface_count(&self) -> u64 {
        self.scanned_surface_count
    }
}
