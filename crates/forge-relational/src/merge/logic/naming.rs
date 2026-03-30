use std::borrow::Cow;

use crate::logic::runtime::RelationalRuntime;
use crate::symbols::data::InternedString;

pub(crate) fn resolve_interned_string<'a>(
    runtime: &'a RelationalRuntime,
    value: &'a InternedString,
) -> Option<Cow<'a, str>> {
    match value {
        InternedString::Raw(raw) => Some(Cow::Borrowed(raw.as_str())),
        InternedString::Symbol(symbol) => runtime.resolve_symbol(*symbol).map(Cow::Borrowed),
    }
}
