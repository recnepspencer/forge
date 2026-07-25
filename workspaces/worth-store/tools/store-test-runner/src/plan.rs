use std::collections::BTreeMap;
use std::path::Path;

use crate::catalog::TestCatalog;
use crate::classification::CiTestLane;
use crate::product::TestProduct;

mod execution_unit;
mod integration_product;
mod offline_observer_build;
mod owner_product;
mod smoke_product;
mod structural_product;

use execution_unit::apply_ci_profiles;
pub(crate) use execution_unit::TestExecutionUnit;
use integration_product::integration_lane;
use offline_observer_build::offline_observer_build;
use owner_product::{owner, owner_ci};
use smoke_product::smoke;
use structural_product::structural;

#[derive(Debug, Clone)]
pub(crate) struct TestPlan {
    product: TestProduct,
    units: Vec<TestExecutionUnit>,
}

impl TestPlan {
    pub(crate) fn build(
        product: &TestProduct,
        catalog: &TestCatalog,
        workspace_root: &Path,
    ) -> Result<Self, String> {
        let mut units = match product {
            TestProduct::Owner { package } => owner(package, catalog, workspace_root)?,
            TestProduct::Smoke => smoke(catalog, workspace_root)?,
            TestProduct::Ui => integration_lane(CiTestLane::Ui, None, catalog, workspace_root)?,
            TestProduct::Mutants => {
                return Err("mutation campaigns execute outside the ordinary test plan".into());
            }
            TestProduct::Courtrooms { .. } => {
                return Err("courtroom campaigns execute outside the ordinary test plan".into());
            }
            TestProduct::Ci {
                lane: selected,
                shard,
            } => match selected {
                CiTestLane::OwnerUnit if shard.is_none() => owner_ci(workspace_root),
                CiTestLane::Structural if shard.is_none() => structural(workspace_root)?,
                CiTestLane::OwnerUnit | CiTestLane::Structural => {
                    return Err(format!("the {selected} partition is not shardable"))
                }
                selected_lane => integration_lane(*selected_lane, *shard, catalog, workspace_root)?,
            },
        };
        if matches!(product, TestProduct::Ci { .. }) {
            apply_ci_profiles(&mut units);
        }
        Self::new(product.clone(), units)
    }

    fn new(product: TestProduct, mut units: Vec<TestExecutionUnit>) -> Result<Self, String> {
        if units.is_empty() {
            return Err(format!(
                "test product `{}` selected zero units",
                product.name()
            ));
        }
        units.sort_by(|left, right| left.identity().cmp(right.identity()));
        let mut origins = BTreeMap::new();
        for unit in &units {
            if let Some(first) =
                origins.insert(unit.identity().to_owned(), unit.origin().to_owned())
            {
                return Err(format!(
                    "duplicate execution unit `{}` from `{first}` and `{}`",
                    unit.identity(),
                    unit.origin()
                ));
            }
        }
        Ok(Self { product, units })
    }

    pub(crate) fn product_name(&self) -> String {
        self.product.name()
    }

    pub(crate) fn units(&self) -> &[TestExecutionUnit] {
        &self.units
    }
}

#[cfg(test)]
mod tests;
