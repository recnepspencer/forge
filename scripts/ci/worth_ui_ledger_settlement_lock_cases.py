from __future__ import annotations

import subprocess
import sys
import tempfile
import time
from pathlib import Path


class LedgerSettlementLockCases:
    def test_ledger_settlement_lock_is_cross_process_exclusive(self) -> None:
        script = (
            "import pathlib,sys; from close_worth_ui_3141_ledger import ledger_lock; "
            "guard=ledger_lock(pathlib.Path(sys.argv[1])); guard.__enter__(); "
            "print('acquired',flush=True); sys.stdin.readline(); guard.__exit__(None,None,None)"
        )
        with tempfile.TemporaryDirectory() as directory:
            identity = str(Path(directory) / "ledger.lock")
            first = subprocess.Popen(
                [sys.executable, "-c", script, identity], cwd=Path(__file__).parent,
                stdin=subprocess.PIPE, stdout=subprocess.PIPE, text=True,
            )
            self.assertEqual(first.stdout.readline().strip(), "acquired")
            second = subprocess.Popen(
                [sys.executable, "-c", script, identity], cwd=Path(__file__).parent,
                stdin=subprocess.PIPE, stdout=subprocess.PIPE, text=True,
            )
            time.sleep(0.2)
            self.assertIsNone(second.poll())
            first.stdin.write("\n")
            first.stdin.flush()
            self.assertEqual(second.stdout.readline().strip(), "acquired")
            second.stdin.write("\n")
            second.stdin.flush()
            self.assertEqual(first.wait(timeout=5), 0)
            self.assertEqual(second.wait(timeout=5), 0)
            for stream in (first.stdin, first.stdout, second.stdin, second.stdout):
                stream.close()
