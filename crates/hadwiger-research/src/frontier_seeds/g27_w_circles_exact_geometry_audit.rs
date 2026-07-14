use sha2::{Digest, Sha256};

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
    parse_w_retained_edges, parse_w_vertices, replay_w_unit_edges, EXPECTED_EDGE_COUNT,
    EXPECTED_VERTEX_COUNT, W_VERTICES,
};

const VERTEX_SHA256: &str =
    "sha256:5ccc75a58b5768f49816c4231a228f4e0430118f5fafa03f0f660e23c0469e95";

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct G27WCirclesExactGeometryAuditReport {
    core: HadwigerArtifactCore,
    vertex_count: usize,
    retained_edge_count: usize,
    replayed_edge_count: usize,
    vertex_source_sha256: String,
    shared_field_basis: Vec<String>,
    conclusion: String,
}

impl G27WCirclesExactGeometryAuditReport {
    pub fn vertex_count(&self) -> usize {
        self.vertex_count
    }

    pub fn retained_edge_count(&self) -> usize {
        self.retained_edge_count
    }

    pub fn replayed_edge_count(&self) -> usize {
        self.replayed_edge_count
    }

    pub fn vertex_source_sha256(&self) -> &str {
        &self.vertex_source_sha256
    }

    pub fn shared_field_basis(&self) -> &[String] {
        &self.shared_field_basis
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

impl_hadwiger_artifact!(G27WCirclesExactGeometryAuditReport, core);

pub fn audit_g27_w_circles_607_exact_geometry_checked(
    handle: &HadwigerResearchHandle,
) -> Result<G27WCirclesExactGeometryAuditReport, G27GeometricFractionalError> {
    let finite_core = audit_g27_w_circles_607_finite_fractional_core_checked(handle)?;
    if sha256_token(W_VERTICES.as_bytes()) != VERTEX_SHA256 {
        return Err(G27GeometricFractionalError::MalformedData {
            source: "w_circles_607_vertices_sha256",
        });
    }
    let vertices = parse_w_vertices()?;
    let retained_edges = parse_w_retained_edges(vertices.len())?;
    let replayed_edges = replay_w_unit_edges(&vertices);
    if vertices.len() != EXPECTED_VERTEX_COUNT
        || retained_edges.len() != EXPECTED_EDGE_COUNT
        || replayed_edges != retained_edges
    {
        return Err(G27GeometricFractionalError::MalformedData {
            source: "w_circles_607_exact_geometry_replay",
        });
    }
    let shared_field_basis = shared_field_basis();
    let conclusion = "W_circles_607 exact geometry replays in the same Q(sqrt3,sqrt11,sqrt33) field as retained G27; same-field pressure-donor fusion is eligible for a real interface search"
        .to_string();
    let core = artifact_core(
        HadwigerArtifactKind::G27WCirclesExactGeometryAuditReport,
        HadwigerArtifactAuthorityOwner::HadwigerArtifactBuilder,
        HadwigerArtifactSourceReference::ArtifactConstruction {
            operation: "g27_w_circles_607_exact_geometry_audit".to_string(),
        },
        vec![finite_core.reference()],
        payload(
            vertices.len(),
            retained_edges.len(),
            replayed_edges.len(),
            &shared_field_basis,
            &conclusion,
        ),
    )?;
    Ok(G27WCirclesExactGeometryAuditReport {
        core,
        vertex_count: vertices.len(),
        retained_edge_count: retained_edges.len(),
        replayed_edge_count: replayed_edges.len(),
        vertex_source_sha256: VERTEX_SHA256.to_string(),
        shared_field_basis,
        conclusion,
    })
}

fn shared_field_basis() -> Vec<String> {
    vec![
        "1".to_string(),
        "sqrt3".to_string(),
        "sqrt11".to_string(),
        "sqrt33".to_string(),
    ]
}

fn payload(
    vertex_count: usize,
    retained_edge_count: usize,
    replayed_edge_count: usize,
    shared_field_basis: &[String],
    conclusion: &str,
) -> Vec<HadwigerArtifactPayloadEntry> {
    let mut payload = vec![
        HadwigerArtifactPayloadEntry::text(
            "schema",
            "forge.hadwiger.g27_w_circles_exact_geometry.v1",
        ),
        HadwigerArtifactPayloadEntry::unsigned("vertex_count", vertex_count as u128),
        HadwigerArtifactPayloadEntry::unsigned("retained_edge_count", retained_edge_count as u128),
        HadwigerArtifactPayloadEntry::unsigned("replayed_edge_count", replayed_edge_count as u128),
        HadwigerArtifactPayloadEntry::text("vertex_source_sha256", VERTEX_SHA256),
        HadwigerArtifactPayloadEntry::text("conclusion", conclusion),
    ];
    for basis in shared_field_basis {
        payload.push(HadwigerArtifactPayloadEntry::text(
            "shared_field_basis",
            basis,
        ));
    }
    payload
}

fn sha256_token(bytes: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    format!("sha256:{:x}", hasher.finalize())
}
