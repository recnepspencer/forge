from __future__ import annotations

import json
import tempfile
import unittest
from contextlib import ExitStack
from pathlib import Path
from unittest.mock import patch

from runner.authority.run_identity import RuntimePaths
from runner.facade.runtime_inspection import (
    active_runs,
    archive_run,
    artifact_inventory,
    doctor_report,
    run_report,
)
from runner.facade.runtime_state import append_runtime_event, refresh_projection_for_run
from runner.generation import ScaffoldRequest, generate_scaffold


class RuntimeInspectionTests(unittest.TestCase):
    def test_operator_pause_is_unhealthy_paused_state(self) -> None:
        status = {
            "notification_delivery_failure": None,
            "telegram": {},
            "current": {"phase": 1, "turn": "plan"},
            "last_event": {"event_type": "operator_pause"},
            "stopped": False,
            "awaiting_operator": {"reason": "spent"},
        }
        from runner.facade.runtime_inspection import doctor_findings, classify_run_state

        self.assertEqual(classify_run_state(status), "paused")
        self.assertEqual(doctor_findings(status)[0]["code"], "awaiting_operator")

    def test_report_artifacts_doctor_and_archive_are_authority_first(self) -> None:
        with inspection_world() as world:
            run_id = world.start_run("inspection-active")
            world.write_unhealthy_telegram_poller()

            report = run_report(run_id)
            self.assertEqual(report["state"], "active")
            self.assertEqual(report["event_count"], 1)
            self.assertEqual(report["next_operator_action"], "restart or inspect Telegram poller")

            doctor = doctor_report(run_id)
            self.assertFalse(doctor["healthy"])
            self.assertEqual(doctor["findings"][0]["code"], "telegram_poller_unhealthy")

            artifacts = artifact_inventory(run_id)["artifacts"]
            retention_classes = {item["lane"]: item["retention_class"] for item in artifacts}
            self.assertEqual(retention_classes["events"], "authority")
            self.assertEqual(retention_classes["telegram"], "derived")
            self.assertEqual(retention_classes["archives"], "archive")

            archive = archive_run(run_id)
            archive_root = Path(archive["archive_root"])
            self.assertTrue((archive_root / "events.jsonl").exists())
            self.assertTrue((archive_root / "config.json").exists())
            self.assertTrue((archive_root / "projection.json").exists())
            self.assertTrue((archive_root / "report.json").exists())
            self.assertEqual(archive["pruned"], [])
            with self.assertRaises(ValueError):
                archive_run(run_id, prune_derived=True)

    def test_active_runs_lists_unfinished_runs_only(self) -> None:
        with inspection_world() as world:
            active_run_id = world.start_run("inspection-active")
            completed_run_id = world.start_run("inspection-complete")
            append_runtime_event(RuntimePaths(completed_run_id), "run_completed", payload={"reason": "done"})
            refresh_projection_for_run(completed_run_id)

            listed = active_runs()["active"]
            self.assertEqual([item["run_id"] for item in listed], [active_run_id])

    def test_archive_prune_removes_run_scoped_derived_state_only_after_completion(self) -> None:
        with inspection_world() as world:
            run_id = world.start_run("inspection-complete")
            paths = RuntimePaths(run_id)
            paths.log.parent.mkdir(parents=True, exist_ok=True)
            paths.log.write_text("log\n", encoding="utf-8")
            paths.notification_delivery.parent.mkdir(parents=True, exist_ok=True)
            paths.notification_delivery.write_text("{}\n", encoding="utf-8")
            paths.telegram_alerts.parent.mkdir(parents=True, exist_ok=True)
            paths.telegram_alerts.write_text("{}\n", encoding="utf-8")
            append_runtime_event(paths, "run_completed", payload={"reason": "done"})
            refresh_projection_for_run(run_id)

            archive = archive_run(run_id, prune_derived=True)
            self.assertTrue((Path(archive["archive_root"]) / "events.jsonl").exists())
            self.assertFalse(paths.projection.exists())
            self.assertFalse(paths.log.exists())
            self.assertFalse(paths.notification_delivery.exists())
            self.assertFalse(paths.telegram_alerts.exists())
            self.assertTrue(paths.events.exists())


class inspection_world:
    def __enter__(self):
        self.tempdir = tempfile.TemporaryDirectory()
        self.root = Path(self.tempdir.name)
        self.runtime_root = self.root / "runtime"
        self.project_root = self.root / "project"
        self.stack = ExitStack()
        self.patch_runtime_roots()
        self.project_root.mkdir(parents=True)
        (self.project_root / "spec.md").write_text("inspection spec", encoding="utf-8")
        self.scaffold = generate_scaffold(ScaffoldRequest("single_prompt", "inspection", self.project_root, "spec.md"))
        return self

    def __exit__(self, exc_type, exc, tb):
        self.stack.close()
        self.tempdir.cleanup()

    def patch_runtime_roots(self) -> None:
        for target in (
            "runner.authority.run_identity.runtime_paths.CANONICAL_RUNTIME_ROOT",
            "runner.authority.run_identity.CANONICAL_RUNTIME_ROOT",
            "runner.facade.runtime_inspection.CANONICAL_RUNTIME_ROOT",
        ):
            self.stack.enter_context(patch(target, self.runtime_root))

    def start_run(self, run_id: str) -> str:
        append_runtime_event(
            RuntimePaths(run_id),
            "run_started",
            payload={"config_path": str(self.scaffold.config_path.resolve())},
        )
        refresh_projection_for_run(run_id)
        return run_id

    def write_unhealthy_telegram_poller(self) -> None:
        path = self.runtime_root / "telegram" / "poller-health.json"
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(json.dumps({"healthy": False, "error": "poll failed"}) + "\n", encoding="utf-8")


if __name__ == "__main__":
    unittest.main()
