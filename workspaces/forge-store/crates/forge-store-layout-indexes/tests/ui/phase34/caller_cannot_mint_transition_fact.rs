use forge_store_layout_indexes::layout_certification::{
    S9LayoutMachineState, S9LayoutMachineTransition, S9LayoutProductionOperation,
    S9LayoutProductionTransition, S9LayoutStateMachine,
};

fn main() {
    let _forged = S9LayoutProductionTransition::new(
        S9LayoutStateMachine::AccessLowering,
        S9LayoutProductionOperation::LowerSelectedAccess,
        S9LayoutMachineState::Budgeted,
        S9LayoutMachineTransition::Lower,
        S9LayoutMachineState::Lowered,
    );
}
