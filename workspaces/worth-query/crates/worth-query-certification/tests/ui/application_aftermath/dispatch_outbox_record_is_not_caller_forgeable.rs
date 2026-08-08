//! Q8.25-C2: a caller cannot author what the outbox co-commits.
//!
//! The record is public because a host must be able to *read* what was
//! committed. Naming the effect, wire identity, payload bytes, or byte bound is
//! the runtime's job: those four come from the installed contract and admitted
//! emission, never from whoever holds the record type.

use worth_query_execution::facade::primary_graph::WorthQueryDispatchOutboxRecord;

fn main() {
    let _ = <WorthQueryDispatchOutboxRecord>::from_installed_contract;
}
