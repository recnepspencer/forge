use std::collections::BTreeMap;

use forge_query::facade::ForgeQueryApplicationFacade;
use forge_query::facade::{
    ForgeQueryAdmittedDeclarationProgression, ForgeQueryDeclarationCanonicalValue,
    ForgeQueryDeclarationEntryInspection, ForgeQueryDeclarationEntryInspectionInput,
    ForgeQueryDeclarationEntryReadinessReport, ForgeQueryDeclarationEntryReadinessStatus,
    ForgeQueryDeclarationInput, ForgeQueryDomainEntryMarker, ForgeQueryDomainOperatingContext,
};

use crate::binding::rebinding::{
    primitive_rebinding_workflow_transport, PrimitiveRebindingWorkflowTransport,
};
use crate::binding::workflow_boundary::{
    envelope_checked_summary, ordinary_outcome_shape, receipt_checked_summary,
    route_checked_summary, KernelCanonicalQueryWorkflowArtifactSet,
};
use crate::facade::authoring::binding::{
    PrimitiveBindingDeclarationEntry, PrimitiveBindingQueryDomain, PrimitiveBindingQueryWorld,
    PrimitiveRebindingDeclarationEntry, PrimitiveRebindingQueryDomain,
    PrimitiveRebindingQueryWorld,
};

pub(crate) fn admitted_binding_handle(
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

pub(crate) fn admitted_rebinding_handle(
    world: &'static str,
) -> forge_query::facade::ForgeQueryAdmittedConfiguredDomainHandle<
    PrimitiveRebindingQueryDomain,
    PrimitiveRebindingQueryWorld,
> {
    ForgeQueryApplicationFacade::runtime_backed_default()
        .domain(PrimitiveRebindingQueryDomain)
        .with_operating_context(PrimitiveRebindingQueryWorld::new(world))
        .validate()
        .expect("rebinding query handle should validate")
        .admit()
        .expect("rebinding query handle should admit")
}

pub(crate) fn progress_binding_entry(
    entry: &PrimitiveBindingDeclarationEntry,
    handle: &forge_query::facade::ForgeQueryAdmittedConfiguredDomainHandle<
        PrimitiveBindingQueryDomain,
        PrimitiveBindingQueryWorld,
    >,
) -> ForgeQueryAdmittedDeclarationProgression<
    PrimitiveBindingQueryDomain,
    PrimitiveBindingDeclarationEntry,
> {
    entry
        .progress_with_query(handle)
        .unwrap_or_else(|_| panic!("binding declaration progression"))
}

pub(crate) fn inspect_progressed_binding_entry(
    handle: &forge_query::facade::ForgeQueryAdmittedConfiguredDomainHandle<
        PrimitiveBindingQueryDomain,
        PrimitiveBindingQueryWorld,
    >,
    progression: ForgeQueryAdmittedDeclarationProgression<
        PrimitiveBindingQueryDomain,
        PrimitiveBindingDeclarationEntry,
    >,
) -> ForgeQueryDeclarationEntryInspection<
    PrimitiveBindingQueryDomain,
    PrimitiveBindingDeclarationEntry,
> {
    handle
        .inspect_declaration_entry(ForgeQueryDeclarationEntryInspectionInput::envelope_checked(
            handle.orchestrate_envelope_from_progressed_checked(progression),
        ))
        .unwrap_or_else(|_| panic!("binding declaration inspection"))
}

#[allow(dead_code)]
pub(crate) fn binding_workflow_artifacts(
    entry: &PrimitiveBindingDeclarationEntry,
    handle: &forge_query::facade::ForgeQueryAdmittedConfiguredDomainHandle<
        PrimitiveBindingQueryDomain,
        PrimitiveBindingQueryWorld,
    >,
) -> KernelCanonicalQueryWorkflowArtifactSet<
    PrimitiveBindingQueryDomain,
    PrimitiveBindingDeclarationEntry,
> {
    entry
        .canonical_workflow_artifacts_with_query(handle)
        .unwrap_or_else(|_| panic!("binding workflow artifacts"))
}

pub(crate) fn progress_rebinding_entry(
    entry: &PrimitiveRebindingDeclarationEntry,
    handle: &forge_query::facade::ForgeQueryAdmittedConfiguredDomainHandle<
        PrimitiveRebindingQueryDomain,
        PrimitiveRebindingQueryWorld,
    >,
) -> ForgeQueryAdmittedDeclarationProgression<
    PrimitiveRebindingQueryDomain,
    PrimitiveRebindingDeclarationEntry,
> {
    entry
        .progress_with_query(handle)
        .unwrap_or_else(|_| panic!("rebinding declaration progression"))
}

pub(crate) fn inspect_progressed_rebinding_entry(
    handle: &forge_query::facade::ForgeQueryAdmittedConfiguredDomainHandle<
        PrimitiveRebindingQueryDomain,
        PrimitiveRebindingQueryWorld,
    >,
    progression: ForgeQueryAdmittedDeclarationProgression<
        PrimitiveRebindingQueryDomain,
        PrimitiveRebindingDeclarationEntry,
    >,
) -> ForgeQueryDeclarationEntryInspection<
    PrimitiveRebindingQueryDomain,
    PrimitiveRebindingDeclarationEntry,
> {
    handle
        .inspect_declaration_entry(ForgeQueryDeclarationEntryInspectionInput::envelope_checked(
            handle.orchestrate_envelope_from_progressed_checked(progression),
        ))
        .unwrap_or_else(|_| panic!("rebinding declaration inspection"))
}

pub(crate) fn rebinding_workflow_artifacts(
    entry: &PrimitiveRebindingDeclarationEntry,
    handle: &forge_query::facade::ForgeQueryAdmittedConfiguredDomainHandle<
        PrimitiveRebindingQueryDomain,
        PrimitiveRebindingQueryWorld,
    >,
) -> KernelCanonicalQueryWorkflowArtifactSet<
    PrimitiveRebindingQueryDomain,
    PrimitiveRebindingDeclarationEntry,
> {
    entry
        .canonical_workflow_artifacts_with_query(handle)
        .unwrap_or_else(|_| panic!("rebinding workflow artifacts"))
}

pub(crate) fn rebinding_workflow_transport(
    entry: &PrimitiveRebindingDeclarationEntry,
    handle: &forge_query::facade::ForgeQueryAdmittedConfiguredDomainHandle<
        PrimitiveRebindingQueryDomain,
        PrimitiveRebindingQueryWorld,
    >,
) -> PrimitiveRebindingWorkflowTransport {
    primitive_rebinding_workflow_transport(entry, handle)
        .unwrap_or_else(|_| panic!("rebinding workflow transport"))
}

pub(crate) fn assert_workflow_artifact_parity<
    D: ForgeQueryDomainEntryMarker,
    C: ForgeQueryDomainOperatingContext<D>,
    I: ForgeQueryDeclarationInput<D> + Clone,
>(
    ergonomic: &KernelCanonicalQueryWorkflowArtifactSet<D, I>,
    handle: &forge_query::facade::ForgeQueryAdmittedConfiguredDomainHandle<D, C>,
    progression: ForgeQueryAdmittedDeclarationProgression<D, I>,
    entry: I,
) {
    let generic_route_checked =
        handle.orchestrate_routes_from_progressed_checked(progression.clone());
    let generic_receipt_checked =
        handle.orchestrate_receipt_from_progressed_checked(progression.clone());
    let generic_envelope_checked =
        handle.orchestrate_envelope_from_progressed_checked(progression.clone());
    let generic_inspection = handle
        .inspect_declaration_entry(ForgeQueryDeclarationEntryInspectionInput::envelope_checked(
            handle.orchestrate_envelope_from_progressed_checked(progression.clone()),
        ))
        .unwrap_or_else(|_| panic!("generic inspection"));
    let generic_ordinary = handle.orchestrate_declaration_entry_outcome(entry);
    let generic_readiness = handle.declaration_entry_readiness::<I>();

    assert_eq!(
        readiness_shape(ergonomic.readiness()),
        readiness_shape(&generic_readiness)
    );
    assert_eq!(
        ergonomic.progression().progression_digest(),
        progression.progression_digest()
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

pub(crate) fn declaration_digest_string(
    progression: &ForgeQueryAdmittedDeclarationProgression<
        PrimitiveBindingQueryDomain,
        PrimitiveBindingDeclarationEntry,
    >,
) -> String {
    format!(
        "{:?}",
        progression.canonical_declaration().declaration_digest()
    )
}

pub(crate) fn rebinding_declaration_digest_string(
    progression: &ForgeQueryAdmittedDeclarationProgression<
        PrimitiveRebindingQueryDomain,
        PrimitiveRebindingDeclarationEntry,
    >,
) -> String {
    format!(
        "{:?}",
        progression.canonical_declaration().declaration_digest()
    )
}

#[allow(dead_code)]
pub(crate) fn canonical_text_entries(
    entry: &PrimitiveBindingDeclarationEntry,
) -> BTreeMap<String, String> {
    entry
        .canonical_declaration_entries()
        .into_iter()
        .filter_map(|entry| match entry.value() {
            ForgeQueryDeclarationCanonicalValue::ExactText(value)
            | ForgeQueryDeclarationCanonicalValue::DecimalText(value) => {
                Some((entry.locus().to_string(), value.clone()))
            }
            _ => None,
        })
        .collect()
}

pub(crate) fn canonical_text_entries_for_rebinding(
    entry: &PrimitiveRebindingDeclarationEntry,
) -> BTreeMap<String, String> {
    entry
        .canonical_declaration_entries()
        .into_iter()
        .filter_map(|entry| match entry.value() {
            ForgeQueryDeclarationCanonicalValue::ExactText(value)
            | ForgeQueryDeclarationCanonicalValue::DecimalText(value) => {
                Some((entry.locus().to_string(), value.clone()))
            }
            _ => None,
        })
        .collect()
}

fn readiness_shape<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>>(
    report: &ForgeQueryDeclarationEntryReadinessReport<D, I>,
) -> (
    &'static str,
    String,
    Vec<(
        &'static str,
        &'static str,
        ForgeQueryDeclarationEntryReadinessStatus,
        &'static str,
    )>,
) {
    (
        report.declaration_family_key(),
        report.readiness_digest().to_string(),
        report
            .rows()
            .iter()
            .map(|row| {
                (
                    row.crossing_row().entrypoint_key(),
                    row.crossing_row().surface().as_str(),
                    row.status(),
                    row.reason(),
                )
            })
            .collect(),
    )
}
