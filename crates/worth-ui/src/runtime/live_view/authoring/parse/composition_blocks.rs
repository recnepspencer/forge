use crate::runtime::{
    WorthUiAuthoredCompositionDeclaration, WorthUiAuthoredCompositionEdgeDeclaration,
    WorthUiAuthoredCompositionNodeDeclaration, WorthUiAuthoredCompositionPolicyDeclaration,
    WorthUiAuthoredCompositionRootDeclaration, WorthUiAuthoredLiveViewParseDenial,
    WorthUiCompositionChildSizing, WorthUiCompositionNodeKind, WorthUiCompositionPolicyKind,
    WorthUiCompositionRootKind, WorthUiPrimitiveSourceSpan,
};

use super::composition_accessibility_blocks::parse_accessibility_block;
use super::content_blocks::{content_header, parse_content_node, push_default_content_if_needed};
use super::context_blocks::{context_header, parse_context_block};
use super::parse_denial;

pub(super) fn parse_composition_declaration<'a, I>(
    composition_id: &str,
    source_lines: &mut std::iter::Peekable<I>,
) -> Result<WorthUiAuthoredCompositionDeclaration, WorthUiAuthoredLiveViewParseDenial>
where
    I: Iterator<Item = (usize, &'a str)>,
{
    let mut composition = WorthUiAuthoredCompositionDeclaration::new(
        composition_id,
        WorthUiAuthoredCompositionRootDeclaration::surface(composition_id),
    );
    let mut root_order = 0;
    loop {
        let Some((index, raw_line)) = source_lines.next() else {
            return Err(WorthUiAuthoredLiveViewParseDenial::new(
                0,
                "unterminated composition block",
            ));
        };
        let line = raw_line.trim();
        if line.is_empty() {
            continue;
        }
        if line == "}" {
            return Ok(composition);
        }
        if let Some(root) = parse_root(index, line)? {
            composition = WorthUiAuthoredCompositionDeclaration::new(composition_id, root);
            root_order = 0;
            continue;
        }
        if let Some(context) = context_header(line) {
            composition.push_context(parse_context_block(index, context, source_lines)?);
            continue;
        }
        if line == "accessibility {" {
            parse_accessibility_block(source_lines, &mut composition)?;
            continue;
        }
        if let Some((kind, container_id)) = container_header(line) {
            parse_container_node(
                kind,
                container_id,
                source_span(index),
                None,
                root_order,
                source_lines,
                &mut composition,
            )?;
            root_order += 1;
            continue;
        }
        if let Some(content_id) = content_header(line) {
            parse_content_node(
                content_id,
                source_span(index),
                None,
                root_order,
                source_lines,
                &mut composition,
            )?;
            root_order += 1;
            continue;
        }
        if let Some((kind, id, sizing)) = child_statement(index, line)? {
            let span = source_span(index);
            let node = WorthUiAuthoredCompositionNodeDeclaration::spanned(kind, id, span);
            let node_id = node.node_id().to_owned();
            composition.push_node(node);
            push_default_content_if_needed(kind, id, span, &mut composition);
            composition.push_edge(
                WorthUiAuthoredCompositionEdgeDeclaration::root_child_spanned(
                    node_id, root_order, sizing, span,
                ),
            );
            root_order += 1;
            continue;
        }
        return Err(parse_denial(
            index,
            "expected root, container, child, or } in composition block",
        ));
    }
}

fn parse_container_node<'a, I>(
    kind: WorthUiCompositionNodeKind,
    container_id: &str,
    container_span: WorthUiPrimitiveSourceSpan,
    parent_id: Option<&str>,
    order: u32,
    source_lines: &mut std::iter::Peekable<I>,
    composition: &mut WorthUiAuthoredCompositionDeclaration,
) -> Result<(), WorthUiAuthoredLiveViewParseDenial>
where
    I: Iterator<Item = (usize, &'a str)>,
{
    composition.push_node(WorthUiAuthoredCompositionNodeDeclaration::spanned(
        kind,
        container_id,
        container_span,
    ));
    composition.push_edge(match parent_id {
        Some(parent_id) => WorthUiAuthoredCompositionEdgeDeclaration::child_spanned(
            parent_id,
            container_id,
            order,
            WorthUiCompositionChildSizing::Auto,
            container_span,
        ),
        None => WorthUiAuthoredCompositionEdgeDeclaration::root_child_spanned(
            container_id,
            order,
            WorthUiCompositionChildSizing::Auto,
            container_span,
        ),
    });
    let mut child_order = 0;
    loop {
        let Some((index, raw_line)) = source_lines.next() else {
            return Err(WorthUiAuthoredLiveViewParseDenial::new(
                0,
                "unterminated container block",
            ));
        };
        let line = raw_line.trim();
        if line.is_empty() {
            continue;
        }
        if line == "}" {
            return Ok(());
        }
        if let Some(policy) = policy_statement(index, container_id, line)? {
            composition.push_policy(policy);
            continue;
        }
        if let Some(context) = context_header(line) {
            composition.push_context(parse_context_block(index, context, source_lines)?);
            continue;
        }
        if line == "accessibility {" {
            parse_accessibility_block(source_lines, composition)?;
            continue;
        }
        if let Some((kind, container_child_id)) = container_header(line) {
            parse_container_node(
                kind,
                container_child_id,
                source_span(index),
                Some(container_id),
                child_order,
                source_lines,
                composition,
            )?;
            child_order += 1;
            continue;
        }
        if let Some(content_id) = content_header(line) {
            parse_content_node(
                content_id,
                source_span(index),
                Some(container_id),
                child_order,
                source_lines,
                composition,
            )?;
            child_order += 1;
            continue;
        }
        if let Some((kind, id, sizing)) = child_statement(index, line)? {
            let span = source_span(index);
            let node = WorthUiAuthoredCompositionNodeDeclaration::spanned(kind, id, span);
            let node_id = node.node_id().to_owned();
            composition.push_node(node);
            push_default_content_if_needed(kind, id, span, composition);
            composition.push_edge(WorthUiAuthoredCompositionEdgeDeclaration::child_spanned(
                container_id,
                node_id,
                child_order,
                sizing,
                span,
            ));
            child_order += 1;
            continue;
        }
        return Err(parse_denial(
            index,
            "expected policy, nested container, child, or } in container block",
        ));
    }
}

fn parse_root(
    line_index: usize,
    line: &str,
) -> Result<Option<WorthUiAuthoredCompositionRootDeclaration>, WorthUiAuthoredLiveViewParseDenial> {
    let Some(value) = line.strip_prefix("root ") else {
        return Ok(None);
    };
    let parts = value.split_whitespace().collect::<Vec<_>>();
    if parts.len() != 2 {
        return Err(parse_denial(
            line_index,
            "composition root syntax is root <kind> <identity>",
        ));
    }
    let Some(kind) = WorthUiCompositionRootKind::from_token(parts[0]) else {
        return Err(parse_denial(
            line_index,
            "composition root kind must be surface, page_content_slot, component_instance, portal_entry, collection_item, or diagnostic_panel",
        ));
    };
    Ok(Some(WorthUiAuthoredCompositionRootDeclaration::new(
        kind, parts[1],
    )))
}

fn policy_statement(
    line_index: usize,
    node_id: &str,
    line: &str,
) -> Result<Option<WorthUiAuthoredCompositionPolicyDeclaration>, WorthUiAuthoredLiveViewParseDenial>
{
    let Some(value) = line.strip_prefix("policy ") else {
        return Ok(None);
    };
    let parts = value.split_whitespace().collect::<Vec<_>>();
    if parts.len() != 2 {
        return Err(parse_denial(
            line_index,
            "composition policy syntax is policy <kind> <identity>",
        ));
    }
    let kind = match parts[0] {
        "local_layout" => WorthUiCompositionPolicyKind::LocalLayout,
        "interaction_containment" => WorthUiCompositionPolicyKind::InteractionContainment,
        "diagnostic_placement" => WorthUiCompositionPolicyKind::DiagnosticPlacement,
        "viewport_boundary" => WorthUiCompositionPolicyKind::ViewportBoundary,
        _ => {
            return Err(parse_denial(
                line_index,
                "composition policy kind must be local_layout, interaction_containment, diagnostic_placement, or viewport_boundary",
            ));
        }
    };
    Ok(Some(WorthUiAuthoredCompositionPolicyDeclaration::spanned(
        node_id,
        kind,
        parts[1],
        source_span(line_index),
    )))
}

fn child_statement(
    line_index: usize,
    line: &str,
) -> Result<
    Option<(
        WorthUiCompositionNodeKind,
        &str,
        WorthUiCompositionChildSizing,
    )>,
    WorthUiAuthoredLiveViewParseDenial,
> {
    let Some(value) = line.strip_prefix("child ") else {
        return Ok(None);
    };
    let parts = value.split_whitespace().collect::<Vec<_>>();
    if parts.len() < 2 {
        return Err(parse_denial(
            line_index,
            "composition child syntax is child <kind> <identity>",
        ));
    }
    let sizing = parse_child_sizing(line_index, &parts)?;
    if parts.len() > 2 && !matches!(parts[2], "sizing") {
        return Err(parse_denial(
            line_index,
            "composition child supports only optional sizing posture after identity",
        ));
    }
    let kind = match parts[0] {
        "control" => WorthUiCompositionNodeKind::Control,
        "interaction" => WorthUiCompositionNodeKind::Interaction,
        "content" => WorthUiCompositionNodeKind::Content,
        "text" => WorthUiCompositionNodeKind::Text,
        "icon" => WorthUiCompositionNodeKind::Icon,
        "diagnostic_panel" => WorthUiCompositionNodeKind::DiagnosticPanel,
        "portal_host" => WorthUiCompositionNodeKind::PortalHost,
        _ => {
            return Err(parse_denial(
                line_index,
                "composition child kind must be control, interaction, content, text, icon, diagnostic_panel, or portal_host",
            ));
        }
    };
    Ok(Some((kind, parts[1], sizing)))
}

fn parse_child_sizing(
    line_index: usize,
    parts: &[&str],
) -> Result<WorthUiCompositionChildSizing, WorthUiAuthoredLiveViewParseDenial> {
    match parts.len() {
        2 => Ok(WorthUiCompositionChildSizing::Auto),
        4 if parts[2] == "sizing" => sizing_posture(line_index, parts[3]),
        _ => Err(parse_denial(
            line_index,
            "composition child sizing syntax is sizing hug or sizing fill(<positive-weight>)",
        )),
    }
}

fn sizing_posture(
    line_index: usize,
    token: &str,
) -> Result<WorthUiCompositionChildSizing, WorthUiAuthoredLiveViewParseDenial> {
    if token == "hug" {
        return Ok(WorthUiCompositionChildSizing::Hug);
    }
    let Some(weight) = token
        .strip_prefix("fill(")
        .and_then(|value| value.strip_suffix(')'))
        .and_then(|value| value.parse::<u32>().ok())
    else {
        return Err(parse_denial(
            line_index,
            "composition child sizing must be hug or fill(<positive-weight>)",
        ));
    };
    if weight == 0 {
        return Err(parse_denial(
            line_index,
            "composition child fill sizing weight must be positive",
        ));
    }
    Ok(WorthUiCompositionChildSizing::Fill(weight))
}

fn container_header(line: &str) -> Option<(WorthUiCompositionNodeKind, &str)> {
    if let Some(id) = line
        .strip_prefix("container ")
        .and_then(|line| line.strip_suffix(" {"))
        .map(str::trim)
    {
        return Some((WorthUiCompositionNodeKind::Container, id));
    }
    line.strip_prefix("surface ")
        .and_then(|line| line.strip_suffix(" {"))
        .map(str::trim)
        .map(|id| (WorthUiCompositionNodeKind::Surface, id))
}

fn source_span(line_index: usize) -> WorthUiPrimitiveSourceSpan {
    WorthUiPrimitiveSourceSpan::new(line_index + 1, line_index + 1)
}
