use crate::domain_artifacts::core_artifact::{
    impl_hadwiger_artifact, HadwigerArtifactAuthorityOwner, HadwigerArtifactCore,
    HadwigerArtifactKind, HadwigerArtifactSourceReference,
};
use crate::domain_artifacts::digest_basis::{artifact_core, HadwigerArtifactPayloadEntry};
use crate::domain_artifacts::HadwigerCanonicalArtifact;
use crate::mathematical_verification::ExactRational;
use crate::query_entry::HadwigerResearchHandle;

use super::g27_geometric_fractional::G27GeometricFractionalError;
use super::g27_moser_anchor_scan::{
    scan_g27_row_685_moser_anchor_breakers_checked, G27MoserAnchorScanReport,
};
use super::g27_outside_moser_anchor::{
    G27OutsideMoserAnchorCandidate, G27OutsideMoserAxis, G27QuadraticAnchorExtension,
};

const RADICANDS: [i128; 2] = [2, 3];
const AXES: [G27OutsideMoserAxis; 2] = [G27OutsideMoserAxis::X, G27OutsideMoserAxis::Y];
const MAX_RETAINED_SURVIVORS: usize = 16;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct G27QuadraticAnchorSearchReport {
    core: HadwigerArtifactCore,
    moser_scan: G27MoserAnchorScanReport,
    radicands: Vec<i128>,
    bases_checked: usize,
    candidates_checked: usize,
    retained_survivors: Vec<G27OutsideMoserAnchorCandidate>,
}

impl G27QuadraticAnchorSearchReport {
    pub fn moser_scan(&self) -> &G27MoserAnchorScanReport {
        &self.moser_scan
    }

    pub fn radicands(&self) -> &[i128] {
        &self.radicands
    }

    pub fn bases_checked(&self) -> usize {
        self.bases_checked
    }

    pub fn candidates_checked(&self) -> usize {
        self.candidates_checked
    }

    pub fn retained_survivors(&self) -> &[G27OutsideMoserAnchorCandidate] {
        &self.retained_survivors
    }

    pub fn has_surviving_candidates(&self) -> bool {
        !self.retained_survivors.is_empty()
    }

    pub fn admits_theorem_authority(&self) -> bool {
        false
    }

    pub fn registers_query_invariant_authority(&self) -> bool {
        false
    }
}

impl_hadwiger_artifact!(G27QuadraticAnchorSearchReport, core);

pub fn search_g27_bounded_quadratic_anchors_checked(
    handle: &HadwigerResearchHandle,
) -> Result<G27QuadraticAnchorSearchReport, G27GeometricFractionalError> {
    let moser_scan = scan_g27_row_685_moser_anchor_breakers_checked(handle)?;
    let survivors = generate_quadratic_survivors(&moser_scan)?;
    let retained_survivors = survivors
        .iter()
        .take(MAX_RETAINED_SURVIVORS)
        .cloned()
        .collect::<Vec<_>>();
    let core = artifact_core(
        HadwigerArtifactKind::G27QuadraticAnchorSearchReport,
        HadwigerArtifactAuthorityOwner::HadwigerArtifactBuilder,
        HadwigerArtifactSourceReference::ArtifactConstruction {
            operation: "g27_bounded_quadratic_anchor_search".to_string(),
        },
        vec![moser_scan.reference()],
        search_payload(&moser_scan, survivors.len(), &retained_survivors),
    )?;
    Ok(G27QuadraticAnchorSearchReport {
        core,
        moser_scan,
        radicands: RADICANDS.to_vec(),
        bases_checked: retained_survivors_base_count_hint(survivors.len()),
        candidates_checked: survivors.len(),
        retained_survivors,
    })
}

fn generate_quadratic_survivors(
    moser_scan: &G27MoserAnchorScanReport,
) -> Result<Vec<G27OutsideMoserAnchorCandidate>, G27GeometricFractionalError> {
    let mut candidates = Vec::new();
    for (base_index, base) in moser_scan.retained_breakers().iter().enumerate() {
        for radicand in RADICANDS {
            for axis in AXES {
                let extension = G27QuadraticAnchorExtension::new(
                    axis,
                    radicand,
                    ExactRational::fraction(1, 3).map_err(|_| {
                        G27GeometricFractionalError::MalformedData {
                            source: "quadratic_anchor_coefficient",
                        }
                    })?,
                )?;
                candidates.push(G27OutsideMoserAnchorCandidate::quadratic_extension(
                    format!("row685-q{}-{}-{}", base_index + 1, radicand, axis.as_str()),
                    base.coefficients(),
                    extension,
                )?);
            }
        }
    }
    candidates.sort_by_key(G27OutsideMoserAnchorCandidate::stable_token);
    Ok(candidates)
}

fn search_payload(
    moser_scan: &G27MoserAnchorScanReport,
    candidates_checked: usize,
    retained_survivors: &[G27OutsideMoserAnchorCandidate],
) -> Vec<HadwigerArtifactPayloadEntry> {
    let mut payload = vec![
        HadwigerArtifactPayloadEntry::text("schema", "forge.hadwiger.g27_quadratic_search.v1"),
        HadwigerArtifactPayloadEntry::unsigned(
            "bases_checked",
            moser_scan.retained_breakers().len() as u128,
        ),
        HadwigerArtifactPayloadEntry::unsigned("radicand_count", RADICANDS.len() as u128),
        HadwigerArtifactPayloadEntry::unsigned("axis_count", AXES.len() as u128),
        HadwigerArtifactPayloadEntry::unsigned("candidates_checked", candidates_checked as u128),
        HadwigerArtifactPayloadEntry::unsigned(
            "retained_survivor_count",
            retained_survivors.len() as u128,
        ),
    ];
    for survivor in retained_survivors {
        payload.push(HadwigerArtifactPayloadEntry::text(
            "retained_survivor",
            survivor.stable_token(),
        ));
    }
    payload
}

fn retained_survivors_base_count_hint(survivor_count: usize) -> usize {
    if survivor_count == 0 {
        0
    } else {
        survivor_count / (RADICANDS.len() * AXES.len())
    }
}
