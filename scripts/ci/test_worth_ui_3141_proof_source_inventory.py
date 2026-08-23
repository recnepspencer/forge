from __future__ import annotations

import tempfile
import unittest
from pathlib import Path
from unittest.mock import Mock

from worth_ui_3141_proof_plan import proofs, source_inventory
from worth_ui_ledger_row_cache import RowEvidenceCache, source_artifact_bindings
from worth_ui_ledger_row_execution import execute_or_restore


PRODUCER = (
    "workspaces/worth-ui/crates/worth-ui-runtime/src/mounting/presentation/"
    "work_producer.rs"
)
SUCCESSOR_ISSUE = (
    "workspaces/worth-ui/crates/worth-ui-runtime/src/mounting/presentation/"
    "work_producer/successor_issue.rs"
)


class ProofSourceInventoryTests(unittest.TestCase):
    def test_phase_three_inventory_binds_controls_and_extracted_successor_owner(self) -> None:
        for requirement, proof in proofs().items():
            if not requirement.startswith("P3-"):
                continue
            sources = source_inventory(proof)
            if proof.control is not None:
                self.assertIn(proof.control.source, sources, requirement)
            if PRODUCER in proof.sources:
                self.assertIn(SUCCESSOR_ISSUE, sources, requirement)

    def test_successor_owner_only_mutation_changes_the_execution_binding(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            for identity in (PRODUCER, SUCCESSOR_ISSUE):
                source = root / identity
                source.parent.mkdir(parents=True, exist_ok=True)
                source.write_text(identity, encoding="utf-8")
            command = f"runner --source {PRODUCER} --source {SUCCESSOR_ISSUE}"
            before = source_artifact_bindings(root, command, "P3-DELTA-SOURCE-01")
            (root / SUCCESSOR_ISSUE).write_text("mutated successor", encoding="utf-8")
            after = source_artifact_bindings(root, command, "P3-DELTA-SOURCE-01")
            self.assertEqual(before[PRODUCER], after[PRODUCER])
            self.assertNotEqual(before[SUCCESSOR_ISSUE], after[SUCCESSOR_ISSUE])

    def test_successor_owner_only_mutation_forces_fresh_row_execution(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            for identity in (PRODUCER, SUCCESSOR_ISSUE):
                source = root / identity
                source.parent.mkdir(parents=True, exist_ok=True)
                source.write_text(identity, encoding="utf-8")
            row = {
                "requirement": "P3-DELTA-SOURCE-01",
                "exact_command": (
                    f"runner --source {PRODUCER} --source {SUCCESSOR_ISSUE} "
                    "--artifact evidence.json"
                ),
            }
            cache = RowEvidenceCache(
                root, root / "cache" / "portfolio", b"ledger", "a" * 40, "b" * 64
            )
            retained_identity = cache.identity(
                cache.binding(row["requirement"], row["exact_command"], "claim")
            )
            restored = {"posture": "cached"}
            cache.restore = Mock(side_effect=lambda requirement, command, claim: (
                restored
                if cache.identity(cache.binding(requirement, command, claim))
                == retained_identity
                else None
            ))
            cache.retain = Mock()
            (root / SUCCESSOR_ISSUE).write_text("mutated successor", encoding="utf-8")
            execute = Mock(return_value={"posture": "fresh"})
            result = execute_or_restore(
                row, root / "candidate.csv", cache, "claim", execute
            )
            self.assertEqual(result, {"posture": "fresh"})
            execute.assert_called_once()


if __name__ == "__main__":
    unittest.main()
