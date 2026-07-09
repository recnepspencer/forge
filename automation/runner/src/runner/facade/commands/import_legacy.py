from __future__ import annotations

from pathlib import Path

from runner.authority.config import load_config, validate_config
from runner.facade.lifecycle import import_legacy_run


def run_import_legacy_command(args) -> int:
    config_path = Path(args.config)
    config = load_config(config_path)
    errors = validate_config(config, config_path)
    if errors:
        for error in errors:
            print(f"validation error: {error}")
        return 2
    run_id = import_legacy_run(
        Path(args.old_state),
        config_path,
        args.run_id,
    )
    print(run_id)
    return 0
