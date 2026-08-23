from __future__ import annotations

import hashlib
import csv
import json
import os
import sys
import tempfile
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))

from worth_ui_ledger_execution_binding import SCHEMA as BINDING_SCHEMA, digest_json
from worth_ui_ledger_execution_identity import (
    portfolio_execution_identity,
)
from worth_ui_ledger_execution_observation import create_observation
from worth_ui_ledger_execution_observation_store import retain_envelope
from worth_ui_ledger_execution_reference_validation import (
    ExecutionExpectation,
    validate_execution,
)
from worth_ui_ledger_command import CLAIM_FIELDS
from worth_ui_predecessor_handoff_currentness import (
    PredecessorVerification,
    expected_identity,
)


class ExecutionIdentityTests(unittest.TestCase):
    def test_source_state_is_authentication_only(self) -> None:
        bindings = {"WORTH_UI_COMPILE_ARTIFACT": {"sha256": "a" * 64}}
        first = portfolio_execution_identity("main-test", ["cargo", "test"], bindings)
        second = portfolio_execution_identity("main-test", ["cargo", "test"], bindings)
        self.assertEqual(first, second)

    def test_command_binding_and_role_each_change_identity(self) -> None:
        bindings = {"WORTH_UI_COMPILE_ARTIFACT": {"sha256": "a" * 64}}
        base = portfolio_execution_identity("main-test", ["cargo", "test"], bindings)
        self.assertNotEqual(
            base,
            portfolio_execution_identity("control-test", ["cargo", "test"], bindings),
        )
        self.assertNotEqual(
            base,
            portfolio_execution_identity("main-test", ["cargo", "check"], bindings),
        )
        self.assertNotEqual(
            base,
            portfolio_execution_identity(
                "main-test", ["cargo", "test"], {**bindings, "ledger": {"sha256": "b" * 64}}
            ),
        )

    def test_authenticated_receipt_rejects_tampered_identity_envelope(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            command = ["cargo", "test"]
            binding = {
                "schema": BINDING_SCHEMA,
                "command": command,
                "source_revision": "r" * 40,
                "source_state_digest": "s" * 64,
                "artifact_bindings": {},
            }
            receipt = retained_reference(root, binding, "main-test")
            expectation = ExecutionExpectation(
                root, "r" * 40, "s" * 64, "main-test", "P6-TEST-01"
            )
            validated = validate_execution(receipt, expectation)
            self.assertEqual(validated.role, "main-test")
            receipt["command_sha256"] = "0" * 64
            with self.assertRaisesRegex(RuntimeError, "differs from its observation"):
                validate_execution(receipt, expectation)
            receipt["command_sha256"] = digest_json(command)
            observation = receipt["observation_sha256"]
            envelope_path = (
                root / "_docs/worth-ui/milestone-3.14.1-evidence/"
                f"execution-observations/{observation[:2]}/{observation}.json"
            )
            envelope = json.loads(envelope_path.read_text(encoding="utf-8"))
            envelope["record"]["execution_binding"]["command"] = ["cargo", "check"]
            envelope_path.write_text(json.dumps(envelope), encoding="utf-8")
            with self.assertRaisesRegex(RuntimeError, "absent or unauthenticated"):
                validate_execution(receipt, expectation)

    def test_row_causal_reuse_allows_only_its_historical_receipt_binding(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            command = ["cargo", "test", "predecessor_handoff"]
            binding = {
                "schema": BINDING_SCHEMA,
                "command": command,
                "source_revision": "r" * 40,
                "source_state_digest": "s" * 64,
                "artifact_bindings": {
                    "WORTH_UI_PREDECESSOR_ARTIFACT": {"sha256": "a" * 64}
                },
            }
            receipt = retained_reference(root, binding, "main-test")
            with self.assertRaisesRegex(RuntimeError, "stale artifact bindings"):
                validate_execution(receipt, ExecutionExpectation(
                    root, "r" * 40, "s" * 64, "main-test", "P6-TEST-01"
                ))
            validated = validate_execution(
                receipt,
                ExecutionExpectation(
                    root, "current-revision", "current-state", "main-test",
                    "P6-TEST-01", True,
                ),
            )
            self.assertEqual(
                validated.observation_sha256, receipt["observation_sha256"]
            )

    def test_current_predecessor_receipt_may_bind_the_exact_state_scoped_handoff(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            state = "a" * 64
            ledger = root / "ledger.csv"
            fields = (*CLAIM_FIELDS, "result", "reopen_lineage", "final_source")
            with ledger.open("w", encoding="utf-8", newline="") as stream:
                writer = csv.DictWriter(stream, fieldnames=fields, lineterminator="\n")
                writer.writeheader()
                for phase, requirement in ((1, "P1-ROW-01"), (2, "P2-ROW-01")):
                    row = {field: "value" for field in fields}
                    row.update({"phase": str(phase), "requirement": requirement})
                    writer.writerow(row)
            typed, basis = expected_identity(
                PredecessorVerification(root, ledger, 3, "r" * 40, state)
            )
            handoff = typed.destination(root)
            handoff.parent.mkdir(parents=True)
            handoff.write_text(
                json.dumps({
                    "schema": "worth-ui-phase-predecessor-handoff-v4",
                    "through_phase": 2,
                    "source_revision": "r" * 40,
                    "source_state_digest": state,
                    "rows": list(basis.claim_inventory),
                    "verification_basis": basis.payload(),
                }),
                encoding="utf-8",
            )
            command = ["cargo", "test", "predecessor_handoff"]
            binding = {
                "schema": BINDING_SCHEMA,
                "command": command,
                "source_revision": "r" * 40,
                "source_state_digest": state,
                "artifact_bindings": {
                    "WORTH_UI_PREDECESSOR_ARTIFACT": {
                        "sha256": hashlib.sha256(handoff.read_bytes()).hexdigest()
                    }
                },
            }
            receipt = retained_reference(root, binding, "main-test")
            previous = os.environ.get("WORTH_UI_MILESTONE_3141_LEDGER")
            os.environ["WORTH_UI_MILESTONE_3141_LEDGER"] = str(ledger)
            try:
                validated = validate_execution(
                    receipt,
                    ExecutionExpectation(
                        root, "r" * 40, state, "main-test", "P3-PREDECESSOR-01"
                    ),
                )
            finally:
                if previous is None:
                    os.environ.pop("WORTH_UI_MILESTONE_3141_LEDGER", None)
                else:
                    os.environ["WORTH_UI_MILESTONE_3141_LEDGER"] = previous
            self.assertEqual(
                validated.observation_sha256, receipt["observation_sha256"]
            )


def retained_reference(
    root: Path, binding: dict[str, object], role: str
) -> dict[str, object]:
    envelope, reference = create_observation(root, binding, 0, "passed", "", 1)
    retain_envelope(root, envelope)
    return {"role": role, **reference.payload()}


if __name__ == "__main__":
    unittest.main()
