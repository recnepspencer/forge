from __future__ import annotations

import re
from typing import Any

TOKEN = re.compile(r"{([A-Za-z0-9_.]+)}")


def render_template(template: str, context: dict[str, Any]) -> str:
    def replace(match: re.Match[str]) -> str:
        return stringify(resolve_token(context, match.group(1)))

    return TOKEN.sub(replace, template)


def resolve_token(context: dict[str, Any], token: str) -> Any:
    value: Any = context
    for part in token.split("."):
        if isinstance(value, dict) and part in value:
            value = value[part]
            continue
        raise KeyError(f"template variable {{{token}}} is not defined")
    return value


def stringify(value: Any) -> str:
    if value is None:
        return ""
    if isinstance(value, list):
        if not value:
            return "- none"
        return "\n".join(f"- {stringify(item)}" for item in value)
    if isinstance(value, dict):
        if not value:
            return "- none"
        return "\n".join(f"- {key}: {stringify(item)}" for key, item in value.items())
    return str(value)
