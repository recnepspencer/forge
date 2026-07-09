from __future__ import annotations

from pathlib import Path

from runner.authority.config import load_config, validate_config


def run_validate_command(args) -> int:
    config_path = Path(args.config)
    config = load_config(config_path)
    errors = validate_config(config, config_path)
    if errors:
        for error in errors:
            print(f"validation error: {error}")
        return 2
    print("config is valid")
    return 0
