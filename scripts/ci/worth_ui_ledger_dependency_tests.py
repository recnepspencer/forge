import csv
import hashlib
import json
import os
import tempfile
import unittest
from pathlib import Path
from unittest.mock import patch

import worth_ui_ledger_dependency as ledger_dependency
from worth_ui_ledger_command import claim_digest_for_row
from worth_ui_ledger_runner_authentication import authentication_tag
from worth_ui_ledger_settlement_lock_cases import LedgerSettlementLockCases


class LedgerDependencyTests(LedgerSettlementLockCases, unittest.TestCase):
    def test_shared_evidence_requires_a_final_producer_and_exact_artifact(self) -> None:
        source = Path("_docs/worth-ui/milestone-3.14.1-proof-ledger.csv")
        with source.open(encoding="utf-8", newline="") as stream:
            row = next(
                row for row in csv.DictReader(stream)
                if row["requirement"] == "P3-HEADLESS-COST-01"
            )
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            identity = "evidence/producer.json"
            destination = root / identity
            destination.parent.mkdir(parents=True)
            row["retained_result_artifact"] = identity
            row["result"] = "PROVED"
            row["final_source"] = "true"
            artifact = authenticated(root, {
                "claim_digest": claim_digest_for_row(row),
                "source_revision": row["source_revision"],
                "source_digest": row["source_digest"],
                "source_state_digest": row["source_state_digest"],
                "run_nonce": row["run_nonce"],
                "source_identity": row["source_identity"].split(";"),
            })
            destination.write_text(json.dumps(artifact), encoding="utf-8")
            row["result_artifact_digest"] = hashlib.sha256(destination.read_bytes()).hexdigest()
            ledger = root / "candidate.csv"
            with source.open(encoding="utf-8", newline="") as stream:
                fields = next(csv.reader(stream))
            self._write_ledger(ledger, fields, row)
            with patch.dict(os.environ, {"WORTH_UI_MILESTONE_3141_LEDGER": str(ledger)}):
                ledger_dependency.require_proved_artifact(
                    root, row["requirement"], identity, artifact
                )
                rebound = dict(artifact)
                canonical = artifact["source_identity"]
                rebound_source = (
                    root / "workspaces/worth-ui/target/"
                    "worth-ui-3141-verify-test/p3-delta-world.json"
                )
                rebound_source.parent.mkdir(parents=True)
                rebound_source.write_text("fresh governed world", encoding="utf-8")
                executed = list(canonical)
                slot = executed.index(
                    "_docs/worth-ui/milestone-3.14.1-evidence/"
                    "p3-delta-source-01.json"
                )
                executed[slot] = rebound_source.relative_to(root).as_posix()
                rebound["source_identity"] = executed
                rebound["mapping_source_identity"] = artifact["source_identity"]
                rebound["source_rebindings"] = [{
                    "canonical": canonical[slot],
                    "executed": executed[slot],
                    "sha256": hashlib.sha256(rebound_source.read_bytes()).hexdigest(),
                }]
                row["source_identity"] = ";".join(executed)
                rebound["claim_digest"] = claim_digest_for_row(row)
                rebound = authenticated(root, rebound)
                destination.write_text(json.dumps(rebound), encoding="utf-8")
                row["result_artifact_digest"] = hashlib.sha256(
                    destination.read_bytes()
                ).hexdigest()
                self._write_ledger(ledger, fields, row)
                ledger_dependency.require_proved_artifact(
                    root, row["requirement"], identity, rebound
                )
                bad_rebinding = dict(rebound)
                bad_rebinding["source_rebindings"] = [
                    {**rebound["source_rebindings"][0], "sha256": "0" * 64}
                ]
                bad_rebinding = authenticated(root, bad_rebinding)
                with self.assertRaisesRegex(ValueError, "drifted source rebindings"):
                    ledger_dependency.require_proved_artifact(
                        root, row["requirement"], identity, bad_rebinding
                    )
                bad_mapping = dict(rebound)
                bad_mapping["mapping_source_identity"] = ["substitute"]
                bad_mapping = authenticated(root, bad_mapping)
                with self.assertRaisesRegex(ValueError, "invalid executed sources"):
                    ledger_dependency.require_proved_artifact(
                        root, row["requirement"], identity, bad_mapping
                    )
                row["source_identity"] = ";".join(canonical)
                destination.write_text(json.dumps(artifact), encoding="utf-8")
                row["result_artifact_digest"] = hashlib.sha256(
                    destination.read_bytes()
                ).hexdigest()
                self._write_ledger(ledger, fields, row)
                for field in [
                    "claim_digest", "source_revision", "source_digest",
                    "source_state_digest", "run_nonce",
                ]:
                    mutant = dict(artifact)
                    mutant[field] = "substitute"
                    mutant = authenticated(root, mutant)
                    with self.assertRaisesRegex(ValueError, "drifted"):
                        ledger_dependency.require_proved_artifact(
                            root, row["requirement"], identity, mutant
                        )
                mutant = dict(artifact)
                mutant["source_identity"] = ["substitute"] * len(canonical)
                mutant["mapping_source_identity"] = canonical
                mutant["source_rebindings"] = []
                mutant = authenticated(root, mutant)
                with self.assertRaisesRegex(ValueError, "drifted sources"):
                    ledger_dependency.require_proved_artifact(
                        root, row["requirement"], identity, mutant
                    )
                row["result_artifact_digest"] = "substitute"
                self._write_ledger(ledger, fields, row)
                with self.assertRaisesRegex(ValueError, "artifact digest drifted"):
                    ledger_dependency.require_proved_artifact(
                        root, row["requirement"], identity, artifact
                    )
                row["result_artifact_digest"] = hashlib.sha256(
                    destination.read_bytes()
                ).hexdigest()
                row["result"] = "OPEN"
                self._write_ledger(ledger, fields, row)
                with self.assertRaisesRegex(ValueError, "not final-source proved"):
                    ledger_dependency.require_proved_artifact(
                        root, row["requirement"], identity, artifact
                    )

    @staticmethod
    def _write_ledger(path: Path, fields: list[str], row: dict[str, str]) -> None:
        with path.open("w", encoding="utf-8", newline="") as stream:
            writer = csv.DictWriter(stream, fieldnames=fields)
            writer.writeheader()
            writer.writerow(row)


def authenticated(root: Path, payload: dict[str, object]) -> dict[str, object]:
    result = {
        key: value for key, value in payload.items() if key != "runner_authentication"
    }
    result["runner_authentication"] = authentication_tag(result, root)
    return result
