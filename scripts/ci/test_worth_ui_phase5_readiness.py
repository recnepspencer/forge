from __future__ import annotations

import sys
import unittest
from pathlib import Path


CI = Path(__file__).resolve().parent
if str(CI) not in sys.path:
    sys.path.insert(0, str(CI))

import close_worth_ui_3141_ledger as ledger_closer


class PhaseFiveReadinessTests(unittest.TestCase):
    def test_topology_readiness_cannot_close_feature_rows(self) -> None:
        configured = ledger_closer.phase_proofs(5)
        self.assertEqual(set(configured), {"P5-PREDECESSOR-01"})
        rows = [
            {"phase": "5", "requirement": requirement}
            for requirement in (
                "P5-PREDECESSOR-01",
                "P5-GLYPH-RASTER-01",
                "P5-CLOSE-01",
            )
        ]
        with self.assertRaisesRegex(RuntimeError, "proof mappings are incomplete"):
            ledger_closer.require_complete_phase_mapping(rows, 5, configured)


if __name__ == "__main__":
    unittest.main()
