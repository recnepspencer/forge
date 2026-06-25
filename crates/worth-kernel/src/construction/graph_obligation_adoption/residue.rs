#[cfg(test)]
use forge_query::facade::consumer_kit::{
    ForgeQueryBoundaryAuditSourceSet, ForgeQueryGraphObligationLocalCeremonyAudit,
};
use forge_query::facade::consumer_kit::{
    ForgeQueryGraphObligationConsumerKitError, ForgeQueryGraphObligationResidueManifest,
    ForgeQueryGraphObligationResidueRow,
};

use crate::construction::family::PRIMITIVE_CONSTRUCTION_FAMILIES;

pub(crate) const PHASE_EIGHTEEN_SPEC_PRIMITIVE_FAMILY_COUNT: usize = 7;
const PRIMITIVE_RESIDUE_INTRODUCED_IN: &str = "forge-query-9.9-phase-18";
const PRIMITIVE_RESIDUE_HANDOFF_ONLY_CLASS: &str = "kernel-handoff-only-result-helper";
const PRIMITIVE_RESIDUE_MOTION_PREFLIGHT_CLASS: &str = "kernel-motion-preflight-sequencing";
const PRIMITIVE_RESIDUE_FAMILY_CARDINALITY_CLASS: &str = "kernel-primitive-family-cardinality-gap";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct PrimitiveConstructionFamilyCardinalityCloseout {
    spec_expected_family_count: usize,
    runtime_family_count: usize,
    missing_family_count: usize,
    capped_residue_class: &'static str,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct PrimitiveConstructionResidueContractRow {
    class: &'static str,
    owner: &'static str,
    introduced_in: &'static str,
    current_count: usize,
    must_not_exceed_count: usize,
    blocker: &'static str,
    removal_trigger: &'static str,
    decision: &'static str,
}

#[cfg(test)]
pub(crate) fn primitive_construction_graph_obligation_local_ceremony_audit(
) -> ForgeQueryGraphObligationLocalCeremonyAudit {
    ForgeQueryGraphObligationLocalCeremonyAudit::evaluate(
        &primitive_construction_graph_obligation_audit_sources(),
    )
}

#[cfg(test)]
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
    ForgeQueryGraphObligationResidueManifest::capped(
        primitive_construction_graph_obligation_residue_contract()
            .into_iter()
            .map(PrimitiveConstructionResidueContractRow::to_residue_row)
            .collect::<Result<Vec<_>, _>>()?,
    )
}

pub(crate) fn primitive_construction_phase_eighteen_family_count_gap() -> usize {
    PHASE_EIGHTEEN_SPEC_PRIMITIVE_FAMILY_COUNT.saturating_sub(PRIMITIVE_CONSTRUCTION_FAMILIES.len())
}

pub(crate) fn primitive_construction_family_cardinality_closeout(
) -> PrimitiveConstructionFamilyCardinalityCloseout {
    PrimitiveConstructionFamilyCardinalityCloseout {
        spec_expected_family_count: PHASE_EIGHTEEN_SPEC_PRIMITIVE_FAMILY_COUNT,
        runtime_family_count: PRIMITIVE_CONSTRUCTION_FAMILIES.len(),
        missing_family_count: primitive_construction_phase_eighteen_family_count_gap(),
        capped_residue_class: PRIMITIVE_RESIDUE_FAMILY_CARDINALITY_CLASS,
    }
}

pub(crate) fn primitive_construction_graph_obligation_residue_contract(
) -> [PrimitiveConstructionResidueContractRow; 3] {
    let family_count_gap = primitive_construction_phase_eighteen_family_count_gap();
    [
        PrimitiveConstructionResidueContractRow {
            class: PRIMITIVE_RESIDUE_HANDOFF_ONLY_CLASS,
            owner: "worth-kernel primitive construction result surface",
            introduced_in: PRIMITIVE_RESIDUE_INTRODUCED_IN,
            current_count: 1,
            must_not_exceed_count: 1,
            blocker: "legacy tests still exercise handoff-only prepared results without a workspace-backed compose execution",
            removal_trigger: "all construction result helpers require a workspace-backed executed compose artifact or move behind a compatibility-only test helper",
            decision: "kept as explicit residue so handoff preparation is not mistaken for covered execution",
        },
        PrimitiveConstructionResidueContractRow {
            class: PRIMITIVE_RESIDUE_MOTION_PREFLIGHT_CLASS,
            owner: "worth-kernel motion construction support",
            introduced_in: PRIMITIVE_RESIDUE_INTRODUCED_IN,
            current_count: 1,
            must_not_exceed_count: 1,
            blocker: "motion compound-lowering support now returns typed spatial denials but is not yet represented as its own graph-obligation preflight registration",
            removal_trigger: "motion branch-preview sequencing is represented as a typed preflight graph obligation with denial evidence",
            decision: "kept as explicit residue after deleting unreachable sequencing so the remaining graph-obligation migration boundary stays visible",
        },
        PrimitiveConstructionResidueContractRow {
            class: PRIMITIVE_RESIDUE_FAMILY_CARDINALITY_CLASS,
            owner: "worth-kernel primitive construction family inventory",
            introduced_in: PRIMITIVE_RESIDUE_INTRODUCED_IN,
            current_count: family_count_gap,
            must_not_exceed_count: family_count_gap,
            blocker: "phase 18 certification language names seven primitive birth families while the current kernel request enum exposes six",
            removal_trigger: "the spec and kernel primitive family inventory agree, or a seventh family is added and covered by compose execution",
            decision: "kept as explicit residue so the phase cannot silently certify six families as the seven-family adversarial requirement",
        },
    ]
}

impl PrimitiveConstructionFamilyCardinalityCloseout {
    pub(crate) const fn spec_expected_family_count(&self) -> usize {
        self.spec_expected_family_count
    }

    pub(crate) const fn runtime_family_count(&self) -> usize {
        self.runtime_family_count
    }

    pub(crate) const fn missing_family_count(&self) -> usize {
        self.missing_family_count
    }

    pub(crate) const fn capped_residue_class(&self) -> &'static str {
        self.capped_residue_class
    }
}

impl PrimitiveConstructionResidueContractRow {
    fn to_residue_row(
        self,
    ) -> Result<ForgeQueryGraphObligationResidueRow, ForgeQueryGraphObligationConsumerKitError>
    {
        ForgeQueryGraphObligationResidueRow::explicit(
            self.class,
            self.owner,
            self.introduced_in,
            self.current_count,
            self.must_not_exceed_count,
            self.blocker,
            self.removal_trigger,
            self.decision,
        )
    }

    pub(crate) const fn class(&self) -> &'static str {
        self.class
    }

    pub(crate) const fn owner(&self) -> &'static str {
        self.owner
    }

    pub(crate) const fn introduced_in(&self) -> &'static str {
        self.introduced_in
    }

    pub(crate) const fn current_count(&self) -> usize {
        self.current_count
    }

    pub(crate) const fn must_not_exceed_count(&self) -> usize {
        self.must_not_exceed_count
    }

    pub(crate) const fn blocker(&self) -> &'static str {
        self.blocker
    }

    pub(crate) const fn removal_trigger(&self) -> &'static str {
        self.removal_trigger
    }

    pub(crate) const fn decision(&self) -> &'static str {
        self.decision
    }
}

#[cfg(test)]
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

#[cfg(test)]
const fn source(
    label: &'static str,
    path: &'static str,
    content: &'static str,
) -> (&'static str, &'static str, &'static str) {
    (label, path, content)
}
