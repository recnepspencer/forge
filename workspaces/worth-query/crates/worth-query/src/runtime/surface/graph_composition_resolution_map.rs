use crate::memory_workspace::WorthQueryEntityIdentity;
use crate::runtime::{
    WorthQueryAspectTouch, WorthQueryMutationSymbolIdentity,
    WorthQueryMutationTargetCollectionIdentity, WorthQuerySymbolicAspectResolutionEvidence,
    WorthQuerySymbolicTargetReferenceEvidence, WorthQueryWriteReceipt,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryGraphCompositionResolutionEntry {
    component_index: usize,
    aspect_touch: Option<WorthQueryAspectTouch>,
    symbol: WorthQueryMutationSymbolIdentity,
    resolved_entity_identity: WorthQueryEntityIdentity,
    target_collection: Option<WorthQueryMutationTargetCollectionIdentity>,
}

impl WorthQueryGraphCompositionResolutionEntry {
    fn symbolic_target_reference(
        component_index: usize,
        evidence: &WorthQuerySymbolicTargetReferenceEvidence,
    ) -> Self {
        Self {
            component_index,
            aspect_touch: None,
            symbol: evidence.symbol().clone(),
            resolved_entity_identity: evidence.resolved_entity_identity().clone(),
            target_collection: evidence.target_collection().cloned(),
        }
    }

    fn symbolic_aspect_resolution(
        component_index: usize,
        evidence: &WorthQuerySymbolicAspectResolutionEvidence,
    ) -> Self {
        Self {
            component_index,
            aspect_touch: Some(evidence.aspect_touch().clone()),
            symbol: evidence.symbol().clone(),
            resolved_entity_identity: evidence.resolved_entity_identity().clone(),
            target_collection: evidence.target_collection().cloned(),
        }
    }

    pub fn component_index(&self) -> usize {
        self.component_index
    }

    pub fn aspect_touch(&self) -> Option<&WorthQueryAspectTouch> {
        self.aspect_touch.as_ref()
    }

    pub fn symbol(&self) -> &WorthQueryMutationSymbolIdentity {
        &self.symbol
    }

    pub fn resolved_entity_identity(&self) -> &WorthQueryEntityIdentity {
        &self.resolved_entity_identity
    }

    pub fn target_collection(&self) -> Option<&WorthQueryMutationTargetCollectionIdentity> {
        self.target_collection.as_ref()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryGraphCompositionResolutionMap {
    entries: Vec<WorthQueryGraphCompositionResolutionEntry>,
}

impl WorthQueryGraphCompositionResolutionMap {
    pub(in crate::runtime) fn from_write_receipts(
        write_receipts: &[WorthQueryWriteReceipt],
    ) -> Self {
        let entries = write_receipts
            .iter()
            .enumerate()
            .flat_map(|(component_index, receipt)| {
                receipt
                    .symbolic_target_reference_evidence()
                    .into_iter()
                    .map(move |evidence| {
                        WorthQueryGraphCompositionResolutionEntry::symbolic_target_reference(
                            component_index,
                            evidence,
                        )
                    })
                    .chain(receipt.symbolic_aspect_resolution_evidence().iter().map(
                        move |evidence| {
                            WorthQueryGraphCompositionResolutionEntry::symbolic_aspect_resolution(
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

    pub fn entries(&self) -> &[WorthQueryGraphCompositionResolutionEntry] {
        &self.entries
    }
}
