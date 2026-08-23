from __future__ import annotations

import csv
import hashlib
import json
import tempfile
import unittest
from pathlib import Path
from unittest import mock

from worth_ui_ledger_artifact_identity import (
    phase_invalidation,
    predecessor_handoff,
    require_row_evidence_identity,
    row_evidence,
)
import worth_ui_ledger_artifact_publication as artifact_publication
from worth_ui_ledger_artifact_publication import publish_json_artifact
from worth_ui_ledger_artifact_drift import (
    DriftCaptureRequest,
    capture_artifact_drift,
)
from worth_ui_ledger_phase_invalidation import InvalidationRequest, invalidate_phase


class ArtifactIdentityTests(unittest.TestCase):
    def test_row_artifact_path_is_derived_from_its_requirement(self) -> None:
        identity = require_row_evidence_identity(
            "P6-WINDOWS-WORLD-01",
            "_docs/worth-ui/milestone-3.14.1-evidence/p6-windows-world-01.json",
        )
        self.assertEqual(identity, row_evidence("P6-WINDOWS-WORLD-01"))
        with self.assertRaisesRegex(ValueError, "artifact must be"):
            require_row_evidence_identity(
                "P6-WINDOWS-WORLD-01",
                "_docs/worth-ui/milestone-3.14.1-evidence/p6-predecessor-01.json",
            )

    def test_json_publication_rejects_cross_kind_payload_before_writing(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            identity = row_evidence("P6-WINDOWS-WORLD-01")
            with self.assertRaisesRegex(RuntimeError, "schema version"):
                publish_json_artifact(
                    root,
                    identity,
                    {
                        "schema": "worth-ui-phase-predecessor-handoff-v4",
                        "through_phase": 2,
                    },
                )
            self.assertFalse(identity.destination(root).exists())

    def test_json_publication_retries_windows_invalid_argument_replace(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            identity = row_evidence("P6-WINDOWS-WORLD-01")
            payload = {"schema_version": 7, "requirement": "P6-WINDOWS-WORLD-01"}
            replace = artifact_publication.os.replace
            attempts = 0

            def replace_after_transient(source: Path, destination: Path) -> None:
                nonlocal attempts
                attempts += 1
                if attempts == 1:
                    raise OSError(22, "Invalid argument", str(destination))
                if attempts == 2:
                    windows_error = OSError(13, "Invalid argument", str(destination))
                    windows_error.winerror = 22
                    raise windows_error
                replace(source, destination)

            with mock.patch.object(
                artifact_publication.os, "replace", side_effect=replace_after_transient
            ), mock.patch.object(artifact_publication.time, "sleep"):
                publish_json_artifact(root, identity, payload)
            self.assertEqual(attempts, 3)
            self.assertEqual(
                json.loads(identity.destination(root).read_text(encoding="utf-8")),
                payload,
            )

    def test_predecessor_and_invalidation_payloads_bind_their_phase(self) -> None:
        predecessor = predecessor_handoff(6)
        predecessor.validate_json_payload(
            {
                "schema": "worth-ui-phase-predecessor-handoff-v4",
                "through_phase": 5,
            }
        )
        with self.assertRaisesRegex(RuntimeError, "phase"):
            predecessor.validate_json_payload(
                {
                    "schema": "worth-ui-phase-predecessor-handoff-v4",
                    "through_phase": 4,
                }
            )
        invalidation = phase_invalidation(6, "a" * 64)
        with self.assertRaisesRegex(RuntimeError, "phase"):
            invalidation.validate_json_payload(
                {"schema": "worth-ui-ledger-phase-invalidation-v1", "phase": 5}
            )


class PhaseInvalidationTests(unittest.TestCase):
    FIELDS = [
        "phase",
        "requirement",
        "retained_result_artifact",
        "matched_test_count",
        "command_result",
        "source_revision",
        "source_digest",
        "source_state_digest",
        "run_nonce",
        "result",
        "reopen_lineage",
        "final_source",
        "result_artifact_digest",
    ]

    def test_phase_invalidation_archives_incident_and_reopens_proved_rows(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root, ledger, corrupt, observed = self.fixture(directory)
            result = invalidate_phase(
                root,
                ledger,
                InvalidationRequest(
                    6,
                    "P6-INCIDENT-01",
                    observed,
                    (
                        "artifact-kind-mismatch",
                        "independent-review-rejected",
                    ),
                    "a" * 40,
                ),
            )
            rows = self.rows(ledger)
            by_requirement = {row["requirement"]: row for row in rows}
            self.assertEqual(by_requirement["P5-PREVIOUS-01"]["result"], "PROVED")
            for requirement in ("P6-ONE-01", "P6-INCIDENT-01"):
                row = by_requirement[requirement]
                self.assertEqual((row["result"], row["final_source"]), ("OPEN", "false"))
                self.assertIn("invalidation:", row["reopen_lineage"])
                self.assertIn("supersedes:", row["reopen_lineage"])
            self.assertEqual(by_requirement["P6-CLOSE-01"]["reopen_lineage"], "none")
            self.assertEqual(
                (by_requirement["P7-DESCENDANT-01"]["result"],
                 by_requirement["P7-DESCENDANT-01"]["final_source"]),
                ("OPEN", "false"),
            )
            archive = root / str(result["superseded_artifact"])
            self.assertEqual(archive.read_bytes(), corrupt)
            receipt = json.loads((root / str(result["receipt"])).read_text(encoding="utf-8"))
            self.assertEqual(receipt["schema"], "worth-ui-ledger-phase-invalidation-v2")
            self.assertEqual(receipt["incident"]["observed_artifact_kind"], "predecessor-handoff")
            self.assertEqual(
                [row["requirement"] for row in receipt["invalidated_rows"]],
                ["P6-ONE-01", "P6-INCIDENT-01"],
            )
            self.assertEqual(receipt["preserved_open_requirements"], ["P6-CLOSE-01"])
            self.assertEqual(
                [row["requirement"] for row in receipt["causally_reopened_rows"]],
                ["P7-DESCENDANT-01"],
            )

    def test_digest_precondition_refuses_partial_invalidation(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root, ledger, _corrupt, _observed = self.fixture(directory)
            before = ledger.read_bytes()
            with self.assertRaisesRegex(RuntimeError, "digest changed"):
                invalidate_phase(
                    root,
                    ledger,
                    InvalidationRequest(
                        6,
                        "P6-INCIDENT-01",
                        "f" * 64,
                        ("artifact-kind-mismatch",),
                        "a" * 40,
                    ),
                )
            self.assertEqual(ledger.read_bytes(), before)

    def fixture(self, directory: str) -> tuple[Path, Path, bytes, str]:
        root = Path(directory)
        incident_identity = row_evidence("P6-INCIDENT-01")
        incident = incident_identity.destination(root)
        incident.parent.mkdir(parents=True)
        corrupt = (
            json.dumps(
                {
                    "schema": "worth-ui-phase-predecessor-handoff-v1",
                    "through_phase": 2,
                    "rows": [],
                }
            )
            + "\n"
        ).encode("utf-8")
        incident.write_bytes(corrupt)
        observed = hashlib.sha256(corrupt).hexdigest()
        ledger = root / "ledger.csv"
        with ledger.open("w", encoding="utf-8", newline="") as stream:
            writer = csv.DictWriter(stream, fieldnames=self.FIELDS, lineterminator="\n")
            writer.writeheader()
            writer.writerows(
                [
                    self.row(5, "P5-PREVIOUS-01", "PROVED", "true", "1" * 64),
                    self.row(6, "P6-ONE-01", "PROVED", "true", "2" * 64),
                    self.row(6, "P6-INCIDENT-01", "PROVED", "true", "3" * 64),
                    self.row(6, "P6-CLOSE-01", "OPEN", "false", "not-bound"),
                    self.row(7, "P7-DESCENDANT-01", "PROVED", "true", "4" * 64),
                ]
            )
        return root, ledger, corrupt, observed

    def row(
        self, phase: int, requirement: str, result: str, final: str, digest: str
    ) -> dict[str, str]:
        return {
            "phase": str(phase),
            "requirement": requirement,
            "retained_result_artifact": row_evidence(requirement).relative_path,
            "matched_test_count": "1" if result == "PROVED" else "0",
            "command_result": "passed" if result == "PROVED" else "not-run",
            "source_revision": "revision" if result == "PROVED" else "not-bound",
            "source_digest": "source" if result == "PROVED" else "not-bound",
            "source_state_digest": "state" if result == "PROVED" else "not-bound",
            "run_nonce": "nonce" if result == "PROVED" else "not-bound",
            "result": result,
            "reopen_lineage": "none",
            "final_source": final,
            "result_artifact_digest": digest,
        }

    def rows(self, ledger: Path) -> list[dict[str, str]]:
        with ledger.open(encoding="utf-8", newline="") as stream:
            return list(csv.DictReader(stream))


class ArtifactDriftInventoryTests(unittest.TestCase):
    def test_inventory_retains_exact_drifted_row_evidence(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            identity = row_evidence("P2-ROW-01")
            payload = {
                "schema_version": 5,
                "requirement": "P2-ROW-01",
                "source_state_digest": "state",
            }
            content = (json.dumps(payload) + "\n").encode("utf-8")
            destination = identity.destination(root)
            destination.parent.mkdir(parents=True)
            destination.write_bytes(content)
            ledger = root / "ledger.csv"
            with ledger.open("w", encoding="utf-8", newline="") as stream:
                writer = csv.DictWriter(
                    stream,
                    fieldnames=[
                        "phase",
                        "requirement",
                        "retained_result_artifact",
                        "result",
                        "result_artifact_digest",
                    ],
                    lineterminator="\n",
                )
                writer.writeheader()
                writer.writerow(
                    {
                        "phase": "2",
                        "requirement": "P2-ROW-01",
                        "retained_result_artifact": identity.relative_path,
                        "result": "PROVED",
                        "result_artifact_digest": "1" * 64,
                    }
                )
            result = capture_artifact_drift(
                root,
                ledger,
                DriftCaptureRequest(
                    "a" * 64,
                    "invalidation.json",
                    "b" * 64,
                    1,
                ),
            )
            inventory = json.loads(
                (root / str(result["inventory"])).read_text(encoding="utf-8")
            )
            retained = root / inventory["rows"][0]["retained_observation"]
            self.assertEqual(retained.read_bytes(), content)


if __name__ == "__main__":
    unittest.main()
