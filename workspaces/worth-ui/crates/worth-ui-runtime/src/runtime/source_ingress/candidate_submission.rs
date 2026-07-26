use crate::capability::{CapabilitySnapshot, CapabilitySnapshotDigest};
use crate::runtime::replacement::candidate::{
    file_authored_replacement_candidate, rust_authored_replacement_candidate,
};
use crate::runtime::source_ingress::counters::WorthUiSourceIngressCounters;
use crate::runtime::source_ingress::debounce::WorthUiSettledSourceSnapshot;
use crate::runtime::source_ingress::denial::{
    WorthUiSourceIngressDenial, WorthUiSourceIngressDenialReason,
};
use crate::runtime::source_ingress::ordering_receipt::WorthUiCandidateOrderingReceipt;
use crate::runtime::source_ingress::provider::WorthUiSourceProvider;
use crate::runtime::source_ingress::revision::WorthUiSourcePackageRevision;
use crate::runtime::source_ingress::{
    prepare_semantic_handoff, WorthUiCandidateComposition, WorthUiCandidateCompositionBasis,
    WorthUiCandidatePreparationHandoff, WorthUiPreparedSemanticHandoffMaterial,
};
use crate::runtime::WorthUiReplacementCause;
use crate::runtime::{WorthUiReplacementCandidate, WorthUiReplacementCandidateDenial};
use crate::source::WorthUiArtifact;
use worth_ui_dsl::{
    WorthUiAuthoredSourceInput, WorthUiDslCompileReport, WorthUiDslCompiler,
    WorthUiRustAuthoredArtifactInput, WorthUiSourceModuleId,
};

#[derive(Debug, Eq, PartialEq)]
pub struct WorthUiWatchedCandidateSubmission {
    composition: WorthUiCandidateComposition,
    revision: WorthUiSourcePackageRevision,
    ordering_receipt: WorthUiCandidateOrderingReceipt,
    counters: WorthUiSourceIngressCounters,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthUiWatchedCandidateSubmissionDenial {
    DslCompilation(WorthUiDslCompileReport),
    SourceIngress(WorthUiSourceIngressDenial),
    RuntimePreparation(crate::runtime::WorthUiSemanticHandoffPreparationDenial),
    Candidate(WorthUiReplacementCandidateDenial),
}

pub(crate) enum WorthUiAuthoredCompositionPreparationDenial {
    DslCompilation(WorthUiDslCompileReport),
    RuntimePreparation(crate::runtime::WorthUiSemanticHandoffPreparationDenial),
    Candidate(WorthUiReplacementCandidateDenial),
}

pub(crate) fn prepare_rust_authored_handoff(
    input: &WorthUiRustAuthoredArtifactInput,
    snapshot: &CapabilitySnapshot,
) -> Result<WorthUiCandidatePreparationHandoff, WorthUiAuthoredCompositionPreparationDenial> {
    let source_revision_digest = input.source_revision_digest();
    let sealed_package = WorthUiDslCompiler::compile_rust_authored(input)
        .map_err(WorthUiAuthoredCompositionPreparationDenial::DslCompilation)?;
    let material = prepare_semantic_handoff(sealed_package, snapshot)
        .map_err(WorthUiAuthoredCompositionPreparationDenial::RuntimePreparation)?;
    let (artifact, declaration_material, handoff) = material.into_parts();
    let candidate = rust_authored_replacement_candidate(
        artifact,
        snapshot.digest(),
        WorthUiReplacementCause::rust_authored_input_change(source_revision_digest),
    )
    .map_err(WorthUiAuthoredCompositionPreparationDenial::Candidate)?;
    Ok(
        WorthUiCandidateComposition::rust_authored(candidate, declaration_material, handoff)
            .into_preparation_handoff(),
    )
}

impl WorthUiSettledSourceSnapshot {
    pub fn lower_to_candidate_submission(
        self,
        snapshot: &CapabilitySnapshot,
    ) -> Result<WorthUiWatchedCandidateSubmission, WorthUiWatchedCandidateSubmissionDenial> {
        let (provider, revision, ordering_receipt, mut counters) = self.into_parts();
        let lowered =
            lower_provider_to_candidate(&provider, snapshot, &revision, &ordering_receipt)?;
        counters.emit_candidate_submission();
        Ok(WorthUiWatchedCandidateSubmission {
            composition: lowered,
            revision,
            ordering_receipt,
            counters,
        })
    }
}

impl WorthUiWatchedCandidateSubmission {
    pub fn ordering_receipt(&self) -> &WorthUiCandidateOrderingReceipt {
        &self.ordering_receipt
    }

    pub fn source_revision(&self) -> &WorthUiSourcePackageRevision {
        &self.revision
    }

    pub fn counters(&self) -> WorthUiSourceIngressCounters {
        self.counters
    }

    pub fn composition_basis(&self) -> &WorthUiCandidateCompositionBasis {
        self.composition.basis()
    }

    pub fn authoring_lane(&self) -> crate::runtime::WorthUiCandidateAuthoringLane {
        self.composition.authoring_lane()
    }

    pub(crate) fn candidate_snapshot_digest(&self) -> u64 {
        self.composition.snapshot_digest()
    }

    pub(crate) fn into_preparation_handoff(self) -> WorthUiCandidatePreparationHandoff {
        self.composition.into_preparation_handoff()
    }

    pub(crate) fn into_replacement_handoff(self) -> WorthUiCandidatePreparationHandoff {
        self.composition.into_preparation_handoff()
    }
}

fn lower_provider_to_candidate(
    provider: &WorthUiSourceProvider,
    snapshot: &CapabilitySnapshot,
    revision: &WorthUiSourcePackageRevision,
    ordering_receipt: &WorthUiCandidateOrderingReceipt,
) -> Result<WorthUiCandidateComposition, WorthUiWatchedCandidateSubmissionDenial> {
    if !ordering_receipt.matches_revision(revision) {
        return Err(source_denial(
            WorthUiSourceIngressDenialReason::OrderingReceiptDrift,
        ));
    }
    reject_ambiguous_candidate_material(provider)?;
    if let Some(input) = provider.rust_authored_inputs().first() {
        let material = rust_authored_material(input, snapshot)?;
        let (artifact, declaration_source, handoff) = material.into_parts();
        return rust_authored_candidate(artifact, snapshot.digest(), revision).map(|candidate| {
            WorthUiCandidateComposition::rust_authored(candidate, declaration_source, handoff)
        });
    }
    if !provider.source_modules().is_empty() {
        let (material, primary_module_id) = file_authored_material(provider, snapshot)?;
        let (artifact, declaration_source, handoff) = material.into_parts();
        return file_authored_candidate(artifact, snapshot.digest(), primary_module_id, revision)
            .map(|candidate| {
                WorthUiCandidateComposition::file_authored(candidate, declaration_source, handoff)
            });
    }
    Err(source_denial(
        WorthUiSourceIngressDenialReason::NoCandidateMaterial,
    ))
}

fn reject_ambiguous_candidate_material(
    provider: &WorthUiSourceProvider,
) -> Result<(), WorthUiWatchedCandidateSubmissionDenial> {
    if !provider.source_modules().is_empty() && !provider.rust_authored_inputs().is_empty() {
        return Err(source_denial(
            WorthUiSourceIngressDenialReason::MixedCandidateMaterial,
        ));
    }
    if provider.rust_authored_inputs().len() > 1 {
        return Err(source_denial(
            WorthUiSourceIngressDenialReason::MultipleRustAuthoredInputs,
        ));
    }
    Ok(())
}

fn file_authored_material(
    provider: &WorthUiSourceProvider,
    snapshot: &CapabilitySnapshot,
) -> Result<
    (
        WorthUiPreparedSemanticHandoffMaterial,
        WorthUiSourceModuleId,
    ),
    WorthUiWatchedCandidateSubmissionDenial,
> {
    let mut input = WorthUiAuthoredSourceInput::rooted_at(provider.workspace_root());
    for module in provider.source_modules() {
        input = input.with_module(module.relative_path(), module.source_text());
    }
    let sealed_package =
        WorthUiDslCompiler::compile_source(input).map_err(dsl_compilation_denial)?;
    let primary_module_id = sealed_package
        .module_ids()
        .first()
        .cloned()
        .ok_or_else(|| source_denial(WorthUiSourceIngressDenialReason::NoCandidateMaterial))?;
    prepare_semantic_handoff(sealed_package, snapshot)
        .map_err(WorthUiWatchedCandidateSubmissionDenial::RuntimePreparation)
        .map(|material| (material, primary_module_id))
}

fn rust_authored_material(
    input: &WorthUiRustAuthoredArtifactInput,
    snapshot: &CapabilitySnapshot,
) -> Result<WorthUiPreparedSemanticHandoffMaterial, WorthUiWatchedCandidateSubmissionDenial> {
    let sealed_package =
        WorthUiDslCompiler::compile_rust_authored(input).map_err(dsl_compilation_denial)?;
    prepare_semantic_handoff(sealed_package, snapshot)
        .map_err(WorthUiWatchedCandidateSubmissionDenial::RuntimePreparation)
}

fn rust_authored_candidate(
    artifact: WorthUiArtifact,
    snapshot_digest: CapabilitySnapshotDigest,
    revision: &WorthUiSourcePackageRevision,
) -> Result<WorthUiReplacementCandidate, WorthUiWatchedCandidateSubmissionDenial> {
    rust_authored_replacement_candidate(
        artifact,
        snapshot_digest,
        WorthUiReplacementCause::rust_authored_input_change(revision.final_package_digest()),
    )
    .map_err(WorthUiWatchedCandidateSubmissionDenial::Candidate)
}

fn file_authored_candidate(
    artifact: WorthUiArtifact,
    snapshot_digest: CapabilitySnapshotDigest,
    primary_module_id: WorthUiSourceModuleId,
    revision: &WorthUiSourcePackageRevision,
) -> Result<WorthUiReplacementCandidate, WorthUiWatchedCandidateSubmissionDenial> {
    file_authored_replacement_candidate(
        artifact,
        snapshot_digest,
        WorthUiReplacementCause::file_source_change(
            primary_module_id,
            revision.final_package_digest(),
        ),
    )
    .map_err(WorthUiWatchedCandidateSubmissionDenial::Candidate)
}

fn dsl_compilation_denial(
    report: WorthUiDslCompileReport,
) -> WorthUiWatchedCandidateSubmissionDenial {
    WorthUiWatchedCandidateSubmissionDenial::DslCompilation(report)
}

fn source_denial(
    reason: WorthUiSourceIngressDenialReason,
) -> WorthUiWatchedCandidateSubmissionDenial {
    WorthUiWatchedCandidateSubmissionDenial::SourceIngress(WorthUiSourceIngressDenial::new(reason))
}
