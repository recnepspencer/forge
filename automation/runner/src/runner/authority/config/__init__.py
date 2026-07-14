from __future__ import annotations

from pathlib import Path
from typing import Any


def load_config(path: Path) -> dict[str, Any]:
    from runner.authority.config.loader import load_config as _load_config

    return _load_config(path)


def validate_config(config: dict[str, Any], config_path: Path) -> list[str]:
    from runner.authority.config.validator import validate_config as _validate_config

    return _validate_config(config, config_path)


__all__ = ["load_config", "validate_config"]
