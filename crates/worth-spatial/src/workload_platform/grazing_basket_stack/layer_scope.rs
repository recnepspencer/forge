use worth_primitives::{truth_digest_parts, TruthDigestScope};

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct BasketLayerIndex(usize);

impl BasketLayerIndex {
    pub fn new(index: usize) -> Self {
        Self(index)
    }

    pub fn get(self) -> usize {
        self.0
    }

    pub fn human_name(self) -> String {
        format!("basket layer {}", self.0)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BasketBoundaryScope {
    layer: BasketLayerIndex,
    boundary_index: usize,
    boundary_identity: String,
}

impl BasketBoundaryScope {
    pub(crate) fn new(
        layer: BasketLayerIndex,
        boundary_index: usize,
        stack_identity: &str,
    ) -> Self {
        let boundary_identity = truth_digest_parts(
            TruthDigestScope::ArtifactIdentity,
            &[
                "grazing-basket-boundary".to_string(),
                stack_identity.to_string(),
                format!("layer:{}", layer.get()),
                format!("boundary:{boundary_index}"),
            ],
        );
        Self {
            layer,
            boundary_index,
            boundary_identity,
        }
    }

    pub fn layer(&self) -> BasketLayerIndex {
        self.layer
    }

    pub fn boundary_index(&self) -> usize {
        self.boundary_index
    }

    pub fn boundary_identity(&self) -> &str {
        &self.boundary_identity
    }
}
