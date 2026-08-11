#![deny(unused_must_use)]

use worth_query_host::facade::installed::domain_computation::{
    WorthQueryDirectRunCleanupFailure, WorthQueryDirectRunTerminal,
};

fn ignore_terminal(terminal: WorthQueryDirectRunTerminal) {
    std::convert::identity(terminal);
}

fn ignore_failure(failure: WorthQueryDirectRunCleanupFailure) {
    std::convert::identity(failure);
}

fn ignore_retry(failure: WorthQueryDirectRunCleanupFailure) {
    failure.retry();
}

fn ignore_restored_terminal(failure: WorthQueryDirectRunCleanupFailure) {
    failure.into_terminal();
}

fn main() {}
