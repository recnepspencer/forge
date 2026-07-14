use worth_query::facade::consumer_kit::{
    WorthQueryRuntimeFacadeFamily, WorthQuerySupportPinFinding,
    WorthQuerySupportPinFindingKind,
};

fn main() {
    let _ = WorthQuerySupportPinFinding {
        kind: WorthQuerySupportPinFindingKind::StatusMismatch,
        family: Some(WorthQueryRuntimeFacadeFamily::Write),
        surface: String::new(),
        expected: None,
        found: None,
        blocking: true,
        finding_digest: String::new(),
    };
}
