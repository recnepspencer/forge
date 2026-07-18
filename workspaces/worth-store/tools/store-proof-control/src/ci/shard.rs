use serde::{Deserialize, Serialize};

use crate::evidence::sha256_serialized;
use crate::selection::{ProofExecutionUnit, ProofProcessModel};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CiShardAssignment {
    pub shard_index: usize,
    pub estimated_weight: u64,
    pub unit_identities: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CiShardPlan {
    pub plan_identity: String,
    pub partition: String,
    pub shard_count: usize,
    pub selected_shard: usize,
    pub assignments: Vec<CiShardAssignment>,
}

impl CiShardPlan {
    pub fn lower(
        partition: &str,
        units: &[ProofExecutionUnit],
        shard_count: usize,
        selected_shard: usize,
    ) -> Result<Self, String> {
        if shard_count == 0 || selected_shard >= shard_count || shard_count > units.len() {
            return Err(format!(
                "invalid CI shard selection {selected_shard}/{shard_count}"
            ));
        }
        let mut assignments: Vec<_> = (0..shard_count)
            .map(|shard_index| CiShardAssignment {
                shard_index,
                estimated_weight: 0,
                unit_identities: Vec::new(),
            })
            .collect();
        let mut weighted: Vec<_> = units
            .iter()
            .map(|unit| (unit.identity(), estimated_weight(unit)))
            .collect();
        weighted.sort_by(|left, right| right.1.cmp(&left.1).then_with(|| left.0.cmp(&right.0)));
        for (unit_identity, weight) in weighted {
            let assignment = assignments
                .iter_mut()
                .min_by_key(|assignment| (assignment.estimated_weight, assignment.shard_index))
                .expect("positive shard count creates assignments");
            assignment.estimated_weight += weight;
            assignment.unit_identities.push(unit_identity);
        }
        for assignment in &mut assignments {
            assignment.unit_identities.sort();
        }
        let mut plan = Self {
            plan_identity: String::new(),
            partition: partition.to_owned(),
            shard_count,
            selected_shard,
            assignments,
        };
        plan.plan_identity = sha256_serialized(&(
            "worth-store-ci-shard-plan-v1",
            &plan.partition,
            plan.shard_count,
            &plan.assignments,
        ))?;
        Ok(plan)
    }

    pub fn includes(&self, unit: &ProofExecutionUnit) -> bool {
        self.assignments[self.selected_shard]
            .unit_identities
            .binary_search(&unit.identity())
            .is_ok()
    }
}

fn estimated_weight(unit: &ProofExecutionUnit) -> u64 {
    match unit.process_model {
        ProofProcessModel::StandardizedUiHarness => 12,
        ProofProcessModel::LibtestWithFreshChildProcess
        | ProofProcessModel::LibtestWithDeclaredSubprocesses
        | ProofProcessModel::LibtestWithNestedCargoProcess => 10,
        ProofProcessModel::AllocatorGlobalProcess => 8,
        ProofProcessModel::ExternalToolProcess => 20,
        ProofProcessModel::RustdocTestProcess | ProofProcessModel::CargoCheckProcess => 3,
        ProofProcessModel::NestedCargoProcess => 6,
        ProofProcessModel::LibtestProcess => 1,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn invalid_shard_coordinates_deny_before_assignment() {
        assert!(CiShardPlan::lower("partition", &[], 0, 0).is_err());
        assert!(CiShardPlan::lower("partition", &[], 2, 2).is_err());
    }
}
