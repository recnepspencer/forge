from __future__ import annotations

import sys
import unittest
from pathlib import Path
from types import SimpleNamespace
from unittest.mock import patch

sys.path.insert(0, str(Path(__file__).resolve().parent))

import worth_ui_ledger_execution_binding as binding
from worth_ui_ledger_artifact_identity import requirement_phase
from worth_ui_ledger_row_cache import owned_artifact_identities


class RequirementPhaseTests(unittest.TestCase):
    def test_phase_nine_and_ten_never_alias(self) -> None:
        self.assertEqual(requirement_phase("P9-CLOSE-01"), 9)
        self.assertEqual(requirement_phase("P10-CLOSE-01"), 10)
        with patch.object(
            binding, "execution_input_digest", side_effect=lambda _, phase, __: str(phase)
        ) as digest:
            self.assertEqual(
                binding.ledger_claim_prefix_digest(Path("ledger"), "P9-CLOSE-01"),
                "9",
            )
            self.assertEqual(
                binding.ledger_claim_prefix_digest(Path("ledger"), "P10-CLOSE-01"),
                "10",
            )
        self.assertEqual([call.args[1] for call in digest.call_args_list], [9, 10])

    def test_phase_ten_predecessor_uses_phase_nine_basis_and_identity(self) -> None:
        basis = SimpleNamespace(candidate_prefix_digest="phase-nine")
        with patch.object(binding, "from_path", return_value=basis) as from_path:
            self.assertEqual(
                binding.ledger_claim_prefix_digest(
                    Path("ledger"), "P10-PREDECESSOR-01"
                ),
                "phase-nine",
            )
        from_path.assert_called_once_with(Path("ledger"), 9)
        command = (
            "python runner --source _docs/worth-ui/milestone-3.14.1-evidence/"
            "p10-predecessor-handoff.json --artifact result.json"
        )
        self.assertEqual(
            owned_artifact_identities("P10-PREDECESSOR-01", command)[1],
            "_docs/worth-ui/milestone-3.14.1-evidence/"
            "p10-predecessor-handoff.json",
        )


if __name__ == "__main__":
    unittest.main()
