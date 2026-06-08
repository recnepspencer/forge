use std::collections::BTreeMap;

use forge_query::facade::ForgeQueryApplicationFacade;
use forge_query::facade::{
    ForgeQueryAdmittedDeclarationProgression, ForgeQueryDeclarationCanonicalValue,
    ForgeQueryDeclarationEntryInspection, ForgeQueryDeclarationEntryInspectionInput,
    ForgeQueryDeclarationInput,
};

use crate::binding::workflow_boundary::KernelCanonicalQueryWorkflowArtifactSet;
use crate::facade::authoring::anchoring::{
    PrimitiveAnchorBindingDeclarationEntry, PrimitiveAnchorBindingQueryDomain,
    PrimitiveAnchorBindingQueryWorld,
};

pub(crate) fn admitted_anchor_binding_handle(
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

pub(crate) fn progress_anchor_binding_entry(
    entry: &PrimitiveAnchorBindingDeclarationEntry,
    handle: &forge_query::facade::ForgeQueryAdmittedConfiguredDomainHandle<
        PrimitiveAnchorBindingQueryDomain,
        PrimitiveAnchorBindingQueryWorld,
    >,
) -> ForgeQueryAdmittedDeclarationProgression<
    PrimitiveAnchorBindingQueryDomain,
    PrimitiveAnchorBindingDeclarationEntry,
> {
    entry
        .progress_with_query(handle)
        .unwrap_or_else(|_| panic!("anchor binding declaration progression"))
}

pub(crate) fn inspect_progressed_anchor_binding_entry(
    handle: &forge_query::facade::ForgeQueryAdmittedConfiguredDomainHandle<
        PrimitiveAnchorBindingQueryDomain,
        PrimitiveAnchorBindingQueryWorld,
    >,
    progression: ForgeQueryAdmittedDeclarationProgression<
        PrimitiveAnchorBindingQueryDomain,
        PrimitiveAnchorBindingDeclarationEntry,
    >,
) -> ForgeQueryDeclarationEntryInspection<
    PrimitiveAnchorBindingQueryDomain,
    PrimitiveAnchorBindingDeclarationEntry,
> {
    handle
        .inspect_declaration_entry(ForgeQueryDeclarationEntryInspectionInput::envelope_checked(
            handle.orchestrate_envelope_from_progressed_checked(progression),
        ))
        .unwrap_or_else(|_| panic!("anchor binding declaration inspection"))
}

pub(crate) fn anchor_binding_workflow_artifacts(
    entry: &PrimitiveAnchorBindingDeclarationEntry,
    handle: &forge_query::facade::ForgeQueryAdmittedConfiguredDomainHandle<
        PrimitiveAnchorBindingQueryDomain,
        PrimitiveAnchorBindingQueryWorld,
    >,
) -> KernelCanonicalQueryWorkflowArtifactSet<
    PrimitiveAnchorBindingQueryDomain,
    PrimitiveAnchorBindingDeclarationEntry,
> {
    entry
        .canonical_workflow_artifacts_with_query(handle)
        .unwrap_or_else(|_| panic!("anchor binding workflow artifacts"))
}

pub(crate) fn canonical_text_entries_for_anchor_binding(
    entry: &PrimitiveAnchorBindingDeclarationEntry,
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

pub(crate) fn anchor_declaration_digest_string(
    entry: &PrimitiveAnchorBindingDeclarationEntry,
    handle: &forge_query::facade::ForgeQueryAdmittedConfiguredDomainHandle<
        PrimitiveAnchorBindingQueryDomain,
        PrimitiveAnchorBindingQueryWorld,
    >,
) -> String {
    format!(
        "{:?}",
        progress_anchor_binding_entry(entry, handle)
            .canonical_declaration()
            .declaration_digest()
    )
}

pub(crate) fn anchor_progression_digest_string(
    entry: &PrimitiveAnchorBindingDeclarationEntry,
    handle: &forge_query::facade::ForgeQueryAdmittedConfiguredDomainHandle<
        PrimitiveAnchorBindingQueryDomain,
        PrimitiveAnchorBindingQueryWorld,
    >,
) -> String {
    progress_anchor_binding_entry(entry, handle)
        .progression_digest()
        .to_string()
}

pub(crate) fn anchor_inspection_digest_string(
    entry: &PrimitiveAnchorBindingDeclarationEntry,
    handle: &forge_query::facade::ForgeQueryAdmittedConfiguredDomainHandle<
        PrimitiveAnchorBindingQueryDomain,
        PrimitiveAnchorBindingQueryWorld,
    >,
) -> String {
    inspect_progressed_anchor_binding_entry(handle, progress_anchor_binding_entry(entry, handle))
        .inspection_digest()
        .to_string()
}
