mod node_parser;
mod parser_state;
mod sizing_parser;
mod slot_validation;

use crate::source::{
    WorthUiLayoutAxis, WorthUiLayoutTopologyDiagnostic, WorthUiLayoutTopologyDiagnosticCode,
    WorthUiLayoutTopologyNode, WorthUiLayoutTopologyReport, WorthUiSourceToken,
};

use parser_state::WorthUiLayoutTopologyParser;

pub(crate) fn parse_layout_topology(
    tokens: &[WorthUiSourceToken],
    layout_locus: &str,
) -> Result<WorthUiLayoutTopologyNode, WorthUiLayoutTopologyReport> {
    if tokens.is_empty() {
        return Err(WorthUiLayoutTopologyReport::new(vec![
            WorthUiLayoutTopologyDiagnostic::new(
                WorthUiLayoutTopologyDiagnosticCode::MissingLayoutRoot,
                layout_locus,
                "layout declaration requires one root row or column node",
            ),
        ]));
    }

    let mut parser = WorthUiLayoutTopologyParser::new(tokens, layout_locus);
    let root = parser.parse_root();
    parser.validate_slot_uniqueness(&root);
    if parser.has_trailing_tokens() {
        parser.push(
            WorthUiLayoutTopologyDiagnosticCode::InvalidLayoutRoot,
            "layout declaration contains unexpected trailing tokens",
        );
    }

    let diagnostics = parser.into_diagnostics();
    if diagnostics.is_empty() {
        Ok(root)
    } else {
        Err(WorthUiLayoutTopologyReport::new(diagnostics))
    }
}

pub(super) fn empty_invalid_root() -> WorthUiLayoutTopologyNode {
    WorthUiLayoutTopologyNode::new(
        WorthUiLayoutAxis::Column,
        None,
        None,
        None,
        None,
        false,
        false,
        false,
        Vec::new(),
    )
}
