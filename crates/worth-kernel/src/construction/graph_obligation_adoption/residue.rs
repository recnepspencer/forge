use forge_query::facade::consumer_kit::{
    ForgeQueryBoundaryAuditSourceSet, ForgeQueryGraphObligationConsumerKitError,
    ForgeQueryGraphObligationLocalCeremonyAudit, ForgeQueryGraphObligationResidueManifest,
    ForgeQueryGraphObligationResidueRow,
};

use crate::construction::request::PRIMITIVE_CONSTRUCTION_FAMILIES;

pub(crate) const PHASE_EIGHTEEN_SPEC_PRIMITIVE_FAMILY_COUNT: usize = 7;

pub(crate) fn primitive_construction_graph_obligation_local_ceremony_audit(
) -> ForgeQueryGraphObligationLocalCeremonyAudit {
    ForgeQueryGraphObligationLocalCeremonyAudit::evaluate(
        &primitive_construction_graph_obligation_audit_sources(),
    )
}

pub(crate) fn primitive_construction_graph_obligation_audit_sources(
) -> ForgeQueryBoundaryAuditSourceSet {
    let mut sources = ForgeQueryBoundaryAuditSourceSet::new("worth-kernel");
    for (label, path, source) in primitive_construction_phase_eighteen_audit_sources() {
        sources = sources.source_file(label, path, source);
    }
    sources
}

pub(crate) fn primitive_construction_graph_obligation_residue_manifest(
) -> Result<ForgeQueryGraphObligationResidueManifest, ForgeQueryGraphObligationConsumerKitError> {
    let family_count_gap = primitive_construction_phase_eighteen_family_count_gap();
    ForgeQueryGraphObligationResidueManifest::capped([
        ForgeQueryGraphObligationResidueRow::explicit(
            "kernel-handoff-only-result-helper",
            "worth-kernel primitive construction result surface",
            "forge-query-9.9-phase-18",
            1,
            1,
            "legacy tests still exercise handoff-only prepared results without a workspace-backed compose execution",
            "all construction result helpers require a workspace-backed executed compose artifact or move behind a compatibility-only test helper",
            "kept as explicit residue so handoff preparation is not mistaken for covered execution",
        )?,
        ForgeQueryGraphObligationResidueRow::explicit(
            "kernel-motion-preflight-sequencing",
            "worth-kernel motion construction support",
            "forge-query-9.9-phase-18",
            1,
            1,
            "motion compound-lowering support now returns typed spatial denials but is not yet represented as its own graph-obligation preflight registration",
            "motion branch-preview sequencing is represented as a typed preflight graph obligation with denial evidence",
            "kept as explicit residue after deleting unreachable sequencing so the remaining graph-obligation migration boundary stays visible",
        )?,
        ForgeQueryGraphObligationResidueRow::explicit(
            "kernel-primitive-family-cardinality-gap",
            "worth-kernel primitive construction family inventory",
            "forge-query-9.9-phase-18",
            family_count_gap,
            family_count_gap,
            "phase 18 certification language names seven primitive birth families while the current kernel request enum exposes six",
            "the spec and kernel primitive family inventory agree, or a seventh family is added and covered by compose execution",
            "kept as explicit residue so the phase cannot silently certify six families as the seven-family adversarial requirement",
        )?,
    ])
}

pub(crate) fn primitive_construction_phase_eighteen_family_count_gap() -> usize {
    PHASE_EIGHTEEN_SPEC_PRIMITIVE_FAMILY_COUNT.saturating_sub(PRIMITIVE_CONSTRUCTION_FAMILIES.len())
}

fn primitive_construction_phase_eighteen_audit_sources(
) -> [(&'static str, &'static str, &'static str); 9] {
    [
        source(
            "kernel.construction.admitted-scaffold.mod",
            "crates/worth-kernel/src/construction/phase_chain/admitted_scaffold/mod.rs",
            include_str!("../phase_chain/admitted_scaffold/mod.rs"),
        ),
        source(
            "kernel.construction.admitted-scaffold.artifact",
            "crates/worth-kernel/src/construction/phase_chain/admitted_scaffold/admitted_artifact.rs",
            include_str!("../phase_chain/admitted_scaffold/admitted_artifact.rs"),
        ),
        source(
            "kernel.construction.admitted-scaffold.topology-ready-birth",
            "crates/worth-kernel/src/construction/phase_chain/admitted_scaffold/topology_ready_birth.rs",
            include_str!("../phase_chain/admitted_scaffold/topology_ready_birth.rs"),
        ),
        source(
            "kernel.construction.result-surface.result",
            "crates/worth-kernel/src/construction/result_surface/result.rs",
            include_str!("../result_surface/result.rs"),
        ),
        source(
            "kernel.construction.result-surface.artifact",
            "crates/worth-kernel/src/construction/result_surface/artifact.rs",
            include_str!("../result_surface/artifact.rs"),
        ),
        source(
            "kernel.construction.result-surface.outcome",
            "crates/worth-kernel/src/construction/result_surface/outcome.rs",
            include_str!("../result_surface/outcome.rs"),
        ),
        source(
            "kernel.construction.query-authority.entry",
            "crates/worth-kernel/src/construction/query_authority/authority_entry.rs",
            include_str!("../query_authority/authority_entry.rs"),
        ),
        source(
            "kernel.construction.compound-lowering.motion",
            "crates/worth-kernel/src/construction/tests/support/compound_lowering/motion.rs",
            include_str!("../tests/support/compound_lowering/motion.rs"),
        ),
        source(
            "kernel.construction.compound-lowering.relations",
            "crates/worth-kernel/src/construction/tests/support/compound_lowering/relations.rs",
            include_str!("../tests/support/compound_lowering/relations.rs"),
        ),
    ]
}

const fn source(
    label: &'static str,
    path: &'static str,
    content: &'static str,
) -> (&'static str, &'static str, &'static str) {
    (label, path, content)
}
