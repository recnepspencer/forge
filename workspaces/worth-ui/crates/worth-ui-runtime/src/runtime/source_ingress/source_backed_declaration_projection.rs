use std::collections::BTreeMap;

use crate::capability::MeasurementConstraint;
use crate::declaration::UiDeclaredMeasurementConstraintModifier;
use crate::source::{
    WorthUiLegallyStructuredArtifactInput, WorthUiLegallyStructuredArtifactInputNode,
};

use super::{
    WorthUiSourceBackedDeclarationClaims, WorthUiSourceBackedDeclarationWitness,
    WorthUiSourceIngressDenial, WorthUiSourceIngressDenialReason,
    WorthUiWatchedCandidateSubmissionDenial,
};

pub(super) fn project_source_backed_declaration_witness(
    structured: &WorthUiLegallyStructuredArtifactInput,
) -> Result<WorthUiSourceBackedDeclarationWitness, WorthUiWatchedCandidateSubmissionDenial> {
    source_backed_contracts(structured).map(WorthUiSourceBackedDeclarationWitness::new)
}

fn source_backed_contracts(
    structured: &WorthUiLegallyStructuredArtifactInput,
) -> Result<
    BTreeMap<(String, usize), WorthUiSourceBackedDeclarationClaims>,
    WorthUiWatchedCandidateSubmissionDenial,
> {
    structured
        .module_ids()
        .iter()
        .flat_map(|module_id| {
            structured
                .module(module_id)
                .into_iter()
                .flat_map(|module| module.nodes().iter())
        })
        .map(source_backed_contract_entry)
        .collect::<Result<Vec<_>, _>>()
        .map(|entries| {
            entries
                .into_iter()
                .flatten()
                .map(WorthUiSourceBackedContractEntry::into_map_entry)
                .collect()
        })
}

struct WorthUiSourceBackedContractEntry {
    source_location: (String, usize),
    claims: WorthUiSourceBackedDeclarationClaims,
}

impl WorthUiSourceBackedContractEntry {
    fn into_map_entry(self) -> ((String, usize), WorthUiSourceBackedDeclarationClaims) {
        (self.source_location, self.claims)
    }
}

fn source_backed_contract_entry(
    node: &WorthUiLegallyStructuredArtifactInputNode,
) -> Result<Option<WorthUiSourceBackedContractEntry>, WorthUiWatchedCandidateSubmissionDenial> {
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
        | WorthUiLegallyStructuredArtifactInputNode::Token(_) => Ok(None),
    }
}

fn source_backed_claim_entry(
    module_path: &str,
    declaration_index: usize,
    membership_identity: String,
    structure: &crate::source::WorthUiMosaicStructureFacts,
) -> Result<Option<WorthUiSourceBackedContractEntry>, WorthUiWatchedCandidateSubmissionDenial> {
    let Some(sizing_contract_id) = structure.unique_root_sizing_contract_id().map_err(|_| {
        source_denial(WorthUiSourceIngressDenialReason::SourceBackedDeclarationProjectionRejected)
    })?
    else {
        return Ok(None);
    };
    Ok(Some(WorthUiSourceBackedContractEntry {
        source_location: (module_path.to_owned(), declaration_index),
        claims: WorthUiSourceBackedDeclarationClaims::new(
            format!("source-artifact:{module_path}|{membership_identity}"),
            source_backed_measurement_constraint_modifier(structure),
            source_backed_measurement_basis_source(structure),
            sizing_contract_id,
        ),
    }))
}

fn source_backed_measurement_basis_source(
    structure: &crate::source::WorthUiMosaicStructureFacts,
) -> Option<crate::declaration::UiDeclaredMeasurementBasisSource> {
    use crate::capability::MosaicSizingBehavior;
    use crate::declaration::UiDeclaredMeasurementBasisSource;

    if structure.root_regions().iter().any(|region| {
        matches!(
            region.descriptor().sizing_behavior(),
            Some(MosaicSizingBehavior::OverlayAnchored)
        )
    }) {
        return Some(UiDeclaredMeasurementBasisSource::PortalAnchor);
    }
    None
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

fn source_denial(
    reason: WorthUiSourceIngressDenialReason,
) -> WorthUiWatchedCandidateSubmissionDenial {
    WorthUiWatchedCandidateSubmissionDenial::SourceIngress(WorthUiSourceIngressDenial::new(reason))
}
