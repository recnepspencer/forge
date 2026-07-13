use crate::runtime::{WorthQueryReadBuilder, WorthQueryReadDenial, WorthQueryReadGraph};

/// Stable identity for one canonical read declaration.
///
/// This is an observation of Query-owned canonical meaning, not execution or
/// basis authority.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct WorthQueryReadDeclarationIdentity(String);

impl WorthQueryReadDeclarationIdentity {
    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub(crate) fn from_read_graph(read_graph: &WorthQueryReadGraph) -> Self {
        Self(read_graph.digest().to_string())
    }
}

/// A Query-minted, canonical one-shot read declaration.
///
/// Its fields are private so consumers can inspect canonical identity without
/// manufacturing or replacing the graph Query admitted during authoring.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryReadDeclaration {
    identity: WorthQueryReadDeclarationIdentity,
    read_graph: WorthQueryReadGraph,
}

impl WorthQueryReadDeclaration {
    pub fn identity(&self) -> &WorthQueryReadDeclarationIdentity {
        &self.identity
    }

    pub(crate) fn into_read_graph(self) -> WorthQueryReadGraph {
        self.read_graph
    }

    fn from_read_graph(read_graph: WorthQueryReadGraph) -> Self {
        let identity = WorthQueryReadDeclarationIdentity::from_read_graph(&read_graph);
        Self {
            identity,
            read_graph,
        }
    }
}

/// Authoring stop produced before runtime admission or lower-runtime contact.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryReadDeclarationStop {
    denial: WorthQueryReadDenial,
}

impl WorthQueryReadDeclarationStop {
    pub fn denial(&self) -> &WorthQueryReadDenial {
        &self.denial
    }

    pub fn next_action(&self) -> super::WorthQueryReadNextAction {
        super::WorthQueryReadNextAction::ReviseDeclaration
    }

    fn new(denial: WorthQueryReadDenial) -> Self {
        Self { denial }
    }
}

/// Declare one bounded read capability through Query's canonical authoring
/// path.
///
/// The closure receives only read-family vocabulary. Canonicalization,
/// validation, and plan construction occur inside the selected read operation;
/// no raw canonical bundle or planning context is returned to the consumer.
pub fn declare(
    author: impl FnOnce(WorthQueryReadBuilder) -> Result<WorthQueryReadGraph, WorthQueryReadDenial>,
) -> Result<WorthQueryReadDeclaration, WorthQueryReadDeclarationStop> {
    author(WorthQueryReadBuilder::standalone())
        .map(WorthQueryReadDeclaration::from_read_graph)
        .map_err(WorthQueryReadDeclarationStop::new)
}
