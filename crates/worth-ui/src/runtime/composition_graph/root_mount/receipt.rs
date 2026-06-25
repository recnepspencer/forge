use super::super::digest::digest_parts;
use crate::capability::{ComponentId, SurfaceId};
use crate::runtime::{
    WorthUiCompositionRootKind, WorthUiCompositionRootReceipt, WorthUiQueryGraphExecutionReceipt,
    WorthUiRuntimeFactId,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiCompositionRootMountReceipt {
    root: WorthUiCompositionRootReceipt,
    resolved_authority: WorthUiCompositionRootMountResolvedAuthority,
    consumed_facts: Vec<WorthUiRuntimeFactId>,
    query_graph_execution: WorthUiQueryGraphExecutionReceipt,
    counters: WorthUiCompositionRootMountCounters,
    receipt_digest: u64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthUiCompositionRootMountResolvedAuthority {
    Surface {
        surface_id: SurfaceId,
        component_id: ComponentId,
    },
    PageContentSlot {
        page_name: String,
        slot_name: String,
        surface_id: SurfaceId,
        component_id: ComponentId,
    },
    External {
        kind: WorthUiCompositionRootKind,
        authority_identity: String,
        surface_id: SurfaceId,
        component_id: ComponentId,
    },
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WorthUiCompositionRootMountCounters {
    page_slot_lookup_count: usize,
    page_slot_scan_count: usize,
    surface_lookup_count: usize,
    selected_graph_obligation_count: usize,
    source_reparse_count: usize,
    renderer_parse_count: usize,
}

impl WorthUiCompositionRootMountReceipt {
    pub(crate) fn new(
        root: WorthUiCompositionRootReceipt,
        resolved_authority: WorthUiCompositionRootMountResolvedAuthority,
        consumed_facts: Vec<WorthUiRuntimeFactId>,
        query_graph_execution: WorthUiQueryGraphExecutionReceipt,
        page_slot_lookup_count: usize,
        page_slot_scan_count: usize,
        surface_lookup_count: usize,
    ) -> Self {
        let mut consumed_facts = consumed_facts;
        consumed_facts.sort();
        consumed_facts.dedup();
        let counters = WorthUiCompositionRootMountCounters {
            page_slot_lookup_count,
            page_slot_scan_count,
            surface_lookup_count,
            selected_graph_obligation_count: query_graph_execution.selected_obligation_count(),
            source_reparse_count: 0,
            renderer_parse_count: 0,
        };
        let receipt_digest = digest_parts(
            [
                "composition_root_mount".to_owned(),
                root.receipt_digest().to_string(),
                resolved_authority.digest_basis(),
                query_graph_execution.execution_digest().to_string(),
            ]
            .into_iter()
            .chain(consumed_facts.iter().map(|fact| fact.identity().to_owned())),
        );
        Self {
            root,
            resolved_authority,
            consumed_facts,
            query_graph_execution,
            counters,
            receipt_digest,
        }
    }

    pub fn root(&self) -> &WorthUiCompositionRootReceipt {
        &self.root
    }

    pub fn root_kind(&self) -> WorthUiCompositionRootKind {
        self.root.kind()
    }

    pub fn resolved_authority(&self) -> &WorthUiCompositionRootMountResolvedAuthority {
        &self.resolved_authority
    }

    pub fn consumed_facts(&self) -> &[WorthUiRuntimeFactId] {
        &self.consumed_facts
    }

    pub fn query_graph_execution(&self) -> &WorthUiQueryGraphExecutionReceipt {
        &self.query_graph_execution
    }

    pub fn counters(&self) -> WorthUiCompositionRootMountCounters {
        self.counters
    }

    pub fn receipt_digest(&self) -> u64 {
        self.receipt_digest
    }
}

impl WorthUiCompositionRootMountResolvedAuthority {
    pub fn surface_id(&self) -> &SurfaceId {
        match self {
            Self::Surface { surface_id, .. }
            | Self::PageContentSlot { surface_id, .. }
            | Self::External { surface_id, .. } => surface_id,
        }
    }

    pub fn component_id(&self) -> &ComponentId {
        match self {
            Self::Surface { component_id, .. }
            | Self::PageContentSlot { component_id, .. }
            | Self::External { component_id, .. } => component_id,
        }
    }

    pub fn page_name(&self) -> Option<&str> {
        match self {
            Self::Surface { .. } | Self::External { .. } => None,
            Self::PageContentSlot { page_name, .. } => Some(page_name),
        }
    }

    pub fn slot_name(&self) -> Option<&str> {
        match self {
            Self::Surface { .. } | Self::External { .. } => None,
            Self::PageContentSlot { slot_name, .. } => Some(slot_name),
        }
    }

    pub fn external_kind(&self) -> Option<WorthUiCompositionRootKind> {
        match self {
            Self::External { kind, .. } => Some(*kind),
            Self::Surface { .. } | Self::PageContentSlot { .. } => None,
        }
    }

    pub fn authority_identity(&self) -> Option<&str> {
        match self {
            Self::External {
                authority_identity, ..
            } => Some(authority_identity),
            Self::Surface { .. } | Self::PageContentSlot { .. } => None,
        }
    }

    fn digest_basis(&self) -> String {
        match self {
            Self::Surface {
                surface_id,
                component_id,
            } => format!("surface:{}:{}", surface_id.as_str(), component_id.as_str()),
            Self::PageContentSlot {
                page_name,
                slot_name,
                surface_id,
                component_id,
            } => format!(
                "page_content_slot:{page_name}:{slot_name}:{}:{}",
                surface_id.as_str(),
                component_id.as_str()
            ),
            Self::External {
                kind,
                authority_identity,
                surface_id,
                component_id,
            } => format!(
                "external:{}:{authority_identity}:{}:{}",
                kind.token(),
                surface_id.as_str(),
                component_id.as_str()
            ),
        }
    }
}

impl WorthUiCompositionRootMountCounters {
    pub fn page_slot_lookup_count(self) -> usize {
        self.page_slot_lookup_count
    }

    pub fn page_slot_scan_count(self) -> usize {
        self.page_slot_scan_count
    }

    pub fn surface_lookup_count(self) -> usize {
        self.surface_lookup_count
    }

    pub fn selected_graph_obligation_count(self) -> usize {
        self.selected_graph_obligation_count
    }

    pub fn source_reparse_count(self) -> usize {
        self.source_reparse_count
    }

    pub fn renderer_parse_count(self) -> usize {
        self.renderer_parse_count
    }
}
