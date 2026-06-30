use std::collections::BTreeSet;

use super::closeout::require_complete_cluster_coverage;
use super::current_matrix::current_coverage_targets;
use super::dependency_row::{
    KernelCompiledProductConsumerClusterIdentity, KernelCompiledProductConsumerDependencyRow,
};
use super::error::{
    KernelCompiledProductConsumerDependencyError, KernelCompiledProductConsumerDependencyErrorKind,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct KernelCompiledProductConsumerDependencyMatrix {
    rows: Vec<KernelCompiledProductConsumerDependencyRow>,
}

pub fn current_kernel_compiled_product_consumer_dependency_matrix() -> Result<
    KernelCompiledProductConsumerDependencyMatrix,
    KernelCompiledProductConsumerDependencyError,
> {
    let targets = current_coverage_targets()?;
    let rows = targets
        .iter()
        .map(|target| target.lower_row())
        .collect::<Result<Vec<_>, _>>()?;
    KernelCompiledProductConsumerDependencyMatrix::new(rows, &targets)
}

impl KernelCompiledProductConsumerDependencyMatrix {
    pub(crate) fn new(
        mut rows: Vec<KernelCompiledProductConsumerDependencyRow>,
        targets: &[super::coverage_target::KernelCompiledProductConsumerCoverageTarget],
    ) -> Result<Self, KernelCompiledProductConsumerDependencyError> {
        let mut seen = BTreeSet::new();
        for row in &rows {
            if !seen.insert(row.cluster_identity()) {
                return Err(KernelCompiledProductConsumerDependencyError::new(
                    KernelCompiledProductConsumerDependencyErrorKind::DuplicateClusterBinding,
                    format!(
                        "kernel compiled-product consumer cluster `{}` was classified more than once",
                        row.cluster_identity().as_str()
                    ),
                ));
            }
        }
        require_complete_cluster_coverage(targets, &rows)?;
        rows.sort_by_key(|row| row.cluster_identity());
        Ok(Self { rows })
    }

    pub fn rows(&self) -> &[KernelCompiledProductConsumerDependencyRow] {
        &self.rows
    }

    pub fn require_cluster(
        &self,
        cluster_identity: KernelCompiledProductConsumerClusterIdentity,
    ) -> Result<
        &KernelCompiledProductConsumerDependencyRow,
        KernelCompiledProductConsumerDependencyError,
    > {
        self.rows
            .iter()
            .find(|row| row.cluster_identity() == cluster_identity)
            .ok_or_else(|| {
                KernelCompiledProductConsumerDependencyError::new(
                    KernelCompiledProductConsumerDependencyErrorKind::MissingRequiredCluster,
                    format!(
                        "kernel compiled-product consumer cluster `{}` is missing",
                        cluster_identity.as_str()
                    ),
                )
            })
    }
}
