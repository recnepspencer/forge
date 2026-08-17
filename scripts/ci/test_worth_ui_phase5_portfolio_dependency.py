from __future__ import annotations

import csv
import hashlib
import json
import os
import sys
import tempfile
import unittest
from pathlib import Path
from types import SimpleNamespace
from unittest.mock import patch


CI = Path(__file__).resolve().parent
ROOT = CI.parents[1]
if str(CI) not in sys.path:
    sys.path.insert(0, str(CI))

from worth_ui_3141_proof_plan import prepare_claim, proofs
from worth_ui_3141_p5_case_contracts import hostile_cases, positive_cases
from worth_ui_3141_supporting_world import validate_phase5_atlas_dependency
from worth_ui_ledger_command import claim_digest
from worth_ui_ledger_operational_successors import stage_execution_claim
from worth_ui_ledger_phase_five_portfolio import PhaseFivePortfolioExecution
from worth_ui_ledger_runner_authentication import authentication_tag


ATLAS = "P5-ATLAS-01"
PINNING = "P5-ATLAS-PINNING-01"


class PhaseFivePortfolioDependencyTests(unittest.TestCase):
    def test_pinning_consumes_the_staged_authenticated_atlas_and_rejects_mutation(self) -> None:
        self._execute_portfolio(mutate_atlas=False)
        with self.assertRaisesRegex(ValueError, "artifact digest drifted"):
            self._execute_portfolio(mutate_atlas=True)

    def _execute_portfolio(self, mutate_atlas: bool) -> None:
        source = ROOT / "_docs/worth-ui/milestone-3.14.1-proof-ledger.csv"
        with source.open(encoding="utf-8", newline="") as stream:
            rows = [row for row in csv.DictReader(stream) if row["requirement"] in {ATLAS, PINNING}]
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            ledger = root / "ledger.csv"
            candidate = root / "candidate.csv"
            ledger.write_bytes(source.read_bytes())
            candidate.write_bytes(source.read_bytes())
            temporary = root / "temporary"
            temporary.mkdir()

            def rerun(row, artifact, compile_artifact, **values):
                del compile_artifact
                if row["requirement"] == ATLAS:
                    return self._write_atlas(root, candidate, row, artifact)
                supporting = values["supporting_world_artifact"]
                if mutate_atlas:
                    (root / supporting).write_text("{}\n", encoding="utf-8")
                test = SimpleNamespace(requirement=PINNING, sources=(supporting,))
                with patch.dict(os.environ, {
                    "WORTH_UI_MILESTONE_3141_LEDGER": str(candidate),
                    "WORTH_UI_SUPPORTING_WORLD_ARTIFACT": supporting,
                }):
                    validate_phase5_atlas_dependency(test, "revision", "state", root)
                artifact.write_text("{}\n", encoding="utf-8")
                return self._execution_payload(row, artifact, row["source_identity"].split(";"))

            PhaseFivePortfolioExecution(
                root, ledger, temporary, candidate, rows, {}, rerun, "compile.json", []
            ).execute()

    def _write_atlas(
        self, root: Path, candidate: Path, row: dict[str, str], artifact: Path
    ) -> dict[str, object]:
        identity = artifact.relative_to(root).as_posix()
        current = dict(row)
        prepare_claim(current, proofs()[ATLAS])
        command = current["exact_command"].split()
        command[command.index("--artifact") + 1] = identity
        stage_execution_claim(candidate, current, identity, command)
        sources = [command[index + 1] for index, word in enumerate(command) if word == "--source"]
        with patch.dict(os.environ, {"WORTH_UI_MILESTONE_3141_LEDGER": str(candidate)}):
            bound_claim = claim_digest(ATLAS)
        evidence = {
            "schema_version": 5,
            "requirement": ATLAS,
            "package": "worth-ui-host-native",
            "target_kind": "lib",
            "target_name": "lib",
            "test_name": proofs()[ATLAS].test_name,
            "matched_test_count": 1,
            "executed_test_count": 1,
            "passed_test_count": 1,
            "ignored_test_count": 0,
            "exit_posture": "passed",
            "source_revision": "revision",
            "source_digest": "sources",
            "source_state_digest": "state",
            "run_nonce": "nonce",
            "claim_digest": bound_claim,
            "source_identity": sources,
            "mapping_source_identity": sources,
            "source_rebindings": [],
            "structural_counter": "physical-signal-runtimes=1",
            "governed_cases": list(positive_cases(ATLAS) or ()),
            "hostile_control": {
                "mutation_cases": list(hostile_cases(ATLAS) or ()),
            },
        }
        evidence["runner_authentication"] = authentication_tag(evidence, root)
        artifact.write_text(json.dumps(evidence), encoding="utf-8")
        return self._execution_payload(row, artifact, sources)

    @staticmethod
    def _execution_payload(
        row: dict[str, str], artifact: Path, sources: list[str]
    ) -> dict[str, object]:
        return {
            "executed_exact_command": row["exact_command"],
            "source_identity": sources,
            "matched_test_count": 1,
            "source_revision": "revision",
            "source_digest": "sources",
            "source_state_digest": "state",
            "run_nonce": "nonce",
            "artifact_sha256": hashlib.sha256(artifact.read_bytes()).hexdigest(),
        }


if __name__ == "__main__":
    unittest.main()
