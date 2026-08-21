from __future__ import annotations

import csv
import json
import sys
import tempfile
import unittest
from pathlib import Path
from unittest.mock import patch

CI = Path(__file__).resolve().parent
if str(CI) not in sys.path:
    sys.path.insert(0, str(CI))

import worth_ui_ledger_acceptance as acceptance

from worth_ui_ledger_causal_revalidation import (
    digest,
    revalidate_joined_predecessor_payload,
    revalidate_row_payload,
    source_digest_at,
    validate_causal_reuse,
)
from worth_ui_ledger_durable_receipts import persist_envelope
from worth_ui_ledger_execution_cache import (
    CANDIDATE_LEDGER_ENV,
    causal_artifact_dependencies,
    digest_json,
    execution_binding,
)
from worth_ui_ledger_runner_authentication import authentication_tag, authenticates


class CausalRevalidationTests(unittest.TestCase):
    def test_p2_non_reader_reuses_authenticated_legacy_ledger_receipt(self) -> None:
        root = CI.parents[1]
        ledger = root / "_docs/worth-ui/milestone-3.14.1-proof-ledger.csv"
        with ledger.open(encoding="utf-8", newline="") as stream:
            row = next(
                row
                for row in csv.DictReader(stream)
                if row["requirement"] == "P2-WORLD-01"
            )
        artifact = root / row["retained_result_artifact"]
        payload = json.loads(artifact.read_text(encoding="utf-8"))
        control = next(
            receipt
            for receipt in payload["execution_receipts"]
            if receipt["role"] == "control-test"
        )
        envelope_path = root / (
            "_docs/worth-ui/milestone-3.14.1-evidence/executions/"
            f"{control['key'][:2]}/{control['key']}.json"
        )
        command = json.loads(envelope_path.read_text(encoding="utf-8"))["record"][
            "command"
        ]
        self.assertNotIn(
            CANDIDATE_LEDGER_ENV,
            causal_artifact_dependencies(command, control["role"]),
        )
        self.assertIsNotNone(
            revalidate_row_payload(
                root,
                row,
                payload,
                digest(artifact.read_bytes()),
                payload["claim_digest"],
                "d" * 40,
                "e" * 64,
            )
        )

    def test_shared_row_inherits_authenticated_historical_receipt_lineage(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            shared_identity = root / "shared.json"
            shared_identity.write_text(
                json.dumps(
                    {
                        "source_revision": "a" * 40,
                        "source_state_digest": "b" * 64,
                        "source_digest": "c" * 64,
                        "run_nonce": "d" * 32,
                        "causal_reuse": {
                            "predecessor_source_state_digest": "e" * 64,
                        },
                    }
                ),
                encoding="utf-8",
            )
            row = {
                "requirement": "P1-HEADLESS-COST-01",
                "exact_command": "runner --artifact derived.json",
            }
            payload = {
                "shared_main_artifact": "shared.json",
                "shared_main_artifact_digest": "f" * 64,
                "execution_receipts": [{"key": "1" * 64}],
            }
            with (
                patch.object(
                    acceptance, "validate_causal_reuse", return_value={"1" * 64}
                ),
                patch.object(
                    acceptance,
                    "source_artifact_bindings",
                    return_value={"source.rs": "2" * 64},
                ),
            ):
                acceptance.inherit_shared_receipt_lineage(
                    row, payload, root, "a" * 40, "b" * 64, "3" * 64
                )
            reuse = payload["causal_reuse"]
            self.assertEqual(reuse["predecessor_source_state_digest"], "e" * 64)
            self.assertEqual(reuse["execution_receipt_keys"], ["1" * 64])
            self.assertEqual(reuse["claim_digest"], "3" * 64)

    def test_current_evidence_keeps_its_exact_nonce(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            row, payload, artifact = fixture(root)
            current = revalidate_row_payload(
                root, row, payload, digest(artifact), "c" * 64, "a" * 40, "b" * 64
            )
            self.assertIsNotNone(current)
            assert current is not None
            self.assertEqual(current["run_nonce"], payload["run_nonce"])
            self.assertNotIn("causal_reuse", current)

    def test_unrelated_global_state_reuses_exact_causal_evidence(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            row, payload, artifact = fixture(root)
            current = revalidate_row_payload(
                root, row, payload, digest(artifact), "c" * 64, "d" * 40, "e" * 64
            )
            self.assertIsNotNone(current)
            assert current is not None
            self.assertEqual(current["source_revision"], "d" * 40)
            self.assertEqual(current["source_state_digest"], "e" * 64)
            self.assertEqual(
                validate_causal_reuse(root, current, "d" * 40, "e" * 64),
                {payload["execution_receipts"][0]["key"]},
            )
            unsigned = {
                key: value
                for key, value in current.items()
                if key != "runner_authentication"
            }
            self.assertTrue(
                authenticates(unsigned, current["runner_authentication"], root)
            )

    def test_transport_artifact_digest_is_not_part_of_signed_evidence(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            row, payload, artifact = fixture(root)
            transported = {**payload, "artifact_sha256": digest(artifact)}
            current = revalidate_row_payload(
                root,
                row,
                transported,
                digest(artifact),
                "c" * 64,
                "d" * 40,
                "e" * 64,
            )
            self.assertIsNotNone(current)

    def test_claim_command_or_source_drift_requires_execution(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            row, payload, artifact = fixture(root)
            self.assertIsNone(
                revalidate_row_payload(
                    root, row, payload, digest(artifact), "0" * 64, "d" * 40, "e" * 64
                )
            )
            row["exact_command"] += " --changed"
            self.assertIsNone(
                revalidate_row_payload(
                    root, row, payload, digest(artifact), "c" * 64, "d" * 40, "e" * 64
                )
            )
            row["exact_command"] = row["exact_command"].removesuffix(" --changed")
            row["independent_oracle"] = "oracle::changed"
            self.assertIsNone(
                revalidate_row_payload(
                    root, row, payload, digest(artifact), "c" * 64, "d" * 40, "e" * 64
                )
            )
            row["independent_oracle"] = "oracle::adjudicate"
            (root / "source.rs").write_text("changed", encoding="utf-8")
            self.assertIsNone(
                revalidate_row_payload(
                    root, row, payload, digest(artifact), "c" * 64, "d" * 40, "e" * 64
                )
            )

    def test_forged_causal_binding_is_rejected(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            row, payload, artifact = fixture(root)
            current = revalidate_row_payload(
                root, row, payload, digest(artifact), "c" * 64, "d" * 40, "e" * 64
            )
            assert current is not None
            current["causal_reuse"]["execution_receipt_keys"] = []
            with self.assertRaisesRegex(RuntimeError, "differs from its causal binding"):
                validate_causal_reuse(root, current, "d" * 40, "e" * 64)

    def test_dependency_or_receipt_authentication_drift_requires_execution(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            row, payload, artifact = fixture(root)
            key = payload["execution_receipts"][0]["key"]
            envelope_path = root / (
                f"_docs/worth-ui/milestone-3.14.1-evidence/executions/"
                f"{key[:2]}/{key}.json"
            )
            envelope = json.loads(envelope_path.read_text(encoding="utf-8"))
            envelope["record"]["artifact_bindings"] = {"changed": {"sha256": "0" * 64}}
            envelope_path.write_text(json.dumps(envelope), encoding="utf-8")
            self.assertIsNone(
                revalidate_row_payload(
                    root, row, payload, digest(artifact), "c" * 64, "d" * 40, "e" * 64
                )
            )

    def test_joined_reissue_updates_the_top_level_claim(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            row, claim_evidence, artifact = fixture(root)
            retained = dict(claim_evidence)
            retained["claim_digest"] = "f" * 64
            retained["artifact_sha256"] = digest(artifact)
            retained.pop("runner_authentication")
            retained["runner_authentication"] = authentication_tag(retained, root)
            current = revalidate_joined_predecessor_payload(
                root,
                row,
                retained,
                claim_evidence,
                digest(artifact),
                "c" * 64,
                "d" * 40,
                "e" * 64,
                True,
            )
            self.assertIsNotNone(current)
            assert current is not None
            self.assertEqual(current["claim_digest"], "c" * 64)
            self.assertEqual(current["causal_reuse"]["claim_digest"], "c" * 64)


def fixture(root: Path) -> tuple[dict[str, str], dict[str, object], bytes]:
    (root / "source.rs").write_text("owned source", encoding="utf-8")
    command = "python runner --source source.rs --artifact result.json"
    row = {
        "requirement": "P5-ROW-01",
        "exact_command": command,
        "source_identity": "source.rs",
        "production_entry": "owner::perform",
        "independent_oracle": "oracle::adjudicate",
    }
    payload: dict[str, object] = {
        "requirement": row["requirement"],
        "exit_posture": "passed",
        "claim_digest": "c" * 64,
        "source_revision": "a" * 40,
        "source_state_digest": "b" * 64,
        "source_digest": source_digest_at(root, ("source.rs",)),
        "source_identity": ["source.rs"],
        "mapping_source_identity": ["source.rs"],
        "source_rebindings": [],
        "run_nonce": "1" * 32,
        "production_entry": row["production_entry"],
        "independent_oracle": row["independent_oracle"],
        "executed_exact_command": command,
        "execution_receipts": [],
    }
    command_parts = command.split()
    binding = execution_binding(command_parts, root, "a" * 40, "b" * 64)
    key = digest_json(binding)
    record = {
        **binding,
        "key": key,
        "returncode": 0,
        "stdout": "passed",
        "stderr": "",
        "duration_ms": 7,
    }
    envelope = {
        "record": record,
        "receipt_sha256": digest_json(record),
        "runner_authentication": authentication_tag(record, root),
    }
    persist_envelope(root, key, envelope)
    payload["execution_receipts"] = [
        {
            "role": "main-test",
            "key": key,
            "command_sha256": digest_json(command_parts),
            "duration_ms": 7,
        }
    ]
    payload["runner_authentication"] = authentication_tag(payload, root)
    artifact = (json.dumps(payload, indent=2) + "\n").encode("utf-8")
    return row, payload, artifact


if __name__ == "__main__":
    unittest.main()
