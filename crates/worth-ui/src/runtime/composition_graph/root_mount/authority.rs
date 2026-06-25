use std::collections::BTreeMap;

use crate::capability::{ComponentId, SurfaceId};
use crate::runtime::{
    WorthUiCompositionRootKind, WorthUiMosaicPlacementLegalityReceipt, WorthUiPageHostPlan,
    WorthUiRuntimeFactId,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiCompositionRootMountAuthoritySet {
    page_host_plan: WorthUiPageHostPlan,
    mosaic_legality: WorthUiMosaicPlacementLegalityReceipt,
    external_authorities:
        BTreeMap<ExternalRootAuthorityKey, WorthUiExternalCompositionRootMountAuthorityReceipt>,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct ExternalRootAuthorityKey {
    kind: WorthUiCompositionRootKind,
    identity: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiExternalCompositionRootMountAuthorityReceipt {
    kind: WorthUiCompositionRootKind,
    authority_identity: String,
    surface_id: SurfaceId,
    component_id: ComponentId,
    consumed_facts: Vec<WorthUiRuntimeFactId>,
    receipt_digest: u64,
}

impl WorthUiCompositionRootMountAuthoritySet {
    pub fn from_page_plan(
        page_host_plan: WorthUiPageHostPlan,
        mosaic_legality: WorthUiMosaicPlacementLegalityReceipt,
    ) -> Self {
        Self {
            page_host_plan,
            mosaic_legality,
            external_authorities: BTreeMap::new(),
        }
    }

    pub fn page_host_plan(&self) -> &WorthUiPageHostPlan {
        &self.page_host_plan
    }

    pub fn mosaic_legality(&self) -> &WorthUiMosaicPlacementLegalityReceipt {
        &self.mosaic_legality
    }

    pub fn with_component_instance(
        mut self,
        authority_identity: impl Into<String>,
        surface_id: SurfaceId,
        component_id: ComponentId,
    ) -> Self {
        self.insert_external_authority(
            WorthUiCompositionRootKind::ComponentInstance,
            authority_identity,
            surface_id,
            component_id,
        );
        self
    }

    pub fn with_portal_entry(
        mut self,
        authority_identity: impl Into<String>,
        surface_id: SurfaceId,
        component_id: ComponentId,
    ) -> Self {
        self.insert_external_authority(
            WorthUiCompositionRootKind::PortalEntry,
            authority_identity,
            surface_id,
            component_id,
        );
        self
    }

    pub fn with_collection_item(
        mut self,
        authority_identity: impl Into<String>,
        surface_id: SurfaceId,
        component_id: ComponentId,
    ) -> Self {
        self.insert_external_authority(
            WorthUiCompositionRootKind::CollectionItem,
            authority_identity,
            surface_id,
            component_id,
        );
        self
    }

    pub fn with_diagnostic_panel(
        mut self,
        authority_identity: impl Into<String>,
        surface_id: SurfaceId,
        component_id: ComponentId,
    ) -> Self {
        self.insert_external_authority(
            WorthUiCompositionRootKind::DiagnosticPanel,
            authority_identity,
            surface_id,
            component_id,
        );
        self
    }

    pub(crate) fn external_authority(
        &self,
        kind: WorthUiCompositionRootKind,
        authority_identity: &str,
    ) -> Option<&WorthUiExternalCompositionRootMountAuthorityReceipt> {
        self.external_authorities.get(&ExternalRootAuthorityKey {
            kind,
            identity: authority_identity.to_owned(),
        })
    }

    fn insert_external_authority(
        &mut self,
        kind: WorthUiCompositionRootKind,
        authority_identity: impl Into<String>,
        surface_id: SurfaceId,
        component_id: ComponentId,
    ) {
        let authority_identity = authority_identity.into();
        let receipt = WorthUiExternalCompositionRootMountAuthorityReceipt::new(
            kind,
            authority_identity.clone(),
            surface_id,
            component_id,
        );
        self.external_authorities.insert(
            ExternalRootAuthorityKey {
                kind,
                identity: authority_identity,
            },
            receipt,
        );
    }
}

impl WorthUiExternalCompositionRootMountAuthorityReceipt {
    fn new(
        kind: WorthUiCompositionRootKind,
        authority_identity: String,
        surface_id: SurfaceId,
        component_id: ComponentId,
    ) -> Self {
        let mut consumed_facts = vec![
            WorthUiRuntimeFactId::composition_root_mount_authority(format!(
                "{}:{}",
                kind.token(),
                authority_identity
            )),
            WorthUiRuntimeFactId::surface_mount(&surface_id),
            WorthUiRuntimeFactId::component(&component_id),
        ];
        consumed_facts.sort();
        consumed_facts.dedup();
        let receipt_digest = super::super::digest::digest_parts(
            [
                "external_composition_root_authority".to_owned(),
                kind.token().to_owned(),
                authority_identity.clone(),
                surface_id.as_str().to_owned(),
                component_id.as_str().to_owned(),
            ]
            .into_iter()
            .chain(consumed_facts.iter().map(|fact| fact.identity().to_owned())),
        );
        Self {
            kind,
            authority_identity,
            surface_id,
            component_id,
            consumed_facts,
            receipt_digest,
        }
    }

    pub fn kind(&self) -> WorthUiCompositionRootKind {
        self.kind
    }

    pub fn authority_identity(&self) -> &str {
        &self.authority_identity
    }

    pub fn surface_id(&self) -> &SurfaceId {
        &self.surface_id
    }

    pub fn component_id(&self) -> &ComponentId {
        &self.component_id
    }

    pub fn consumed_facts(&self) -> &[WorthUiRuntimeFactId] {
        &self.consumed_facts
    }

    pub fn receipt_digest(&self) -> u64 {
        self.receipt_digest
    }
}
