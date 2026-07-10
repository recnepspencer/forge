from __future__ import annotations

import json

from runner.facade.runtime_inspection import archive_run


def run_archive_command(args) -> int:
    print(json.dumps(archive_run(args.run_id, prune_derived=args.prune_derived), indent=2))
    return 0
