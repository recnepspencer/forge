import json
import tomllib
from dataclasses import dataclass
from pathlib import Path
from typing import Any


@dataclass(frozen=True)
class Violation:
    rule: str
    detail: str


def load_json(path: Path) -> dict[str, Any]:
    value = json.loads(path.read_text(encoding="utf-8"))
    if not isinstance(value, dict):
        raise ValueError("test topology budget must be a JSON object")
    return value


def load_toml(path: Path) -> dict[str, Any]:
    with path.open("rb") as source:
        value = tomllib.load(source)
    if not isinstance(value, dict):
        raise ValueError(f"manifest must be a TOML object: {path}")
    return value


def required_string(value: dict[str, Any], key: str) -> str:
    item = value.get(key)
    if not isinstance(item, str) or not item:
        raise ValueError(f"{key} must be a non-empty string")
    return item


def required_int(value: dict[str, Any], key: str) -> int:
    item = value.get(key)
    if not isinstance(item, int) or item < 0:
        raise ValueError(f"{key} must be a non-negative integer")
    return item


def required_string_list(value: dict[str, Any], key: str) -> list[str]:
    items = value.get(key)
    if not isinstance(items, list) or not all(
        isinstance(item, str) and item for item in items
    ):
        raise ValueError(f"{key} must be a list of non-empty strings")
    return items


def required_string_values(value: Any, key: str) -> list[str]:
    if not isinstance(value, list) or not all(
        isinstance(item, str) and item for item in value
    ):
        raise ValueError(f"{key} must be a list of non-empty strings")
    return value
