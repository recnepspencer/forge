use std::sync::Arc;

use sha2::{Digest, Sha256};

const REQUIRED_PRODUCT_COUNT: usize = 128;
const REQUIRED_COMPONENTS: [&str; 5] = ["copper", "glass", "labor", "rubber", "steel"];

fn join_arc_str(values: &[Arc<str>]) -> String {
    values
        .iter()
        .map(|value| value.as_ref())
        .collect::<Vec<_>>()
        .join(",")
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BridgeSubscriptionReferenceWorkloadManifestRejectionKind {
    ProductCountMismatch,
    MissingRequiredComponent,
    EmptyLaneSet,
}

impl BridgeSubscriptionReferenceWorkloadManifestRejectionKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProductCountMismatch => "product_count_mismatch",
            Self::MissingRequiredComponent => "missing_required_component",
            Self::EmptyLaneSet => "empty_lane_set",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeSubscriptionReferenceWorkloadManifestRejection {
    rejection_kind: BridgeSubscriptionReferenceWorkloadManifestRejectionKind,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeSubscriptionReferenceWorkloadManifestRejection {
    fn new(rejection_kind: BridgeSubscriptionReferenceWorkloadManifestRejectionKind) -> Self {
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-subscription-reference-workload-manifest-rejection|kind={}",
            rejection_kind.as_str()
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Self {
            rejection_kind,
            canonical_basis,
            digest: Arc::from(format!(
                "bridge-subscription-reference-workload-manifest-rejection:sha256:{digest:x}"
            )),
        }
    }

    pub fn rejection_kind(&self) -> BridgeSubscriptionReferenceWorkloadManifestRejectionKind {
        self.rejection_kind
    }

    pub fn canonical_basis(&self) -> &str {
        self.canonical_basis.as_ref()
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeSubscriptionReferenceWorkloadManifestDraft {
    schema_version: Arc<str>,
    product_ids: Vec<Arc<str>>,
    component_ids: Vec<Arc<str>>,
    lane_ids: Vec<Arc<str>>,
}

impl BridgeSubscriptionReferenceWorkloadManifestDraft {
    pub(crate) fn new(
        product_ids: Vec<impl Into<Arc<str>>>,
        component_ids: Vec<impl Into<Arc<str>>>,
        lane_ids: Vec<impl Into<Arc<str>>>,
    ) -> Self {
        Self {
            schema_version: Arc::from("subscription-reference-workload-manifest-v1"),
            product_ids: product_ids.into_iter().map(Into::into).collect(),
            component_ids: component_ids.into_iter().map(Into::into).collect(),
            lane_ids: lane_ids.into_iter().map(Into::into).collect(),
        }
    }

    pub(crate) fn seal(
        self,
    ) -> Result<
        BridgeSubscriptionReferenceWorkloadManifestSealed,
        BridgeSubscriptionReferenceWorkloadManifestRejection,
    > {
        let mut product_ids = self.product_ids;
        product_ids.sort();
        product_ids.dedup();
        if product_ids.len() != REQUIRED_PRODUCT_COUNT {
            return Err(BridgeSubscriptionReferenceWorkloadManifestRejection::new(
                BridgeSubscriptionReferenceWorkloadManifestRejectionKind::ProductCountMismatch,
            ));
        }

        let mut component_ids = self.component_ids;
        component_ids.sort();
        component_ids.dedup();
        if REQUIRED_COMPONENTS.iter().any(|required| {
            !component_ids
                .iter()
                .any(|component| component.as_ref() == *required)
        }) {
            return Err(BridgeSubscriptionReferenceWorkloadManifestRejection::new(
                BridgeSubscriptionReferenceWorkloadManifestRejectionKind::MissingRequiredComponent,
            ));
        }

        let mut lane_ids = self.lane_ids;
        lane_ids.sort();
        lane_ids.dedup();
        if lane_ids.is_empty() {
            return Err(BridgeSubscriptionReferenceWorkloadManifestRejection::new(
                BridgeSubscriptionReferenceWorkloadManifestRejectionKind::EmptyLaneSet,
            ));
        }

        let canonical_basis = Arc::<str>::from(format!(
            "bridge-subscription-reference-workload-manifest|schema={}|products={}|components={}|lanes={}",
            self.schema_version,
            join_arc_str(&product_ids),
            join_arc_str(&component_ids),
            join_arc_str(&lane_ids),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Ok(BridgeSubscriptionReferenceWorkloadManifestSealed {
            schema_version: self.schema_version,
            product_ids,
            component_ids,
            lane_ids,
            canonical_basis,
            digest: Arc::from(format!(
                "bridge-subscription-reference-workload-manifest:sha256:{digest:x}"
            )),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeSubscriptionReferenceWorkloadManifestSealed {
    schema_version: Arc<str>,
    product_ids: Vec<Arc<str>>,
    component_ids: Vec<Arc<str>>,
    lane_ids: Vec<Arc<str>>,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeSubscriptionReferenceWorkloadManifestSealed {
    pub fn schema_version(&self) -> &str {
        self.schema_version.as_ref()
    }

    pub fn product_ids(&self) -> &[Arc<str>] {
        &self.product_ids
    }

    pub fn component_ids(&self) -> &[Arc<str>] {
        &self.component_ids
    }

    pub fn lane_ids(&self) -> &[Arc<str>] {
        &self.lane_ids
    }

    pub fn canonical_basis(&self) -> &str {
        self.canonical_basis.as_ref()
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}
