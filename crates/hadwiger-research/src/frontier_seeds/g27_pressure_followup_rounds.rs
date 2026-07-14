use crate::domain_artifacts::core_artifact::{
    impl_hadwiger_artifact, HadwigerArtifactAuthorityOwner, HadwigerArtifactCore,
    HadwigerArtifactKind, HadwigerArtifactSourceReference,
};
use crate::domain_artifacts::digest_basis::artifact_core;
use crate::domain_artifacts::HadwigerCanonicalArtifact;
use crate::query_entry::HadwigerResearchHandle;

use super::g27_geometric_fractional::G27GeometricFractionalError;
use super::g27_geometric_fractional_data::retained_g27_coefficients;
use super::g27_geometric_fractional_dual_replay::retained_g27_tight_atom_masks;
use super::g27_geometric_fractional_lead_report::materialize_g27_pressure_escape_lead_checked;
use super::g27_pressure_followup_support::{
    count_moser_basis_common_anchors, enumerate_combinations, hits_all, hitting_payload,
    mask_vertices, one_anchor_payload, select_spindle_fragment, spindle_payload,
};

const MAX_TRANSVERSAL_SIZE: usize = 5;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum G27HittingSetPosture {
    SmallTransversalFalsified,
    FoundSmallTransversal,
}

impl G27HittingSetPosture {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::SmallTransversalFalsified => "small_transversal_falsified",
            Self::FoundSmallTransversal => "found_small_transversal",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct G27TightAtomTransversal {
    vertices: Vec<String>,
}

impl G27TightAtomTransversal {
    pub fn vertices(&self) -> &[String] {
        &self.vertices
    }

    fn from_mask(mask: u32) -> Self {
        Self {
            vertices: mask_vertices(mask),
        }
    }

    pub(super) fn stable_token(&self) -> String {
        self.vertices.join("-")
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct G27TightAtomHittingSetReport {
    core: HadwigerArtifactCore,
    tight_atom_count: usize,
    minimum_hitting_set_size: usize,
    size_le_four_hitting_sets: usize,
    retained_minimum_transversals: Vec<G27TightAtomTransversal>,
    posture: G27HittingSetPosture,
    conclusion: String,
}

impl G27TightAtomHittingSetReport {
    pub fn tight_atom_count(&self) -> usize {
        self.tight_atom_count
    }

    pub fn minimum_hitting_set_size(&self) -> usize {
        self.minimum_hitting_set_size
    }

    pub fn size_le_four_hitting_sets(&self) -> usize {
        self.size_le_four_hitting_sets
    }

    pub fn retained_minimum_transversals(&self) -> &[G27TightAtomTransversal] {
        &self.retained_minimum_transversals
    }

    pub fn posture(&self) -> G27HittingSetPosture {
        self.posture
    }

    pub fn conclusion(&self) -> &str {
        &self.conclusion
    }

    pub fn admits_theorem_authority(&self) -> bool {
        false
    }

    pub fn registers_query_invariant_authority(&self) -> bool {
        false
    }
}

impl_hadwiger_artifact!(G27TightAtomHittingSetReport, core);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum G27OneAnchorTransversalPosture {
    SmallTransversalFalsified,
    NoMoserBasisCommonAnchor,
}

impl G27OneAnchorTransversalPosture {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::SmallTransversalFalsified => "small_transversal_falsified",
            Self::NoMoserBasisCommonAnchor => "no_moser_basis_common_anchor",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct G27OneAnchorTransversalReport {
    core: HadwigerArtifactCore,
    hitting_report: G27TightAtomHittingSetReport,
    tested_transversal: G27TightAtomTransversal,
    moser_basis_common_anchor_count: usize,
    posture: G27OneAnchorTransversalPosture,
    conclusion: String,
}

impl G27OneAnchorTransversalReport {
    pub fn tested_transversal(&self) -> &G27TightAtomTransversal {
        &self.tested_transversal
    }

    pub fn moser_basis_common_anchor_count(&self) -> usize {
        self.moser_basis_common_anchor_count
    }

    pub fn posture(&self) -> G27OneAnchorTransversalPosture {
        self.posture
    }

    pub fn conclusion(&self) -> &str {
        &self.conclusion
    }

    pub fn admits_theorem_authority(&self) -> bool {
        false
    }

    pub fn registers_query_invariant_authority(&self) -> bool {
        false
    }
}

impl_hadwiger_artifact!(G27OneAnchorTransversalReport, core);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum G27SpindlePreflightPosture {
    FundManufacturedRotation,
}

impl G27SpindlePreflightPosture {
    pub fn as_str(self) -> &'static str {
        "fund_manufactured_rotation"
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct G27PressureSkeletonSpindleReport {
    core: HadwigerArtifactCore,
    one_anchor_report: G27OneAnchorTransversalReport,
    hinge_vertex: String,
    fragment_vertices: Vec<String>,
    tight_atoms_containing_fragment: usize,
    posture: G27SpindlePreflightPosture,
    next_test: String,
}

impl G27PressureSkeletonSpindleReport {
    pub fn hinge_vertex(&self) -> &str {
        &self.hinge_vertex
    }

    pub fn fragment_vertices(&self) -> &[String] {
        &self.fragment_vertices
    }

    pub fn tight_atoms_containing_fragment(&self) -> usize {
        self.tight_atoms_containing_fragment
    }

    pub fn posture(&self) -> G27SpindlePreflightPosture {
        self.posture
    }

    pub fn next_test(&self) -> &str {
        &self.next_test
    }

    pub fn admits_theorem_authority(&self) -> bool {
        false
    }

    pub fn registers_query_invariant_authority(&self) -> bool {
        false
    }
}

impl_hadwiger_artifact!(G27PressureSkeletonSpindleReport, core);

pub fn enumerate_g27_tight_atom_hitting_sets_checked(
    handle: &HadwigerResearchHandle,
) -> Result<G27TightAtomHittingSetReport, G27GeometricFractionalError> {
    let source_lead = materialize_g27_pressure_escape_lead_checked(handle)?;
    let tight_atoms = retained_g27_tight_atom_masks()?;
    let mut size_le_four_hitting_sets = 0usize;
    let mut minimum_size = 0usize;
    let mut minimum_masks = Vec::new();
    for size in 1..=MAX_TRANSVERSAL_SIZE {
        let mut masks = Vec::new();
        enumerate_combinations(size, 0, 0, &mut |mask| {
            if hits_all(mask, &tight_atoms) {
                if size <= 4 {
                    size_le_four_hitting_sets += 1;
                }
                masks.push(mask);
            }
        });
        if !masks.is_empty() {
            minimum_size = size;
            minimum_masks = masks;
            break;
        }
    }
    let posture = if size_le_four_hitting_sets == 0 {
        G27HittingSetPosture::SmallTransversalFalsified
    } else {
        G27HittingSetPosture::FoundSmallTransversal
    };
    let retained_minimum_transversals = minimum_masks
        .iter()
        .take(3)
        .copied()
        .map(G27TightAtomTransversal::from_mask)
        .collect::<Vec<_>>();
    let conclusion = format!(
        "minimum tight-atom transversal size is {minimum_size}; no size <= 4 transversal exists"
    );
    let core = artifact_core(
        HadwigerArtifactKind::G27TightAtomHittingSetReport,
        HadwigerArtifactAuthorityOwner::HadwigerArtifactBuilder,
        HadwigerArtifactSourceReference::ArtifactConstruction {
            operation: "g27_tight_atom_hitting_set".to_string(),
        },
        vec![source_lead.reference()],
        hitting_payload(
            tight_atoms.len(),
            minimum_size,
            size_le_four_hitting_sets,
            posture,
            &retained_minimum_transversals,
            &conclusion,
        ),
    )?;
    Ok(G27TightAtomHittingSetReport {
        core,
        tight_atom_count: tight_atoms.len(),
        minimum_hitting_set_size: minimum_size,
        size_le_four_hitting_sets,
        retained_minimum_transversals,
        posture,
        conclusion,
    })
}

pub fn test_g27_parameterized_one_anchor_transversal_checked(
    handle: &HadwigerResearchHandle,
) -> Result<G27OneAnchorTransversalReport, G27GeometricFractionalError> {
    let hitting_report = enumerate_g27_tight_atom_hitting_sets_checked(handle)?;
    let tested_transversal = hitting_report
        .retained_minimum_transversals()
        .last()
        .cloned()
        .ok_or(G27GeometricFractionalError::MalformedData {
            source: "g27_minimum_transversal",
        })?;
    let coefficients = retained_g27_coefficients()?;
    let anchor_count = count_moser_basis_common_anchors(&coefficients, &tested_transversal)?;
    let posture = if hitting_report.size_le_four_hitting_sets() == 0 {
        G27OneAnchorTransversalPosture::SmallTransversalFalsified
    } else {
        G27OneAnchorTransversalPosture::NoMoserBasisCommonAnchor
    };
    let conclusion =
        "one-anchor route is not eligible: the tight face needs a size-5 transversal".to_string();
    let core = artifact_core(
        HadwigerArtifactKind::G27OneAnchorTransversalReport,
        HadwigerArtifactAuthorityOwner::HadwigerArtifactBuilder,
        HadwigerArtifactSourceReference::ArtifactConstruction {
            operation: "g27_one_anchor_transversal".to_string(),
        },
        vec![hitting_report.reference()],
        one_anchor_payload(&tested_transversal, anchor_count, posture, &conclusion),
    )?;
    Ok(G27OneAnchorTransversalReport {
        core,
        hitting_report,
        tested_transversal,
        moser_basis_common_anchor_count: anchor_count,
        posture,
        conclusion,
    })
}

pub fn preflight_g27_pressure_skeleton_spindle_checked(
    handle: &HadwigerResearchHandle,
) -> Result<G27PressureSkeletonSpindleReport, G27GeometricFractionalError> {
    let one_anchor_report = test_g27_parameterized_one_anchor_transversal_checked(handle)?;
    let tight_atoms = retained_g27_tight_atom_masks()?;
    let fragment_mask = select_spindle_fragment(&tight_atoms)?;
    let fragment_vertices = mask_vertices(fragment_mask);
    let tight_atoms_containing_fragment = tight_atoms
        .iter()
        .filter(|atom| **atom & fragment_mask == fragment_mask)
        .count();
    let posture = G27SpindlePreflightPosture::FundManufacturedRotation;
    let next_test = "rotate retained pressure fragment about vertex 8 and pin with one manufactured outside-Moser closure"
        .to_string();
    let core = artifact_core(
        HadwigerArtifactKind::G27PressureSkeletonSpindleReport,
        HadwigerArtifactAuthorityOwner::HadwigerArtifactBuilder,
        HadwigerArtifactSourceReference::ArtifactConstruction {
            operation: "g27_pressure_skeleton_spindle_preflight".to_string(),
        },
        vec![one_anchor_report.reference()],
        spindle_payload(
            &fragment_vertices,
            tight_atoms_containing_fragment,
            posture,
            &next_test,
        ),
    )?;
    Ok(G27PressureSkeletonSpindleReport {
        core,
        one_anchor_report,
        hinge_vertex: "8".to_string(),
        fragment_vertices,
        tight_atoms_containing_fragment,
        posture,
        next_test,
    })
}
