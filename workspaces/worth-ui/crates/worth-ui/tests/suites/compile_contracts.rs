//! Batched public-facade compiler contracts.
//!
//! `compile_contract_cases.csv` is the complete reconciliation inventory.
//! This executable subset retains every relational and typestate denial plus
//! one compiler-checked representative for each structurally equivalent
//! privacy/export family. The topology budget checker proves that this subset
//! remains complete with respect to the inventory.

const CASES: &str = include_str!("compile_contract_execution.csv");

#[test]
fn public_facade_compile_contracts() {
    let tests = trybuild::TestCases::new();
    for row in CASES.lines().skip(1) {
        let (kind, remainder) = row.split_once(',').expect("compile case kind");
        let (path, _legacy_harness) = remainder.rsplit_once(',').expect("compile case owner");
        if kind == "fail" {
            tests.compile_fail(path);
        } else {
            tests.pass(path);
        }
    }
}
