use worth_store_certification::S6ProductionReadinessClosureInput;

fn main() {
    let terminal_projection = String::from("{\"kind\":\"s6-closeout\"}");
    let _ = S6ProductionReadinessClosureInput::from_phase13_adoption(terminal_projection);
}
