import json
import sys
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))

from worth_ui_3141_phase4_case_contracts import hostile_cases, positive_cases
from worth_ui_ledger_observation import governed_case_observation, mutation_case_observation


REQUIREMENT = "P4-FONT-COLLECTION-01"


class PhaseFourCaseEvidenceTests(unittest.TestCase):
    def observation(self, prefix: str, cases: tuple[str, ...]) -> str:
        return prefix + json.dumps({REQUIREMENT: list(cases)}, separators=(",", ":"))

    def test_exact_positive_and_hostile_case_sets_are_observed(self) -> None:
        positive = positive_cases(REQUIREMENT)
        hostile = hostile_cases(REQUIREMENT)
        self.assertIsNotNone(positive)
        self.assertIsNotNone(hostile)
        self.assertEqual(
            governed_case_observation(
                self.observation("WORTH_UI_LEDGER_CASES=", positive), REQUIREMENT
            ),
            list(positive),
        )
        self.assertEqual(
            mutation_case_observation(
                self.observation("WORTH_UI_LEDGER_MUTATION_CASES=", hostile), REQUIREMENT
            ),
            list(hostile),
        )

    def test_case_deletion_substitution_reordering_or_duplication_is_rejected(self) -> None:
        for prefix, cases, parser in [
            ("WORTH_UI_LEDGER_CASES=", positive_cases(REQUIREMENT), governed_case_observation),
            (
                "WORTH_UI_LEDGER_MUTATION_CASES=",
                hostile_cases(REQUIREMENT),
                mutation_case_observation,
            ),
        ]:
            for mutant in [
                cases[:-1],
                ("cooperative-substitute",) + cases[1:],
                tuple(reversed(cases)),
                cases + (cases[-1],),
            ]:
                self.assertIsNone(parser(self.observation(prefix, mutant), REQUIREMENT))


if __name__ == "__main__":
    unittest.main()
