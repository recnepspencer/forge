use std::collections::BTreeMap;

use forge_query::facade::ForgeQueryApplicationFacade;
use forge_query::facade::{
    ForgeQueryAdmittedDeclarationProgression, ForgeQueryDeclarationCanonicalValue,
    ForgeQueryDeclarationEntryInspection, ForgeQueryDeclarationEntryInspectionInput,
    ForgeQueryDeclarationInput,
};

use crate::binding::workflow_boundary::KernelCanonicalQueryWorkflowArtifactSet;
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
