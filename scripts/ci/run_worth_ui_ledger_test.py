from __future__ import annotations

import json
import subprocess
import sys
from typing import Any

from worth_ui_ledger_command import ROOT, parse_args
from worth_ui_ledger_row_evidence import result_payload
from worth_ui_ledger_runner_authentication import authentication_tag
from worth_ui_ledger_artifact_identity import row_evidence
from worth_ui_ledger_artifact_publication import publish_json_artifact


def write_artifact(requirement: str, payload: dict[str, Any]) -> str:
    payload.pop("runner_authentication", None)
    payload["runner_authentication"] = authentication_tag(payload, ROOT)
    return publish_json_artifact(ROOT, row_evidence(requirement), payload)


def main() -> int:
    try:
        governed_test = parse_args()
        payload, exit_code = result_payload(governed_test)
        artifact_digest = write_artifact(governed_test.requirement, payload)
    except (OSError, RuntimeError, ValueError, subprocess.TimeoutExpired) as error:
        print(f"ledger evidence runner: {error}", file=sys.stderr)
        return 2
    print(json.dumps({"artifact_sha256": artifact_digest, **payload}, sort_keys=True))
    return exit_code


if __name__ == "__main__":
    raise SystemExit(main())
