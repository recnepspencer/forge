from __future__ import annotations

import csv
import hashlib
import json
import tempfile
import unittest
from pathlib import Path
from unittest.mock import Mock, call, patch

import worth_ui_predecessor_causal_refresh as refresh
import worth_ui_predecessor_refresh_runtime as refresh_runtime
from worth_ui_ledger_artifact_identity import predecessor_handoff
from worth_ui_ledger_command import CLAIM_FIELDS
from worth_ui_predecessor_candidate import import_candidate_prefix


class PredecessorCausalRefreshTests(unittest.TestCase):
    def test_only_the_row_with_changed_causal_evidence_executes(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            (root / "first.json").write_text("{}", encoding="utf-8")
            (root / "second.json").write_text("{}", encoding="utf-8")
            (root / "ledger.csv").write_text("phase,requirement\n", encoding="utf-8")
            rows = [row("P1-FIRST-01", "first.json"), row("P1-SECOND-01", "second.json")]
            reused = {"requirement": "P1-FIRST-01"}
            executed = {"requirement": "P1-SECOND-01"}
            with (
                patch.object(refresh, "governed_rows", return_value=rows),
                patch.object(refresh, "proofs", return_value={}),
                patch.object(refresh.RowEvidenceCache, "restore", return_value=None),
                patch.object(
                    refresh,
                    "revalidate_row_payload",
                    side_effect=[reused, None],
                ),
                patch.object(refresh, "execute_row", return_value=executed) as execute,
                patch.object(
                    refresh,
                    "retain_current_artifact",
                    side_effect=lambda identity, payload: {
                        **payload, "artifact_sha256": "c" * 64
                    },
                ),
                patch.object(refresh, "write_candidate_ledger"),
                patch.object(refresh, "persist_observation_receipts") as persist,
                patch.object(refresh, "closure_tests", return_value=2),
            ):
                observations, reuse_count, execute_count, closure_count = refresh.current_observations(
                    root,
                    root / "ledger.csv",
                    2,
                    "a" * 40,
                    "b" * 64,
                )
            self.assertEqual(reuse_count, 1)
            self.assertEqual(execute_count, 1)
            self.assertEqual(closure_count, 2)
            self.assertEqual([item["requirement"] for item in observations], [
                "P1-FIRST-01",
                "P1-SECOND-01",
            ])
            execute.assert_called_once()
            self.assertEqual(
                persist.call_args_list,
                [
                    call(root, "b" * 64, [{
                        **reused, "artifact_sha256": "c" * 64,
                    }]),
                    call(root, "b" * 64, [executed]),
                ],
            )

    def test_shared_world_providers_precede_consumers_and_phase_close(self) -> None:
        rows = [
            {"phase": "1", "requirement": "P1-HEADLESS-COST-01"},
            {"phase": "1", "requirement": "P1-CLOSE-01"},
            {"phase": "1", "requirement": "P1-WORLDS-01"},
        ]
        self.assertEqual(
            [row["requirement"] for row in refresh.ordered_rows(rows)],
            ["P1-WORLDS-01", "P1-HEADLESS-COST-01", "P1-CLOSE-01"],
        )

    def test_nested_refresh_replaces_the_outer_candidate_observation(self) -> None:
        original = row("P1-CLOSE-01", "close.json")
        observations = {"P1-CLOSE-01": {"run_nonce": "a" * 32}}
        settled = {
            "P1-CLOSE-01": (original, observations["P1-CLOSE-01"]),
        }
        refreshed = {
            "requirement": "P1-CLOSE-01",
            "run_nonce": "b" * 32,
        }
        with patch.object(refresh, "proofs", return_value={}):
            refresh.import_refreshed_observations(
                {"P1-CLOSE-01": original}, [refreshed], observations, settled,
                lambda current: current,
            )
        self.assertIs(observations["P1-CLOSE-01"], refreshed)
        self.assertIs(settled["P1-CLOSE-01"][1], refreshed)

    def test_refresh_publishes_the_complete_prefix_into_a_temporary_candidate(self) -> None:
        fields = [
            "phase", "requirement", "matched_test_count", "source_revision",
            "source_digest", "source_state_digest", "run_nonce", "command_result",
            "result_artifact_digest", "result", "final_source",
        ]
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            candidate = root / "candidate.csv"
            with candidate.open("w", encoding="utf-8", newline="") as stream:
                writer = csv.DictWriter(stream, fieldnames=fields)
                writer.writeheader()
                writer.writerow({
                    **{field: "old" for field in fields},
                    "phase": "1", "requirement": "P1-CLOSE-01",
                })
            observation = {
                "requirement": "P1-CLOSE-01", "matched_test_count": 1,
                "source_revision": "a" * 40, "source_digest": "b" * 64,
                "source_state_digest": "c" * 64, "run_nonce": "d" * 32,
                "artifact_sha256": "e" * 64,
            }
            refresh.publish_refreshed_prefix(
                root, candidate, [observation], lambda current: current
            )
            with candidate.open(encoding="utf-8", newline="") as stream:
                current = next(csv.DictReader(stream))
        self.assertEqual(current["run_nonce"], "d" * 32)
        self.assertEqual(current["result_artifact_digest"], "e" * 64)

    def test_runner_refreshed_prefix_is_read_from_its_candidate_binding(self) -> None:
        fields = ["phase", "requirement", "retained_result_artifact", "run_nonce",
                  "result_artifact_digest"]
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            artifact = root / "row.json"
            artifact.write_text(json.dumps({
                "requirement": "P1-CLOSE-01", "run_nonce": "a" * 32,
            }), encoding="utf-8")
            digest = hashlib.sha256(artifact.read_bytes()).hexdigest()
            candidate = root / "candidate.csv"
            with candidate.open("w", encoding="utf-8", newline="") as stream:
                writer = csv.DictWriter(stream, fieldnames=fields)
                writer.writeheader()
                writer.writerow({
                    "phase": "1", "requirement": "P1-CLOSE-01",
                    "retained_result_artifact": "row.json", "run_nonce": "b" * 32,
                    "result_artifact_digest": "c" * 64,
                })
            observations = refresh.read_refreshed_prefix(root, candidate, 3)
        self.assertEqual(observations[0]["artifact_sha256"], digest)
        self.assertEqual(observations[0]["run_nonce"], "a" * 32)

    def test_outer_closer_imports_the_runner_refreshed_prefix(self) -> None:
        fields = ["phase", "requirement", "run_nonce"]
        rows = [
            {"phase": "1", "requirement": "P1-CLOSE-01", "run_nonce": "a" * 32},
            {"phase": "5", "requirement": "P5-PREDECESSOR-01", "run_nonce": "c" * 32},
        ]
        with tempfile.TemporaryDirectory() as directory:
            candidate = Path(directory) / "candidate.csv"
            with candidate.open("w", encoding="utf-8", newline="") as stream:
                writer = csv.DictWriter(stream, fieldnames=fields)
                writer.writeheader()
                writer.writerow({
                    "phase": "1", "requirement": "P1-CLOSE-01",
                    "run_nonce": "b" * 32,
                })
                writer.writerow(rows[1])
            imported = import_candidate_prefix(rows, candidate, 5)
        self.assertEqual(imported, {"P1-CLOSE-01"})
        self.assertEqual(rows[0]["run_nonce"], "b" * 32)

    def test_refresh_reports_reuse_and_execution_counts_in_handoff(self) -> None:
        with (
            patch.object(refresh, "source_revision", return_value="a" * 40),
            patch.object(refresh, "source_state_digest", return_value="b" * 64),
            patch.object(refresh, "ensure_compile_artifact") as ensure_compile,
            patch.object(
                refresh,
                "current_observations",
                return_value=([{"requirement": "P1-ROW-01"}], 1, 0, 2),
            ),
            patch.object(refresh, "persist_observation_receipts") as persist,
            patch.object(refresh, "publish_refreshed_prefix") as publish,
            patch.object(refresh, "from_path", return_value=object()),
            patch.object(refresh, "predecessor_artifact", return_value={}) as build,
            patch.object(refresh, "write_artifact") as write,
        ):
            refresh.refresh_handoff(
                Path("."), Path("ledger.csv"), 3, predecessor_handoff(3)
            )
        ensure_compile.assert_called_once_with(Path("."), "a" * 40, "b" * 64)
        persist.assert_called_once_with(
            Path("."), "b" * 64, [{"requirement": "P1-ROW-01"}]
        )
        build.assert_called_once()
        publish.assert_called_once()
        written = write.call_args.args[2]
        self.assertEqual(written["causal_reused_requirement_count"], 1)
        self.assertEqual(written["executed_requirement_count"], 0)

    def test_candidate_ledger_binds_each_settled_artifact_before_closure(self) -> None:
        fields = [
            "phase", "requirement", "matched_test_count", "source_revision",
            "source_digest", "source_state_digest", "run_nonce", "command_result",
            "result_artifact_digest", "result", "final_source",
        ]
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            source = root / "source.csv"
            candidate = root / "candidate.csv"
            with source.open("w", encoding="utf-8", newline="") as stream:
                writer = csv.DictWriter(stream, fieldnames=fields)
                writer.writeheader()
                writer.writerow({
                    **{field: "old" for field in fields},
                    "phase": "1", "requirement": "old",
                })
                writer.writerow({
                    **{field: "retained" for field in fields},
                    "phase": "2", "requirement": "P2-LATER",
                    "result": "PROVED", "final_source": "true",
                })
            prepared = {field: "old" for field in fields}
            prepared.update({"phase": "1", "requirement": "P1-ONE"})
            observation = {
                "matched_test_count": 1,
                "source_revision": "a" * 40,
                "source_digest": "b" * 64,
                "source_state_digest": "c" * 64,
                "run_nonce": "d" * 32,
                "artifact_sha256": "e" * 64,
            }
            refresh.write_candidate_ledger(
                source, candidate, {"old": (prepared, observation)}
            )
            with candidate.open(encoding="utf-8", newline="") as stream:
                current, downstream = list(csv.DictReader(stream))
        self.assertEqual(current["source_revision"], "a" * 40)
        self.assertEqual(current["result_artifact_digest"], "e" * 64)
        self.assertEqual(current["result"], "PROVED")
        self.assertEqual(current["final_source"], "true")
        self.assertEqual(downstream["result"], "OPEN")
        self.assertEqual(downstream["final_source"], "false")

    def test_current_compile_artifact_is_reused_without_execution(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            identity = root / refresh_runtime.COMPILE_ARTIFACT
            identity.parent.mkdir(parents=True)
            identity.write_text(json.dumps({
                "exit_posture": "passed",
                "source_revision": "a" * 40,
                "source_state_digest": "b" * 64,
            }), encoding="utf-8")
            with patch.object(refresh_runtime.subprocess, "run") as execute:
                refresh.ensure_compile_artifact(root, "a" * 40, "b" * 64)
            execute.assert_not_called()

    def test_current_artifact_precedes_the_reduced_handoff_projection(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            artifact = root / "row.json"
            artifact.write_text(
                json.dumps({"requirement": "P1-ROW-01", "schema_version": 5}),
                encoding="utf-8",
            )
            context = refresh.RefreshContext(
                root,
                root / "candidate.csv",
                "a" * 40,
                "b" * 64,
                Mock(),
                Mock(),
                {"P1-ROW-01": {"requirement": "P1-ROW-01"}},
            )
            context.row_cache.restore.return_value = None
            _, payload, _, _, retained = refresh.retained_payload_for_row(
                context,
                row("P1-ROW-01", "row.json"),
                "c" * 64,
            )
        self.assertEqual(payload["schema_version"], 5)
        self.assertNotIn("schema_version", retained)

    def test_root_refresh_preserves_claims_after_candidate_reopens_a_row(self) -> None:
        current = row("P3-PREDECESSOR-01", "row.json")
        current["result"] = "OPEN"
        current["final_source"] = "false"
        with patch.object(refresh, "prepare_claim") as prepare:
            prepared = refresh.prepared_row(current, refresh.RefreshMode.root(6))
        self.assertEqual(prepared, current)
        prepare.assert_not_called()

    def test_stale_compile_artifact_is_refreshed_once(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            with patch.object(refresh_runtime.subprocess, "run") as execute:
                refresh.ensure_compile_artifact(root, "a" * 40, "b" * 64)
            execute.assert_called_once()
            self.assertEqual(execute.call_args.kwargs, {"cwd": root, "check": True})


def row(requirement: str, artifact: str) -> dict[str, str]:
    return {
        **{field: "value" for field in CLAIM_FIELDS},
        "phase": "1",
        "requirement": requirement,
        "retained_result_artifact": artifact,
        "source_identity": "source.rs",
        "exact_command": f"python runner --source source.rs --artifact {artifact}",
        "production_entry": "owner::perform",
        "independent_oracle": "oracle::adjudicate",
    }


if __name__ == "__main__":
    unittest.main()
