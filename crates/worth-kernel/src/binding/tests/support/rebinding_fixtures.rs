use worth_geom::facade::{ParameterDomain, ParameterSpacePoint};
use worth_spatial::facade::bindings::{
    attach_parameter_space_point_to_face, AnchorCarrierOwnership,
    CarrierOwnedParameterPointAnchorSpec, FaceBindingSite, FaceSurfaceBindingSpec,
    LocalTopologyReplacementNeighborhood, NeighborhoodBindingFamily, ReplacementCandidate,
    ReplacementCandidateSet,
};

use super::{canonical_geometry, orthotope_contract};

pub(crate) fn anchored_surface(
    face_identity: &str,
    persistent_name: &str,
    parameter: [f64; 2],
    extent: f64,
) -> worth_spatial::facade::bindings::AdmittedFaceSurfacePointAnchorBinding {
    attach_parameter_space_point_to_face(
        FaceSurfaceBindingSpec::new(
            FaceBindingSite::new(face_identity).with_persistent_name(persistent_name),
            orthotope_contract(),
            canonical_geometry([[0.0, 0.0, 0.0], [extent, 0.0, 0.0]]),
        ),
        CarrierOwnedParameterPointAnchorSpec::new(
            AnchorCarrierOwnership::for_face_surface(face_identity, ParameterDomain::plane())
                .expect("ownership"),
            ParameterSpacePoint::try_new(parameter).expect("parameter"),
        )
        .expect("anchor spec"),
    )
    .expect("anchored surface")
}

pub(crate) fn replacement_neighborhood(
    family: NeighborhoodBindingFamily,
    prior_site: &str,
    candidates: Vec<ReplacementCandidate>,
) -> LocalTopologyReplacementNeighborhood {
    LocalTopologyReplacementNeighborhood::new(
        family,
        prior_site,
        ReplacementCandidateSet::new(candidates).expect("candidate set"),
    )
    .expect("replacement neighborhood")
}

pub(crate) fn retained_digest_for_decision(
    decision: &worth_spatial::facade::bindings::AdmittedRebindingDecision,
) -> String {
    let explanation = decision.explanation();
    let mut candidate_identities = explanation.candidate_identities().to_vec();
    let mut candidate_labels = explanation.candidate_labels().to_vec();
    let mut candidate_sites = explanation.candidate_site_identities().to_vec();
    candidate_identities.sort();
    candidate_labels.sort();
    candidate_sites.sort();
    worth_primitives::truth_digest_parts(
        worth_primitives::TruthDigestScope::ArtifactIdentity,
        &[
            format!("outcome:{:?}", decision.outcome_class()),
            format!("continuity:{:?}", explanation.continuity_class()),
            format!("motion:{:?}", explanation.motion_posture()),
            format!("family:{:?}", explanation.neighborhood_family()),
            format!("prior:{}", explanation.prior_identity()),
            format!("prior_site:{}", explanation.prior_site_identity()),
            format!(
                "selected_identity:{}",
                explanation.selected_candidate_identity().unwrap_or("none")
            ),
            format!(
                "selected_label:{}",
                explanation.selected_candidate_label().unwrap_or("none")
            ),
            format!("unsupported:{:?}", explanation.unsupported_reason()),
            format!("candidate_identities:{}", candidate_identities.join("|")),
            format!("candidate_labels:{}", candidate_labels.join("|")),
            format!("candidate_sites:{}", candidate_sites.join("|")),
        ],
    )
}
