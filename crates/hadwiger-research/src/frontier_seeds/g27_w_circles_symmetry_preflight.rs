use std::collections::BTreeSet;

use crate::domain_artifacts::core_artifact::{
    impl_hadwiger_artifact, HadwigerArtifactAuthorityOwner, HadwigerArtifactCore,
    HadwigerArtifactKind, HadwigerArtifactSourceReference,
};
use crate::domain_artifacts::digest_basis::{artifact_core, HadwigerArtifactPayloadEntry};
use crate::domain_artifacts::HadwigerCanonicalArtifact;
use crate::query_entry::HadwigerResearchHandle;

use super::g27_finite_fractional_core_audit::audit_g27_w_circles_607_finite_fractional_core_checked;
use super::g27_geometric_fractional::G27GeometricFractionalError;
use super::g27_w_circles_exact_geometry_support::{
    parse_w_integer_weights, parse_w_retained_edges, parse_w_vertices, WExactPoint,
    EXPECTED_EDGE_COUNT, EXPECTED_VERTEX_COUNT, K4,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum G27WCirclesSymmetryTransformStatus {
    ValidWeightedAutomorphism,
    MissingTransformedVertex,
    WeightMismatch,
    EdgeMismatch,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct G27WCirclesSymmetryTransformRow {
    name: String,
    status: G27WCirclesSymmetryTransformStatus,
    fixed_vertex_count: usize,
}

impl G27WCirclesSymmetryTransformRow {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn status(&self) -> G27WCirclesSymmetryTransformStatus {
        self.status
    }

    pub fn fixed_vertex_count(&self) -> usize {
        self.fixed_vertex_count
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum G27WCirclesSymmetryPreflightStatus {
    FundOrbitAwareBranchCertificate,
    WeakSymmetryRecordOnly,
    RetiredSymmetryCompression,
}

impl G27WCirclesSymmetryPreflightStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::FundOrbitAwareBranchCertificate => "fund_orbit_aware_branch_certificate",
            Self::WeakSymmetryRecordOnly => "weak_symmetry_record_only",
            Self::RetiredSymmetryCompression => "retired_symmetry_compression",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct G27WCirclesSymmetryPreflightReport {
    core: HadwigerArtifactCore,
    vertex_count: usize,
    edge_count: usize,
    weight_count: usize,
    transform_rows: Vec<G27WCirclesSymmetryTransformRow>,
    valid_transform_count: usize,
    group_size: usize,
    vertex_orbit_count: usize,
    largest_vertex_orbit: usize,
    singleton_vertex_orbit_count: usize,
    edge_orbit_count: usize,
    status: G27WCirclesSymmetryPreflightStatus,
    conclusion: String,
}

impl G27WCirclesSymmetryPreflightReport {
    pub fn shape_summary(&self) -> (usize, usize, usize) {
        (self.vertex_count, self.edge_count, self.weight_count)
    }

    pub fn symmetry_summary(&self) -> (usize, usize, usize, usize, usize, usize) {
        (
            self.valid_transform_count,
            self.group_size,
            self.vertex_orbit_count,
            self.largest_vertex_orbit,
            self.singleton_vertex_orbit_count,
            self.edge_orbit_count,
        )
    }

    pub fn transform_rows(&self) -> &[G27WCirclesSymmetryTransformRow] {
        &self.transform_rows
    }

    pub fn status(&self) -> G27WCirclesSymmetryPreflightStatus {
        self.status
    }

    pub fn conclusion(&self) -> &str {
        &self.conclusion
    }

    pub fn admits_theorem_authority(&self) -> bool {
        false
    }
}

impl_hadwiger_artifact!(G27WCirclesSymmetryPreflightReport, core);

pub fn preflight_g27_w_circles_symmetry_checked(
    handle: &HadwigerResearchHandle,
) -> Result<G27WCirclesSymmetryPreflightReport, G27GeometricFractionalError> {
    let source = audit_g27_w_circles_607_finite_fractional_core_checked(handle)?;
    let vertices = parse_w_vertices()?;
    let weights = parse_w_integer_weights()?;
    let edges = parse_w_retained_edges(EXPECTED_VERTEX_COUNT)?;
    if vertices.len() != EXPECTED_VERTEX_COUNT
        || weights.len() != EXPECTED_VERTEX_COUNT
        || edges.len() != EXPECTED_EDGE_COUNT
    {
        return Err(G27GeometricFractionalError::MalformedData {
            source: "w607_symmetry_shape",
        });
    }

    let transform_specs = [
        (
            "identity",
            transform_identity as fn(WExactPoint) -> WExactPoint,
        ),
        ("reflect_y", transform_reflect_y),
        ("reflect_x_eq_3", transform_reflect_x_eq_3),
        ("half_turn_about_3_0", transform_half_turn_about_3_0),
    ];
    let mut transform_rows = Vec::new();
    let mut valid_permutations = Vec::new();
    for (name, transform) in transform_specs {
        let (row, permutation) = screen_transform(name, transform, &vertices, &weights, &edges);
        if let Some(permutation) = permutation {
            valid_permutations.push(permutation);
        }
        transform_rows.push(row);
    }

    let group = permutation_closure(EXPECTED_VERTEX_COUNT, &valid_permutations);
    let vertex_orbits = vertex_orbit_sizes(EXPECTED_VERTEX_COUNT, &group);
    let vertex_orbit_count = vertex_orbits.len();
    let largest_vertex_orbit = vertex_orbits.iter().copied().max().unwrap_or(0);
    let singleton_vertex_orbit_count = vertex_orbits.iter().filter(|size| **size == 1).count();
    let edge_orbit_count = edge_orbit_count(&edges, &group);
    let valid_transform_count = valid_permutations.len();
    let status = status(group.len(), vertex_orbit_count, edge_orbit_count);
    let conclusion = conclusion(status, group.len(), vertex_orbit_count, edge_orbit_count);
    let core = artifact_core(
        HadwigerArtifactKind::G27WCirclesSymmetryPreflightReport,
        HadwigerArtifactAuthorityOwner::HadwigerArtifactBuilder,
        HadwigerArtifactSourceReference::ArtifactConstruction {
            operation: "g27_w_circles_symmetry_preflight".to_string(),
        },
        vec![source.reference()],
        payload(
            valid_transform_count,
            group.len(),
            vertex_orbit_count,
            largest_vertex_orbit,
            singleton_vertex_orbit_count,
            edge_orbit_count,
            status,
            &conclusion,
        ),
    )?;
    Ok(G27WCirclesSymmetryPreflightReport {
        core,
        vertex_count: EXPECTED_VERTEX_COUNT,
        edge_count: EXPECTED_EDGE_COUNT,
        weight_count: weights.len(),
        transform_rows,
        valid_transform_count,
        group_size: group.len(),
        vertex_orbit_count,
        largest_vertex_orbit,
        singleton_vertex_orbit_count,
        edge_orbit_count,
        status,
        conclusion,
    })
}

fn screen_transform(
    name: &str,
    transform: fn(WExactPoint) -> WExactPoint,
    vertices: &[WExactPoint],
    weights: &[i128],
    edges: &BTreeSet<(usize, usize)>,
) -> (G27WCirclesSymmetryTransformRow, Option<Vec<usize>>) {
    let mut permutation = Vec::with_capacity(vertices.len());
    for vertex in vertices {
        let transformed = transform(*vertex);
        let Some(index) = vertices
            .iter()
            .position(|candidate| *candidate == transformed)
        else {
            return (
                transform_row(
                    name,
                    G27WCirclesSymmetryTransformStatus::MissingTransformedVertex,
                    0,
                ),
                None,
            );
        };
        permutation.push(index);
    }
    let fixed_vertex_count = permutation
        .iter()
        .enumerate()
        .filter(|(index, image)| *index == **image)
        .count();
    if (0..weights.len()).any(|index| weights[index] != weights[permutation[index]]) {
        return (
            transform_row(
                name,
                G27WCirclesSymmetryTransformStatus::WeightMismatch,
                fixed_vertex_count,
            ),
            None,
        );
    }
    if edges.iter().any(|(left, right)| {
        let image = normalized_edge(permutation[*left - 1] + 1, permutation[*right - 1] + 1);
        !edges.contains(&image)
    }) {
        return (
            transform_row(
                name,
                G27WCirclesSymmetryTransformStatus::EdgeMismatch,
                fixed_vertex_count,
            ),
            None,
        );
    }
    (
        transform_row(
            name,
            G27WCirclesSymmetryTransformStatus::ValidWeightedAutomorphism,
            fixed_vertex_count,
        ),
        Some(permutation),
    )
}

fn transform_row(
    name: &str,
    status: G27WCirclesSymmetryTransformStatus,
    fixed_vertex_count: usize,
) -> G27WCirclesSymmetryTransformRow {
    G27WCirclesSymmetryTransformRow {
        name: name.to_string(),
        status,
        fixed_vertex_count,
    }
}

fn transform_identity(point: WExactPoint) -> WExactPoint {
    point
}

fn transform_reflect_y(point: WExactPoint) -> WExactPoint {
    WExactPoint {
        x: point.x,
        y: point.y.scale(-1),
    }
}

fn transform_reflect_x_eq_3(point: WExactPoint) -> WExactPoint {
    WExactPoint {
        x: K4::rational(6, 1).sub(point.x),
        y: point.y,
    }
}

fn transform_half_turn_about_3_0(point: WExactPoint) -> WExactPoint {
    WExactPoint {
        x: K4::rational(6, 1).sub(point.x),
        y: point.y.scale(-1),
    }
}

fn permutation_closure(vertex_count: usize, generators: &[Vec<usize>]) -> Vec<Vec<usize>> {
    let identity = (0..vertex_count).collect::<Vec<_>>();
    let mut group = vec![identity];
    let mut changed = true;
    while changed {
        changed = false;
        let snapshot = group.clone();
        for left in &snapshot {
            for right in generators.iter().chain(snapshot.iter()) {
                let composed = compose_permutations(left, right);
                if !group.iter().any(|existing| *existing == composed) {
                    group.push(composed);
                    changed = true;
                }
            }
        }
    }
    group
}

fn compose_permutations(left: &[usize], right: &[usize]) -> Vec<usize> {
    (0..left.len()).map(|index| left[right[index]]).collect()
}

fn vertex_orbit_sizes(vertex_count: usize, group: &[Vec<usize>]) -> Vec<usize> {
    let mut seen = vec![false; vertex_count];
    let mut orbit_sizes = Vec::new();
    for start in 0..vertex_count {
        if seen[start] {
            continue;
        }
        let mut stack = vec![start];
        seen[start] = true;
        let mut size = 0;
        while let Some(vertex) = stack.pop() {
            size += 1;
            for permutation in group {
                let next = permutation[vertex];
                if !seen[next] {
                    seen[next] = true;
                    stack.push(next);
                }
            }
        }
        orbit_sizes.push(size);
    }
    orbit_sizes
}

fn edge_orbit_count(edges: &BTreeSet<(usize, usize)>, group: &[Vec<usize>]) -> usize {
    let mut reps = BTreeSet::new();
    for (left, right) in edges {
        let rep = group
            .iter()
            .map(|permutation| {
                normalized_edge(permutation[*left - 1] + 1, permutation[*right - 1] + 1)
            })
            .min()
            .expect("nonempty permutation group");
        reps.insert(rep);
    }
    reps.len()
}

fn normalized_edge(left: usize, right: usize) -> (usize, usize) {
    if left < right {
        (left, right)
    } else {
        (right, left)
    }
}

fn status(
    group_size: usize,
    vertex_orbit_count: usize,
    edge_orbit_count: usize,
) -> G27WCirclesSymmetryPreflightStatus {
    if group_size >= 4 && vertex_orbit_count <= 200 && edge_orbit_count * 3 <= EXPECTED_EDGE_COUNT {
        G27WCirclesSymmetryPreflightStatus::FundOrbitAwareBranchCertificate
    } else if group_size == 2 && vertex_orbit_count <= 350 {
        G27WCirclesSymmetryPreflightStatus::WeakSymmetryRecordOnly
    } else {
        G27WCirclesSymmetryPreflightStatus::RetiredSymmetryCompression
    }
}

fn conclusion(
    status: G27WCirclesSymmetryPreflightStatus,
    group_size: usize,
    vertex_orbit_count: usize,
    edge_orbit_count: usize,
) -> String {
    match status {
        G27WCirclesSymmetryPreflightStatus::FundOrbitAwareBranchCertificate => format!(
            "weighted W_circles_607 has group size {group_size}, {vertex_orbit_count} vertex orbits, and {edge_orbit_count} edge orbits; fund an orbit-aware branch-certificate schema, not an orbit quotient MWIS shortcut"
        ),
        G27WCirclesSymmetryPreflightStatus::WeakSymmetryRecordOnly => format!(
            "weighted W_circles_607 has only weak symmetry compression: group size {group_size}, {vertex_orbit_count} vertex orbits, and {edge_orbit_count} edge orbits; record it but require real branch-certificate shrinkage before funding"
        ),
        G27WCirclesSymmetryPreflightStatus::RetiredSymmetryCompression => format!(
            "weighted W_circles_607 symmetry compression is not fundable: group size {group_size}, {vertex_orbit_count} vertex orbits, and {edge_orbit_count} edge orbits; use imported proof artifacts or stronger replayable certificates"
        ),
    }
}

fn payload(
    valid_transform_count: usize,
    group_size: usize,
    vertex_orbit_count: usize,
    largest_vertex_orbit: usize,
    singleton_vertex_orbit_count: usize,
    edge_orbit_count: usize,
    status: G27WCirclesSymmetryPreflightStatus,
    conclusion: &str,
) -> Vec<HadwigerArtifactPayloadEntry> {
    vec![
        HadwigerArtifactPayloadEntry::text("schema", "forge.hadwiger.w607_symmetry_preflight.v1"),
        HadwigerArtifactPayloadEntry::unsigned("vertex_count", EXPECTED_VERTEX_COUNT as u128),
        HadwigerArtifactPayloadEntry::unsigned("edge_count", EXPECTED_EDGE_COUNT as u128),
        HadwigerArtifactPayloadEntry::unsigned(
            "valid_transform_count",
            valid_transform_count as u128,
        ),
        HadwigerArtifactPayloadEntry::unsigned("group_size", group_size as u128),
        HadwigerArtifactPayloadEntry::unsigned("vertex_orbit_count", vertex_orbit_count as u128),
        HadwigerArtifactPayloadEntry::unsigned(
            "largest_vertex_orbit",
            largest_vertex_orbit as u128,
        ),
        HadwigerArtifactPayloadEntry::unsigned(
            "singleton_vertex_orbit_count",
            singleton_vertex_orbit_count as u128,
        ),
        HadwigerArtifactPayloadEntry::unsigned("edge_orbit_count", edge_orbit_count as u128),
        HadwigerArtifactPayloadEntry::text("status", status.as_str()),
        HadwigerArtifactPayloadEntry::text("conclusion", conclusion),
    ]
}
