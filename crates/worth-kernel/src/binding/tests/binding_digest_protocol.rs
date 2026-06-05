use std::collections::BTreeSet;

use worth_geom::facade::{ParameterDomain, ParameterSpacePoint};
use worth_spatial::facade::bindings::{
    attach_parameter_space_point_to_face, attach_surface_to_face, AnchorCarrierOwnership,
    CarrierOwnedParameterPointAnchorSpec, FaceBindingSite, FaceSurfaceBindingSpec,
    LocalTopologyReplacementNeighborhood, NeighborhoodBindingFamily, ReplacementCandidate,
    ReplacementCandidateSet, SpatialAdmittedPrimitiveBinding,
};

use crate::facade::authoring::anchoring::{
    author_primitive_anchor_binding_declaration, AuthorPrimitiveAnchorBindingIntent,
};
use crate::facade::authoring::binding::{
    author_primitive_binding_declaration, author_primitive_rebinding_declaration,
    AuthorPrimitiveBindingIntent, AuthorPrimitiveRebindingIntent,
};

use super::support::{
    admitted_anchor_binding_handle, admitted_binding_handle, admitted_rebinding_handle,
    anchor_declaration_digest_string, anchor_inspection_digest_string,
    anchor_progression_digest_string, canonical_geometry, canonical_text_entries,
    canonical_text_entries_for_anchor_binding, canonical_text_entries_for_rebinding,
    declaration_digest_string, inspect_progressed_binding_entry,
    inspect_progressed_rebinding_entry, orthotope_contract, progress_binding_entry,
    progress_rebinding_entry, rebinding_declaration_digest_string,
};

#[test]
fn canonical_binding_identity_digest_protocol_is_shared_across_kernel_spatial_and_retained_paths() {
    let first_binding = binding_entry(
        "face-1",
        "surface-alpha",
        [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]],
    );
    let second_binding =
        binding_entry("face-1", "surface-beta", [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]);
    let direct_first = direct_binding_identity(
        "face-1",
        "surface-alpha",
        [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]],
    );
    let direct_second =
        direct_binding_identity("face-1", "surface-beta", [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]);
    let binding_handle = admitted_binding_handle("phase5-binding");
    let first_progression = progress_binding_entry(&first_binding, &binding_handle);
    let second_progression = progress_binding_entry(&second_binding, &binding_handle);
    let first_inspection =
        inspect_progressed_binding_entry(&binding_handle, first_progression.clone());
    let second_inspection =
        inspect_progressed_binding_entry(&binding_handle, second_progression.clone());

    assert_eq!(direct_first, direct_second);
    assert_eq!(
        canonical_text_entries(&first_binding),
        canonical_text_entries(&second_binding)
    );
    assert!(!canonical_text_entries(&first_binding).contains_key("persistent_name"));
    assert_eq!(
        declaration_digest_string(&first_progression),
        declaration_digest_string(&second_progression)
    );
    assert_eq!(
        first_progression.progression_digest(),
        second_progression.progression_digest()
    );
    assert_eq!(
        first_inspection.inspection_digest(),
        second_inspection.inspection_digest()
    );

    let first_anchor = anchor_entry("face-1", "surface-alpha", [0.25, 3.0]);
    let second_anchor = anchor_entry(
        "face-1",
        "surface-beta",
        [std::f64::consts::TAU + 0.25, 3.0],
    );
    let direct_first_anchor = direct_anchor("face-1", "surface-alpha", [0.25, 3.0]);
    let direct_second_anchor = direct_anchor(
        "face-1",
        "surface-beta",
        [std::f64::consts::TAU + 0.25, 3.0],
    );
    let anchor_handle = admitted_anchor_binding_handle("phase5-anchor");

    assert_eq!(
        direct_first_anchor.identity(),
        direct_second_anchor.identity()
    );
    assert_eq!(
        canonical_text_entries_for_anchor_binding(&first_anchor),
        canonical_text_entries_for_anchor_binding(&second_anchor)
    );
    assert_eq!(
        anchor_declaration_digest_string(&first_anchor, &anchor_handle),
        anchor_declaration_digest_string(&second_anchor, &anchor_handle)
    );
    assert_eq!(
        anchor_progression_digest_string(&first_anchor, &anchor_handle),
        anchor_progression_digest_string(&second_anchor, &anchor_handle)
    );
    assert_eq!(
        anchor_inspection_digest_string(&first_anchor, &anchor_handle),
        anchor_inspection_digest_string(&second_anchor, &anchor_handle)
    );

    let prior = direct_first_anchor.clone();
    let exact = direct_anchor("face-new-a", "surface-gamma", [0.25, 3.0]);
    let weaker = direct_anchor("face-new-b", "surface-delta", [0.5, 3.0]);
    let neighborhood = replacement_neighborhood(prior.clone(), exact.clone(), weaker.clone());
    let rebinding = author_primitive_rebinding_declaration(
        AuthorPrimitiveRebindingIntent::replace_surface_binding(
            SpatialAdmittedPrimitiveBinding::FaceSurfacePointAnchor(prior.clone()),
            neighborhood,
        ),
    );
    let rebinding_handle = admitted_rebinding_handle("phase5-rebinding");
    let rebinding_progression = progress_rebinding_entry(&rebinding, &rebinding_handle);
    let rebinding_inspection =
        inspect_progressed_rebinding_entry(&rebinding_handle, rebinding_progression.clone());
    let rebinding_entries = canonical_text_entries_for_rebinding(&rebinding);

    assert_eq!(
        rebinding_entries.get("prior_identity").map(String::as_str),
        Some(direct_first_anchor.identity().as_str())
    );
    assert_eq!(
        rebinding_entries
            .iter()
            .filter_map(|(key, value)| key.ends_with(".identity").then_some(value.clone()))
            .collect::<BTreeSet<_>>(),
        BTreeSet::from([
            exact.identity().as_str().to_string(),
            weaker.identity().as_str().to_string(),
        ])
    );
    assert_eq!(
        rebinding_entries.get("binding_kind").map(String::as_str),
        Some("face_surface")
    );
    assert_eq!(
        rebinding_declaration_digest_string(&rebinding_progression),
        format!(
            "{:?}",
            rebinding_progression
                .canonical_declaration()
                .declaration_digest()
        )
    );
    assert_eq!(
        Some(rebinding_progression.progression_digest()),
        rebinding_inspection.progression_digest()
    );
}

#[test]
fn canonical_binding_digest_changes_when_geometry_meaning_changes_but_not_when_formatting_changes()
{
    let first = binding_entry(
        "face-1",
        "surface-alpha",
        [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]],
    );
    let renamed = binding_entry("face-1", "surface-beta", [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]);
    let changed = binding_entry(
        "face-1",
        "surface-alpha",
        [[0.0, 0.0, 0.0], [2.0, 0.0, 0.0]],
    );
    let direct_first = direct_binding_identity(
        "face-1",
        "surface-alpha",
        [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]],
    );
    let direct_renamed =
        direct_binding_identity("face-1", "surface-beta", [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]);
    let direct_changed = direct_binding_identity(
        "face-1",
        "surface-alpha",
        [[0.0, 0.0, 0.0], [2.0, 0.0, 0.0]],
    );
    let handle = admitted_binding_handle("phase5-geometry");
    let first_progression = progress_binding_entry(&first, &handle);
    let renamed_progression = progress_binding_entry(&renamed, &handle);
    let changed_progression = progress_binding_entry(&changed, &handle);
    let first_inspection = inspect_progressed_binding_entry(&handle, first_progression.clone());
    let renamed_inspection = inspect_progressed_binding_entry(&handle, renamed_progression.clone());
    let changed_inspection = inspect_progressed_binding_entry(&handle, changed_progression.clone());
    let first_entries = canonical_text_entries(&first);
    let renamed_entries = canonical_text_entries(&renamed);
    let changed_entries = canonical_text_entries(&changed);

    assert_eq!(direct_first, direct_renamed);
    assert_ne!(direct_first, direct_changed);
    assert_eq!(first_entries, renamed_entries);
    assert_ne!(
        first_entries.get("geometry_digest"),
        changed_entries.get("geometry_digest")
    );
    assert!(!first_entries.contains_key("persistent_name"));
    assert_eq!(
        declaration_digest_string(&first_progression),
        declaration_digest_string(&renamed_progression)
    );
    assert_ne!(
        declaration_digest_string(&first_progression),
        declaration_digest_string(&changed_progression)
    );
    assert_eq!(
        first_progression.progression_digest(),
        renamed_progression.progression_digest()
    );
    assert_ne!(
        first_progression.progression_digest(),
        changed_progression.progression_digest()
    );
    assert_eq!(
        first_inspection.inspection_digest(),
        renamed_inspection.inspection_digest()
    );
    assert_ne!(
        first_inspection.inspection_digest(),
        changed_inspection.inspection_digest()
    );

    let first_anchor = anchor_entry("face-1", "surface-alpha", [0.25, 3.0]);
    let periodic_anchor = anchor_entry(
        "face-1",
        "surface-beta",
        [std::f64::consts::TAU + 0.25, 3.0],
    );
    let anchor_handle = admitted_anchor_binding_handle("phase5-anchor-format");

    assert_eq!(
        canonical_text_entries_for_anchor_binding(&first_anchor),
        canonical_text_entries_for_anchor_binding(&periodic_anchor)
    );
    assert_eq!(
        anchor_declaration_digest_string(&first_anchor, &anchor_handle),
        anchor_declaration_digest_string(&periodic_anchor, &anchor_handle)
    );
}

fn binding_entry(
    face_identity: &str,
    persistent_name: &str,
    points: [[f64; 3]; 2],
) -> crate::facade::authoring::binding::PrimitiveBindingDeclarationEntry {
    author_primitive_binding_declaration(AuthorPrimitiveBindingIntent::attach_surface_to_face(
        face_binding_spec(face_identity, persistent_name, points),
    ))
}

fn direct_binding_identity(
    face_identity: &str,
    persistent_name: &str,
    points: [[f64; 3]; 2],
) -> String {
    attach_surface_to_face(face_binding_spec(face_identity, persistent_name, points))
        .expect("direct binding")
        .identity()
        .as_str()
        .to_string()
}

fn anchor_entry(
    face_identity: &str,
    persistent_name: &str,
    parameter: [f64; 2],
) -> crate::facade::authoring::anchoring::PrimitiveAnchorBindingDeclarationEntry {
    author_primitive_anchor_binding_declaration(
        AuthorPrimitiveAnchorBindingIntent::attach_parameter_space_point_to_face(
            face_binding_spec(
                face_identity,
                persistent_name,
                [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]],
            ),
            point_anchor_spec(face_identity, parameter),
        ),
    )
}

fn direct_anchor(
    face_identity: &str,
    persistent_name: &str,
    parameter: [f64; 2],
) -> worth_spatial::facade::bindings::AdmittedFaceSurfacePointAnchorBinding {
    attach_parameter_space_point_to_face(
        face_binding_spec(
            face_identity,
            persistent_name,
            [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]],
        ),
        point_anchor_spec(face_identity, parameter),
    )
    .expect("direct anchor")
}

fn face_binding_spec(
    face_identity: &str,
    persistent_name: &str,
    points: [[f64; 3]; 2],
) -> FaceSurfaceBindingSpec {
    FaceSurfaceBindingSpec::new(
        FaceBindingSite::new(face_identity).with_persistent_name(persistent_name),
        orthotope_contract(),
        canonical_geometry(points),
    )
}

fn point_anchor_spec(
    face_identity: &str,
    parameter: [f64; 2],
) -> CarrierOwnedParameterPointAnchorSpec {
    CarrierOwnedParameterPointAnchorSpec::new(
        AnchorCarrierOwnership::for_face_surface(face_identity, ParameterDomain::cylinder())
            .expect("face ownership"),
        ParameterSpacePoint::try_new(parameter).expect("parameter point"),
    )
    .expect("point anchor spec")
}

fn replacement_neighborhood(
    prior: worth_spatial::facade::bindings::AdmittedFaceSurfacePointAnchorBinding,
    first: worth_spatial::facade::bindings::AdmittedFaceSurfacePointAnchorBinding,
    second: worth_spatial::facade::bindings::AdmittedFaceSurfacePointAnchorBinding,
) -> LocalTopologyReplacementNeighborhood {
    LocalTopologyReplacementNeighborhood::new(
        NeighborhoodBindingFamily::FaceSurfacePointAnchor,
        prior.binding().site().topology_face_identity(),
        ReplacementCandidateSet::new(vec![
            ReplacementCandidate::new(
                "first",
                SpatialAdmittedPrimitiveBinding::FaceSurfacePointAnchor(first),
            )
            .expect("first candidate"),
            ReplacementCandidate::new(
                "second",
                SpatialAdmittedPrimitiveBinding::FaceSurfacePointAnchor(second),
            )
            .expect("second candidate"),
        ])
        .expect("candidate set"),
    )
    .expect("replacement neighborhood")
}
