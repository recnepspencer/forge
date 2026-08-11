use syn::parse::Parser;
use syn::punctuated::Punctuated;
use syn::{Expr, Lit, Meta, Token};

pub(super) fn can_reach_production(attrs: &[syn::Attribute]) -> bool {
    attrs
        .iter()
        .filter(|attribute| attribute.path().is_ident("cfg"))
        .all(cfg_attribute_can_reach_production)
}

fn cfg_attribute_can_reach_production(attribute: &syn::Attribute) -> bool {
    let Meta::List(list) = &attribute.meta else {
        return true;
    };
    let parser = Punctuated::<Meta, Token![,]>::parse_terminated;
    let Ok(predicates) = parser.parse2(list.tokens.clone()) else {
        return true;
    };
    predicates
        .iter()
        .all(|predicate| truth(predicate) != Truth::AlwaysFalse)
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub(super) enum Truth {
    AlwaysFalse,
    Variable,
    AlwaysTrue,
}

pub(super) fn truth(predicate: &Meta) -> Truth {
    match predicate {
        Meta::Path(path) if path.is_ident("test") => Truth::AlwaysFalse,
        Meta::Path(_) => Truth::Variable,
        Meta::NameValue(value) => name_value_truth(value),
        Meta::List(list) if list.path.is_ident("any") => aggregate(list, false),
        Meta::List(list) if list.path.is_ident("all") => aggregate(list, true),
        Meta::List(list) if list.path.is_ident("not") => negate(single_truth(list)),
        Meta::List(_) => Truth::Variable,
    }
}

fn name_value_truth(value: &syn::MetaNameValue) -> Truth {
    if value.path.is_ident("feature")
        && matches!(
            &value.value,
            Expr::Lit(literal)
                if matches!(
                    &literal.lit,
                    Lit::Str(feature) if feature.value() == "certification-test-authority"
                )
        )
    {
        Truth::AlwaysFalse
    } else {
        Truth::Variable
    }
}

fn aggregate(list: &syn::MetaList, conjunction: bool) -> Truth {
    let parser = Punctuated::<Meta, Token![,]>::parse_terminated;
    let Ok(children) = parser.parse2(list.tokens.clone()) else {
        return Truth::Variable;
    };
    if conjunction {
        children.iter().fold(Truth::AlwaysTrue, and)
    } else {
        children.iter().fold(Truth::AlwaysFalse, or)
    }
}

fn single_truth(list: &syn::MetaList) -> Truth {
    let parser = Punctuated::<Meta, Token![,]>::parse_terminated;
    let Ok(children) = parser.parse2(list.tokens.clone()) else {
        return Truth::Variable;
    };
    children.first().map_or(Truth::Variable, truth)
}

fn and(left: Truth, right: &Meta) -> Truth {
    match (left, truth(right)) {
        (Truth::AlwaysFalse, _) | (_, Truth::AlwaysFalse) => Truth::AlwaysFalse,
        (Truth::AlwaysTrue, Truth::AlwaysTrue) => Truth::AlwaysTrue,
        _ => Truth::Variable,
    }
}

fn or(left: Truth, right: &Meta) -> Truth {
    match (left, truth(right)) {
        (Truth::AlwaysTrue, _) | (_, Truth::AlwaysTrue) => Truth::AlwaysTrue,
        (Truth::AlwaysFalse, Truth::AlwaysFalse) => Truth::AlwaysFalse,
        _ => Truth::Variable,
    }
}

fn negate(value: Truth) -> Truth {
    match value {
        Truth::AlwaysFalse => Truth::AlwaysTrue,
        Truth::Variable => Truth::Variable,
        Truth::AlwaysTrue => Truth::AlwaysFalse,
    }
}
