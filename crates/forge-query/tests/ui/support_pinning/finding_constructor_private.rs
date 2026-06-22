use forge_query::facade::consumer_kit::{
    ForgeQueryRuntimeFacadeFamily, ForgeQuerySupportPinFinding,
    ForgeQuerySupportPinFindingKind,
};

fn main() {
    let _ = ForgeQuerySupportPinFinding {
        kind: ForgeQuerySupportPinFindingKind::StatusMismatch,
        family: Some(ForgeQueryRuntimeFacadeFamily::Write),
        surface: String::new(),
        expected: None,
        found: None,
        blocking: true,
        finding_digest: String::new(),
    };
}
