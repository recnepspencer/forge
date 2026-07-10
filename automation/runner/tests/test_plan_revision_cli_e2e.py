from __future__ import annotations

import importlib.util
import json
import os
import subprocess
import sys
import tempfile
import unittest

LANGGRAPH_INSTALLED = importlib.util.find_spec("langgraph") is not None
from contextlib import ExitStack
from pathlib import Path
from unittest.mock import patch

from runner.authority.events import load_events
from runner.authority.run_identity import RuntimePaths
from runner.facade.plan_revision import adopt_plan_payload
from runner.facade.runtime_state import append_runtime_event, refresh_projection_for_run
from runner.generation import ScaffoldRequest, generate_scaffold
from runner.prompt_library.rendering.turn_preparation import prepare_prompt_turn


class PlanRevisionCliEndToEndTests(unittest.TestCase):
    def test_cli_diff_revise_override_external_and_fork_use_real_event_ledger(self) -> None:
        with cli_plan_world() as world:
            world.adopt_run("cli-run")
            revised = world.write_revised_config(lambda config: config["phases"].append(world.phase(2, "future work")))

            diff = world.run_plan_json("diff", "cli-run", "--config", str(revised))
            self.assertEqual(diff["revision_class"], "future_only")

            revise = world.run_plan_json("revise", "cli-run", "--config", str(revised), "--reason", "cli revision")
            self.assertEqual(revise["revision_class"], "future_only")

            world.run_plan("override-prompt", "cli-run", "--phase-key", "phase_1", "--assembly-id", "turns/default")
            world.run_plan(
                "mark-external",
                "cli-run",
                "--phase-key",
                "phase_1",
                "--agent",
                "manual-codex-thread",
                "--summary",
                "Manual agent completed phase 1.",
                "--evidence",
                "commit abc123",
            )
            fork = world.run_plan_json("fork", "cli-run", "--config", str(revised), "--new-run-id", "cli-fork")

            self.assertEqual(fork["parent_run_id"], "cli-run")
            self.assertEqual(
                [event["event_type"] for event in load_events(RuntimePaths("cli-run").events)],
                [
                    "run_started",
                    "plan_adopted",
                    "plan_revised",
                    "operator_prompt_override",
                    "external_phase_completed",
                ],
            )
            self.assertEqual(
                [event["event_type"] for event in load_events(RuntimePaths("cli-fork").events)],
                ["run_started", "plan_adopted", "run_forked"],
            )

    def test_cli_diff_still_works_after_bound_config_drift(self) -> None:
        with cli_plan_world() as world:
            world.adopt_run("drift-run")
            world.mutate_config(lambda config: config["phases"][0].update({"instructions": "drifted"}))
            revised = world.write_revised_config(lambda config: config["phases"].append(world.phase(2, "new future")))

            diff = world.run_plan_json("diff", "drift-run", "--config", str(revised))

            self.assertEqual(diff["revision_class"], "current_restart_required")
            self.assertEqual(diff["current_phase_key"], "phase_1")
            self.assertIn(
                {"kind": "modify_phase", "phase_key": "phase_1", "disposition": "current_restart_required"},
                diff["changes"],
            )

    def test_cli_resume_refuses_drift_before_graph_runtime_starts(self) -> None:
        with cli_plan_world() as world:
            world.adopt_run("resume-drift-run")
            world.mutate_config(lambda config: config["phases"][0].update({"instructions": "drifted"}))

            completed = world.run_cli("resume", "resume-drift-run", check=False)

            self.assertNotEqual(completed.returncode, 0)
            self.assertIn("config changed since plan_version=1", completed.stderr)
            self.assertNotIn("No module named 'langgraph'", completed.stderr)

    @unittest.skipIf(LANGGRAPH_INSTALLED, "asserts graceful failure when the langgraph runtime is absent")
    def test_cli_start_missing_graph_runtime_does_not_leave_phantom_plan(self) -> None:
        with cli_plan_world() as world:
            completed = world.run_cli("start", str(world.scaffold.config_path), "--run-id", "start-failed", check=False)

            self.assertNotEqual(completed.returncode, 0)
            self.assertIn("No module named 'langgraph'", completed.stderr)
            self.assertFalse(RuntimePaths("start-failed").events.exists())

    def test_prompt_override_changes_actual_rendered_prompt(self) -> None:
        with cli_plan_world() as world:
            world.write_custom_prompt_assembly("custom override prompt")
            world.adopt_run("render-run")
            world.run_plan(
                "override-prompt",
                "render-run",
                "--phase-key",
                "phase_1",
                "--assembly-id",
                "turns/custom",
                "--reason",
                "render custom prompt",
            )

            projection = refresh_projection_for_run("render-run")
            prepared = prepare_prompt_turn(
                world.config(),
                projection,
                world.scaffold.config_path,
                RuntimePaths("render-run").projection,
                RuntimePaths("render-run").events,
            )

            self.assertIn("custom override prompt", prepared.rendered_prompt)
            self.assertEqual(prepared.prompt_assembly_id, "turns/custom")

    def test_repeating_identical_revision_is_idempotent(self) -> None:
        with cli_plan_world() as world:
            world.adopt_run("retry-run")
            revised = world.write_revised_config(lambda config: config["phases"].append(world.phase(2, "future")))

            world.run_plan("revise", "retry-run", "--config", str(revised))
            retry = world.run_plan_json("revise", "retry-run", "--config", str(revised))

            self.assertEqual(retry["revision_class"], "no_change")
            plan_events = [
                event for event in load_events(RuntimePaths("retry-run").events)
                if event["event_type"] in {"plan_adopted", "plan_revised"}
            ]
            self.assertEqual([event["payload"]["plan_version"] for event in plan_events], [1, 2])

    def test_append_repairs_crash_truncated_tail_before_operator_command(self) -> None:
        with cli_plan_world() as world:
            world.adopt_run("truncated-run")
            paths = RuntimePaths("truncated-run")
            with paths.events.open("a", encoding="utf-8") as output:
                output.write('{"run_id":"truncated-run","sequence":3')

            world.run_plan(
                "override-prompt",
                "truncated-run",
                "--phase-key",
                "phase_1",
                "--assembly-id",
                "turns/default",
            )

            events = load_events(paths.events)
            self.assertEqual([event["sequence"] for event in events], [1, 2, 3])
            self.assertEqual(events[-1]["event_type"], "operator_prompt_override")
            self.assertTrue(paths.events.read_text(encoding="utf-8").endswith("\n"))

    def test_stale_or_malformed_operator_commands_leave_ledger_unchanged(self) -> None:
        with cli_plan_world() as world:
            world.adopt_run("operator-denial-run")
            paths = RuntimePaths("operator-denial-run")
            baseline = paths.events.read_bytes()

            bad_turn = world.run_cli(
                "plan", "override-prompt", "operator-denial-run",
                "--phase-key", "phase_1", "--turn", "invented-turn",
                "--assembly-id", "turns/default", check=False,
            )
            no_evidence = world.run_cli(
                "plan", "mark-external", "operator-denial-run",
                "--phase-key", "phase_1", "--agent", "manual",
                "--summary", "trust me", check=False,
            )

            self.assertNotEqual(bad_turn.returncode, 0)
            self.assertIn("does not support turn 'invented-turn'", bad_turn.stderr)
            self.assertNotEqual(no_evidence.returncode, 0)
            self.assertIn("requires at least one evidence item", no_evidence.stderr)
            self.assertEqual(paths.events.read_bytes(), baseline)

    def test_completed_phase_rejects_late_override_and_duplicate_completion(self) -> None:
        with cli_plan_world() as world:
            world.adopt_run("completed-run")
            world.run_plan(
                "mark-external", "completed-run", "--phase-key", "phase_1",
                "--agent", "manual", "--summary", "done", "--evidence", "commit abc",
            )
            paths = RuntimePaths("completed-run")
            baseline = paths.events.read_bytes()

            late_override = world.run_cli(
                "plan", "override-prompt", "completed-run", "--phase-key", "phase_1",
                "--assembly-id", "turns/default", check=False,
            )
            duplicate = world.run_cli(
                "plan", "mark-external", "completed-run", "--phase-key", "phase_1",
                "--agent", "other", "--summary", "also done", "--evidence", "commit def", check=False,
            )

            self.assertIn("already complete", late_override.stderr)
            self.assertIn("already complete", duplicate.stderr)
            self.assertEqual(paths.events.read_bytes(), baseline)


class cli_plan_world:
    def __enter__(self):
        self.tempdir = tempfile.TemporaryDirectory()
        self.root = Path(self.tempdir.name)
        self.runtime_root = self.root / "runtime"
        self.project_root = self.root / "project"
        self.stack = ExitStack()
        self.patch_runtime_roots()
        self.project_root.mkdir(parents=True)
        (self.project_root / "spec.md").write_text("plan spec", encoding="utf-8")
        self.scaffold = generate_scaffold(ScaffoldRequest("single_prompt", "plan", self.project_root, "spec.md"))
        return self

    def __exit__(self, exc_type, exc, tb):
        self.stack.close()
        self.tempdir.cleanup()

    def patch_runtime_roots(self) -> None:
        for target in (
            "runner.authority.run_identity.runtime_paths.CANONICAL_RUNTIME_ROOT",
            "runner.authority.run_identity.CANONICAL_RUNTIME_ROOT",
        ):
            self.stack.enter_context(patch(target, self.runtime_root))

    def cli_env(self) -> dict[str, str]:
        env = os.environ.copy()
        env["PYTHONPATH"] = str((Path.cwd() / "automation" / "runner" / "src").resolve())
        env["AUTOMATION_RUNNER_RUNTIME_ROOT"] = str(self.runtime_root.resolve())
        return env

    def run_plan(self, *args: str) -> subprocess.CompletedProcess[str]:
        return self.run_cli("plan", *args)

    def run_cli(self, *args: str, check: bool = True) -> subprocess.CompletedProcess[str]:
        command = [sys.executable, "-m", "runner.facade.cli", *args]
        return subprocess.run(
            command,
            cwd=Path.cwd(),
            env=self.cli_env(),
            text=True,
            capture_output=True,
            check=check,
        )

    def run_plan_json(self, *args: str) -> dict:
        completed = self.run_plan(*args)
        return json.loads(completed.stdout)

    def config(self) -> dict:
        config = json.loads(self.scaffold.config_path.read_text(encoding="utf-8"))
        config["_config_path"] = str(self.scaffold.config_path.resolve())
        return config

    def adopt_run(self, run_id: str) -> None:
        config = self.config()
        append_runtime_event(
            RuntimePaths(run_id),
            "run_started",
            payload={"config_path": str(self.scaffold.config_path.resolve())},
        )
        append_runtime_event(RuntimePaths(run_id), "plan_adopted", payload=adopt_plan_payload(self.scaffold.config_path, config))
        refresh_projection_for_run(run_id)

    def phase(self, phase_id: int, title: str) -> dict:
        phase = dict(self.config()["phases"][0])
        phase["id"] = phase_id
        phase["phase_key"] = f"phase_{phase_id}"
        phase["title"] = title
        return phase

    def mutate_config(self, mutator) -> None:
        config = json.loads(self.scaffold.config_path.read_text(encoding="utf-8"))
        mutator(config)
        self.scaffold.config_path.write_text(json.dumps(config, indent=2) + "\n", encoding="utf-8")

    def write_revised_config(self, mutator) -> Path:
        config = json.loads(self.scaffold.config_path.read_text(encoding="utf-8"))
        mutator(config)
        path = self.scaffold.config_path.with_name("plan-revised.json")
        path.write_text(json.dumps(config, indent=2) + "\n", encoding="utf-8")
        return path

    def write_custom_prompt_assembly(self, text: str) -> None:
        asset = self.project_root / "automation" / "project_prompts" / "assets" / "turns" / "custom.md"
        assembly = self.project_root / "automation" / "project_prompts" / "assemblies" / "turns" / "custom.json"
        asset.write_text(text + "\n", encoding="utf-8")
        assembly.write_text('{"kind":"assembly","parts":[{"asset_id":"turns/custom"}]}\n', encoding="utf-8")


if __name__ == "__main__":
    unittest.main()
