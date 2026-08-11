//! Admit item macros only when their local definition can emit implementation items alone.

use super::super::super::crate_modules::ModuleGraph;
use proc_macro2::{Delimiter, TokenStream, TokenTree};
use std::collections::BTreeMap;
use syn::Item;

pub(super) fn verify(graph: &ModuleGraph) -> Result<(), String> {
    let definitions = definitions(graph);
    for node in graph.modules.values() {
        for invocation in node.items.iter().filter_map(|item| match item {
            Item::Macro(item) if !item.mac.path.is_ident("macro_rules") => Some(item),
            _ => None,
        }) {
            let name = invocation
                .mac
                .path
                .segments
                .last()
                .map(|segment| segment.ident.to_string())
                .ok_or_else(|| "public-value item macro has no resolvable name".to_owned())?;
            let path = invocation
                .mac
                .path
                .segments
                .iter()
                .map(|segment| segment.ident.to_string())
                .collect::<Vec<_>>();
            if path.as_slice() != ["crate", name.as_str()] {
                return Err(format!(
                    "item macro `{}` is not an exact crate-root macro path and cannot be inventoried",
                    path.join("::")
                ));
            }
            let candidates = definitions.get(&name).map(Vec::as_slice).unwrap_or(&[]);
            let [definition] = candidates else {
                return Err(format!(
                    "item macro `{}` cannot be proven implementation-only; expanded public values cannot be inventoried",
                    path.join("::")
                ));
            };
            verify_definition(name.as_str(), definition.mac.tokens.clone())?;
        }
    }
    Ok(())
}

fn definitions(graph: &ModuleGraph) -> BTreeMap<String, Vec<&syn::ItemMacro>> {
    let mut definitions = BTreeMap::<String, Vec<&syn::ItemMacro>>::new();
    for node in graph.modules.values() {
        for definition in node.items.iter().filter_map(|item| match item {
            Item::Macro(item) if item.mac.path.is_ident("macro_rules") => Some(item),
            _ => None,
        }) {
            if let Some(name) = &definition.ident {
                definitions
                    .entry(name.to_string())
                    .or_default()
                    .push(definition);
            }
        }
    }
    definitions
}

fn verify_definition(name: &str, tokens: TokenStream) -> Result<(), String> {
    let trees = tokens.into_iter().collect::<Vec<_>>();
    let mut cursor = 0;
    let mut arms = 0;
    while cursor < trees.len() {
        let matcher = group_at(&trees, cursor, "matcher", name)?;
        verify_matcher_fragments(name, matcher.stream())?;
        cursor += 1;
        expect_punct(&trees, cursor, '=', name)?;
        expect_punct(&trees, cursor + 1, '>', name)?;
        let transcriber = group_at(&trees, cursor + 2, "transcriber", name)?;
        verify_transcriber(name, transcriber.stream())?;
        cursor += 3;
        if matches!(trees.get(cursor), Some(TokenTree::Punct(punct)) if punct.as_char() == ';') {
            cursor += 1;
        }
        arms += 1;
    }
    if arms == 0 {
        return Err(format!(
            "item macro `{name}` has no mechanically inspectable arms"
        ));
    }
    Ok(())
}

fn verify_matcher_fragments(name: &str, tokens: TokenStream) -> Result<(), String> {
    let trees = tokens.into_iter().collect::<Vec<_>>();
    for window in trees.windows(4) {
        let [TokenTree::Punct(dollar), TokenTree::Ident(_), TokenTree::Punct(colon), TokenTree::Ident(fragment)] =
            window
        else {
            continue;
        };
        if dollar.as_char() == '$'
            && colon.as_char() == ':'
            && fragment != "ident"
            && fragment != "path"
        {
            return Err(format!(
                "item macro `{name}` uses ambiguous `{fragment}` metavariables"
            ));
        }
    }
    for tree in trees {
        if let TokenTree::Group(group) = tree {
            verify_matcher_fragments(name, group.stream())?;
        }
    }
    Ok(())
}

fn verify_transcriber(name: &str, tokens: TokenStream) -> Result<(), String> {
    let trees = tokens.into_iter().collect::<Vec<_>>();
    let mut cursor = 0;
    while cursor < trees.len() {
        if let Some((body, next)) = repetition_at(&trees, cursor) {
            verify_transcriber(name, body)?;
            cursor = next;
            continue;
        }
        if is_self_call(&trees, cursor, name) {
            cursor += 7;
            if matches!(trees.get(cursor), Some(TokenTree::Punct(punct)) if punct.as_char() == ';')
            {
                cursor += 1;
            }
            continue;
        }
        if matches!(trees.get(cursor), Some(TokenTree::Ident(ident)) if ident == "impl") {
            cursor = consume_impl(name, &trees, cursor + 1)?;
            continue;
        }
        return Err(format!(
            "item macro `{name}` has top-level output that is not mechanically proven to be an impl"
        ));
    }
    Ok(())
}

fn repetition_at(trees: &[TokenTree], cursor: usize) -> Option<(TokenStream, usize)> {
    let Some(TokenTree::Punct(dollar)) = trees.get(cursor) else {
        return None;
    };
    let Some(TokenTree::Group(group)) = trees.get(cursor + 1) else {
        return None;
    };
    let Some(TokenTree::Punct(operator)) = trees.get(cursor + 2) else {
        return None;
    };
    if dollar.as_char() == '$' && matches!(operator.as_char(), '*' | '+' | '?') {
        Some((group.stream(), cursor + 3))
    } else {
        None
    }
}

fn consume_impl(name: &str, trees: &[TokenTree], mut cursor: usize) -> Result<usize, String> {
    while let Some(tree) = trees.get(cursor) {
        if matches!(tree, TokenTree::Group(group) if group.delimiter() == Delimiter::Brace) {
            return Ok(cursor + 1);
        }
        cursor += 1;
    }
    Err(format!(
        "item macro `{name}` has an unterminated impl output"
    ))
}

fn is_self_call(trees: &[TokenTree], cursor: usize, name: &str) -> bool {
    matches!(trees.get(cursor), Some(TokenTree::Punct(punct)) if punct.as_char() == '$')
        && matches!(trees.get(cursor + 1), Some(TokenTree::Ident(ident)) if ident == "crate")
        && matches!(trees.get(cursor + 2), Some(TokenTree::Punct(punct)) if punct.as_char() == ':')
        && matches!(trees.get(cursor + 3), Some(TokenTree::Punct(punct)) if punct.as_char() == ':')
        && matches!(trees.get(cursor + 4), Some(TokenTree::Ident(ident)) if ident == name)
        && matches!(trees.get(cursor + 5), Some(TokenTree::Punct(punct)) if punct.as_char() == '!')
        && matches!(trees.get(cursor + 6), Some(TokenTree::Group(_)))
}

fn group_at<'a>(
    trees: &'a [TokenTree],
    cursor: usize,
    role: &str,
    name: &str,
) -> Result<&'a proc_macro2::Group, String> {
    match trees.get(cursor) {
        Some(TokenTree::Group(group)) => Ok(group),
        _ => Err(format!("item macro `{name}` has no {role} group")),
    }
}

fn expect_punct(
    trees: &[TokenTree],
    cursor: usize,
    expected: char,
    name: &str,
) -> Result<(), String> {
    if matches!(trees.get(cursor), Some(TokenTree::Punct(punct)) if punct.as_char() == expected) {
        Ok(())
    } else {
        Err(format!(
            "item macro `{name}` has an unrecognized rule separator"
        ))
    }
}
