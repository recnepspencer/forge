from __future__ import annotations

import subprocess
import sys
import unittest
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]


class LedgerCloserCliTests(unittest.TestCase):
    def test_help_is_read_only_and_does_not_enter_closure(self) -> None:
        ledger = ROOT / "_docs/worth-ui/milestone-3.14.1-proof-ledger.csv"
        before = ledger.read_bytes()
        completed = subprocess.run(
            [sys.executable, "scripts/ci/close_worth_ui_3141_ledger.py", "--help"],
            cwd=ROOT,
            capture_output=True,
            text=True,
            check=False,
        )
        self.assertEqual(completed.returncode, 0, completed.stderr)
        self.assertIn("--through-phase", completed.stdout)
        self.assertIn("--prepare-only", completed.stdout)
        self.assertEqual(ledger.read_bytes(), before)


if __name__ == "__main__":
    unittest.main()
