use super::graph_mutation_stage::UiGraphMutationStage;
use crate::graph::{
    UiGraphInstantiationPlan, UiGraphMutationCommitDenial, UiGraphMutationCommitResult,
    UiGraphWorldProfile,
};

impl UiGraphInstantiationPlan {
    pub fn commit_initial_generation(
        &self,
        world_profile: UiGraphWorldProfile,
    ) -> Result<UiGraphMutationCommitResult, UiGraphMutationCommitDenial> {
        if !self.local_denials().is_empty() {
            return Err(UiGraphMutationCommitDenial::from_local_denials(
                self.local_denials().to_vec(),
            ));
        }

        Ok(UiGraphMutationCommitResult::new(
            UiGraphMutationStage::from_initial_plan(self, world_profile).commit(),
        ))
    }
}
