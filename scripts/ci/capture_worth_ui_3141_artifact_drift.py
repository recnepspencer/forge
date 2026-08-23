from __future__ import annotations

import argparse
import json
from pathlib import Path

from worth_ui_ledger_artifact_drift import DriftCaptureRequest, capture_artifact_drift
from worth_ui_ledger_closure_storage import ledger_lock


ROOT = Path(__file__).resolve().parents[2]
LEDGER = ROOT / "_docs/worth-ui/milestone-3.14.1-proof-ledger.csv"
LEDGER_LOCK = LEDGER.with_suffix(".lock")


def parse_arguments() -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description="Retain collateral artifact drift from a governed incident"
    )
    parser.add_argument("--incident-digest", required=True)
    parser.add_argument("--parent-invalidation", required=True)
    parser.add_argument("--parent-invalidation-digest", required=True)
    parser.add_argument("--expected-count", type=int, required=True)
    return parser.parse_args()


def main() -> int:
    arguments = parse_arguments()
    with ledger_lock(LEDGER_LOCK):
        result = capture_artifact_drift(
            ROOT,
            LEDGER,
            DriftCaptureRequest(
                arguments.incident_digest,
                arguments.parent_invalidation,
                arguments.parent_invalidation_digest,
                arguments.expected_count,
            ),
        )
    print(json.dumps(result, sort_keys=True))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
