from __future__ import annotations

import hashlib
import json
import os
import subprocess
import sys
import tempfile
from typing import Any

from worth_ui_ledger_command import (
    ROOT,
    ControlTest,
    GovernedTest,
    cargo_command,
    claim_digest,
    control_budget_ms,
    control_cargo_command,
    execution_budget_ms,
    execution_counts,
    exact_test_duration_ms,
    ignored_list_command,
    listed_test_names,
    parse_args,
    repository_path,
    source_digest,
    source_revision,
)
from worth_ui_ledger_execution_cache import timed_execution
from worth_ui_3141_ledger_contracts import construction_cost, execution_cost
from worth_ui_ledger_governed_snapshot import (
    governed_snapshot_changed,
    governed_sources_changed,
    refresh_predecessor_handoff,
)
from worth_ui_ledger_hostile_control_evidence import control_payload
from worth_ui_ledger_portfolio_snapshot import source_state_for_row
from worth_ui_ledger_observation import (
    mutation_control_observation,
    p1_counter_observation,
    p2_counter_observation,
)
from worth_ui_ledger_row_evidence import result_payload
from worth_ui_ledger_runner_authentication import authentication_tag


def write_artifact(identity: str, payload: dict[str, Any]) -> str:
    destination = repository_path(identity)
    payload.pop("runner_authentication", None)
    payload["runner_authentication"] = authentication_tag(payload, ROOT)
    destination.parent.mkdir(parents=True, exist_ok=True)
    descriptor, temporary = tempfile.mkstemp(
        prefix=f".{destination.name}.", dir=destination.parent
    )
    try:
        with os.fdopen(descriptor, "w", encoding="utf-8") as output:
            json.dump(payload, output, indent=2)
            output.write("\n")
        os.replace(temporary, destination)
    finally:
        if os.path.exists(temporary):
            os.unlink(temporary)
    return hashlib.sha256(destination.read_bytes()).hexdigest()


def main() -> int:
    try:
        governed_test = parse_args()
        payload, exit_code = result_payload(governed_test)
        artifact_digest = write_artifact(governed_test.artifact, payload)
    except (OSError, RuntimeError, ValueError, subprocess.TimeoutExpired) as error:
        print(f"ledger evidence runner: {error}", file=sys.stderr)
        return 2
    print(json.dumps({"artifact_sha256": artifact_digest, **payload}, sort_keys=True))
    return exit_code


if __name__ == "__main__":
    raise SystemExit(main())
