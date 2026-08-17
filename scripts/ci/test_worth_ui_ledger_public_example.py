from __future__ import annotations

import unittest
from types import SimpleNamespace

from worth_ui_ledger_public_example import execute_if_required


class PublicExampleEvidenceTests(unittest.TestCase):
    def test_only_font_collection_executes_the_public_example_once(self) -> None:
        calls = []

        def execute(command: list[str], role: str):
            calls.append((command, role))
            return SimpleNamespace(returncode=0), 17

        self.assertIsNone(execute_if_required("P4-BIDI-01", execute))
        evidence = execute_if_required("P4-FONT-COLLECTION-01", execute)
        self.assertEqual(len(calls), 1)
        self.assertEqual(calls[0][1], "public-example")
        self.assertEqual(evidence["exit_code"], 0)
        self.assertEqual(evidence["command"][-1], "text_platform")


if __name__ == "__main__":
    unittest.main()
