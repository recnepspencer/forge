use crate::runtime::{
    ForgeQuerySymbolicAspectResolutionEvidence, ForgeQuerySymbolicTargetReferenceEvidence,
    ForgeQueryWriteReceipt,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryGraphCompositionResolutionEntry {
    component_index: usize,
    aspect_path: Option<String>,
    symbol: String,
    resolved_entity_identity: String,
    target_collection: Option<String>,
}

impl ForgeQueryGraphCompositionResolutionEntry {
    fn symbolic_target_reference(
        component_index: usize,
        evidence: &ForgeQuerySymbolicTargetReferenceEvidence,
    ) -> Self {
        Self {
            component_index,
            aspect_path: None,
            symbol: evidence.symbol().to_string(),
            resolved_entity_identity: evidence.resolved_entity_identity().to_string(),
            target_collection: evidence.target_collection().map(str::to_string),
        }
    }

    fn symbolic_aspect_resolution(
        component_index: usize,
        evidence: &ForgeQuerySymbolicAspectResolutionEvidence,
    ) -> Self {
        Self {
            component_index,
            aspect_path: Some(evidence.aspect_path().to_string()),
            symbol: evidence.symbol().to_string(),
            resolved_entity_identity: evidence.resolved_entity_identity().to_string(),
            target_collection: evidence.target_collection().map(str::to_string),
        }
    }

    pub fn component_index(&self) -> usize {
        self.component_index
    }

    pub fn aspect_path(&self) -> Option<&str> {
        self.aspect_path.as_deref()
    }

    pub fn symbol(&self) -> &str {
        &self.symbol
    }

    pub fn resolved_entity_identity(&self) -> &str {
        &self.resolved_entity_identity
    }

    pub fn target_collection(&self) -> Option<&str> {
        self.target_collection.as_deref()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryGraphCompositionResolutionMap {
    entries: Vec<ForgeQueryGraphCompositionResolutionEntry>,
}

impl ForgeQueryGraphCompositionResolutionMap {
    pub(in crate::runtime) fn from_write_receipts(
        write_receipts: &[ForgeQueryWriteReceipt],
    ) -> Self {
        let entries = write_receipts
            .iter()
            .enumerate()
            .flat_map(|(component_index, receipt)| {
                receipt
                    .symbolic_target_reference_evidence()
                    .into_iter()
                    .map(move |evidence| {
                        ForgeQueryGraphCompositionResolutionEntry::symbolic_target_reference(
                            component_index,
                            evidence,
                        )
                    })
                    .chain(receipt.symbolic_aspect_resolution_evidence().iter().map(
                        move |evidence| {
                            ForgeQueryGraphCompositionResolutionEntry::symbolic_aspect_resolution(
                                component_index,
                                evidence,
                            )
                        },
                    ))
            })
            .collect();
        Self { entries }
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn entries(&self) -> &[ForgeQueryGraphCompositionResolutionEntry] {
        &self.entries
    }
}
