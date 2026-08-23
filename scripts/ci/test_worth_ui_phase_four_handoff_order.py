from __future__ import annotations

import csv
import io
import sys
import tempfile
import unittest
from pathlib import Path
from unittest.mock import patch

sys.path.insert(0, str(Path(__file__).resolve().parent))

import worth_ui_predecessor_causal_refresh as causal_refresh
from worth_ui_ledger_command import CLAIM_FIELDS


class PhaseFourHandoffOrderTests(unittest.TestCase):
    def test_handoff_follows_refreshed_prefix_publication(self) -> None:
        events = []
        observations = [{"requirement": "P3-FIXTURE", "exit_posture": "passed"}]
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            ledger = root / "ledger.csv"
            stream = io.StringIO(newline="")
            writer = csv.DictWriter(stream, fieldnames=CLAIM_FIELDS)
            writer.writeheader()
            writer.writerow(
                {
                    field: (
                        "3"
                        if field == "phase"
                        else "P3-FIXTURE"
                        if field == "requirement"
                        else "value"
                    )
                    for field in CLAIM_FIELDS
                }
            )
            ledger.write_text(stream.getvalue(), encoding="utf-8")
            with (
                patch.object(causal_refresh, "source_revision", return_value="revision"),
                patch.object(causal_refresh, "source_state_digest", return_value="state"),
                patch.object(causal_refresh, "ensure_compile_artifact"),
                patch.object(causal_refresh, "retained_observations", return_value={}),
                patch.object(
                    causal_refresh,
                    "current_observations",
                    side_effect=lambda *_args, **_kwargs: (
                        events.append("observations") or observations,
                        0,
                        1,
                        2,
                    ),
                ),
                patch.object(causal_refresh, "persist_observation_receipts"),
                patch.object(causal_refresh, "predecessor_artifact", return_value={}),
                patch.object(
                    causal_refresh,
                    "write_artifact",
                    side_effect=lambda *_args: events.append("handoff"),
                ),
                patch.object(
                    causal_refresh,
                    "publish_refreshed_prefix",
                    side_effect=lambda *_args: events.append("prefix"),
                ),
            ):
                self.assertEqual(
                    causal_refresh.refresh_handoff(root, ledger, 4), observations
                )
        self.assertEqual(events, ["observations", "prefix", "handoff"])


if __name__ == "__main__":
    unittest.main()
