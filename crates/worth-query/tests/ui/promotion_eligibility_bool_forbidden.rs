use worth_query::facade::PreviewSessionQueryContext;
use worth_runtime_bridge::facade::{BridgePreviewSession, PreviewDeclared};

fn main() {
    let session: BridgePreviewSession<PreviewDeclared> = todo!();
    let _ = PreviewSessionQueryContext::declared(&session, true);
}
