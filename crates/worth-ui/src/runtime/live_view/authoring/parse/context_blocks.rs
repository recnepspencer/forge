use crate::runtime::{
    WorthUiAuthoredLiveViewParseDenial, WorthUiCompositionContextDefinition,
    WorthUiCompositionLocalePosture, WorthUiCompositionRuntimeMode,
    WorthUiCompositionTextDirection, WorthUiCompositionValidationPosture,
    WorthUiPrimitiveSourceSpan,
};

use super::parse_denial;

pub(super) enum ContextHeader<'a> {
    Root,
    Node(&'a str),
}

pub(super) fn context_header(line: &str) -> Option<ContextHeader<'_>> {
    if line == "context root {" {
        return Some(ContextHeader::Root);
    }
    line.strip_prefix("context node ")?
        .strip_suffix(" {")
        .map(str::trim)
        .map(ContextHeader::Node)
}

pub(super) fn parse_context_block<'a, I>(
    header_index: usize,
    header: ContextHeader<'_>,
    source_lines: &mut std::iter::Peekable<I>,
) -> Result<WorthUiCompositionContextDefinition, WorthUiAuthoredLiveViewParseDenial>
where
    I: Iterator<Item = (usize, &'a str)>,
{
    let mut context = match header {
        ContextHeader::Root => WorthUiCompositionContextDefinition::root(),
        ContextHeader::Node(node_id) => WorthUiCompositionContextDefinition::for_node(node_id),
    }
    .with_source_span(source_span(header_index));
    loop {
        let Some((index, raw_line)) = source_lines.next() else {
            return Err(parse_denial(header_index, "unterminated context block"));
        };
        let line = raw_line.trim();
        if line.is_empty() {
            continue;
        }
        if line == "}" {
            return Ok(context);
        }
        context = parse_context_field(index, context, line)?;
    }
}

fn parse_context_field(
    line_index: usize,
    context: WorthUiCompositionContextDefinition,
    line: &str,
) -> Result<WorthUiCompositionContextDefinition, WorthUiAuthoredLiveViewParseDenial> {
    let Some((key, raw_value)) = line.split_once(char::is_whitespace) else {
        return Err(parse_denial(
            line_index,
            "context field syntax is <kind> <value>",
        ));
    };
    let value = raw_value.trim();
    match key {
        "override" if value == "allowed" => Ok(context.allow_local_override()),
        "theme" => Ok(context.theme(value)),
        "density" => Ok(context.density(value)),
        "disabled" => Ok(context.disabled(parse_bool(line_index, value)?)),
        "inert" => Ok(context.inert(parse_bool(line_index, value)?)),
        "text_direction" => Ok(context.text_direction(parse_text_direction(line_index, value)?)),
        "locale" => Ok(context.locale(parse_locale(value))),
        "validation" => Ok(context.validation(parse_validation(line_index, value)?)),
        "focus_scope" => Ok(context.focus_scope(value)),
        "runtime_mode" => Ok(context.runtime_mode(parse_runtime_mode(line_index, value)?)),
        _ => Err(parse_denial(
            line_index,
            "context field must be override allowed, theme, density, disabled, inert, text_direction, locale, validation, focus_scope, or runtime_mode",
        )),
    }
}

fn parse_bool(line_index: usize, value: &str) -> Result<bool, WorthUiAuthoredLiveViewParseDenial> {
    match value {
        "true" => Ok(true),
        "false" => Ok(false),
        _ => Err(parse_denial(
            line_index,
            "context boolean must be true or false",
        )),
    }
}

fn parse_text_direction(
    line_index: usize,
    value: &str,
) -> Result<WorthUiCompositionTextDirection, WorthUiAuthoredLiveViewParseDenial> {
    match value {
        "ltr" => Ok(WorthUiCompositionTextDirection::Ltr),
        "rtl" => Ok(WorthUiCompositionTextDirection::Rtl),
        "auto" => Ok(WorthUiCompositionTextDirection::Auto),
        _ => Err(parse_denial(
            line_index,
            "text_direction must be ltr, rtl, or auto",
        )),
    }
}

fn parse_locale(value: &str) -> WorthUiCompositionLocalePosture {
    if let Some(locale) = value.strip_prefix("ready:") {
        WorthUiCompositionLocalePosture::Ready(locale.to_owned())
    } else if let Some(locale) = value.strip_prefix("limited:") {
        WorthUiCompositionLocalePosture::Limited(locale.to_owned())
    } else {
        WorthUiCompositionLocalePosture::Unsupported(value.to_owned())
    }
}

fn parse_validation(
    line_index: usize,
    value: &str,
) -> Result<WorthUiCompositionValidationPosture, WorthUiAuthoredLiveViewParseDenial> {
    match value {
        "valid" => Ok(WorthUiCompositionValidationPosture::Valid),
        "invalid" => Ok(WorthUiCompositionValidationPosture::Invalid),
        "unknown" => Ok(WorthUiCompositionValidationPosture::Unknown),
        _ => Err(parse_denial(
            line_index,
            "validation must be valid, invalid, or unknown",
        )),
    }
}

fn parse_runtime_mode(
    line_index: usize,
    value: &str,
) -> Result<WorthUiCompositionRuntimeMode, WorthUiAuthoredLiveViewParseDenial> {
    match value {
        "interactive" => Ok(WorthUiCompositionRuntimeMode::Interactive),
        "preview" => Ok(WorthUiCompositionRuntimeMode::Preview),
        "diagnostic" => Ok(WorthUiCompositionRuntimeMode::Diagnostic),
        _ => Err(parse_denial(
            line_index,
            "runtime_mode must be interactive, preview, or diagnostic",
        )),
    }
}

fn source_span(line_index: usize) -> WorthUiPrimitiveSourceSpan {
    WorthUiPrimitiveSourceSpan::new(line_index + 1, line_index + 1)
}
