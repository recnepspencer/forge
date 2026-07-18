use worth_store_authority::ControlStoreFencingAuthority;
use worth_store_operations::certification_scenario::ExactScenarioControlSelection;
use worth_store_operations::{
    ControlStoreSelectionIndeterminate, ControlStoreTrustPosture,
    DivergentControlGenerationSelectionReceipt,
};

use super::ExecutedOwnerWorld;

impl ExecutedOwnerWorld {
    pub fn controlled_selected_prefix_defect(&self) -> ControlStoreSelectionIndeterminate {
        let control = self.media.control_store();
        let provider = ExactScenarioControlSelection::current(self.media.authority(), &control)
            .with_selected_prefix_digest_for_controlled_defect([0xee; 32]);
        let fencing =
            ControlStoreFencingAuthority::for_current_store(self.media.authority(), &provider);
        let worth_store_operations::ControlStoreTrustPosture::Indeterminate(denial) =
            control.inspect_generations(&fencing)
        else {
            panic!("a provider-selected foreign prefix must fail closed");
        };
        denial
    }

    pub fn divergent_control_generation_selection(
        &self,
    ) -> DivergentControlGenerationSelectionReceipt {
        let left = self.media.independent_control_store_at(
            self.media.workspace_root().join("control-copies/left.log"),
            "control-copy-left",
        );
        let right = self.media.independent_control_store_at(
            self.media.workspace_root().join("control-copies/right.log"),
            "control-copy-right",
        );
        let _ = self.media.abandon("control-copy-common", &left);
        let _ = self.media.abandon("control-copy-common", &right);
        let _ = self.media.abandon("control-copy-left-advance", &left);
        let _ = self.media.abandon("control-copy-right-advance", &right);
        let rejected_copy = right
            .observe_selection_coordinates()
            .unwrap()
            .expect("right copy has durable generations");
        let provider = ExactScenarioControlSelection::current(self.media.authority(), &left);
        let fencing =
            ControlStoreFencingAuthority::for_current_store(self.media.authority(), &provider);
        let ControlStoreTrustPosture::Selected(selected) =
            worth_store_operations::inspect_control_store_copies(&[&left, &right], &fencing)
        else {
            panic!("the externally selected left copy must be the only current generation");
        };
        let ControlStoreTrustPosture::Indeterminate(rejection) =
            right.inspect_generations(&fencing)
        else {
            panic!("the independently advanced right copy must not select itself");
        };
        DivergentControlGenerationSelectionReceipt::from_selected_generation_and_rejected_copy(
            &selected,
            rejected_copy,
            &rejection,
        )
        .unwrap()
    }
}
