use crate::runtime::{
    WorthUiAuthoredLiveViewParseDenial, WorthUiLiveViewConditionExpression,
    WorthUiLiveViewConditionalProjectionDeclaration, WorthUiLiveViewInteractionIntentDeclaration,
    WorthUiLiveViewInteractionIntentKind, WorthUiLiveViewParticipationPosture,
    WorthUiLiveViewPayloadProjectionDeclaration, WorthUiLiveViewPayloadShape,
    WorthUiLiveViewReadinessProjectionDeclaration,
};

use super::{parse_denial, parse_interaction_primitive_prop, unquote};

pub(super) fn parse_readiness_projection<'a, I>(
    readiness_id: &str,
    source_lines: &mut std::iter::Peekable<I>,
) -> Result<WorthUiLiveViewReadinessProjectionDeclaration, WorthUiAuthoredLiveViewParseDenial>
where
    I: Iterator<Item = (usize, &'a str)>,
{
    let mut required = Vec::new();
    loop {
        let Some((index, raw_line)) = source_lines.next() else {
            return Err(WorthUiAuthoredLiveViewParseDenial::new(
                0,
                "unterminated readiness block",
            ));
        };
        let line = raw_line.trim();
        if line.is_empty() {
            continue;
        }
        if line == "}" {
            return Ok(WorthUiLiveViewReadinessProjectionDeclaration::new(
                readiness_id,
                required,
            ));
        }
        if let Some(value) = line.strip_prefix("required ") {
            required.extend(
                value
                    .split(',')
                    .map(str::trim)
                    .filter(|value| !value.is_empty())
                    .map(ToOwned::to_owned),
            );
        } else {
            return Err(parse_denial(index, "expected required or }"));
        }
    }
}

pub(super) fn parse_payload_projection<'a, I>(
    payload_id: &str,
    source_lines: &mut std::iter::Peekable<I>,
) -> Result<WorthUiLiveViewPayloadProjectionDeclaration, WorthUiAuthoredLiveViewParseDenial>
where
    I: Iterator<Item = (usize, &'a str)>,
{
    let mut shape = WorthUiLiveViewPayloadShape::Unsupported(String::new());
    loop {
        let Some((index, raw_line)) = source_lines.next() else {
            return Err(WorthUiAuthoredLiveViewParseDenial::new(
                0,
                "unterminated payload block",
            ));
        };
        let line = raw_line.trim();
        if line.is_empty() {
            continue;
        }
        if line == "}" {
            return Ok(WorthUiLiveViewPayloadProjectionDeclaration::new(
                payload_id, shape,
            ));
        }
        if let Some(value) = line.strip_prefix("shape ") {
            shape = payload_shape(value.trim());
        } else {
            return Err(parse_denial(index, "expected shape or }"));
        }
    }
}

pub(super) fn parse_conditional_projection<'a, I>(
    control_id: &str,
    source_lines: &mut std::iter::Peekable<I>,
) -> Result<WorthUiLiveViewConditionalProjectionDeclaration, WorthUiAuthoredLiveViewParseDenial>
where
    I: Iterator<Item = (usize, &'a str)>,
{
    let mut condition = WorthUiLiveViewConditionExpression::Unsupported(String::new());
    let mut when_true = WorthUiLiveViewParticipationPosture::Present;
    let mut when_false = WorthUiLiveViewParticipationPosture::AbsentRetainingState;
    loop {
        let Some((index, raw_line)) = source_lines.next() else {
            return Err(WorthUiAuthoredLiveViewParseDenial::new(
                0,
                "unterminated condition block",
            ));
        };
        let line = raw_line.trim();
        if line.is_empty() {
            continue;
        }
        if line == "}" {
            return Ok(WorthUiLiveViewConditionalProjectionDeclaration::new(
                control_id, condition, when_true, when_false,
            ));
        }
        if let Some(value) = line.strip_prefix("when ") {
            condition = parse_condition_expression(value.trim());
        } else if let Some(value) = line.strip_prefix("true ") {
            when_true = participation_posture(value.trim());
        } else if let Some(value) = line.strip_prefix("false ") {
            when_false = participation_posture(value.trim());
        } else {
            return Err(parse_denial(index, "expected when, true, false, or }"));
        }
    }
}

pub(super) fn parse_interaction_intent<'a, I>(
    interaction_id: &str,
    source_lines: &mut std::iter::Peekable<I>,
) -> Result<WorthUiLiveViewInteractionIntentDeclaration, WorthUiAuthoredLiveViewParseDenial>
where
    I: Iterator<Item = (usize, &'a str)>,
{
    let mut declaration = WorthUiLiveViewInteractionIntentDeclaration::new(interaction_id);
    let mut primitive_props = Vec::new();
    loop {
        let Some((index, raw_line)) = source_lines.next() else {
            return Err(WorthUiAuthoredLiveViewParseDenial::new(
                0,
                "unterminated interaction block",
            ));
        };
        let line = raw_line.trim();
        if line.is_empty() {
            continue;
        }
        if line == "}" {
            return Ok(declaration.with_primitive_props(primitive_props));
        }
        if let Some(value) = line.strip_prefix("kind ") {
            declaration = declaration.with_kind(interaction_kind(value.trim()));
        } else if let Some(value) = line.strip_prefix("effect ") {
            declaration = declaration.with_effect(value.trim());
        } else if let Some(value) = line.strip_prefix("readiness ") {
            declaration = declaration.with_readiness(value.trim());
        } else if let Some(value) = line.strip_prefix("payload ") {
            declaration = declaration.with_payload(value.trim());
        } else if let Some(value) = line.strip_prefix("label ") {
            declaration = declaration.with_label(unquote(value.trim()));
        } else if let Some(prop) = parse_interaction_primitive_prop(index, line) {
            primitive_props.push(prop);
        } else {
            return Err(parse_denial(
                index,
                "expected kind, effect, readiness, payload, label, primitive prop, or }",
            ));
        }
    }
}

fn payload_shape(value: &str) -> WorthUiLiveViewPayloadShape {
    match value {
        "payload_values" => WorthUiLiveViewPayloadShape::PayloadValues,
        "data_payload_values" => WorthUiLiveViewPayloadShape::DataPayloadValues,
        other => WorthUiLiveViewPayloadShape::Unsupported(other.to_owned()),
    }
}

fn parse_condition_expression(value: &str) -> WorthUiLiveViewConditionExpression {
    let parts = value.split_whitespace().collect::<Vec<_>>();
    if parts.len() >= 3 && parts[1] == "equals" {
        let literal = parts[2..].join(" ");
        WorthUiLiveViewConditionExpression::binding_equals_literal(
            parts[0],
            unquote(literal.trim()),
        )
    } else {
        WorthUiLiveViewConditionExpression::Unsupported(value.to_owned())
    }
}

fn participation_posture(value: &str) -> WorthUiLiveViewParticipationPosture {
    match value {
        "present" => WorthUiLiveViewParticipationPosture::Present,
        "absent_retaining_state" => WorthUiLiveViewParticipationPosture::AbsentRetainingState,
        _ => WorthUiLiveViewParticipationPosture::Unsupported,
    }
}

fn interaction_kind(value: &str) -> WorthUiLiveViewInteractionIntentKind {
    match value {
        "submit" => WorthUiLiveViewInteractionIntentKind::Submit,
        other => WorthUiLiveViewInteractionIntentKind::Unsupported(other.to_owned()),
    }
}
