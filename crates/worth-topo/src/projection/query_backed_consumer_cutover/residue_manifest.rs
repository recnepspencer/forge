use std::fs;
use std::path::Path;
use std::sync::OnceLock;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum QueryBackedConsumerResidueOwner {
    WorthTopo,
    ForgeQuery,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum QueryBackedConsumerResidueDisposition {
    ExplicitResidue,
    QueryGap,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum QueryBackedConsumerQueryGapKind {
    MissingArtifact,
    NotAdmittedOnSupportedPath,
    NotExposedAtBoundary,
    IdentitySemanticsInsufficient,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QueryBackedConsumerResidueRow {
    source_path: String,
    current_surface: String,
    owner: QueryBackedConsumerResidueOwner,
    disposition: QueryBackedConsumerResidueDisposition,
    query_gap_kind: Option<QueryBackedConsumerQueryGapKind>,
    blocker: String,
    removal_trigger: String,
}

impl QueryBackedConsumerResidueRow {
    fn new(
        source_path: impl Into<String>,
        current_surface: impl Into<String>,
        owner: QueryBackedConsumerResidueOwner,
        disposition: QueryBackedConsumerResidueDisposition,
        query_gap_kind: Option<QueryBackedConsumerQueryGapKind>,
        blocker: impl Into<String>,
        removal_trigger: impl Into<String>,
    ) -> Self {
        Self {
            source_path: source_path.into(),
            current_surface: current_surface.into(),
            owner,
            disposition,
            query_gap_kind,
            blocker: blocker.into(),
            removal_trigger: removal_trigger.into(),
        }
    }

    pub fn source_path(&self) -> &str {
        &self.source_path
    }

    pub fn current_surface(&self) -> &str {
        &self.current_surface
    }

    pub const fn owner(&self) -> QueryBackedConsumerResidueOwner {
        self.owner
    }

    pub const fn disposition(&self) -> QueryBackedConsumerResidueDisposition {
        self.disposition
    }

    pub const fn query_gap_kind(&self) -> Option<QueryBackedConsumerQueryGapKind> {
        self.query_gap_kind
    }

    pub fn blocker(&self) -> &str {
        &self.blocker
    }

    pub fn removal_trigger(&self) -> &str {
        &self.removal_trigger
    }
}

const PARITY_SOURCE_PATH: &str =
    "crates/worth-topo/src/projection/read_views/domain/read_proof/parity.rs";
const HISTORICAL_REUSE_SOURCE_PATH: &str =
    "crates/worth-topo/src/projection/runtime_boundary/read_execution/basis_context.rs";

pub fn current_query_backed_consumer_residue_manifest() -> &'static [QueryBackedConsumerResidueRow]
{
    static CACHE: OnceLock<Vec<QueryBackedConsumerResidueRow>> = OnceLock::new();
    CACHE.get_or_init(build_live_query_backed_consumer_residue_manifest)
}

fn build_live_query_backed_consumer_residue_manifest() -> Vec<QueryBackedConsumerResidueRow> {
    let mut rows = Vec::new();
    if source_contains(PARITY_SOURCE_PATH, "view_digest_hex") {
        rows.push(QueryBackedConsumerResidueRow::new(
            PARITY_SOURCE_PATH,
            "TopologyReadViewParityArtifact::view_digest_hex",
            QueryBackedConsumerResidueOwner::WorthTopo,
            QueryBackedConsumerResidueDisposition::ExplicitResidue,
            None,
            "determinism proof still records rendered-view parity alongside typed query/runtime proof for read-model certification",
            "remove once every public read-model consumer lowers only compiled-product and query-boundary identities without view-parity accompaniment",
        ));
    }
    if source_contains(
        HISTORICAL_REUSE_SOURCE_PATH,
        "HistoricalPathReuseDescriptor::retained_reuse()",
    ) {
        rows.push(QueryBackedConsumerResidueRow::new(
            HISTORICAL_REUSE_SOURCE_PATH,
            "historical_context_for_family(... HistoricalPathReuseDescriptor::retained_reuse())",
            QueryBackedConsumerResidueOwner::WorthTopo,
            QueryBackedConsumerResidueDisposition::ExplicitResidue,
            None,
            "topology historical read execution still lowers retained historical path reuse through a local basis-context adapter instead of consuming one typed Query-owned retained historical admission surface",
            "remove once topology historical read execution deletes the local basis-context retained-reuse adapter and lowers through one carried Query-owned retained historical admission product",
        ));
    }
    rows.sort_by(|left, right| {
        left.source_path
            .cmp(&right.source_path)
            .then(left.current_surface.cmp(&right.current_surface))
    });
    rows
}

fn source_contains(source_path: &str, needle: &str) -> bool {
    read_source(source_path).contains(needle)
}

fn read_source(source_path: &str) -> String {
    fs::read_to_string(workspace_root().join(source_path)).unwrap_or_else(|error| {
        panic!("failed to read `{source_path}` for live residue derivation: {error}")
    })
}

fn workspace_root() -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("worth-topo manifest should live under the workspace crates directory")
        .to_path_buf()
}

impl QueryBackedConsumerQueryGapKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MissingArtifact => "missing",
            Self::NotAdmittedOnSupportedPath => "not-admitted",
            Self::NotExposedAtBoundary => "not-exposed",
            Self::IdentitySemanticsInsufficient => "identity-semantics-insufficient",
        }
    }
}
