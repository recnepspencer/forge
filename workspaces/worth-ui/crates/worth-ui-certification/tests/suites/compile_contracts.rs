//! Batched hostile compiler contracts for certification boundaries.

const CASES: &str = include_str!("compile_contract_cases.csv");

#[test]
fn certification_compile_contracts() {
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
