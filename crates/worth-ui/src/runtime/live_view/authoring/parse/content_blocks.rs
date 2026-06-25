use crate::runtime::{
    AuthoredPrimitiveContentProp, WorthUiAuthoredCompositionContentDeclaration,
    WorthUiAuthoredCompositionDeclaration, WorthUiAuthoredCompositionEdgeDeclaration,
    WorthUiAuthoredCompositionNodeDeclaration, WorthUiAuthoredLiveViewParseDenial,
    WorthUiCompositionChildSizing, WorthUiCompositionNodeKind, WorthUiPrimitiveSourceSpan,
};

use super::{parse_denial, unquote};

pub(super) fn content_header(line: &str) -> Option<&str> {
    line.strip_prefix("content ")?
        .strip_suffix(" {")
        .map(str::trim)
}

pub(super) fn push_default_content_if_needed(
    kind: WorthUiCompositionNodeKind,
    id: &str,
    span: WorthUiPrimitiveSourceSpan,
    composition: &mut WorthUiAuthoredCompositionDeclaration,
) {
    if kind == WorthUiCompositionNodeKind::Content {
        composition.push_content(WorthUiAuthoredCompositionContentDeclaration::new(
            id,
            Vec::new(),
            span,
        ));
    }
}

pub(super) fn parse_content_node<'a, I>(
    content_id: &str,
    content_span: WorthUiPrimitiveSourceSpan,
    parent_id: Option<&str>,
    order: u32,
    source_lines: &mut std::iter::Peekable<I>,
    composition: &mut WorthUiAuthoredCompositionDeclaration,
) -> Result<(), WorthUiAuthoredLiveViewParseDenial>
where
    I: Iterator<Item = (usize, &'a str)>,
{
    let mut props = Vec::new();
    loop {
        let Some((index, raw_line)) = source_lines.next() else {
            return Err(WorthUiAuthoredLiveViewParseDenial::new(
                0,
                "unterminated content block",
            ));
        };
        let line = raw_line.trim();
        if line.is_empty() {
            continue;
        }
        if line == "}" {
            push_content_node(
                content_id,
                content_span,
                parent_id,
                order,
                composition,
                props,
            );
            return Ok(());
        }
        let Some(prop) = content_prop(index, line) else {
            return Err(parse_denial(
                index,
                "expected content primitive prop or } in content block",
            ));
        };
        props.push(prop);
    }
}

fn push_content_node(
    content_id: &str,
    content_span: WorthUiPrimitiveSourceSpan,
    parent_id: Option<&str>,
    order: u32,
    composition: &mut WorthUiAuthoredCompositionDeclaration,
    props: Vec<AuthoredPrimitiveContentProp>,
) {
    let node = WorthUiAuthoredCompositionNodeDeclaration::spanned(
        WorthUiCompositionNodeKind::Content,
        content_id,
        content_span,
    );
    let node_id = node.node_id().to_owned();
    composition.push_node(node);
    composition.push_edge(match parent_id {
        Some(parent_id) => WorthUiAuthoredCompositionEdgeDeclaration::child_spanned(
            parent_id,
            &node_id,
            order,
            WorthUiCompositionChildSizing::Auto,
            content_span,
        ),
        None => WorthUiAuthoredCompositionEdgeDeclaration::root_child_spanned(
            &node_id,
            order,
            WorthUiCompositionChildSizing::Auto,
            content_span,
        ),
    });
    composition.push_content(WorthUiAuthoredCompositionContentDeclaration::new(
        content_id,
        props,
        content_span,
    ));
}

fn content_prop(line_index: usize, line: &str) -> Option<AuthoredPrimitiveContentProp> {
    let (key, value) = line.split_once(char::is_whitespace)?;
    key.starts_with("content_").then(|| {
        AuthoredPrimitiveContentProp::new(
            key,
            unquote(value.trim()),
            Some(WorthUiPrimitiveSourceSpan::new(
                line_index + 1,
                line_index + 1,
            )),
        )
    })
}
