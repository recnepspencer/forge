use crate::domain_artifacts::core_artifact::{
    impl_hadwiger_artifact, HadwigerArtifactAuthorityOwner, HadwigerArtifactCore,
    HadwigerArtifactKind, HadwigerArtifactSourceReference,
};
use crate::domain_artifacts::digest_basis::{artifact_core, HadwigerArtifactPayloadEntry};
use crate::domain_artifacts::HadwigerCanonicalArtifact;
use crate::query_entry::HadwigerResearchHandle;

use super::g27_geometric_fractional::G27GeometricFractionalError;
use super::g27_geometric_fractional_data::retained_g27_coefficients;
use super::g27_geometric_fractional_dual_replay::retained_g27_tight_atom_masks;
use super::g27_same_field_pressure_interface_support::{approx_unit_distance, g27_points};
use super::g27_w_circles_exact_geometry_audit::audit_g27_w_circles_607_exact_geometry_checked;
use super::g27_w_circles_exact_geometry_support::{
    parse_w_integer_weights, parse_w_vertices, squared_distance,
};

const G27_ANCHOR_INDEX: usize = 22;
const W_ANCHOR_INDEX: usize = 253;
const LIFT_GAP_WEIGHT_NUMERATOR: i128 = 51_749;
const TOP_ATOM_LIMIT: usize = 10;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum G27TightAtomContactPosture {
    FundFixedDualPricing,
    RetiredWeakTightAtomContact,
}

impl G27TightAtomContactPosture {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::FundFixedDualPricing => "fund_fixed_dual_pricing",
            Self::RetiredWeakTightAtomContact => "retired_weak_tight_atom_contact",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct G27TightAtomContactChannel {
    atom_mask: u32,
    atom_size: usize,
    touched_vertex_count: usize,
    contact_weight_sum: i128,
    touched_vertices: Vec<usize>,
}

impl G27TightAtomContactChannel {
    pub fn atom_size(&self) -> usize {
        self.atom_size
    }

    pub fn touched_vertex_count(&self) -> usize {
        self.touched_vertex_count
    }

    pub fn contact_weight_sum(&self) -> i128 {
        self.contact_weight_sum
    }

    pub fn touched_vertices(&self) -> &[usize] {
        &self.touched_vertices
    }

    fn stable_token(&self) -> String {
        let vertices = self
            .touched_vertices
            .iter()
            .map(usize::to_string)
            .collect::<Vec<_>>()
            .join("-");
        format!(
            "mask{:x}:size{}:touched{}:weight{}:vertices{}",
            self.atom_mask,
            self.atom_size,
            self.touched_vertex_count,
            self.contact_weight_sum,
            vertices
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct G27SameFieldTightAtomContactReport {
    core: HadwigerArtifactCore,
    g27_anchor: usize,
    w_anchor: usize,
    exact_contact_count: usize,
    contact_weight_sum: i128,
    contacted_g27_vertex_count: usize,
    tight_atom_count: usize,
    contacted_tight_atom_count: usize,
    top_channels: Vec<G27TightAtomContactChannel>,
    posture: G27TightAtomContactPosture,
    conclusion: String,
}

impl G27SameFieldTightAtomContactReport {
    pub fn g27_anchor(&self) -> usize {
        self.g27_anchor
    }

    pub fn w_anchor(&self) -> usize {
        self.w_anchor
    }

    pub fn exact_contact_count(&self) -> usize {
        self.exact_contact_count
    }

    pub fn contact_weight_sum(&self) -> i128 {
        self.contact_weight_sum
    }

    pub fn contacted_g27_vertex_count(&self) -> usize {
        self.contacted_g27_vertex_count
    }

    pub fn tight_atom_count(&self) -> usize {
        self.tight_atom_count
    }

    pub fn contacted_tight_atom_count(&self) -> usize {
        self.contacted_tight_atom_count
    }

    pub fn top_channels(&self) -> &[G27TightAtomContactChannel] {
        &self.top_channels
    }

    pub fn posture(&self) -> G27TightAtomContactPosture {
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

impl_hadwiger_artifact!(G27SameFieldTightAtomContactReport, core);

pub fn analyze_g27_w_circles_tight_atom_contacts_checked(
    handle: &HadwigerResearchHandle,
) -> Result<G27SameFieldTightAtomContactReport, G27GeometricFractionalError> {
    let exact_geometry = audit_g27_w_circles_607_exact_geometry_checked(handle)?;
    let g_points = g27_points(&retained_g27_coefficients()?)?;
    let w_points = parse_w_vertices()?;
    let w_weights = parse_w_integer_weights()?;
    let contact_weights = contact_weights_by_g27_vertex(&g_points, &w_points, &w_weights);
    let tight_atoms = retained_g27_tight_atom_masks()?;
    let mut channels = tight_atom_channels(&tight_atoms, &contact_weights);
    channels.sort_by(|left, right| {
        right
            .contact_weight_sum
            .cmp(&left.contact_weight_sum)
            .then_with(|| right.touched_vertex_count.cmp(&left.touched_vertex_count))
            .then_with(|| left.stable_token().cmp(&right.stable_token()))
    });
    let contacted_tight_atom_count = channels
        .iter()
        .filter(|channel| channel.contact_weight_sum > 0)
        .count();
    channels.truncate(TOP_ATOM_LIMIT);
    let exact_contact_count = contact_weights.iter().map(|row| row.1).sum::<usize>();
    let contact_weight_sum = contact_weights.iter().map(|row| row.2).sum::<i128>();
    let contacted_g27_vertex_count = contact_weights
        .iter()
        .filter(|(_, count, _)| *count > 0)
        .count();
    let posture = if channels
        .first()
        .is_some_and(|channel| channel.contact_weight_sum >= LIFT_GAP_WEIGHT_NUMERATOR)
    {
        G27TightAtomContactPosture::FundFixedDualPricing
    } else {
        G27TightAtomContactPosture::RetiredWeakTightAtomContact
    };
    let conclusion = conclusion(posture, channels.first());
    let core = artifact_core(
        HadwigerArtifactKind::G27SameFieldPressureInterfaceSearchReport,
        HadwigerArtifactAuthorityOwner::HadwigerArtifactBuilder,
        HadwigerArtifactSourceReference::ArtifactConstruction {
            operation: "g27_w_circles_tight_atom_contact".to_string(),
        },
        vec![exact_geometry.reference()],
        payload(
            exact_contact_count,
            contact_weight_sum,
            contacted_g27_vertex_count,
            tight_atoms.len(),
            contacted_tight_atom_count,
            &channels,
            posture,
            &conclusion,
        ),
    )?;
    Ok(G27SameFieldTightAtomContactReport {
        core,
        g27_anchor: G27_ANCHOR_INDEX + 1,
        w_anchor: W_ANCHOR_INDEX + 1,
        exact_contact_count,
        contact_weight_sum,
        contacted_g27_vertex_count,
        tight_atom_count: tight_atoms.len(),
        contacted_tight_atom_count,
        top_channels: channels,
        posture,
        conclusion,
    })
}

fn contact_weights_by_g27_vertex(
    g_points: &[super::g27_w_circles_exact_geometry_support::WExactPoint],
    w_points: &[super::g27_w_circles_exact_geometry_support::WExactPoint],
    w_weights: &[i128],
) -> Vec<(usize, usize, i128)> {
    let translation = g_points[G27_ANCHOR_INDEX].sub(w_points[W_ANCHOR_INDEX]);
    let mut rows = (0..g_points.len())
        .map(|index| (index, 0usize, 0i128))
        .collect::<Vec<_>>();
    for (g_index, g_point) in g_points.iter().enumerate() {
        for (w_index, w_point) in w_points.iter().enumerate() {
            if g_index == G27_ANCHOR_INDEX && w_index == W_ANCHOR_INDEX {
                continue;
            }
            let translated_w = w_point.add(translation);
            if approx_unit_distance(*g_point, translated_w)
                && squared_distance(*g_point, translated_w).is_one()
            {
                rows[g_index].1 += 1;
                rows[g_index].2 += w_weights[w_index];
            }
        }
    }
    rows
}

fn tight_atom_channels(
    tight_atoms: &[u32],
    contact_weights: &[(usize, usize, i128)],
) -> Vec<G27TightAtomContactChannel> {
    tight_atoms
        .iter()
        .map(|atom| {
            let touched_vertices = contact_weights
                .iter()
                .filter_map(|(index, count, _)| {
                    if *count > 0 && atom & (1u32 << index) != 0 {
                        Some(index + 1)
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>();
            let contact_weight_sum = contact_weights
                .iter()
                .filter(|(index, _, _)| atom & (1u32 << index) != 0)
                .map(|(_, _, weight)| *weight)
                .sum();
            G27TightAtomContactChannel {
                atom_mask: *atom,
                atom_size: atom.count_ones() as usize,
                touched_vertex_count: touched_vertices.len(),
                contact_weight_sum,
                touched_vertices,
            }
        })
        .collect()
}

fn conclusion(
    posture: G27TightAtomContactPosture,
    top: Option<&G27TightAtomContactChannel>,
) -> String {
    let top = top
        .map(G27TightAtomContactChannel::stable_token)
        .unwrap_or_else(|| "none".to_string());
    match posture {
        G27TightAtomContactPosture::FundFixedDualPricing => format!(
            "fund exact fixed-dual pricing: same-field donor contacts hit a retained tight atom above the 51749 lift numerator ({top})"
        ),
        G27TightAtomContactPosture::RetiredWeakTightAtomContact => format!(
            "retire tight-atom contact triage: no retained tight atom receives enough same-field donor contact weight ({top})"
        ),
    }
}

fn payload(
    exact_contact_count: usize,
    contact_weight_sum: i128,
    contacted_g27_vertex_count: usize,
    tight_atom_count: usize,
    contacted_tight_atom_count: usize,
    top_channels: &[G27TightAtomContactChannel],
    posture: G27TightAtomContactPosture,
    conclusion: &str,
) -> Vec<HadwigerArtifactPayloadEntry> {
    let mut payload = vec![
        HadwigerArtifactPayloadEntry::text(
            "schema",
            "forge.hadwiger.g27_same_field_tight_atom_contact.v1",
        ),
        HadwigerArtifactPayloadEntry::unsigned("g27_anchor", (G27_ANCHOR_INDEX + 1) as u128),
        HadwigerArtifactPayloadEntry::unsigned("w_anchor", (W_ANCHOR_INDEX + 1) as u128),
        HadwigerArtifactPayloadEntry::unsigned("exact_contact_count", exact_contact_count as u128),
        HadwigerArtifactPayloadEntry::unsigned("contact_weight_sum", contact_weight_sum as u128),
        HadwigerArtifactPayloadEntry::unsigned(
            "contacted_g27_vertex_count",
            contacted_g27_vertex_count as u128,
        ),
        HadwigerArtifactPayloadEntry::unsigned("tight_atom_count", tight_atom_count as u128),
        HadwigerArtifactPayloadEntry::unsigned(
            "contacted_tight_atom_count",
            contacted_tight_atom_count as u128,
        ),
        HadwigerArtifactPayloadEntry::text("posture", posture.as_str()),
        HadwigerArtifactPayloadEntry::text("conclusion", conclusion),
    ];
    for channel in top_channels {
        payload.push(HadwigerArtifactPayloadEntry::text(
            "top_tight_atom",
            channel.stable_token(),
        ));
    }
    payload
}
