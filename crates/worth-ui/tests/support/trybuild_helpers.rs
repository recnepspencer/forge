#![allow(dead_code)]

pub fn run_pass_cases(paths: &[&str]) {
    let tests = trybuild::TestCases::new();

    for path in paths {
        tests.pass(*path);
    }
}

pub fn run_compile_fail_cases(paths: &[&str]) {
    let tests = trybuild::TestCases::new();

    for path in paths {
        tests.compile_fail(*path);
    }
}
