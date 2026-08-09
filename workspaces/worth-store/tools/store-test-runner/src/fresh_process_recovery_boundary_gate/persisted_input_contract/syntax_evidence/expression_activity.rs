use syn::{Attribute, Expr, Pat};

pub(super) fn path_identity(path: &syn::Path) -> Option<String> {
    (!path.segments.is_empty()).then(|| {
        path.segments
            .iter()
            .map(|segment| segment.ident.to_string())
            .collect::<Vec<_>>()
            .join("::")
    })
}

pub(super) fn expression_identity(expression: &Expr) -> Option<String> {
    match expression {
        Expr::Path(path) => path_identity(&path.path),
        Expr::Field(field) => Some(format!(
            "{}.{}",
            expression_identity(&field.base)?,
            match &field.member {
                syn::Member::Named(name) => name.to_string(),
                syn::Member::Unnamed(index) => index.index.to_string(),
            }
        )),
        Expr::MethodCall(call) => Some(format!(
            "{}.{}()",
            expression_identity(&call.receiver)?,
            call.method
        )),
        Expr::Paren(paren) => expression_identity(&paren.expr),
        Expr::Reference(reference) => expression_identity(&reference.expr),
        _ => None,
    }
}

pub(super) fn known_boolean(expression: &Expr) -> Option<bool> {
    match expression {
        Expr::Lit(literal) => match &literal.lit {
            syn::Lit::Bool(boolean) => Some(boolean.value),
            _ => None,
        },
        Expr::Binary(binary) => match binary.op {
            syn::BinOp::And(_) => match known_boolean(&binary.left)? {
                false => Some(false),
                true => known_boolean(&binary.right),
            },
            syn::BinOp::Or(_) => match known_boolean(&binary.left)? {
                true => Some(true),
                false => known_boolean(&binary.right),
            },
            syn::BinOp::Eq(_) => {
                Some(known_boolean(&binary.left)? == known_boolean(&binary.right)?)
            }
            syn::BinOp::Ne(_) => {
                Some(known_boolean(&binary.left)? != known_boolean(&binary.right)?)
            }
            _ => None,
        },
        Expr::Group(group) => known_boolean(&group.expr),
        Expr::Paren(paren) => known_boolean(&paren.expr),
        Expr::Unary(unary) if matches!(unary.op, syn::UnOp::Not(_)) => {
            Some(!known_boolean(&unary.expr)?)
        }
        _ => None,
    }
}

pub(super) fn pattern_matches_boolean(pattern: &Pat, value: bool) -> Option<bool> {
    match pattern {
        Pat::Lit(literal) => match &literal.lit {
            syn::Lit::Bool(boolean) => Some(boolean.value == value),
            _ => Some(false),
        },
        Pat::Or(alternatives) => alternatives
            .cases
            .iter()
            .map(|pattern| pattern_matches_boolean(pattern, value))
            .try_fold(false, |matched, next| Some(matched || next?)),
        Pat::Ident(binding) => binding.subpat.as_ref().map_or(Some(true), |(_, pattern)| {
            pattern_matches_boolean(pattern, value)
        }),
        Pat::Paren(paren) => pattern_matches_boolean(&paren.pat, value),
        Pat::Wild(_) => Some(true),
        _ => None,
    }
}

macro_rules! expression_attributes {
    ($expression:expr, $($variant:ident),+ $(,)?) => {
        match $expression {
            $(Expr::$variant(expression) => expression.attrs.as_slice(),)+
            _ => &[] as &[Attribute],
        }
    };
}

pub(super) fn expression_has_configuration(expression: &Expr) -> bool {
    expression_attributes!(
        expression, Array, Assign, Async, Await, Binary, Block, Break, Call, Cast, Closure, Const,
        Continue, Field, ForLoop, Group, If, Index, Infer, Let, Lit, Loop, Macro, Match,
        MethodCall, Paren, Path, Range, RawAddr, Reference, Repeat, Return, Struct, Try, TryBlock,
        Tuple, Unary, Unsafe, While, Yield,
    )
    .iter()
    .any(|attribute| attribute.path().is_ident("cfg") || attribute.path().is_ident("cfg_attr"))
}
