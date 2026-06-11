mod anchors;
mod asymmetric_pressure;
mod completeness;
mod curved_pressure;
mod identity;
mod motion_posture;
mod rebinding;
mod rebinding_diagnostics;
mod rebinding_outcomes;
mod recovery;
mod tolerance_precision;

use forge_query::facade::ForgeQueryApplicationFacade;

use crate::bindings::query_native::{
    PrimitiveAnchorBindingQueryDomain, PrimitiveAnchorBindingQueryWorld,
    PrimitiveBindingQueryDomain, PrimitiveBindingQueryWorld,
};
use crate::bindings::query_native_anchor_binding_authoring::PrimitiveAnchorBindingDeclarationEntry;
use crate::bindings::query_native_binding_authoring::PrimitiveBindingDeclarationEntry;
use crate::bindings::query_native_rebinding_candidate_fact::{
    primitive_anchor_binding_rebinding_candidate_fact, primitive_binding_rebinding_candidate_fact,
    PrimitiveRebindingCandidateFactError,
};
use crate::bindings::query_native_rebinding_prior_fact::{
    primitive_anchor_binding_rebinding_prior_binding_fact,
    primitive_binding_rebinding_prior_binding_fact, PrimitiveRebindingPriorBindingFact,
};
use crate::bindings::rebinding::{
    project_curve_rebinding_fact_receipt_with_motion,
    project_geometry_rebinding_fact_receipt_with_motion,
    project_pcurve_rebinding_fact_receipt_with_motion,
    project_surface_rebinding_fact_receipt_with_motion, BindingMotionSemanticsInput,
    LocalTopologyReplacementNeighborhood, PrimitiveRebindingFactReceipt, ReplacementCandidate,
    SpatialRebindingAuthorityError,
};

fn admitted_binding_handle(
    world: &'static str,
) -> forge_query::facade::ForgeQueryAdmittedConfiguredDomainHandle<
    PrimitiveBindingQueryDomain,
    PrimitiveBindingQueryWorld,
> {
    ForgeQueryApplicationFacade::runtime_backed_default()
        .domain(PrimitiveBindingQueryDomain)
        .with_operating_context(PrimitiveBindingQueryWorld::new(world))
        .validate()
        .expect("binding query handle should validate")
        .admit()
        .expect("binding query handle should admit")
}

fn admitted_anchor_binding_handle(
    world: &'static str,
) -> forge_query::facade::ForgeQueryAdmittedConfiguredDomainHandle<
    PrimitiveAnchorBindingQueryDomain,
    PrimitiveAnchorBindingQueryWorld,
> {
    ForgeQueryApplicationFacade::runtime_backed_default()
        .domain(PrimitiveAnchorBindingQueryDomain)
        .with_operating_context(PrimitiveAnchorBindingQueryWorld::new(world))
        .validate()
        .expect("anchor binding query handle should validate")
        .admit()
        .expect("anchor binding query handle should admit")
}

pub(super) fn rebinding_prior_fact_from_binding_declaration(
    declaration: &PrimitiveBindingDeclarationEntry,
    world: &'static str,
) -> PrimitiveRebindingPriorBindingFact {
    primitive_binding_rebinding_prior_binding_fact(declaration, &admitted_binding_handle(world))
        .unwrap_or_else(|error| panic!("expected binding rebinding prior fact, found {error:?}"))
}

pub(super) fn rebinding_prior_fact_from_anchor_declaration(
    declaration: &PrimitiveAnchorBindingDeclarationEntry,
    world: &'static str,
) -> PrimitiveRebindingPriorBindingFact {
    primitive_anchor_binding_rebinding_prior_binding_fact(
        declaration,
        &admitted_anchor_binding_handle(world),
    )
    .unwrap_or_else(|error| panic!("expected anchor rebinding prior fact, found {error:?}"))
}

pub(super) fn rebinding_candidate_from_binding_declaration(
    label: impl Into<String>,
    declaration: &PrimitiveBindingDeclarationEntry,
    world: &'static str,
) -> Result<ReplacementCandidate, SpatialRebindingAuthorityError> {
    ReplacementCandidate::new(
        label,
        primitive_binding_rebinding_candidate_fact(declaration, &admitted_binding_handle(world))
            .map_err(candidate_fact_error)?,
    )
}

pub(super) fn rebinding_candidate_from_anchor_declaration(
    label: impl Into<String>,
    declaration: &PrimitiveAnchorBindingDeclarationEntry,
    world: &'static str,
) -> Result<ReplacementCandidate, SpatialRebindingAuthorityError> {
    ReplacementCandidate::new(
        label,
        primitive_anchor_binding_rebinding_candidate_fact(
            declaration,
            &admitted_anchor_binding_handle(world),
        )
        .map_err(candidate_fact_error)?,
    )
}

pub(super) fn rebind_surface_on_face_from_fact(
    prior_binding: PrimitiveRebindingPriorBindingFact,
    neighborhood: LocalTopologyReplacementNeighborhood,
) -> Result<PrimitiveRebindingFactReceipt, SpatialRebindingAuthorityError> {
    project_surface_rebinding_fact_receipt_with_motion(
        prior_binding,
        neighborhood,
        BindingMotionSemanticsInput::unresolved_without_motion_workflow(),
    )
}

pub(super) fn rebind_curve_on_edge_from_fact(
    prior_binding: PrimitiveRebindingPriorBindingFact,
    neighborhood: LocalTopologyReplacementNeighborhood,
) -> Result<PrimitiveRebindingFactReceipt, SpatialRebindingAuthorityError> {
    project_curve_rebinding_fact_receipt_with_motion(
        prior_binding,
        neighborhood,
        BindingMotionSemanticsInput::unresolved_without_motion_workflow(),
    )
}

pub(super) fn rebind_pcurve_on_coedge_from_fact(
    prior_binding: PrimitiveRebindingPriorBindingFact,
    neighborhood: LocalTopologyReplacementNeighborhood,
) -> Result<PrimitiveRebindingFactReceipt, SpatialRebindingAuthorityError> {
    project_pcurve_rebinding_fact_receipt_with_motion(
        prior_binding,
        neighborhood,
        BindingMotionSemanticsInput::unresolved_without_motion_workflow(),
    )
}

pub(super) fn rebind_geometry_on_vertex_from_fact(
    prior_binding: PrimitiveRebindingPriorBindingFact,
    neighborhood: LocalTopologyReplacementNeighborhood,
) -> Result<PrimitiveRebindingFactReceipt, SpatialRebindingAuthorityError> {
    project_geometry_rebinding_fact_receipt_with_motion(
        prior_binding,
        neighborhood,
        BindingMotionSemanticsInput::unresolved_without_motion_workflow(),
    )
}

fn candidate_fact_error(
    error: PrimitiveRebindingCandidateFactError,
) -> SpatialRebindingAuthorityError {
    match error {
        PrimitiveRebindingCandidateFactError::Binding(error) => {
            panic!("expected query-backed binding rebinding candidate fact, found {error:?}")
        }
        PrimitiveRebindingCandidateFactError::Anchor(error) => {
            panic!("expected query-backed anchor rebinding candidate fact, found {error:?}")
        }
        PrimitiveRebindingCandidateFactError::QueryNotBound => {
            panic!("expected query-backed rebinding candidate fact from a bound declaration")
        }
    }
}
