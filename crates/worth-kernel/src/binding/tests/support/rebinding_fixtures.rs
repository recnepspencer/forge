use worth_geom::facade::{ParameterDomain, ParameterSpacePoint};
use worth_spatial::facade::bindings::{
    author_primitive_anchor_binding_declaration, author_primitive_rebinding_declaration,
    AnchorCarrierOwnership, AuthorPrimitiveAnchorBindingIntent, AuthorPrimitiveRebindingIntent,
    CarrierOwnedParameterPointAnchorSpec, FaceBindingSite, FaceSurfaceBindingSpec,
    LocalTopologyReplacementNeighborhood, NeighborhoodBindingFamily,
    PrimitiveAnchorBindingDeclarationEntry, PrimitiveRebindingDeclarationEntry,
    PrimitiveRebindingPriorBindingFact, ReplacementCandidate, ReplacementCandidateSet,
};

use super::{
    canonical_geometry, orthotope_contract, rebinding_candidate_from_anchor_declaration,
    rebinding_prior_fact_from_anchor_declaration,
};

pub(crate) struct FaceSurfaceRebindingFixture {
    pub(crate) declaration: PrimitiveRebindingDeclarationEntry,
    pub(crate) successor_identity: String,
}

fn anchored_surface_identity(
    face_identity: &str,
    persistent_name: &str,
    parameter: [f64; 2],
    extent: f64,
) -> String {
    anchored_surface_prior_fact_from_declaration(
        &anchored_surface_declaration(face_identity, persistent_name, parameter, extent),
        "rebinding-fixtures-anchored-surface-identity",
    )
    .prior_binding_identity()
    .to_string()
}

pub(crate) fn anchored_surface_declaration(
    face_identity: &str,
    persistent_name: &str,
    parameter: [f64; 2],
    extent: f64,
) -> PrimitiveAnchorBindingDeclarationEntry {
    author_primitive_anchor_binding_declaration(
        AuthorPrimitiveAnchorBindingIntent::attach_parameter_space_point_to_face(
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
        ),
    )
}

pub(crate) fn anchored_surface_prior_fact_from_declaration(
    declaration: &PrimitiveAnchorBindingDeclarationEntry,
    world: &'static str,
) -> PrimitiveRebindingPriorBindingFact {
    rebinding_prior_fact_from_anchor_declaration(declaration, world)
}

pub(crate) fn anchored_surface_candidate_from_declaration(
    label: impl Into<String>,
    declaration: &PrimitiveAnchorBindingDeclarationEntry,
    world: &'static str,
) -> Result<ReplacementCandidate, worth_spatial::facade::bindings::SpatialRebindingAuthorityError> {
    rebinding_candidate_from_anchor_declaration(label, declaration, world)
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

pub(crate) fn face_surface_rebinding_fixture() -> FaceSurfaceRebindingFixture {
    let prior = anchored_surface_declaration("face-old", "surface-alpha", [0.25, 0.5], 1.0);
    let successor = anchored_surface_declaration("face-new", "surface-beta", [0.25, 0.5], 1.0);
    let successor_identity =
        anchored_surface_identity("face-new", "surface-beta", [0.25, 0.5], 1.0);
    let neighborhood = replacement_neighborhood(
        NeighborhoodBindingFamily::FaceSurfacePointAnchor,
        "face-old",
        vec![anchored_surface_candidate_from_declaration(
            "successor",
            &successor,
            "face-surface-rebinding-fixture-successor",
        )
        .expect("successor candidate")],
    );
    let declaration = author_primitive_rebinding_declaration(
        AuthorPrimitiveRebindingIntent::replace_surface_binding(
            anchored_surface_prior_fact_from_declaration(
                &prior,
                "face-surface-rebinding-fixture-prior",
            ),
            neighborhood,
        ),
    );
    FaceSurfaceRebindingFixture {
        declaration,
        successor_identity,
    }
}

pub(crate) fn retained_digest_for_receipt(
    receipt: &worth_spatial::facade::bindings::PrimitiveRebindingFactReceipt,
) -> String {
    let mut candidate_identities = receipt.candidate_identities().to_vec();
    let mut candidate_labels = receipt.candidate_labels().to_vec();
    let mut candidate_sites = receipt.candidate_site_identities().to_vec();
    candidate_identities.sort();
    candidate_labels.sort();
    candidate_sites.sort();
    worth_primitives::truth_digest_parts(
        worth_primitives::TruthDigestScope::ArtifactIdentity,
        &[
            format!("outcome:{:?}", receipt.outcome_class()),
            format!("continuity:{:?}", receipt.continuity_class()),
            format!("motion:{:?}", receipt.motion_posture()),
            format!("family:{:?}", receipt.neighborhood_family()),
            format!("prior:{}", receipt.prior_binding_identity()),
            format!("prior_site:{}", receipt.prior_site_identity()),
            format!(
                "selected_identity:{}",
                receipt.selected_candidate_identity().unwrap_or("none")
            ),
            format!(
                "selected_label:{}",
                receipt.selected_candidate_label().unwrap_or("none")
            ),
            format!("unsupported:{:?}", receipt.unsupported_reason()),
            format!("candidate_identities:{}", candidate_identities.join("|")),
            format!("candidate_labels:{}", candidate_labels.join("|")),
            format!("candidate_sites:{}", candidate_sites.join("|")),
        ],
    )
}
