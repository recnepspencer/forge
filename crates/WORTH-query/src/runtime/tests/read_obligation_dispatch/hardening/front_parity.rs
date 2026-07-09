use super::super::*;
use super::dispatch_catalog::workspace_with_selector_catalog;

#[derive(Clone, Copy)]
pub(super) enum HelperFront {
    Execute,
    Intent,
    Compose,
}

#[derive(Debug, Eq, PartialEq)]
pub(super) struct HelperParityEvidence {
    pub(super) matched_count: usize,
    pub(super) full_scan_count: usize,
    pub(super) envelope_digest: Option<String>,
}

pub(super) fn helper_parity_result(name: &str, front: HelperFront) -> HelperParityEvidence {
    let mut workspace = workspace_with_selector_catalog(name);
    let result = match front {
        HelperFront::Execute => {
            let family = identity_read_family(&mut workspace, "tasks");
            workspace.execute_read_family(&family)
        }
        HelperFront::Intent => {
            let family = identity_read_family(&mut workspace, "tasks");
            workspace.read_family_intent(&family).execute()
        }
        HelperFront::Compose => workspace.compose_read(|read| {
            read.local_detail(
                "user",
                manager_schema(),
                |query| {
                    query.project(
                        AspectFieldSelector::new("identity", "id")
                            .expect("identity projection should build"),
                    )
                },
                |shape| {
                    shape.field(
                        AuthoredResultShapeField::new("identity", "id", "id")
                            .expect("identity result-shape field should build"),
                    )
                },
            )
        }),
    }
    .expect("helper front should execute");
    let dispatch = result.receipt().graph_obligation_dispatch().unwrap();
    HelperParityEvidence {
        matched_count: dispatch.selection().matched_obligation_count(),
        full_scan_count: dispatch
            .selection()
            .counters()
            .registration_full_scan_count(),
        envelope_digest: dispatch.envelope_digest().map(str::to_string),
    }
}
