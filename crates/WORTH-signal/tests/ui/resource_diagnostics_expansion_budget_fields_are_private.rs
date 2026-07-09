use worth_signal::facade::ResourceDiagnosticsExpansionBudget;

fn main() {
    let _WORTHd = ResourceDiagnosticsExpansionBudget {
        allow_cold_reconstruction: true,
        max_replay_reconstruction_width: 1,
    };
}
