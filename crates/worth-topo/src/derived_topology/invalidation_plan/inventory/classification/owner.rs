use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DerivedInvalidationAuthorityOwner {
    WorthTopoDerivedTopology,
    WorthTopoProjectionRuntimeBoundary,
    WorthTopoOperatorCloseout,
    WorthTopoCertification,
    ForgeQuery,
}

impl DerivedInvalidationAuthorityOwner {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WorthTopoDerivedTopology => "worth_topo_derived_topology",
            Self::WorthTopoProjectionRuntimeBoundary => "worth_topo_projection_runtime_boundary",
            Self::WorthTopoOperatorCloseout => "worth_topo_operator_closeout",
            Self::WorthTopoCertification => "worth_topo_certification",
            Self::ForgeQuery => "forge_query",
        }
    }
}
