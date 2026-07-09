use std::sync::Arc;

use sha2::{Digest, Sha256};

mod workload_ids;

pub use workload_ids::{
    BridgeSubscriptionReferenceWorkloadComponentId,
    BridgeSubscriptionReferenceWorkloadComponentIdSet, BridgeSubscriptionReferenceWorkloadLaneId,
    BridgeSubscriptionReferenceWorkloadLaneIdSet, BridgeSubscriptionReferenceWorkloadProductId,
    BridgeSubscriptionReferenceWorkloadProductIdSet,
};

const REQUIRED_PRODUCT_COUNT: usize = 128;
const REQUIRED_COMPONENTS: [&str; 5] = ["copper", "glass", "labor", "rubber", "steel"];

fn join_workload_ids(values: &[impl AsRef<str>]) -> String {
    values
        .iter()
        .map(AsRef::as_ref)
        .collect::<Vec<_>>()
        .join(",")
}

fn contains_empty_workload_id(values: &[impl AsRef<str>]) -> bool {
    values.iter().any(|value| value.as_ref().is_empty())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BridgeSubscriptionReferenceWorkloadManifestRejectionKind {
    ProductCountMismatch,
    MissingRequiredComponent,
    EmptyLaneSet,
    EmptyProductId,
    EmptyComponentId,
    EmptyLaneId,
}

impl BridgeSubscriptionReferenceWorkloadManifestRejectionKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProductCountMismatch => "product_count_mismatch",
            Self::MissingRequiredComponent => "missing_required_component",
            Self::EmptyLaneSet => "empty_lane_set",
            Self::EmptyProductId => "empty_product_id",
            Self::EmptyComponentId => "empty_component_id",
            Self::EmptyLaneId => "empty_lane_id",
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
    product_ids: BridgeSubscriptionReferenceWorkloadProductIdSet,
    component_ids: BridgeSubscriptionReferenceWorkloadComponentIdSet,
    lane_ids: BridgeSubscriptionReferenceWorkloadLaneIdSet,
}

impl BridgeSubscriptionReferenceWorkloadManifestDraft {
    pub(crate) fn new(
        product_ids: BridgeSubscriptionReferenceWorkloadProductIdSet,
        component_ids: BridgeSubscriptionReferenceWorkloadComponentIdSet,
        lane_ids: BridgeSubscriptionReferenceWorkloadLaneIdSet,
    ) -> Self {
        Self {
            schema_version: Arc::from("subscription-reference-workload-manifest-v1"),
            product_ids,
            component_ids,
            lane_ids,
        }
    }

    pub(crate) fn seal(
        self,
    ) -> Result<
        BridgeSubscriptionReferenceWorkloadManifestSealed,
        BridgeSubscriptionReferenceWorkloadManifestRejection,
    > {
        let product_ids = self.product_ids.into_sorted_unique_ids();
        if contains_empty_workload_id(&product_ids) {
            return Err(BridgeSubscriptionReferenceWorkloadManifestRejection::new(
                BridgeSubscriptionReferenceWorkloadManifestRejectionKind::EmptyProductId,
            ));
        }
        if product_ids.len() != REQUIRED_PRODUCT_COUNT {
            return Err(BridgeSubscriptionReferenceWorkloadManifestRejection::new(
                BridgeSubscriptionReferenceWorkloadManifestRejectionKind::ProductCountMismatch,
            ));
        }

        let component_ids = self.component_ids.into_sorted_unique_ids();
        if contains_empty_workload_id(&component_ids) {
            return Err(BridgeSubscriptionReferenceWorkloadManifestRejection::new(
                BridgeSubscriptionReferenceWorkloadManifestRejectionKind::EmptyComponentId,
            ));
        }
        if REQUIRED_COMPONENTS.iter().any(|required| {
            !component_ids
                .iter()
                .any(|component| component.as_ref() == *required)
        }) {
            return Err(BridgeSubscriptionReferenceWorkloadManifestRejection::new(
                BridgeSubscriptionReferenceWorkloadManifestRejectionKind::MissingRequiredComponent,
            ));
        }

        let lane_ids = self.lane_ids.into_sorted_unique_ids();
        if contains_empty_workload_id(&lane_ids) {
            return Err(BridgeSubscriptionReferenceWorkloadManifestRejection::new(
                BridgeSubscriptionReferenceWorkloadManifestRejectionKind::EmptyLaneId,
            ));
        }
        if lane_ids.is_empty() {
            return Err(BridgeSubscriptionReferenceWorkloadManifestRejection::new(
                BridgeSubscriptionReferenceWorkloadManifestRejectionKind::EmptyLaneSet,
            ));
        }

        let canonical_basis = Arc::<str>::from(format!(
            "bridge-subscription-reference-workload-manifest|schema={}|products={}|components={}|lanes={}",
            self.schema_version,
            join_workload_ids(&product_ids),
            join_workload_ids(&component_ids),
            join_workload_ids(&lane_ids),
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
    product_ids: Vec<BridgeSubscriptionReferenceWorkloadProductId>,
    component_ids: Vec<BridgeSubscriptionReferenceWorkloadComponentId>,
    lane_ids: Vec<BridgeSubscriptionReferenceWorkloadLaneId>,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeSubscriptionReferenceWorkloadManifestSealed {
    pub fn schema_version(&self) -> &str {
        self.schema_version.as_ref()
    }

    pub fn product_ids(&self) -> &[BridgeSubscriptionReferenceWorkloadProductId] {
        &self.product_ids
    }

    pub fn component_ids(&self) -> &[BridgeSubscriptionReferenceWorkloadComponentId] {
        &self.component_ids
    }

    pub fn lane_ids(&self) -> &[BridgeSubscriptionReferenceWorkloadLaneId] {
        &self.lane_ids
    }

    pub fn canonical_basis(&self) -> &str {
        self.canonical_basis.as_ref()
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}
