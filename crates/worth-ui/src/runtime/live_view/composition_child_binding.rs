use crate::runtime::live_view::digest::digest_parts;
use crate::runtime::{
    WorthUiCompositionGraphChildAccessRow, WorthUiCompositionNodeKind, WorthUiRuntimeFactId,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiLiveViewCompositionChildSubjectKind {
    Control,
    Interaction,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiLiveViewCompositionChildBindingReceipt {
    subject_kind: WorthUiLiveViewCompositionChildSubjectKind,
    subject_id: String,
    composition_node_id: String,
    authority_identity: String,
    parent_id: String,
    order: u32,
    sizing_token: String,
    child_access_row_digest: u64,
    consumed_facts: Vec<WorthUiRuntimeFactId>,
    binding_digest: u64,
}

impl WorthUiLiveViewCompositionChildBindingReceipt {
    fn from_child_row(
        subject_kind: WorthUiLiveViewCompositionChildSubjectKind,
        subject_id: &str,
        row: &WorthUiCompositionGraphChildAccessRow,
    ) -> Self {
        let mut consumed_facts = vec![
            row.node().fact_id().clone(),
            row.edge().fact_id().clone(),
            WorthUiRuntimeFactId::composition_node(row.node().node_id().as_str()),
            WorthUiRuntimeFactId::composition_edge(row.edge().fact_id().identity()),
        ];
        consumed_facts.sort();
        consumed_facts.dedup();
        let binding_digest = digest_parts(
            [
                subject_kind.token().to_owned(),
                subject_id.to_owned(),
                row.node().node_id().as_str().to_owned(),
                row.node().authority_identity().to_owned(),
                row.parent_id().to_owned(),
                row.order().to_string(),
                row.sizing_token().to_owned(),
                row.row_digest().to_string(),
            ]
            .into_iter()
            .chain(consumed_facts.iter().map(|fact| fact.identity().to_owned())),
        );
        Self {
            subject_kind,
            subject_id: subject_id.to_owned(),
            composition_node_id: row.node().node_id().as_str().to_owned(),
            authority_identity: row.node().authority_identity().to_owned(),
            parent_id: row.parent_id().to_owned(),
            order: row.order(),
            sizing_token: row.sizing_token().to_owned(),
            child_access_row_digest: row.row_digest(),
            consumed_facts,
            binding_digest,
        }
    }

    pub(in crate::runtime::live_view) fn from_admitted_child_row(
        row: &WorthUiCompositionGraphChildAccessRow,
    ) -> Option<Self> {
        let subject_kind = WorthUiLiveViewCompositionChildSubjectKind::from_composition_node_kind(
            row.node().kind(),
        )?;
        Some(Self::from_child_row(
            subject_kind,
            row.node().authority_identity(),
            row,
        ))
    }

    pub fn subject_kind(&self) -> WorthUiLiveViewCompositionChildSubjectKind {
        self.subject_kind
    }

    pub fn subject_id(&self) -> &str {
        &self.subject_id
    }

    pub fn composition_node_id(&self) -> &str {
        &self.composition_node_id
    }

    pub fn authority_identity(&self) -> &str {
        &self.authority_identity
    }

    pub fn parent_id(&self) -> &str {
        &self.parent_id
    }

    pub fn order(&self) -> u32 {
        self.order
    }

    pub fn sizing_token(&self) -> &str {
        &self.sizing_token
    }

    pub fn child_access_row_digest(&self) -> u64 {
        self.child_access_row_digest
    }

    pub fn consumed_facts(&self) -> &[WorthUiRuntimeFactId] {
        &self.consumed_facts
    }

    pub fn binding_digest(&self) -> u64 {
        self.binding_digest
    }
}

impl WorthUiLiveViewCompositionChildSubjectKind {
    pub const fn token(self) -> &'static str {
        match self {
            Self::Control => "control",
            Self::Interaction => "interaction",
        }
    }

    const fn from_composition_node_kind(kind: WorthUiCompositionNodeKind) -> Option<Self> {
        match kind {
            WorthUiCompositionNodeKind::Control => Some(Self::Control),
            WorthUiCompositionNodeKind::Interaction => Some(Self::Interaction),
            _ => None,
        }
    }
}
