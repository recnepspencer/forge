use std::collections::BTreeSet;

use crate::source::{
    WorthUiLayoutAxis, WorthUiLayoutDimension, WorthUiLayoutSizingSpec, WorthUiLayoutSizingValue,
    WorthUiLayoutSlotNode, WorthUiLayoutTopologyChild, WorthUiLayoutTopologyDiagnostic,
    WorthUiLayoutTopologyDiagnosticCode, WorthUiLayoutTopologyNode, WorthUiLayoutTopologyReport,
    WorthUiSourceToken, WorthUiSourceTokenKind,
};

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
    if parser.index < parser.tokens.len() {
        parser.push(
            WorthUiLayoutTopologyDiagnosticCode::InvalidLayoutRoot,
            "layout declaration contains unexpected trailing tokens",
        );
    }

    if parser.diagnostics.is_empty() {
        Ok(root)
    } else {
        Err(WorthUiLayoutTopologyReport::new(parser.diagnostics))
    }
}

struct WorthUiLayoutTopologyParser<'a> {
    tokens: &'a [WorthUiSourceToken],
    index: usize,
    layout_locus: &'a str,
    diagnostics: Vec<WorthUiLayoutTopologyDiagnostic>,
}

impl<'a> WorthUiLayoutTopologyParser<'a> {
    fn new(tokens: &'a [WorthUiSourceToken], layout_locus: &'a str) -> Self {
        Self {
            tokens,
            index: 0,
            layout_locus,
            diagnostics: Vec::new(),
        }
    }

    fn parse_root(&mut self) -> WorthUiLayoutTopologyNode {
        match self.peek_identifier() {
            Some("row") | Some("column") => self.parse_region("root"),
            Some("slot") => {
                self.push(
                    WorthUiLayoutTopologyDiagnosticCode::InvalidLayoutRoot,
                    "layout root must be a row or column node, not a slot leaf",
                );
                WorthUiLayoutTopologyNode::new(
                    WorthUiLayoutAxis::Column,
                    None,
                    None,
                    false,
                    false,
                    false,
                    Vec::new(),
                )
            }
            _ => {
                self.push(
                    WorthUiLayoutTopologyDiagnosticCode::InvalidLayoutRoot,
                    "layout declaration requires one root row or column node",
                );
                WorthUiLayoutTopologyNode::new(
                    WorthUiLayoutAxis::Column,
                    None,
                    None,
                    false,
                    false,
                    false,
                    Vec::new(),
                )
            }
        }
    }

    fn parse_region(&mut self, locus: &str) -> WorthUiLayoutTopologyNode {
        let axis = match self.advance_identifier().as_deref() {
            Some("row") => WorthUiLayoutAxis::Row,
            Some("column") => WorthUiLayoutAxis::Column,
            _ => {
                self.push(
                    WorthUiLayoutTopologyDiagnosticCode::InvalidLayoutNode,
                    "expected a row or column node",
                );
                return WorthUiLayoutTopologyNode::new(
                    WorthUiLayoutAxis::Column,
                    None,
                    None,
                    false,
                    false,
                    false,
                    Vec::new(),
                );
            }
        };

        let mut dimension = None;
        let mut sizing = None;
        let mut scroll_owner = false;
        let mut resizable = false;
        let mut restorable = false;

        while !self.is_at_left_brace() && !self.is_eof() {
            match self.peek_identifier() {
                Some("width") | Some("height") => {
                    if dimension.is_some() || sizing.is_some() {
                        self.push(
                            WorthUiLayoutTopologyDiagnosticCode::InvalidLayoutModifier,
                            "layout node cannot declare more than one sizing dimension",
                        );
                        self.index += 1;
                        continue;
                    }
                    let authored_dimension = self.advance_identifier().unwrap_or_default();
                    dimension = match authored_dimension.as_str() {
                        "width" => Some(WorthUiLayoutDimension::Width),
                        "height" => Some(WorthUiLayoutDimension::Height),
                        _ => None,
                    };
                    if !dimension_matches_axis(&axis, dimension.as_ref()) {
                        self.push(
                            WorthUiLayoutTopologyDiagnosticCode::InvalidLayoutModifier,
                            "rows size by height and columns size by width",
                        );
                    }
                    sizing = self.parse_sizing_spec();
                }
                Some("scroll_owner") => {
                    scroll_owner = true;
                    self.index += 1;
                }
                Some("resizable") => {
                    resizable = true;
                    self.index += 1;
                }
                Some("restore") => {
                    restorable = true;
                    self.index += 1;
                }
                _ => {
                    self.push(
                        WorthUiLayoutTopologyDiagnosticCode::InvalidLayoutModifier,
                        format!("unsupported layout modifier at {locus}"),
                    );
                    self.index += 1;
                }
            }
        }

        if restorable && !resizable {
            self.push(
                WorthUiLayoutTopologyDiagnosticCode::InvalidResizePersistence,
                "layout node cannot request restore without also being resizable",
            );
        }

        self.expect_left_brace(
            WorthUiLayoutTopologyDiagnosticCode::InvalidLayoutNode,
            "layout node requires a child block",
        );
        let mut children = Vec::new();
        let mut child_index = 0usize;

        while !self.is_eof() && !self.is_at_right_brace() {
            match self.peek_identifier() {
                Some("row") | Some("column") => {
                    let child_locus = format!("{locus}/region[{child_index}]");
                    children.push(WorthUiLayoutTopologyChild::Region(
                        self.parse_region(child_locus.as_str()),
                    ));
                    child_index += 1;
                }
                Some("slot") => {
                    children.push(WorthUiLayoutTopologyChild::Slot(
                        self.parse_slot(format!("{locus}/slot[{child_index}]").as_str()),
                    ));
                    child_index += 1;
                }
                _ => {
                    self.push(
                        WorthUiLayoutTopologyDiagnosticCode::InvalidLayoutNode,
                        "layout children must be row, column, or slot declarations",
                    );
                    self.index += 1;
                }
            }
        }
        self.expect_right_brace(
            WorthUiLayoutTopologyDiagnosticCode::InvalidLayoutNode,
            "layout node requires a closing '}'",
        );

        WorthUiLayoutTopologyNode::new(
            axis,
            dimension,
            sizing,
            scroll_owner,
            resizable,
            restorable,
            children,
        )
    }

    fn parse_slot(&mut self, locus: &str) -> WorthUiLayoutSlotNode {
        self.advance_identifier();
        match self.advance_identifier() {
            Some(name) => WorthUiLayoutSlotNode::new(name),
            None => {
                self.push(
                    WorthUiLayoutTopologyDiagnosticCode::InvalidLayoutNode,
                    format!("slot declaration at {locus} requires a slot name"),
                );
                WorthUiLayoutSlotNode::new("invalid-slot")
            }
        }
    }

    fn parse_sizing_spec(&mut self) -> Option<WorthUiLayoutSizingSpec> {
        match self.advance_identifier().as_deref() {
            Some("fit") => Some(WorthUiLayoutSizingSpec::Fit),
            Some("fill") => Some(WorthUiLayoutSizingSpec::Fill),
            Some("fixed") => self
                .parse_parenthesized_value()
                .map(WorthUiLayoutSizingSpec::Fixed),
            Some("share") => self
                .parse_parenthesized_number()
                .map(WorthUiLayoutSizingSpec::Share),
            Some("ratio") => self.parse_ratio_spec(),
            Some("clamp") => self.parse_clamp_spec(),
            _ => {
                self.push(
                    WorthUiLayoutTopologyDiagnosticCode::InvalidLayoutSizing,
                    "unsupported sizing expression",
                );
                None
            }
        }
    }

    fn parse_ratio_spec(&mut self) -> Option<WorthUiLayoutSizingSpec> {
        self.expect_left_paren();
        let numerator = self.advance_number();
        self.expect_comma();
        let denominator = self.advance_number();
        self.expect_right_paren();
        match (numerator, denominator) {
            (Some(numerator), Some(denominator)) if numerator > 0 && denominator > 0 => {
                Some(WorthUiLayoutSizingSpec::Ratio {
                    numerator,
                    denominator,
                })
            }
            _ => {
                self.push(
                    WorthUiLayoutTopologyDiagnosticCode::InvalidLayoutSizing,
                    "ratio sizing requires two positive numeric terms",
                );
                None
            }
        }
    }

    fn parse_clamp_spec(&mut self) -> Option<WorthUiLayoutSizingSpec> {
        self.expect_left_paren();
        self.expect_named_field("min");
        let min = self.parse_sizing_value();
        self.expect_comma();
        self.expect_named_field("preferred");
        let preferred = self.parse_sizing_spec();
        self.expect_comma();
        self.expect_named_field("max");
        let max = self.parse_sizing_value();
        self.expect_right_paren();
        match (min, preferred, max) {
            (Some(min), Some(preferred), Some(max)) => Some(WorthUiLayoutSizingSpec::Clamp {
                min,
                preferred: Box::new(preferred),
                max,
            }),
            _ => None,
        }
    }

    fn parse_parenthesized_value(&mut self) -> Option<WorthUiLayoutSizingValue> {
        self.expect_left_paren();
        let value = self.parse_sizing_value();
        self.expect_right_paren();
        value
    }

    fn parse_parenthesized_number(&mut self) -> Option<u32> {
        self.expect_left_paren();
        let number = self.advance_number();
        self.expect_right_paren();
        number
    }

    fn parse_sizing_value(&mut self) -> Option<WorthUiLayoutSizingValue> {
        match self.tokens.get(self.index).map(WorthUiSourceToken::kind) {
            Some(WorthUiSourceTokenKind::Identifier(text)) => {
                self.index += 1;
                Some(WorthUiLayoutSizingValue::NamedToken(text.clone()))
            }
            Some(WorthUiSourceTokenKind::NumberLiteral(value)) => {
                self.index += 1;
                Some(WorthUiLayoutSizingValue::Number(*value))
            }
            _ => {
                self.push(
                    WorthUiLayoutTopologyDiagnosticCode::InvalidLayoutSizing,
                    "sizing value requires a named token or numeric literal",
                );
                None
            }
        }
    }

    fn validate_slot_uniqueness(&mut self, root: &WorthUiLayoutTopologyNode) {
        let mut seen = BTreeSet::new();
        validate_slots_in_node(root, &mut seen, &mut self.diagnostics, self.layout_locus);
    }

    fn push(&mut self, code: WorthUiLayoutTopologyDiagnosticCode, message: impl Into<String>) {
        self.diagnostics.push(WorthUiLayoutTopologyDiagnostic::new(
            code,
            self.layout_locus,
            message,
        ));
    }

    fn peek_identifier(&self) -> Option<&str> {
        match self.tokens.get(self.index).map(WorthUiSourceToken::kind) {
            Some(WorthUiSourceTokenKind::Identifier(text)) => Some(text.as_str()),
            _ => None,
        }
    }

    fn advance_identifier(&mut self) -> Option<String> {
        let identifier = self.peek_identifier()?.to_owned();
        self.index += 1;
        Some(identifier)
    }

    fn advance_number(&mut self) -> Option<u32> {
        match self.tokens.get(self.index).map(WorthUiSourceToken::kind) {
            Some(WorthUiSourceTokenKind::NumberLiteral(value)) => {
                self.index += 1;
                Some(*value)
            }
            _ => {
                self.push(
                    WorthUiLayoutTopologyDiagnosticCode::InvalidLayoutSizing,
                    "expected a numeric literal",
                );
                None
            }
        }
    }

    fn expect_named_field(&mut self, field_name: &str) {
        if self.advance_identifier().as_deref() != Some(field_name) {
            self.push(
                WorthUiLayoutTopologyDiagnosticCode::InvalidLayoutSizing,
                format!("clamp sizing requires a '{field_name}:' field"),
            );
        }
        if !matches!(
            self.tokens.get(self.index).map(WorthUiSourceToken::kind),
            Some(WorthUiSourceTokenKind::Colon)
        ) {
            self.push(
                WorthUiLayoutTopologyDiagnosticCode::InvalidLayoutSizing,
                format!("clamp sizing requires a ':' after '{field_name}'"),
            );
        } else {
            self.index += 1;
        }
    }

    fn expect_left_paren(&mut self) {
        if self.is_left_paren() {
            self.index += 1;
        } else {
            self.push(
                WorthUiLayoutTopologyDiagnosticCode::InvalidLayoutSizing,
                "sizing expression requires '('",
            );
        }
    }

    fn expect_right_paren(&mut self) {
        if self.is_right_paren() {
            self.index += 1;
        } else {
            self.push(
                WorthUiLayoutTopologyDiagnosticCode::InvalidLayoutSizing,
                "sizing expression requires ')'",
            );
        }
    }

    fn expect_comma(&mut self) {
        if matches!(
            self.tokens.get(self.index).map(WorthUiSourceToken::kind),
            Some(WorthUiSourceTokenKind::Comma)
        ) {
            self.index += 1;
        } else {
            self.push(
                WorthUiLayoutTopologyDiagnosticCode::InvalidLayoutSizing,
                "sizing expression requires ','",
            );
        }
    }

    fn expect_left_brace(&mut self, code: WorthUiLayoutTopologyDiagnosticCode, message: &str) {
        if self.is_at_left_brace() {
            self.index += 1;
        } else {
            self.push(code, message);
        }
    }

    fn expect_right_brace(&mut self, code: WorthUiLayoutTopologyDiagnosticCode, message: &str) {
        if self.is_at_right_brace() {
            self.index += 1;
        } else {
            self.push(code, message);
        }
    }

    fn is_at_left_brace(&self) -> bool {
        matches!(
            self.tokens.get(self.index).map(WorthUiSourceToken::kind),
            Some(WorthUiSourceTokenKind::LeftBrace)
        )
    }

    fn is_at_right_brace(&self) -> bool {
        matches!(
            self.tokens.get(self.index).map(WorthUiSourceToken::kind),
            Some(WorthUiSourceTokenKind::RightBrace)
        )
    }

    fn is_left_paren(&self) -> bool {
        matches!(
            self.tokens.get(self.index).map(WorthUiSourceToken::kind),
            Some(WorthUiSourceTokenKind::LeftParen)
        )
    }

    fn is_right_paren(&self) -> bool {
        matches!(
            self.tokens.get(self.index).map(WorthUiSourceToken::kind),
            Some(WorthUiSourceTokenKind::RightParen)
        )
    }

    fn is_eof(&self) -> bool {
        self.index >= self.tokens.len()
    }
}

fn validate_slots_in_node(
    node: &WorthUiLayoutTopologyNode,
    seen: &mut BTreeSet<String>,
    diagnostics: &mut Vec<WorthUiLayoutTopologyDiagnostic>,
    layout_locus: &str,
) {
    for child in node.children() {
        match child {
            WorthUiLayoutTopologyChild::Region(child_node) => {
                validate_slots_in_node(child_node, seen, diagnostics, layout_locus);
            }
            WorthUiLayoutTopologyChild::Slot(slot) => {
                if !seen.insert(slot.slot_name().to_owned()) {
                    diagnostics.push(WorthUiLayoutTopologyDiagnostic::new(
                        WorthUiLayoutTopologyDiagnosticCode::DuplicateLayoutSlot,
                        layout_locus,
                        format!(
                            "layout topology cannot declare slot '{}' more than once",
                            slot.slot_name()
                        ),
                    ));
                }
            }
        }
    }
}

fn dimension_matches_axis(
    axis: &WorthUiLayoutAxis,
    dimension: Option<&WorthUiLayoutDimension>,
) -> bool {
    matches!(
        (axis, dimension),
        (WorthUiLayoutAxis::Row, Some(WorthUiLayoutDimension::Height))
            | (
                WorthUiLayoutAxis::Column,
                Some(WorthUiLayoutDimension::Width)
            )
    )
}
