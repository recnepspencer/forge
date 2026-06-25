use super::super::{WorthUiLiveViewStateAccess, WorthUiLiveViewStateValueKind};

pub(super) fn authored_live_view_value_kind(token: &str) -> WorthUiLiveViewStateValueKind {
    match token {
        "text" => WorthUiLiveViewStateValueKind::Text,
        "boolean" => WorthUiLiveViewStateValueKind::Boolean,
        "number" => WorthUiLiveViewStateValueKind::Number,
        other => WorthUiLiveViewStateValueKind::Unsupported(other.to_owned()),
    }
}

pub(super) fn authored_live_view_access(token: &str) -> WorthUiLiveViewStateAccess {
    match token {
        "read_write" => WorthUiLiveViewStateAccess::ReadWrite,
        _ => WorthUiLiveViewStateAccess::ReadOnly,
    }
}

pub(super) fn invalid_live_view_identity(value: &str) -> bool {
    value.trim().is_empty() || value.chars().any(char::is_whitespace)
}
