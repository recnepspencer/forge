from __future__ import annotations

import os
import sys
import tempfile
import unittest
from pathlib import Path
from unittest.mock import patch


CI = Path(__file__).resolve().parent
if str(CI) not in sys.path:
    sys.path.insert(0, str(CI))

from worth_ui_ledger_causal_revalidation import (
    available_receipts_match_payload_source,
    receipts_match_payload_source,
)
from worth_ui_ledger_execution_binding import (
    GovernedExecutionSnapshot,
    execution_binding,
)
from worth_ui_ledger_execution_observation import create_observation
from worth_ui_ledger_execution_observation_store import CACHE_ENV, stage


ROOT = CI.parents[1]


class StagedExecutionLineageTests(unittest.TestCase):
    def test_current_staged_observation_is_available_before_publication(self) -> None:
        with tempfile.TemporaryDirectory() as directory, patch.dict(
            os.environ, {CACHE_ENV: directory}
        ):
            payload = staged_payload("b" * 64)
            self.assertTrue(available_receipts_match_payload_source(ROOT, payload))
            self.assertFalse(receipts_match_payload_source(ROOT, payload))

    def test_stale_staged_observation_is_rejected(self) -> None:
        with tempfile.TemporaryDirectory() as directory, patch.dict(
            os.environ, {CACHE_ENV: directory}
        ):
            payload = staged_payload("c" * 64)
            payload["source_state_digest"] = "b" * 64
            self.assertFalse(available_receipts_match_payload_source(ROOT, payload))


def staged_payload(state_digest: str) -> dict[str, object]:
    revision = "a" * 40
    requirement = "P2-APPLICATION-01"
    role = "control-test"
    command = ["cargo", "test", "-p", "owner", "owner::control", "--", "--exact"]
    binding = execution_binding(
        command,
        ROOT,
        GovernedExecutionSnapshot(revision, state_digest),
        requirement=requirement,
    )
    envelope, reference = create_observation(ROOT, binding, 0, "passed", "", 1)
    stage(envelope)
    return {
        "requirement": requirement,
        "source_revision": revision,
        "source_state_digest": state_digest,
        "execution_receipts": [{"role": role, **reference.payload()}],
    }


if __name__ == "__main__":
    unittest.main()
