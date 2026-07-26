use worth_query_execution::facade::domain_computation::WorthQueryDirectRunTerminal;

fn repeat_cleanup(terminal: WorthQueryDirectRunTerminal) {
    let _first = terminal.cleanup();
    let _second = terminal.cleanup();
}

fn main() {}
