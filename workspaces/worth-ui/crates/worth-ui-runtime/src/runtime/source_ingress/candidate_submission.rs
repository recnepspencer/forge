use std::collections::BTreeMap;

use crate::capability::MeasurementConstraint;
use crate::capability::{CapabilitySnapshot, CapabilitySnapshotDigest};
use crate::declaration::UiDeclaredMeasurementConstraintModifier;
use crate::runtime::candidate::{
    file_authored_replacement_candidate, rust_authored_replacement_candidate,
};
use crate::runtime::source_ingress::counters::WorthUiSourceIngressCounters;
use crate::runtime::source_ingress::debounce::WorthUiDebouncedWatcherBatch;
use crate::runtime::source_ingress::denial::{
    WorthUiSourceIngressDenial, WorthUiSourceIngressDenialReason,
};
use crate::runtime::source_ingress::ordering_receipt::WorthUiCandidateOrderingReceipt;
use crate::runtime::source_ingress::provider::WorthUiSourceProvider;
use crate::runtime::source_ingress::revision::WorthUiSourcePackageRevision;
use crate::runtime::source_ingress::source_backed_package_lowering::source_backed_package;
use crate::runtime::source_ingress::{
    WorthUiSourceBackedDeclarationClaims, WorthUiSourceBackedDeclarationWitness,
    WorthUiSourceBackedDslPackage,
};
use crate::runtime::WorthUiReplacementCause;
use crate::runtime::{WorthUiReplacementCandidate, WorthUiReplacementCandidateDenial};
use crate::source::{
    WorthUiArtifact, WorthUiArtifactInputResolver, WorthUiBindingSemanticsLowerer,
    WorthUiCanonicalArtifactAssembler, WorthUiIdentitySeedLowerer,
    WorthUiLegallyStructuredArtifactInputNode, WorthUiParsedSourceToArtifactInputLowerer,
    WorthUiSourcePackageLoader, WorthUiSourceParser, WorthUiStructuralLegalityLowerer,
};

#[derive(Debug, Eq, PartialEq)]
pub struct WorthUiWatchedCandidateSubmission {
    candidate: WorthUiReplacementCandidate,
    revision: WorthUiSourcePackageRevision,
    ordering_receipt: WorthUiCandidateOrderingReceipt,
    counters: WorthUiSourceIngressCounters,
    source_backed_dsl_package: Option<WorthUiSourceBackedDslPackage>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthUiWatchedCandidateSubmissionDenial {
    SourceIngress(WorthUiSourceIngressDenial),
    Candidate(WorthUiReplacementCandidateDenial),
}

impl WorthUiDebouncedWatcherBatch {
    pub fn lower_to_candidate_submission(
        self,
        snapshot: &CapabilitySnapshot,
    ) -> Result<WorthUiWatchedCandidateSubmission, WorthUiWatchedCandidateSubmissionDenial> {
        let (provider, revision, ordering_receipt, mut counters) = self.into_parts();
        let lowered =
            lower_provider_to_candidate(&provider, snapshot, &revision, &ordering_receipt)?;
        counters.emit_candidate_submission();
        Ok(WorthUiWatchedCandidateSubmission {
            candidate: lowered.candidate,
            revision,
            ordering_receipt,
            counters,
            source_backed_dsl_package: lowered.source_backed_dsl_package,
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

    pub fn into_candidate(self) -> WorthUiReplacementCandidate {
        self.candidate
    }

    pub fn source_backed_dsl_package(&self) -> Option<&WorthUiSourceBackedDslPackage> {
        self.source_backed_dsl_package.as_ref()
    }
}

struct LoweredProviderCandidate {
    candidate: WorthUiReplacementCandidate,
    source_backed_dsl_package: Option<WorthUiSourceBackedDslPackage>,
}

fn lower_provider_to_candidate(
    provider: &WorthUiSourceProvider,
    snapshot: &CapabilitySnapshot,
    revision: &WorthUiSourcePackageRevision,
    ordering_receipt: &WorthUiCandidateOrderingReceipt,
) -> Result<LoweredProviderCandidate, WorthUiWatchedCandidateSubmissionDenial> {
    if !ordering_receipt.matches_revision(revision) {
        return Err(source_denial(
            WorthUiSourceIngressDenialReason::OrderingReceiptDrift,
        ));
    }
    reject_ambiguous_candidate_material(provider)?;
    if let Some(input) = provider.artifact_inputs().first() {
        let artifact = input.artifact().cloned().ok_or_else(|| {
            WorthUiWatchedCandidateSubmissionDenial::SourceIngress(WorthUiSourceIngressDenial::new(
                WorthUiSourceIngressDenialReason::NoCandidateMaterial,
            ))
        })?;
        return rust_authored_candidate(artifact, snapshot.digest(), revision).map(|candidate| {
            LoweredProviderCandidate {
                candidate,
                source_backed_dsl_package: None,
            }
        });
    }
    if !provider.source_modules().is_empty() {
        let material = file_authored_material(provider, snapshot)?;
        return file_authored_candidate(material.artifact, snapshot.digest(), provider, revision)
            .map(|candidate| LoweredProviderCandidate {
                candidate,
                source_backed_dsl_package: Some(material.source_backed_dsl_package),
            });
    }
    Err(source_denial(
        WorthUiSourceIngressDenialReason::NoCandidateMaterial,
    ))
}

fn reject_ambiguous_candidate_material(
    provider: &WorthUiSourceProvider,
) -> Result<(), WorthUiWatchedCandidateSubmissionDenial> {
    if !provider.source_modules().is_empty() && !provider.artifact_inputs().is_empty() {
        return Err(source_denial(
            WorthUiSourceIngressDenialReason::MixedCandidateMaterial,
        ));
    }
    if provider.artifact_inputs().len() > 1 {
        return Err(source_denial(
            WorthUiSourceIngressDenialReason::MultipleArtifactInputs,
        ));
    }
    Ok(())
}

struct FileAuthoredCandidateMaterial {
    artifact: WorthUiArtifact,
    source_backed_dsl_package: WorthUiSourceBackedDslPackage,
}

fn file_authored_material(
    provider: &WorthUiSourceProvider,
    snapshot: &CapabilitySnapshot,
) -> Result<FileAuthoredCandidateMaterial, WorthUiWatchedCandidateSubmissionDenial> {
    let mut loader = WorthUiSourcePackageLoader::from_workspace_root(provider.workspace_root());
    for module in provider.source_modules() {
        loader = loader.register_module_with_source(module.relative_path(), module.source_text());
    }
    let source_package = loader
        .compile()
        .map_err(|_| source_denial(WorthUiSourceIngressDenialReason::SourcePackageRejected))?;
    let parsed = WorthUiSourceParser::parse_package(&source_package)
        .map_err(|_| source_denial(WorthUiSourceIngressDenialReason::SourceParseRejected))?;
    let artifact_input = WorthUiParsedSourceToArtifactInputLowerer::lower(&parsed);
    canonical_artifact_from_input(artifact_input, snapshot)
}

fn canonical_artifact_from_input(
    artifact_input: crate::source::WorthUiArtifactInput,
    snapshot: &CapabilitySnapshot,
) -> Result<FileAuthoredCandidateMaterial, WorthUiWatchedCandidateSubmissionDenial> {
    let resolved = WorthUiArtifactInputResolver::resolve(&artifact_input, snapshot)
        .map_err(|_| source_denial(WorthUiSourceIngressDenialReason::ArtifactResolutionRejected))?;
    let structured = WorthUiStructuralLegalityLowerer::lower(&resolved, snapshot)
        .map_err(|_| source_denial(WorthUiSourceIngressDenialReason::StructuralLegalityRejected))?;
    let declaration_witness =
        WorthUiSourceBackedDeclarationWitness::new(source_backed_contracts(&structured));
    let source_backed_dsl_package = WorthUiSourceBackedDslPackage::new(
        source_backed_package(&structured),
        declaration_witness.clone(),
    );
    let bound = WorthUiBindingSemanticsLowerer::lower(&structured, snapshot)
        .map_err(|_| source_denial(WorthUiSourceIngressDenialReason::BindingSemanticsRejected))?;
    let identity_seeded = WorthUiIdentitySeedLowerer::lower(&bound)
        .map_err(|_| source_denial(WorthUiSourceIngressDenialReason::IdentitySeedingRejected))?
        .0;
    let artifact = WorthUiCanonicalArtifactAssembler::assemble(&identity_seeded)
        .map_err(|_| source_denial(WorthUiSourceIngressDenialReason::CanonicalAssemblyRejected))?;
    Ok(FileAuthoredCandidateMaterial {
        artifact,
        source_backed_dsl_package,
    })
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
    provider: &WorthUiSourceProvider,
    revision: &WorthUiSourcePackageRevision,
) -> Result<WorthUiReplacementCandidate, WorthUiWatchedCandidateSubmissionDenial> {
    let module_id = primary_source_module_id(provider)?;
    file_authored_replacement_candidate(
        artifact,
        snapshot_digest,
        WorthUiReplacementCause::file_source_change(module_id, revision.final_package_digest()),
    )
    .map_err(WorthUiWatchedCandidateSubmissionDenial::Candidate)
}

fn primary_source_module_id(
    provider: &WorthUiSourceProvider,
) -> Result<crate::source::WorthUiSourceModuleId, WorthUiWatchedCandidateSubmissionDenial> {
    let primary_source_module = provider
        .source_modules()
        .first()
        .ok_or_else(|| source_denial(WorthUiSourceIngressDenialReason::NoCandidateMaterial))?;
    crate::source::WorthUiSourceModuleId::from_relative_path(std::path::Path::new(
        primary_source_module.relative_path(),
    ))
    .map_err(|_| source_denial(WorthUiSourceIngressDenialReason::SourcePackageRejected))
}

fn source_denial(
    reason: WorthUiSourceIngressDenialReason,
) -> WorthUiWatchedCandidateSubmissionDenial {
    WorthUiWatchedCandidateSubmissionDenial::SourceIngress(WorthUiSourceIngressDenial::new(reason))
}

fn source_backed_contracts(
    structured: &crate::source::WorthUiLegallyStructuredArtifactInput,
) -> BTreeMap<(String, usize), WorthUiSourceBackedDeclarationClaims> {
    structured
        .module_ids()
        .iter()
        .flat_map(|module_id| {
            structured
                .module(module_id)
                .into_iter()
                .flat_map(|module| module.nodes().iter())
        })
        .filter_map(source_backed_contract_entry)
        .collect()
}

fn source_backed_contract_entry(
    node: &WorthUiLegallyStructuredArtifactInputNode,
) -> Option<((String, usize), WorthUiSourceBackedDeclarationClaims)> {
    match node {
        WorthUiLegallyStructuredArtifactInputNode::Component(node) => source_backed_claim_entry(
            node.provenance().module_path(),
            node.provenance().declaration_index(),
            source_backed_membership_identity(
                "component",
                node.authored_identity(),
                node.descriptor().id().as_str(),
            ),
            node.structure(),
        ),
        WorthUiLegallyStructuredArtifactInputNode::Surface(node) => source_backed_claim_entry(
            node.provenance().module_path(),
            node.provenance().declaration_index(),
            source_backed_membership_identity(
                "surface",
                node.authored_identity(),
                node.descriptor().id().as_str(),
            ),
            node.structure(),
        ),
        WorthUiLegallyStructuredArtifactInputNode::Binding(node) => source_backed_claim_entry(
            node.provenance().module_path(),
            node.provenance().declaration_index(),
            source_backed_membership_identity(
                "binding",
                node.authored_identity(),
                node.view_binding().id().as_str(),
            ),
            node.structure(),
        ),
        WorthUiLegallyStructuredArtifactInputNode::Import(_)
        | WorthUiLegallyStructuredArtifactInputNode::Token(_) => None,
    }
}

fn source_backed_claim_entry(
    module_path: &str,
    declaration_index: usize,
    membership_identity: String,
    structure: &crate::source::WorthUiMosaicStructureFacts,
) -> Option<((String, usize), WorthUiSourceBackedDeclarationClaims)> {
    let sizing_contract_id = structure
        .unique_root_sizing_contract_id()
        .expect("source-backed declaration structure should project one root sizing contract")?;
    Some((
        (module_path.to_owned(), declaration_index),
        WorthUiSourceBackedDeclarationClaims::new(
            format!("source-artifact:{module_path}|{membership_identity}"),
            source_backed_measurement_constraint_modifier(structure),
            sizing_contract_id,
        ),
    ))
}

fn source_backed_membership_identity(
    family: &str,
    authored_identity: Option<&str>,
    fallback_identity: &str,
) -> String {
    match authored_identity {
        Some(authored_identity) => format!("{family}:authored:{authored_identity}"),
        None => format!("{family}:identity:{fallback_identity}"),
    }
}

fn source_backed_measurement_constraint_modifier(
    structure: &crate::source::WorthUiMosaicStructureFacts,
) -> Option<UiDeclaredMeasurementConstraintModifier> {
    let constrained = structure.root_regions().iter().any(|region| {
        region
            .sizing_contract()
            .and_then(|(_, descriptor)| descriptor.named_measurement())
            .is_some_and(|measurement| {
                !matches!(
                    measurement.constraint(),
                    MeasurementConstraint::Unconstrained
                )
            })
    });
    constrained.then_some(UiDeclaredMeasurementConstraintModifier::Bounded)
}
