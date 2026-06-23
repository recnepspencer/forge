use crate::source::{
    WorthUiLayoutSizingSpec, WorthUiLayoutSizingValue, WorthUiLayoutTopologyDiagnosticCode,
    WorthUiSourceToken, WorthUiSourceTokenKind,
};

use super::parser_state::WorthUiLayoutTopologyParser;

impl<'a> WorthUiLayoutTopologyParser<'a> {
    pub(super) fn parse_sizing_spec(&mut self) -> Option<WorthUiLayoutSizingSpec> {
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

    pub(super) fn parse_parenthesized_value(&mut self) -> Option<WorthUiLayoutSizingValue> {
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
}
