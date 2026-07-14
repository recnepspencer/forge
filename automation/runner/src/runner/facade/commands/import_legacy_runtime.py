from __future__ import annotations

from runner.generation.legacy_runtime_importer import import_legacy_runtime_authority


def run_import_legacy_runtime_command(args) -> int:
    print(import_legacy_runtime_authority(args.run_id))
    return 0
