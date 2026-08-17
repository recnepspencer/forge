from __future__ import annotations

import sys
import subprocess
import tempfile
import unittest
from pathlib import Path
from unittest import mock


CI = Path(__file__).resolve().parent
ROOT = CI.parents[1]
if str(CI) not in sys.path:
    sys.path.insert(0, str(CI))

import close_worth_ui_3141_ledger as ledger_closer
from worth_ui_ledger_command import GovernedTest, control_budget_ms, execution_budget_ms
from worth_ui_ledger_observation import observed_costs
import worth_ui_ledger_phase_five_portfolio as phase_five_portfolio
import worth_ui_ledger_verifier_rebinding as verifier_rebinding


class PhaseFiveReadinessTests(unittest.TestCase):
    def test_runtime_pin_control_owns_its_observed_budget(self) -> None:
        self.assertEqual(control_budget_ms("P5-ATLAS-PINNING-01"), 30_000)
        self.assertEqual(control_budget_ms("P5-COLOR-EMOJI-01"), 60_000)
        self.assertEqual(execution_budget_ms("P5-COLOR-EMOJI-01"), 180_000)

    def test_product_pin_world_reports_its_actual_process_and_presentation_cost(self) -> None:
        test = GovernedTest(
            "P5-ATLAS-PINNING-01",
            "worth-ui-platform-pulse",
            "test",
            "executable_world",
            ("executable-world",),
            "gate-d-pin-world",
            (),
            "artifact.json",
            None,
        )
        costs = observed_costs(
            test,
            subprocess.CompletedProcess([], 0, "", ""),
            {"executed_test_count": 1},
            {
                "schema": "worth-ui-native-gate-d-pin-world-v2",
                "mounted_bindings": 1,
                "pinned_layouts": 2,
                "presentations": 1,
                "atlas_transactions": 3,
            },
            {"requirement": "P5-ATLAS-01"},
            None,
        )
        self.assertEqual(
            costs,
            (
                "main-tests=1;hostile-controls=1;product-processes=1;"
                "compile-sessions=0;courtroom-worlds=1",
                "executed-tests=2;presentations=1;atlas-transactions=3",
            ),
        )

    def test_gate_d_mappings_do_not_make_later_feature_rows_closable(self) -> None:
        configured = ledger_closer.phase_proofs(5)
        self.assertEqual(
            set(configured),
            {
                "P5-PREDECESSOR-01",
                "P5-GLYPH-RASTER-01",
                "P5-COLOR-EMOJI-01",
                "P5-ATLAS-01",
                "P5-ATLAS-PINNING-01",
            },
        )
        rows = [
            {"phase": "5", "requirement": requirement}
            for requirement in (
                "P5-PREDECESSOR-01",
                "P5-GLYPH-RASTER-01",
                "P5-TEXT-DPI-01",
            )
        ]
        with self.assertRaisesRegex(RuntimeError, "proof mappings are incomplete"):
            ledger_closer.require_complete_phase_mapping(rows, 5, configured)

    def test_gate_d_sources_close_over_current_model_product_and_dependency_owners(self) -> None:
        configured = ledger_closer.phase_proofs(5)
        atlas_sources = configured["P5-ATLAS-01"].sources
        pin_sources = configured["P5-ATLAS-PINNING-01"].sources
        self.assertIn(
            "workspaces/worth-ui/crates/worth-ui-host-native/src/native/"
            "text_atlas/gate_d_model_evidence.rs",
            atlas_sources,
        )
        for source in [
            "workspaces/worth-ui/apps/platform-pulse/tests/executable_world/"
            "product_process/mod.rs",
            "workspaces/worth-ui/apps/platform-pulse/tests/executable_world/"
            "product_process/shutdown.rs",
            "workspaces/worth-ui/crates/worth-ui-runtime/src/native_platform/"
            "text_presentation/mounted_coordinator.rs",
            "workspaces/worth-ui/crates/worth-ui-runtime/src/native_platform/"
            "authorized_native_host.rs",
            "scripts/ci/worth_ui_ledger_verifier_rebinding.py",
        ]:
            self.assertIn(source, pin_sources)
        for sources in (atlas_sources, pin_sources):
            self.assertEqual(len(sources), len(set(sources)))
            for source in sources:
                if source == verifier_rebinding.P5_ATLAS_ARTIFACT:
                    continue
                self.assertTrue(
                    (ROOT / source).is_file(),
                    f"mapped Gate D source does not exist: {source}",
                )
        self.assertIn(verifier_rebinding.P5_ATLAS_ARTIFACT, pin_sources)
        self.assertFalse(any(
            "coordinator/gate_d_pin_evidence" in source for source in pin_sources
        ))

    def test_raster_sources_close_over_each_executed_production_owner(self) -> None:
        configured = ledger_closer.phase_proofs(5)
        glyph_sources = configured["P5-GLYPH-RASTER-01"].sources
        color_sources = configured["P5-COLOR-EMOJI-01"].sources
        for source in [
            "workspaces/worth-ui/crates/worth-ui-text/src/raster/color/bitmap.rs",
            "workspaces/worth-ui/crates/worth-ui-text/src/raster/color/completion.rs",
            "workspaces/worth-ui/crates/worth-ui-text/src/raster/color/pixels.rs",
        ]:
            self.assertIn(source, glyph_sources)
        for source in [
            "workspaces/worth-ui/crates/worth-ui-text/src/font_collection/"
            "application_color_graph_controls.rs",
            "workspaces/worth-ui/profiles/worth-ui-global-text-v2/unicode/emoji/"
            "emoji-test.txt",
        ]:
            self.assertIn(source, color_sources)
        for sources in (glyph_sources, color_sources):
            self.assertEqual(len(sources), len(set(sources)))
            for source in sources:
                self.assertTrue(
                    (ROOT / source).is_file(),
                    f"mapped raster source does not exist: {source}",
                )

    def test_gate_batches_close_atomically_without_claiming_phase_closure(self) -> None:
        selected = [{"requirement": "P5-ATLAS-01"}]
        with mock.patch.object(ledger_closer, "close_selected_atomically") as close:
            ledger_closer.close_phase_five([], [], selected)
        close.assert_called_once_with([], [], selected, verify_phase=None)

    def test_gate_g_close_runs_the_full_phase_verifier(self) -> None:
        selected = [{"requirement": "P5-CLOSE-01"}]
        with mock.patch.object(ledger_closer, "close_selected_atomically") as close:
            ledger_closer.close_phase_five([], [], selected)
        close.assert_called_once_with([], [], selected, verify_phase=5)

    def test_reopening_drops_stale_execution_truth_and_retains_lineage(self) -> None:
        row = {
            "result": "PROVED",
            "final_source": "true",
            "matched_test_count": "2",
            "command_result": "passed",
            "source_revision": "revision",
            "source_digest": "source",
            "source_state_digest": "state",
            "run_nonce": "nonce",
            "result_artifact_digest": "artifact-digest",
            "reopen_lineage": "none",
        }
        with mock.patch.object(ledger_closer, "prepare_claim"):
            ledger_closer.reopen_claim(row, mock.sentinel.proof)
        self.assertEqual(row["result"], "OPEN")
        self.assertEqual(row["final_source"], "false")
        self.assertEqual(row["command_result"], "not-run")
        self.assertEqual(row["matched_test_count"], "0")
        self.assertEqual(row["source_state_digest"], "not-bound")
        self.assertEqual(row["result_artifact_digest"], "not-bound")
        self.assertEqual(row["reopen_lineage"], "supersedes:artifact-digest")

    def test_pinning_executes_after_and_consumes_the_proved_atlas_artifact(self) -> None:
        calls: list[tuple[str, dict[str, object]]] = []

        def rerun(row, artifact, compile_artifact, **values):
            del artifact, compile_artifact
            calls.append((row["requirement"], values))
            return {"requirement": row["requirement"]}

        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            execution = phase_five_portfolio.PhaseFivePortfolioExecution(
                root=root,
                ledger=root / "ledger.csv",
                temporary=root / "temporary",
                candidate=root / "candidate.csv",
                rows=[
                    {"requirement": "P5-ATLAS-PINNING-01"},
                    {"requirement": "P5-ATLAS-01"},
                ],
                replacements={},
                rerun_row=rerun,
                compile_artifact="compile.json",
                observations=[],
            )
            execution.temporary.mkdir()
            with mock.patch.object(
                phase_five_portfolio, "record_proved_execution"
            ):
                execution.execute()

        self.assertEqual([requirement for requirement, _ in calls], [
            "P5-ATLAS-01",
            "P5-ATLAS-PINNING-01",
        ])
        self.assertNotIn("supporting_world_artifact", calls[0][1])
        self.assertEqual(
            calls[1][1]["supporting_world_artifact"],
            "temporary/p5-00.json",
        )

    def test_pinning_rebinds_its_canonical_atlas_source_to_the_staged_artifact(self) -> None:
        command = [
            "runner",
            "--source",
            verifier_rebinding.P5_ATLAS_ARTIFACT,
            "--artifact",
            "pin.json",
        ]
        rebound = verifier_rebinding.bind_fresh_supporting_world(
            command, "workspaces/worth-ui/target/worth-ui-3141-verify-test/atlas.json"
        )
        self.assertEqual(
            rebound[rebound.index("--source") + 1],
            "workspaces/worth-ui/target/worth-ui-3141-verify-test/atlas.json",
        )


if __name__ == "__main__":
    unittest.main()
