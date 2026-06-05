use forge_query::facade::{
    ForgeQueryDeclarationCanonicalEntry, ForgeQueryDeclarationCanonicalValue,
    ForgeQueryDeclarationEntryInspectionInput, ForgeQueryOrdinaryOutcome,
    ForgeQueryOrdinaryPostureKind,
};
use worth_geom::facade::{ParameterDomain, ParameterSpacePoint};
use worth_spatial::facade::bindings::{
    attach_parameter_space_point_to_face, rebind_surface_on_face, AnchorCarrierOwnership,
    CarrierOwnedParameterPointAnchorSpec, FaceBindingSite, FaceSurfaceBindingSpec,
    LocalTopologyReplacementNeighborhood, NeighborhoodBindingFamily, ReplacementCandidate,
    ReplacementCandidateSet, SpatialAdmittedPrimitiveBinding,
};

use crate::facade::authoring::binding::{
    author_primitive_binding_declaration, author_primitive_rebinding_declaration,
    AuthorPrimitiveBindingIntent, AuthorPrimitiveRebindingIntent,
};

use super::support::{
    admitted_binding_handle, admitted_rebinding_handle, binding_workflow_artifacts,
    canonical_geometry, canonical_text_entries, canonical_text_entries_for_rebinding,
    orthotope_contract, rebinding_workflow_artifacts,
};
use crate::binding::workflow_boundary::{
    envelope_checked_summary, receipt_checked_summary, route_checked_summary,
};

#[test]
fn kernel_binding_workflow_consumes_spatial_authority_without_local_rebinding_logic() {
    let prior = attach_parameter_space_point_to_face(
        FaceSurfaceBindingSpec::new(
            FaceBindingSite::new("face-old").with_persistent_name("surface-alpha"),
            orthotope_contract(),
            canonical_geometry([[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]),
        ),
        CarrierOwnedParameterPointAnchorSpec::new(
            AnchorCarrierOwnership::for_face_surface("face-old", ParameterDomain::plane())
                .expect("ownership"),
            ParameterSpacePoint::try_new([0.25, 0.5]).expect("parameter"),
        )
        .expect("anchor spec"),
    )
    .expect("prior");
    let successor = attach_parameter_space_point_to_face(
        FaceSurfaceBindingSpec::new(
            FaceBindingSite::new("face-new").with_persistent_name("surface-beta"),
            orthotope_contract(),
            canonical_geometry([[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]),
        ),
        CarrierOwnedParameterPointAnchorSpec::new(
            AnchorCarrierOwnership::for_face_surface("face-new", ParameterDomain::plane())
                .expect("ownership"),
            ParameterSpacePoint::try_new([0.25, 0.5]).expect("parameter"),
        )
        .expect("anchor spec"),
    )
    .expect("successor");
    let neighborhood = LocalTopologyReplacementNeighborhood::new(
        NeighborhoodBindingFamily::FaceSurfacePointAnchor,
        "face-old",
        ReplacementCandidateSet::new(vec![ReplacementCandidate::new(
            "successor",
            SpatialAdmittedPrimitiveBinding::FaceSurfacePointAnchor(successor.clone()),
        )
        .expect("candidate")])
        .expect("candidate set"),
    )
    .expect("neighborhood");
    let entry = author_primitive_rebinding_declaration(
        AuthorPrimitiveRebindingIntent::replace_surface_binding(
            SpatialAdmittedPrimitiveBinding::FaceSurfacePointAnchor(prior.clone()),
            neighborhood.clone(),
        ),
    );
    let handle = admitted_rebinding_handle("phase-five-rebinding");
    let ergonomic = rebinding_workflow_artifacts(&entry, &handle);

    let generic_progression = entry
        .progress_with_query(&handle)
        .unwrap_or_else(|_| panic!("generic progression"));
    let generic_route_checked =
        handle.orchestrate_routes_from_progressed_checked(generic_progression.clone());
    let generic_receipt_checked =
        handle.orchestrate_receipt_from_progressed_checked(generic_progression.clone());
    let generic_envelope_checked =
        handle.orchestrate_envelope_from_progressed_checked(generic_progression.clone());
    let generic_inspection_envelope_checked =
        handle.orchestrate_envelope_from_progressed_checked(generic_progression.clone());
    let generic_inspection = handle
        .inspect_declaration_entry(ForgeQueryDeclarationEntryInspectionInput::envelope_checked(
            generic_inspection_envelope_checked,
        ))
        .unwrap_or_else(|_| panic!("generic inspection"));
    let generic_ordinary = handle.orchestrate_declaration_entry_outcome(entry.clone());

    let kernel_decision = entry.clone().admit().expect("kernel decision");
    let direct_decision = rebind_surface_on_face(
        SpatialAdmittedPrimitiveBinding::FaceSurfacePointAnchor(prior),
        neighborhood,
    )
    .expect("direct decision");

    assert_eq!(
        kernel_decision.outcome_class(),
        direct_decision.outcome_class()
    );
    assert_eq!(
        kernel_decision.explanation().selected_candidate_identity(),
        direct_decision.explanation().selected_candidate_identity()
    );
    assert_eq!(
        canonical_text_entries_for_rebinding(&entry),
        canonical_text_map(ergonomic.canonical_entries())
    );
    assert_eq!(
        ergonomic.progression().progression_digest(),
        generic_progression.progression_digest()
    );
    assert_eq!(
        ergonomic.route_checked_summary(),
        route_checked_summary(&generic_route_checked)
    );
    assert_eq!(
        ergonomic.receipt_checked_summary(),
        receipt_checked_summary(&generic_receipt_checked)
    );
    assert_eq!(
        ergonomic.envelope_checked_summary(),
        envelope_checked_summary(&generic_envelope_checked)
    );
    assert_eq!(
        ergonomic.inspection().route_plan_digest(),
        generic_inspection.route_plan_digest()
    );
    assert_eq!(
        ergonomic.inspection().receipt_digest(),
        generic_inspection.receipt_digest()
    );
    assert_eq!(
        ergonomic.inspection().envelope_digest(),
        generic_inspection.envelope_digest()
    );
    assert_eq!(
        ergonomic.inspection().inspection_digest(),
        generic_inspection.inspection_digest()
    );
    assert_eq!(
        (
            ergonomic.ordinary_outcome_label(),
            ergonomic.ordinary_posture_kind()
        ),
        ordinary_outcome_shape(&generic_ordinary)
    );
}

#[test]
fn kernel_rebinding_dx_lane_and_generic_query_lane_converge_to_same_artifacts() {
    let entry = author_primitive_binding_declaration(
        AuthorPrimitiveBindingIntent::attach_parameter_space_point_to_face(
            FaceSurfaceBindingSpec::new(
                FaceBindingSite::new("face-1").with_persistent_name("surface-alpha"),
                orthotope_contract(),
                canonical_geometry([[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]),
            ),
            CarrierOwnedParameterPointAnchorSpec::new(
                AnchorCarrierOwnership::for_face_surface("face-1", ParameterDomain::plane())
                    .expect("ownership"),
                ParameterSpacePoint::try_new([0.25, 0.5]).expect("parameter"),
            )
            .expect("anchor spec"),
        ),
    );
    let handle = admitted_binding_handle("phase-five-binding");
    let ergonomic = binding_workflow_artifacts(&entry, &handle);

    let generic_progression = entry
        .progress_with_query(&handle)
        .unwrap_or_else(|_| panic!("generic progression"));
    let generic_route_checked =
        handle.orchestrate_routes_from_progressed_checked(generic_progression.clone());
    let generic_receipt_checked =
        handle.orchestrate_receipt_from_progressed_checked(generic_progression.clone());
    let generic_envelope_checked =
        handle.orchestrate_envelope_from_progressed_checked(generic_progression.clone());
    let generic_inspection_envelope_checked =
        handle.orchestrate_envelope_from_progressed_checked(generic_progression.clone());
    let generic_inspection = handle
        .inspect_declaration_entry(ForgeQueryDeclarationEntryInspectionInput::envelope_checked(
            generic_inspection_envelope_checked,
        ))
        .unwrap_or_else(|_| panic!("generic inspection"));
    let generic_ordinary = handle.orchestrate_declaration_entry_outcome(entry.clone());

    assert_eq!(
        canonical_text_entries(&entry),
        canonical_text_map(ergonomic.canonical_entries())
    );
    assert_eq!(
        ergonomic.progression().progression_digest(),
        generic_progression.progression_digest()
    );
    assert_eq!(
        ergonomic.route_checked_summary(),
        route_checked_summary(&generic_route_checked)
    );
    assert_eq!(
        ergonomic.receipt_checked_summary(),
        receipt_checked_summary(&generic_receipt_checked)
    );
    assert_eq!(
        ergonomic.envelope_checked_summary(),
        envelope_checked_summary(&generic_envelope_checked)
    );
    assert_eq!(
        ergonomic.inspection().declaration_digest(),
        generic_inspection.declaration_digest()
    );
    assert_eq!(
        ergonomic.inspection().progression_digest(),
        generic_inspection.progression_digest()
    );
    assert_eq!(
        ergonomic.inspection().route_plan_digest(),
        generic_inspection.route_plan_digest()
    );
    assert_eq!(
        ergonomic.inspection().receipt_digest(),
        generic_inspection.receipt_digest()
    );
    assert_eq!(
        ergonomic.inspection().envelope_digest(),
        generic_inspection.envelope_digest()
    );
    assert_eq!(
        ergonomic.inspection().envelope_class(),
        generic_inspection.envelope_class()
    );
    assert_eq!(
        (
            ergonomic.ordinary_outcome_label(),
            ergonomic.ordinary_posture_kind()
        ),
        ordinary_outcome_shape(&generic_ordinary)
    );
}

fn ordinary_outcome_shape<T>(
    outcome: &ForgeQueryOrdinaryOutcome<T>,
) -> (&'static str, Option<ForgeQueryOrdinaryPostureKind>) {
    match outcome {
        ForgeQueryOrdinaryOutcome::Bound(_) => ("bound", None),
        ForgeQueryOrdinaryOutcome::Ambiguous(value) => ("ambiguous", Some(value.kind())),
        ForgeQueryOrdinaryOutcome::AspectConflict(value) => ("aspect_conflict", Some(value.kind())),
        ForgeQueryOrdinaryOutcome::AuthorityMismatch(value) => {
            ("authority_mismatch", Some(value.kind()))
        }
        ForgeQueryOrdinaryOutcome::BasisMismatch(value) => ("basis_mismatch", Some(value.kind())),
        ForgeQueryOrdinaryOutcome::Deferred(value) => ("deferred", Some(value.kind())),
        ForgeQueryOrdinaryOutcome::Denied(value) => ("denied", Some(value.kind())),
        ForgeQueryOrdinaryOutcome::ExplicitNarrowingRequired(value) => {
            ("explicit_narrowing_required", Some(value.kind()))
        }
        ForgeQueryOrdinaryOutcome::Failed(value) => ("failed", Some(value.kind())),
        ForgeQueryOrdinaryOutcome::MissingRequiredAspect(value) => {
            ("missing_required_aspect", Some(value.kind()))
        }
        ForgeQueryOrdinaryOutcome::RebindRequired(value) => ("rebind_required", Some(value.kind())),
        ForgeQueryOrdinaryOutcome::Refused(value) => ("refused", Some(value.kind())),
        ForgeQueryOrdinaryOutcome::Stale(value) => ("stale", Some(value.kind())),
        ForgeQueryOrdinaryOutcome::Unavailable(value) => ("unavailable", Some(value.kind())),
        ForgeQueryOrdinaryOutcome::Unsupported(value) => ("unsupported", Some(value.kind())),
        ForgeQueryOrdinaryOutcome::WrongHandle(value) => ("wrong_handle", Some(value.kind())),
        ForgeQueryOrdinaryOutcome::WrongWorld(value) => ("wrong_world", Some(value.kind())),
    }
}

fn canonical_text_map(
    entries: &[ForgeQueryDeclarationCanonicalEntry],
) -> std::collections::BTreeMap<String, String> {
    entries
        .iter()
        .filter_map(|row| match row.value() {
            ForgeQueryDeclarationCanonicalValue::ExactText(value)
            | ForgeQueryDeclarationCanonicalValue::DecimalText(value) => {
                Some((row.locus().to_string(), value.clone()))
            }
            _ => None,
        })
        .collect()
}
