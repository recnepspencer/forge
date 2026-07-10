from __future__ import annotations

import json

from runner.facade.runtime_inspection import artifact_inventory


def run_artifacts_command(args) -> int:
    print(json.dumps(artifact_inventory(args.run_id), indent=2))
    return 0
