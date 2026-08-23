from __future__ import annotations

import argparse
import json
from pathlib import Path

from worth_ui_ledger_closure_storage import ledger_lock
from worth_ui_ledger_command import source_revision
from worth_ui_ledger_phase_invalidation import (
    ALLOWED_CAUSES,
    InvalidationRequest,
    invalidate_phase,
)


ROOT = Path(__file__).resolve().parents[2]
LEDGER = ROOT / "_docs/worth-ui/milestone-3.14.1-proof-ledger.csv"
LEDGER_LOCK = LEDGER.with_suffix(".lock")


def parse_arguments() -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description="Atomically invalidate one Worth UI 3.14.1 ledger phase"
    )
    parser.add_argument("--phase", type=int, required=True)
    parser.add_argument("--incident-requirement", required=True)
    parser.add_argument("--observed-digest", required=True)
    parser.add_argument(
        "--cause",
        action="append",
        choices=sorted(ALLOWED_CAUSES),
        required=True,
    )
    return parser.parse_args()


def main() -> int:
    arguments = parse_arguments()
    request = InvalidationRequest(
        arguments.phase,
        arguments.incident_requirement,
        arguments.observed_digest,
        tuple(arguments.cause),
        source_revision(),
    )
    with ledger_lock(LEDGER_LOCK):
        result = invalidate_phase(ROOT, LEDGER, request)
    print(json.dumps(result, sort_keys=True))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
