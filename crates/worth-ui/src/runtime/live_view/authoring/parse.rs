mod composition_accessibility_blocks;
mod composition_blocks;
mod content_blocks;
mod context_blocks;
mod projection_blocks;

use super::super::interaction_intent::WorthUiLiveViewInteractionPrimitiveProp;
use super::super::{
    WorthUiLiveViewControlOptionDeclaration, WorthUiLiveViewControlOptionsSource,
    WorthUiLiveViewControlPrimitiveProp, WorthUiLiveViewControlProjectionDeclaration,
    WorthUiLiveViewControlProjectionKind,
};
use super::document::{
    WorthUiAuthoredLiveViewDeclaration, WorthUiAuthoredLiveViewDocument,
    WorthUiAuthoredLiveViewParseDenial, WorthUiAuthoredLiveViewPrimitiveProp,
    WorthUiAuthoredLiveViewStateBinding,
};
use crate::runtime::WorthUiPrimitiveSourceSpan;
use composition_blocks::parse_composition_declaration;
use projection_blocks::{
    parse_conditional_projection, parse_interaction_intent, parse_payload_projection,
    parse_readiness_projection,
};

pub(super) fn parse_live_view_document(
    source: &str,
) -> Result<WorthUiAuthoredLiveViewDocument, WorthUiAuthoredLiveViewParseDenial> {
    let mut declarations = Vec::new();
    let mut source_lines = source.lines().enumerate().peekable();
    while let Some((index, raw_line)) = source_lines.next() {
        let line = raw_line.trim();
        if line.is_empty() {
            continue;
        }
        let Some(live_view_id) = live_view_header_id(line) else {
            return Err(parse_denial(index, "expected live_view <id> {"));
        };
        declarations.push(parse_live_view_body(
            live_view_id,
            index,
            &mut source_lines,
        )?);
    }
    Ok(WorthUiAuthoredLiveViewDocument { declarations })
}

fn parse_live_view_body<'a, I>(
    live_view_id: &str,
    header_index: usize,
    source_lines: &mut std::iter::Peekable<I>,
) -> Result<WorthUiAuthoredLiveViewDeclaration, WorthUiAuthoredLiveViewParseDenial>
where
    I: Iterator<Item = (usize, &'a str)>,
{
    let mut declaration = WorthUiAuthoredLiveViewDeclaration::new(live_view_id);
    loop {
        let Some((index, raw_line)) = source_lines.next() else {
            return Err(parse_denial(header_index, "unterminated live_view block"));
        };
        let line = raw_line.trim();
        if line.is_empty() {
            continue;
        }
        if line == "}" {
            return Ok(declaration);
        }
        if let Some(target_slot) = line.strip_prefix("target ") {
            declaration.set_target_slot(target_slot.trim());
            continue;
        }
        if let Some(prop) = parse_live_view_primitive_prop(index, line) {
            declaration.push_primitive_prop(prop);
            continue;
        }
        if let Some(binding_id) = state_header_id(line) {
            declaration.push_binding(parse_state_binding(binding_id, source_lines)?);
            continue;
        }
        if let Some(control_id) = control_header_id(line) {
            declaration.push_control(parse_control_projection(control_id, source_lines)?);
            continue;
        }
        if let Some(control_id) = condition_header_id(line) {
            declaration.push_conditional(parse_conditional_projection(control_id, source_lines)?);
            continue;
        }
        if let Some(readiness_id) = readiness_header_id(line) {
            declaration.push_readiness(parse_readiness_projection(readiness_id, source_lines)?);
            continue;
        }
        if let Some(payload_id) = payload_header_id(line) {
            declaration.push_payload(parse_payload_projection(payload_id, source_lines)?);
            continue;
        }
        if let Some(interaction_id) = interaction_header_id(line) {
            declaration.push_interaction(parse_interaction_intent(interaction_id, source_lines)?);
            continue;
        }
        if let Some(composition_id) = composition_header_id(line) {
            declaration
                .set_composition(parse_composition_declaration(composition_id, source_lines)?);
            continue;
        }
        return Err(parse_denial(
            index,
            "expected target, state, control, condition, readiness, payload, interaction, or composition block",
        ));
    }
}

fn parse_live_view_primitive_prop(
    line_index: usize,
    line: &str,
) -> Option<WorthUiAuthoredLiveViewPrimitiveProp> {
    let (key, value) = line.split_once(char::is_whitespace)?;
    if !is_primitive_key(key) {
        return None;
    }
    Some(WorthUiAuthoredLiveViewPrimitiveProp::new(
        key,
        unquote(value.trim()),
        Some(WorthUiPrimitiveSourceSpan::new(
            line_index + 1,
            line_index + 1,
        )),
    ))
}

fn parse_state_binding<'a, I>(
    binding_id: &str,
    source_lines: &mut std::iter::Peekable<I>,
) -> Result<WorthUiAuthoredLiveViewStateBinding, WorthUiAuthoredLiveViewParseDenial>
where
    I: Iterator<Item = (usize, &'a str)>,
{
    let mut binding = WorthUiAuthoredLiveViewStateBinding::new(binding_id);
    loop {
        let Some((index, raw_line)) = source_lines.next() else {
            return Err(WorthUiAuthoredLiveViewParseDenial::new(
                0,
                "unterminated state block",
            ));
        };
        let line = raw_line.trim();
        if line.is_empty() {
            continue;
        }
        if line == "}" {
            return Ok(binding);
        }
        parse_state_binding_field(&mut binding, index, line)?;
    }
}

fn parse_state_binding_field(
    binding: &mut WorthUiAuthoredLiveViewStateBinding,
    line_index: usize,
    line: &str,
) -> Result<(), WorthUiAuthoredLiveViewParseDenial> {
    if let Some(state_fact) = line.strip_prefix("fact ") {
        binding.set_state_fact(state_fact.trim());
    } else if let Some(value_kind) = line.strip_prefix("kind ") {
        binding.set_value_kind(value_kind.trim());
    } else if let Some(access) = line.strip_prefix("access ") {
        binding.set_access(access.trim());
    } else {
        return Err(parse_denial(
            line_index,
            "expected fact, kind, access, or }",
        ));
    }
    Ok(())
}

fn parse_control_projection<'a, I>(
    control_id: &str,
    source_lines: &mut std::iter::Peekable<I>,
) -> Result<WorthUiLiveViewControlProjectionDeclaration, WorthUiAuthoredLiveViewParseDenial>
where
    I: Iterator<Item = (usize, &'a str)>,
{
    let mut binding_id = String::new();
    let mut projection = WorthUiLiveViewControlProjectionKind::Unsupported(String::new());
    let mut label = control_id.to_owned();
    let mut options = None;
    let mut primitive_props = Vec::new();
    loop {
        let Some((index, raw_line)) = source_lines.next() else {
            return Err(WorthUiAuthoredLiveViewParseDenial::new(
                0,
                "unterminated control block",
            ));
        };
        let line = raw_line.trim();
        if line.is_empty() {
            continue;
        }
        if line == "}" {
            let mut declaration = WorthUiLiveViewControlProjectionDeclaration::new(
                control_id, binding_id, projection, label,
            );
            if let Some(options) = options {
                declaration = declaration.with_options(options);
            }
            declaration = declaration.with_primitive_props(primitive_props);
            return Ok(declaration);
        }
        if let Some(value) = line.strip_prefix("binding ") {
            binding_id = value.trim().to_owned();
        } else if let Some(value) = line.strip_prefix("projection ") {
            projection = control_projection_kind(value.trim());
        } else if let Some(value) = line.strip_prefix("label ") {
            label = unquote(value.trim()).to_owned();
        } else if let Some(value) = line.strip_prefix("options ") {
            options = Some(parse_static_options(control_id, value.trim()));
        } else if let Some(value) = line.strip_prefix("options_source ") {
            options = Some(WorthUiLiveViewControlOptionsSource::Unsupported(
                value.trim().to_owned(),
            ));
        } else if let Some(prop) = parse_control_primitive_prop(index, line) {
            primitive_props.push(prop);
        } else {
            return Err(parse_denial(
                index,
                "expected binding, projection, label, options, options_source, primitive prop, or }",
            ));
        }
    }
}

fn parse_control_primitive_prop(
    line_index: usize,
    line: &str,
) -> Option<WorthUiLiveViewControlPrimitiveProp> {
    let (key, value) = line.split_once(char::is_whitespace)?;
    if !is_primitive_key(key) {
        return None;
    }
    Some(WorthUiLiveViewControlPrimitiveProp::new(
        key,
        unquote(value.trim()),
        Some(WorthUiPrimitiveSourceSpan::new(
            line_index + 1,
            line_index + 1,
        )),
    ))
}

fn live_view_header_id(line: &str) -> Option<&str> {
    line.strip_prefix("live_view ")?
        .strip_suffix(" {")
        .map(str::trim)
}

fn state_header_id(line: &str) -> Option<&str> {
    line.strip_prefix("state ")?
        .strip_suffix(" {")
        .map(str::trim)
}

fn control_header_id(line: &str) -> Option<&str> {
    line.strip_prefix("control ")?
        .strip_suffix(" {")
        .map(str::trim)
}

fn condition_header_id(line: &str) -> Option<&str> {
    line.strip_prefix("condition ")?
        .strip_suffix(" {")
        .map(str::trim)
}

fn readiness_header_id(line: &str) -> Option<&str> {
    line.strip_prefix("readiness ")?
        .strip_suffix(" {")
        .map(str::trim)
}

fn payload_header_id(line: &str) -> Option<&str> {
    line.strip_prefix("payload ")?
        .strip_suffix(" {")
        .map(str::trim)
}

fn interaction_header_id(line: &str) -> Option<&str> {
    line.strip_prefix("interaction ")?
        .strip_suffix(" {")
        .map(str::trim)
}

fn composition_header_id(line: &str) -> Option<&str> {
    line.strip_prefix("composition ")?
        .strip_suffix(" {")
        .map(str::trim)
}

fn control_projection_kind(value: &str) -> WorthUiLiveViewControlProjectionKind {
    match value {
        "text_input" => WorthUiLiveViewControlProjectionKind::TextInput,
        "select" => WorthUiLiveViewControlProjectionKind::Select,
        other => WorthUiLiveViewControlProjectionKind::Unsupported(other.to_owned()),
    }
}

fn parse_static_options(control_id: &str, value: &str) -> WorthUiLiveViewControlOptionsSource {
    let options = value
        .split(',')
        .filter(|entry| !entry.trim().is_empty())
        .map(|entry| {
            let mut parts = entry.splitn(2, ':');
            let option_value = parts.next().unwrap_or_default().trim();
            let option_label = parts.next().unwrap_or(option_value).trim();
            WorthUiLiveViewControlOptionDeclaration::new(option_value, unquote(option_label))
        })
        .collect::<Vec<_>>();
    WorthUiLiveViewControlOptionsSource::static_options(format!("static.{control_id}"), options)
}

pub(super) fn unquote(value: &str) -> &str {
    value
        .strip_prefix('"')
        .and_then(|value| value.strip_suffix('"'))
        .unwrap_or(value)
}

pub(super) fn parse_interaction_primitive_prop(
    line_index: usize,
    line: &str,
) -> Option<WorthUiLiveViewInteractionPrimitiveProp> {
    let (key, value) = line.split_once(char::is_whitespace)?;
    if !is_primitive_key(key) {
        return None;
    }
    Some(WorthUiLiveViewInteractionPrimitiveProp::new(
        key,
        unquote(value.trim()),
        Some(WorthUiPrimitiveSourceSpan::new(
            line_index + 1,
            line_index + 1,
        )),
    ))
}

pub(super) fn is_primitive_key(key: &str) -> bool {
    key.starts_with("flow_")
        || key.starts_with("appearance_")
        || key.starts_with("event_")
        || key.starts_with("content_")
}

pub(super) fn parse_denial(line_index: usize, message: &str) -> WorthUiAuthoredLiveViewParseDenial {
    WorthUiAuthoredLiveViewParseDenial::new(line_index + 1, message)
}
