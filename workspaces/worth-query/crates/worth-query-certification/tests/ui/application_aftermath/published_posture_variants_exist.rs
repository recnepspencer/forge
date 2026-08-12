//! Positive twin for provisional/NoMutation unreachability: law-14 postures exist.

use worth_query_host::facade::domain::PublishedAftermathPosture;

fn main() {
    let _ = PublishedAftermathPosture::Reversible;
    let _ = PublishedAftermathPosture::Compensatable;
    let _ = PublishedAftermathPosture::Reconcilable;
    let _ = PublishedAftermathPosture::Irreversible;
}
