from __future__ import annotations

import hashlib
import json
import sys
import tempfile
import unittest
from pathlib import Path
from unittest.mock import call, patch

sys.path.insert(0, str(Path(__file__).resolve().parent))

from worth_ui_ledger_execution_observation_retention import retain_payload_observations
from worth_ui_ledger_execution_binding import artifact_bindings_match
from worth_ui_ledger_execution_identity import AuthenticatedExecution
from worth_ui_ledger_portfolio_executions import (
    aggregate_executions,
    authenticated_row_payload,
)
from worth_ui_ledger_runner_authentication import authentication_tag


class PortfolioExecutionAdmissionTests(unittest.TestCase):
    def test_mixed_row_receipts_harvest_from_their_own_source_states(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            inherited = "1" * 64
            current = "2" * 64
            payload = {
                "execution_receipts": [
                    {"observation_sha256": inherited},
                    {"observation_sha256": current},
                ],
            }
            with patch(
                "worth_ui_ledger_execution_observation_retention.migrate_payload"
            ), patch(
                "worth_ui_ledger_execution_observation_retention.retain"
            ) as retain:
                retain_payload_observations(root, "c" * 64, payload)
            self.assertEqual(
                retain.call_args_list,
                [
                    call(root, inherited),
                    call(root, current),
                ],
            )

    def test_non_ledger_mutation_control_ignores_only_its_legacy_ledger_binding(self) -> None:
        ledger = "WORTH_UI_MILESTONE_3141_LEDGER"
        non_ledger = [
            "cargo",
            "test",
            "milestone_3141_phase1_ledger::result_artifact::mutation_tests::"
            "phase_two_boundary_observation_rejects_each_causal_mutation",
        ]
        actual_ledger_reader = [
            "cargo",
            "test",
            "milestone_3141_phase1_ledger::phase_five_closure_requires_every_"
            "predecessor_and_phase_five_row",
        ]
        retained = {ledger: {"sha256": "a" * 64}}
        current = {ledger: {"sha256": "b" * 64}}
        self.assertTrue(artifact_bindings_match(retained, current, non_ledger))
        self.assertFalse(artifact_bindings_match(retained, current, actual_ledger_reader))

    def test_historical_receipts_use_authenticated_causal_bindings(self) -> None:
        command = ["cargo", "test", "--list"]
        command_sha = hashlib.sha256(
            json.dumps(command, separators=(",", ":")).encode("utf-8")
        ).hexdigest()

        def payload(requirement: str) -> dict[str, object]:
            receipts = [
                {
                    "schema": "worth-ui-ledger-execution-reference-v1",
                    "role": role,
                    "execution_binding_key": key,
                    "observation_sha256": key,
                    "command_sha256": command_sha,
                    "duration_ms": 1,
                    "acquisition": "executed",
                }
                for role, key in (
                    ("main-discovery", "1" * 64),
                    ("ignored-discovery", requirement[0] * 64),
                    ("main-test", requirement[-1] * 64),
                )
            ]
            return {
                "requirement": requirement,
                "execution_receipts": receipts,
                "list_command": command,
                "ignored_list_command": command,
                "test_command": command,
            }

        causal = payload("A2")
        shared = payload("B3")
        causal["causal_reuse"] = {
            "schema": "worth-ui-ledger-causal-reuse-v2",
            "execution_observation_ids": ["1" * 64],
        }
        with (
            patch(
                "worth_ui_ledger_portfolio_executions.authenticated_row_payload",
                return_value=True,
            ),
            patch(
                "worth_ui_ledger_portfolio_executions.historical_observations",
                side_effect=({"1" * 64}, set()),
            ),
            patch(
                "worth_ui_ledger_portfolio_executions.migrate_payload",
            ),
            patch(
                "worth_ui_ledger_portfolio_executions.validate_execution",
                side_effect=lambda reference, expectation: AuthenticatedExecution(
                    reference["observation_sha256"],
                    reference["execution_binding_key"],
                    expectation.role,
                    {"execution_binding": {
                        "command": command,
                        "artifact_bindings": {},
                    }, "duration_ms": 1},
                    f"{expectation.role}:{reference['observation_sha256']}",
                ),
            ) as validate_receipt,
        ):
            aggregate_executions(
                [causal, shared], Path("."), "a" * 40, "b" * 64, {"A2", "B3"}
            )

        shared_calls = [
            call for call in validate_receipt.call_args_list
            if call.args[0]["observation_sha256"] == "1" * 64
        ]
        self.assertEqual(len(shared_calls), 2)
        self.assertEqual(
            [call.args[1].historical_allowed for call in shared_calls],
            [True, False],
        )

    def test_phase_six_groups_distinct_receipts_by_logical_execution_identity(self) -> None:
        command = ["cargo", "test", "--list"]
        command_sha = hashlib.sha256(
            json.dumps(command, separators=(",", ":")).encode("utf-8")
        ).hexdigest()
        bindings = {"WORTH_UI_COMPILE_ARTIFACT": {"sha256": "a" * 64}}

        def payload(requirement: str, key_digit: str) -> dict[str, object]:
            receipts = [
                {
                    "schema": "worth-ui-ledger-execution-reference-v1",
                    "role": role,
                    "execution_binding_key": key_digit * 64,
                    "observation_sha256": key_digit * 64,
                    "command_sha256": command_sha,
                    "duration_ms": 1,
                    "acquisition": "executed",
                }
                for role in ("main-discovery", "ignored-discovery", "main-test")
            ]
            return {
                "requirement": requirement,
                "execution_receipts": receipts,
                "list_command": command,
                "ignored_list_command": command,
                "test_command": command,
            }

        first = payload("A2", "1")
        second = payload("B3", "2")

        def authenticated(
            reference: dict[str, object],
            expectation: object,
        ) -> AuthenticatedExecution:
            return AuthenticatedExecution(
                str(reference["observation_sha256"]),
                str(reference["execution_binding_key"]),
                expectation.role,
                {"execution_binding": {
                    "command": command,
                    "artifact_bindings": bindings,
                }, "duration_ms": 1},
                f"logical:{expectation.role}",
            )

        with (
            patch(
                "worth_ui_ledger_portfolio_executions.authenticated_row_payload",
                return_value=True,
            ),
            patch(
                "worth_ui_ledger_portfolio_executions.historical_observations",
                return_value=set(),
            ),
            patch(
                "worth_ui_ledger_portfolio_executions.migrate_payload",
            ),
            patch(
                "worth_ui_ledger_portfolio_executions.validate_execution",
                side_effect=authenticated,
            ),
        ):
            executions = aggregate_executions(
                [first, second], Path("."), "a" * 40, "b" * 64, {"A2", "B3"}
            )

        self.assertEqual(len(executions), 3)
        self.assertTrue(all(item["requirements"] == ["A2", "B3"] for item in executions))
        self.assertTrue(all(len(item["observations"]) == 2 for item in executions))
        self.assertEqual(
            {item["role"] for item in executions},
            {"main-discovery", "ignored-discovery", "main-test"},
        )

    def test_predecessor_row_authenticates_its_source_artifact_digest(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            unsigned = {
                "requirement": "P1-AFFINITY-01",
                "artifact_sha256": "a" * 64,
            }
            payload = {
                **unsigned,
                "runner_authentication": authentication_tag(unsigned, root),
            }
            self.assertTrue(authenticated_row_payload(root, payload))
            payload["artifact_sha256"] = "b" * 64
            self.assertFalse(authenticated_row_payload(root, payload))

    def test_in_memory_observation_authenticates_external_artifact_digest(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            artifact = root / "row.json"
            artifact.write_text("signed artifact", encoding="utf-8")
            digest = hashlib.sha256(artifact.read_bytes()).hexdigest()
            signed = {
                "requirement": "P1-AFFINITY-01",
                "executed_exact_command": "python runner --artifact row.json",
            }
            payload = {
                **signed,
                "artifact_sha256": digest,
                "runner_authentication": authentication_tag(signed, root),
            }
            self.assertTrue(authenticated_row_payload(root, payload))
            payload["artifact_sha256"] = "b" * 64
            self.assertFalse(authenticated_row_payload(root, payload))


if __name__ == "__main__":
    unittest.main()
