from __future__ import annotations

import unittest

from run_worth_ui_shared_ledger_control import shared_main_receipts


class SharedLedgerControlTests(unittest.TestCase):
    def test_dependent_row_does_not_inherit_the_producer_hostile_control(self) -> None:
        shared = {
            "execution_receipts": [
                {"role": "main-discovery", "key": "list"},
                {"role": "ignored-discovery", "key": "ignored"},
                {"role": "main-test", "key": "main"},
                {"role": "control-discovery", "key": "producer-control-list"},
                {"role": "control-test", "key": "producer-control"},
            ]
        }
        receipts = shared_main_receipts(shared)
        self.assertEqual(
            [receipt["role"] for receipt in receipts],
            ["main-discovery", "ignored-discovery", "main-test"],
        )


if __name__ == "__main__":
    unittest.main()
