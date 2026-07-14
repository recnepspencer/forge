use crate::domain_artifacts::core_artifact::{
    impl_hadwiger_artifact, HadwigerArtifactAuthorityOwner, HadwigerArtifactCore,
    HadwigerArtifactKind, HadwigerArtifactSourceReference,
};
use crate::domain_artifacts::digest_basis::{artifact_core, HadwigerArtifactPayloadEntry};
use crate::domain_artifacts::HadwigerCanonicalArtifact;
use crate::query_entry::HadwigerResearchHandle;

use super::g27_cross_ring_fusion_preflight::preflight_g27_cross_ring_fusion_column_generation_checked;
use super::g27_exact_moser_basis::audit_g27_exact_moser_basis_checked;
use super::g27_geometric_fractional::G27GeometricFractionalError;
use super::g27_spindle_and_fusion_searches::search_g27_cross_ring_fusion_candidates_checked;

const MIN_SHARED_PRESSURE: usize = 20;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum G27AlgebraicFieldFrictionPosture {
    FundExactForeignGeometryInterfaceReplay,
    RetiredMissingForeignExactGeometry,
    RetiredWeakInterface,
}

impl G27AlgebraicFieldFrictionPosture {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::FundExactForeignGeometryInterfaceReplay => {
                "fund_exact_foreign_geometry_interface_replay"
            }
            Self::RetiredMissingForeignExactGeometry => "retired_missing_foreign_exact_geometry",
            Self::RetiredWeakInterface => "retired_weak_interface",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct G27AlgebraicFieldFrictionCandidate {
    source_label: String,
    foreign_field_token: String,
    shared_vertex: String,
    shared_vertex_pressure: usize,
    retained_foreign_exact_geometry: bool,
    retained_foreign_fractional_core: bool,
    interface_obligation: String,
}

impl G27AlgebraicFieldFrictionCandidate {
    pub fn source_label(&self) -> &str {
        &self.source_label
    }

    pub fn foreign_field_token(&self) -> &str {
        &self.foreign_field_token
    }

    pub fn shared_vertex(&self) -> &str {
        &self.shared_vertex
    }

    pub fn shared_vertex_pressure(&self) -> usize {
        self.shared_vertex_pressure
    }

    pub fn retained_foreign_exact_geometry(&self) -> bool {
        self.retained_foreign_exact_geometry
    }

    pub fn retained_foreign_fractional_core(&self) -> bool {
        self.retained_foreign_fractional_core
    }

    pub fn interface_obligation(&self) -> &str {
        &self.interface_obligation
    }

    fn stable_token(&self) -> String {
        format!(
            "{}:{}:shared{}:pressure{}:exact_geometry{}:finite_core{}:{}",
            self.source_label,
            self.foreign_field_token,
            self.shared_vertex,
            self.shared_vertex_pressure,
            self.retained_foreign_exact_geometry,
            self.retained_foreign_fractional_core,
            self.interface_obligation
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct G27AlgebraicFieldFrictionReport {
    core: HadwigerArtifactCore,
    candidates: Vec<G27AlgebraicFieldFrictionCandidate>,
    posture: G27AlgebraicFieldFrictionPosture,
    conclusion: String,
}

impl G27AlgebraicFieldFrictionReport {
    pub fn candidates(&self) -> &[G27AlgebraicFieldFrictionCandidate] {
        &self.candidates
    }

    pub fn posture(&self) -> G27AlgebraicFieldFrictionPosture {
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

impl_hadwiger_artifact!(G27AlgebraicFieldFrictionReport, core);

pub fn analyze_g27_algebraic_field_friction_checked(
    handle: &HadwigerResearchHandle,
) -> Result<G27AlgebraicFieldFrictionReport, G27GeometricFractionalError> {
    let exact_basis = audit_g27_exact_moser_basis_checked(handle)?;
    let preflight = preflight_g27_cross_ring_fusion_column_generation_checked(handle)?;
    let search = search_g27_cross_ring_fusion_candidates_checked(handle)?;
    let candidates = search
        .retained_candidates()
        .iter()
        .map(|candidate| {
            let shared_pressure = preflight
                .retained_scores()
                .iter()
                .find(|score| {
                    score.core_label() == candidate.core_label()
                        && score.foreign_radicand() == candidate.foreign_radicand()
                })
                .map(|score| score.shared_vertex_pressure())
                .unwrap_or(0);
            let retained_finite_core = candidate.core_label() == "W_circles_607";
            G27AlgebraicFieldFrictionCandidate {
                source_label: candidate.core_label().to_string(),
                foreign_field_token: format!("sqrt{}", candidate.foreign_radicand()),
                shared_vertex: candidate.shared_vertex().to_string(),
                shared_vertex_pressure: shared_pressure,
                retained_foreign_exact_geometry: false,
                retained_foreign_fractional_core: retained_finite_core,
                interface_obligation: interface_obligation(shared_pressure),
            }
        })
        .collect::<Vec<_>>();
    let posture = posture(&candidates);
    let conclusion = conclusion(posture);
    let core = artifact_core(
        HadwigerArtifactKind::G27AlgebraicFieldFrictionReport,
        HadwigerArtifactAuthorityOwner::HadwigerArtifactBuilder,
        HadwigerArtifactSourceReference::ArtifactConstruction {
            operation: "g27_algebraic_field_friction_gate".to_string(),
        },
        vec![
            exact_basis.reference(),
            preflight.reference(),
            search.reference(),
        ],
        payload(&candidates, posture, &conclusion),
    )?;
    Ok(G27AlgebraicFieldFrictionReport {
        core,
        candidates,
        posture,
        conclusion,
    })
}

fn posture(candidates: &[G27AlgebraicFieldFrictionCandidate]) -> G27AlgebraicFieldFrictionPosture {
    if candidates.iter().any(|candidate| {
        candidate.retained_foreign_exact_geometry
            && candidate.shared_vertex_pressure >= MIN_SHARED_PRESSURE
    }) {
        G27AlgebraicFieldFrictionPosture::FundExactForeignGeometryInterfaceReplay
    } else if candidates
        .iter()
        .any(|candidate| candidate.shared_vertex_pressure >= MIN_SHARED_PRESSURE)
    {
        G27AlgebraicFieldFrictionPosture::RetiredMissingForeignExactGeometry
    } else {
        G27AlgebraicFieldFrictionPosture::RetiredWeakInterface
    }
}

fn interface_obligation(shared_pressure: usize) -> String {
    if shared_pressure >= MIN_SHARED_PRESSURE {
        "retain exact foreign coordinate model and replay cross-field unit contacts".to_string()
    } else {
        "shared G27 vertex pressure below field-friction threshold".to_string()
    }
}

fn conclusion(posture: G27AlgebraicFieldFrictionPosture) -> String {
    match posture {
        G27AlgebraicFieldFrictionPosture::FundExactForeignGeometryInterfaceReplay => {
            "fund algebraic-field friction: exact foreign geometry and high-pressure interface are both retained".to_string()
        }
        G27AlgebraicFieldFrictionPosture::RetiredMissingForeignExactGeometry => {
            "retire broad algebraic-field friction for now: high-pressure field labels exist, but no retained exact foreign coordinate model backs the interface".to_string()
        }
        G27AlgebraicFieldFrictionPosture::RetiredWeakInterface => {
            "retire algebraic-field friction: no retained field candidate reaches the G27 interface pressure threshold".to_string()
        }
    }
}

fn payload(
    candidates: &[G27AlgebraicFieldFrictionCandidate],
    posture: G27AlgebraicFieldFrictionPosture,
    conclusion: &str,
) -> Vec<HadwigerArtifactPayloadEntry> {
    let mut payload = vec![
        HadwigerArtifactPayloadEntry::text(
            "schema",
            "forge.hadwiger.g27_algebraic_field_friction.v1",
        ),
        HadwigerArtifactPayloadEntry::unsigned("candidate_count", candidates.len() as u128),
        HadwigerArtifactPayloadEntry::text("posture", posture.as_str()),
        HadwigerArtifactPayloadEntry::text("conclusion", conclusion),
    ];
    for candidate in candidates {
        payload.push(HadwigerArtifactPayloadEntry::text(
            "candidate",
            candidate.stable_token(),
        ));
    }
    payload
}
