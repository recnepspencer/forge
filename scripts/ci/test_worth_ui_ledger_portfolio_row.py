from __future__ import annotations

import json
import sys
import tempfile
import unittest
from pathlib import Path
from unittest.mock import patch

CI = Path(__file__).resolve().parent
if str(CI) not in sys.path:
    sys.path.insert(0, str(CI))

import worth_ui_ledger_portfolio_row as portfolio_row
import worth_ui_ledger_shared_execution_lineage as shared_execution_lineage

from worth_ui_ledger_runner_authentication import authenticates


class PortfolioRowTests(unittest.TestCase):
    def test_mapping_persists_shared_lineage_before_signing(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            target = root / "target"
            target.mkdir()
            (root / "shared.json").write_text(
                json.dumps(
                    {
                        "source_revision": "a" * 40,
                        "source_digest": "c" * 64,
                        "run_nonce": "d" * 32,
                        "causal_reuse": {
                            "predecessor_source_state_digest": "e" * 64,
                        },
                        "execution_receipts": [
                            reference("main-discovery", "1" * 64),
                            reference("ignored-discovery", "2" * 64),
                            reference("main-test", "3" * 64),
                        ],
                    }
                ),
                encoding="utf-8",
            )
            artifact = root / "result.json"
            row = {
                "requirement": "P1-HEADLESS-COST-01",
                "exact_command": "python runner --artifact result.json",
                "source_identity": "source.rs",
                "production_entry": "owner::production",
                "independent_oracle": "oracle::independent",
            }
            payload: dict[str, object] = {
                "source_identity": ["source.rs"],
                "source_revision": "a" * 40,
                "source_state_digest": "b" * 64,
                "claim_digest": "c" * 64,
                "shared_main_artifact": "shared.json",
                "shared_main_artifact_digest": "f" * 64,
                "execution_receipts": [
                    reference("main-discovery", "1" * 64),
                    reference("ignored-discovery", "2" * 64),
                    reference("main-test", "3" * 64),
                ],
            }
            executor = portfolio_row.PortfolioRowExecutor(root, target)
            with (
                patch.object(
                    shared_execution_lineage,
                    "validate_causal_reuse",
                    return_value={"1" * 64, "2" * 64, "3" * 64},
                ),
                patch.object(
                    shared_execution_lineage,
                    "source_artifact_bindings",
                    return_value={"source.rs": "2" * 64},
                ),
            ):
                executor.bind_mapping(
                    payload, row, artifact, ["python", "runner", "--artifact", "result.json"]
                )
            persisted = json.loads(artifact.read_text(encoding="utf-8"))

        reuse = persisted["causal_reuse"]
        self.assertEqual(reuse["claim_digest"], "c" * 64)
        self.assertEqual(reuse["exact_command"], row["exact_command"])
        self.assertEqual(reuse["source_artifact_bindings"], {"source.rs": "2" * 64})
        self.assertEqual(reuse["predecessor_artifact_sha256"], "f" * 64)
        self.assertEqual(
            reuse["execution_observation_ids"], ["1" * 64, "2" * 64, "3" * 64]
        )
        unsigned = {
            key: value for key, value in persisted.items() if key != "runner_authentication"
        }
        self.assertTrue(authenticates(unsigned, persisted["runner_authentication"], root))


def reference(role: str, observation: str) -> dict[str, object]:
    return {
        "schema": "worth-ui-ledger-execution-reference-v1",
        "role": role,
        "execution_binding_key": "a" * 64,
        "observation_sha256": observation,
        "command_sha256": "b" * 64,
        "duration_ms": 1,
        "acquisition": "reused",
    }


if __name__ == "__main__":
    unittest.main()
