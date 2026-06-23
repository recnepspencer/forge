use crate::source::{
    WorthUiLayoutAxis, WorthUiLayoutDimension, WorthUiLayoutSlotNode, WorthUiLayoutTopologyChild,
    WorthUiLayoutTopologyDiagnosticCode, WorthUiLayoutTopologyNode,
};

use super::empty_invalid_root;
use super::parser_state::WorthUiLayoutTopologyParser;
use super::slot_validation::validate_slots_in_node;

impl<'a> WorthUiLayoutTopologyParser<'a> {
    pub(super) fn parse_root(&mut self) -> WorthUiLayoutTopologyNode {
        match self.peek_identifier() {
            Some("row") | Some("column") => self.parse_region("root"),
            Some("slot") => {
                self.push(
                    WorthUiLayoutTopologyDiagnosticCode::InvalidLayoutRoot,
                    "layout root must be a row or column node, not a slot leaf",
                );
                empty_invalid_root()
            }
            _ => {
                self.push(
                    WorthUiLayoutTopologyDiagnosticCode::InvalidLayoutRoot,
                    "layout declaration requires one root row or column node",
                );
                empty_invalid_root()
            }
        }
    }

    pub(super) fn parse_region(&mut self, locus: &str) -> WorthUiLayoutTopologyNode {
        let axis = match self.advance_identifier().as_deref() {
            Some("row") => WorthUiLayoutAxis::Row,
            Some("column") => WorthUiLayoutAxis::Column,
            _ => {
                self.push(
                    WorthUiLayoutTopologyDiagnosticCode::InvalidLayoutNode,
                    "expected a row or column node",
                );
                return empty_invalid_root();
            }
        };

        let mut dimension = None;
        let mut sizing = None;
        let mut gap = None;
        let mut padding = None;
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
                Some("gap") => {
                    if gap.is_some() {
                        self.push(
                            WorthUiLayoutTopologyDiagnosticCode::InvalidLayoutModifier,
                            "layout node cannot declare gap more than once",
                        );
                    }
                    self.index += 1;
                    gap = self.parse_parenthesized_value();
                }
                Some("padding") => {
                    if padding.is_some() {
                        self.push(
                            WorthUiLayoutTopologyDiagnosticCode::InvalidLayoutModifier,
                            "layout node cannot declare padding more than once",
                        );
                    }
                    self.index += 1;
                    padding = self.parse_parenthesized_value();
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
        let children = self.parse_region_children(locus);
        self.expect_right_brace(
            WorthUiLayoutTopologyDiagnosticCode::InvalidLayoutNode,
            "layout node requires a closing '}'",
        );

        WorthUiLayoutTopologyNode::new(
            axis,
            dimension,
            sizing,
            gap,
            padding,
            scroll_owner,
            resizable,
            restorable,
            children,
        )
    }

    pub(super) fn validate_slot_uniqueness(&mut self, root: &WorthUiLayoutTopologyNode) {
        let mut seen = std::collections::BTreeSet::new();
        let diagnostics = validate_slots_in_node(root, &mut seen, self.layout_locus);
        for diagnostic in diagnostics {
            self.push_diagnostic(diagnostic);
        }
    }

    fn parse_region_children(&mut self, locus: &str) -> Vec<WorthUiLayoutTopologyChild> {
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

        children
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
