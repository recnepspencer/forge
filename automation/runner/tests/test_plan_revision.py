from __future__ import annotations

import copy
import json
import tempfile
import unittest
from contextlib import ExitStack
from pathlib import Path
from unittest.mock import patch

from runner.authority.events import load_events
from runner.authority.events.run_authority import load_admitted_run_projection_inputs
from runner.authority.run_identity import RuntimePaths
from runner.authority.config import load_config
from runner.authority.config.validator import validate_config
from runner.facade.plan_revision import (
    adopt_plan_payload,
    diff_plan,
    fork_plan,
    revise_plan,
)
from runner.facade.plan_operator_actions import record_external_completion, record_prompt_override
from runner.facade.runtime_state import append_runtime_event_if_plan_version
from runner.facade.runtime_state import append_runtime_event, refresh_projection_for_run
from runner.generation import ScaffoldRequest, generate_scaffold


class PlanRevisionTests(unittest.TestCase):
    def test_plan_adoption_refuses_silent_config_drift(self) -> None:
        with plan_world() as world:
            run_id = world.adopt_run()
            world.mutate_config(lambda config: config["phases"][0].update({"instructions": "changed"}))

            with self.assertRaisesRegex(ValueError, "config changed since plan_version=1"):
                load_admitted_run_projection_inputs(run_id)

    def test_future_revision_is_accepted_and_moves_config_authority(self) -> None:
        with plan_world() as world:
            run_id = world.adopt_run()
            revised = world.write_revised_config(lambda config: config["phases"].append(world.phase(2, "extra")))

            diff = diff_plan(run_id, revised)
            self.assertEqual(diff["revision_class"], "future_only")

            revise_plan(run_id, revised, allow_current_restart=False, reason="append future work")
            config_path, config, events = load_admitted_run_projection_inputs(run_id)
            self.assertEqual(config_path, revised.resolve())
            self.assertEqual(len(config["phases"]), 2)
            self.assertEqual(events[-1]["event_type"], "plan_revised")

    def test_insert_before_completed_history_requires_fork(self) -> None:
        with plan_world() as world:
            run_id = world.adopt_run()
            append_runtime_event(
                RuntimePaths(run_id),
                "single_prompt_completed",
                phase_id=1,
                turn="single_prompt",
                payload={"notes": {"done": ["done"]}, "summary": "done"},
            )
            revised = world.write_revised_config(lambda config: config["phases"].insert(0, world.phase(2, "inserted")))

            diff = diff_plan(run_id, revised)
            self.assertEqual(diff["revision_class"], "fork_required")
            with self.assertRaisesRegex(ValueError, "use plan fork"):
                revise_plan(run_id, revised, allow_current_restart=False, reason="middle insert")

    def test_insert_between_completed_phases_requires_fork(self) -> None:
        with plan_world() as world:
            run_id = world.adopt_run_with_phases(1, 2, 3, 4, 5)
            for phase_id in (1, 2, 3, 4):
                append_runtime_event(
                    RuntimePaths(run_id),
                    "single_prompt_completed",
                    phase_id=phase_id,
                    turn="single_prompt",
                    payload={"notes": {"done": [f"phase {phase_id} done"]}, "summary": "done"},
                )
            revised = world.write_revised_config(
                lambda config: config["phases"].insert(2, world.phase(6, "inserted middle"))
            )

            diff = diff_plan(run_id, revised)
            self.assertEqual(diff["revision_class"], "fork_required")
            self.assertIn(
                {"kind": "add_phase", "phase_key": "phase_6", "disposition": "fork_required"},
                diff["changes"],
            )

    def test_reordering_completed_phases_requires_fork(self) -> None:
        with plan_world() as world:
            run_id = world.adopt_run_with_phases(1, 2, 3)
            append_runtime_event(
                RuntimePaths(run_id),
                "single_prompt_completed",
                phase_id=1,
                turn="single_prompt",
                payload={"notes": {"done": ["done"]}, "summary": "done"},
            )
            revised = world.write_revised_config(lambda config: config["phases"].reverse())

            diff = diff_plan(run_id, revised)
            self.assertEqual(diff["revision_class"], "fork_required")
            self.assertIn(
                {"kind": "move_phase", "phase_key": "phase_1", "disposition": "fork_required"},
                diff["changes"],
            )

    def test_global_prompt_change_requires_current_restart_approval(self) -> None:
        with plan_world() as world:
            run_id = world.adopt_run()
            revised = world.write_revised_config(
                lambda config: config["prompt_library_policy"]["runner_asset_roots"].append("automation/project_prompts/custom")
            )

            diff = diff_plan(run_id, revised)
            self.assertEqual(diff["revision_class"], "current_restart_required")
            with self.assertRaisesRegex(ValueError, "active cursor"):
                revise_plan(run_id, revised, allow_current_restart=False, reason="global prompts")
            revise_plan(run_id, revised, allow_current_restart=True, reason="global prompts")

    def test_prompt_override_projects_and_external_completion_closes_phase(self) -> None:
        with plan_world() as world:
            run_id = world.adopt_run()
            record_prompt_override(
                run_id,
                "phase_1",
                {"assembly_id": "turns/default"},
                turn="single_prompt",
                reason="use custom phase prompt",
            )
            record_external_completion(
                run_id,
                "phase_1",
                "manual-codex-thread",
                "Manual agent completed the phase.",
                ["commit abc123", "tests passed"],
            )

            projection = refresh_projection_for_run(run_id)
            self.assertEqual(projection["prompt_overrides"][0]["reason"], "use custom phase prompt")
            self.assertEqual(projection["phases"][0]["status"], "complete")
            self.assertEqual(projection["phases"][0]["qa_status"], "passed")
            self.assertIn("commit abc123", projection["phases"][0]["notes"]["verification"])

    def test_prompt_override_and_external_completion_reject_unknown_phase_before_append(self) -> None:
        with plan_world() as world:
            run_id = world.adopt_run()

            with self.assertRaisesRegex(ValueError, "phase_key 'missing'"):
                record_prompt_override(
                    run_id,
                    "missing",
                    {"assembly_id": "turns/default"},
                    turn="single_prompt",
                    reason="bad phase",
                )
            with self.assertRaisesRegex(ValueError, "phase_key 'missing'"):
                record_external_completion(run_id, "missing", "manual", "done", [])

            event_types = [event["event_type"] for event in load_events(RuntimePaths(run_id).events)]
            self.assertNotIn("operator_prompt_override", event_types)
            self.assertNotIn("external_phase_completed", event_types)

    def test_authority_rejects_external_completion_without_real_evidence(self) -> None:
        with plan_world() as world:
            run_id = world.adopt_run()
            paths = RuntimePaths(run_id)
            baseline = paths.events.read_bytes()

            with self.assertRaisesRegex(ValueError, "non-empty list of non-empty strings"):
                append_runtime_event(
                    paths,
                    "external_phase_completed",
                    payload={"phase_key": "phase_1", "agent": "manual", "summary": "done", "evidence": []},
                )

            self.assertEqual(paths.events.read_bytes(), baseline)

    def test_prompt_override_rejects_unknown_prompt_before_append(self) -> None:
        with plan_world() as world:
            run_id = world.adopt_run()

            with self.assertRaises(ValueError):
                record_prompt_override(
                    run_id,
                    "phase_1",
                    {"assembly_id": "turns/does-not-exist"},
                    turn="single_prompt",
                    reason="bad prompt",
                )

            event_types = [event["event_type"] for event in load_events(RuntimePaths(run_id).events)]
            self.assertNotIn("operator_prompt_override", event_types)

    def test_fork_creates_new_lineage_without_mutating_parent(self) -> None:
        with plan_world() as world:
            parent = world.adopt_run()
            revised = world.write_revised_config(lambda config: config["phases"].append(world.phase(2, "fork work")))

            result = fork_plan(parent, revised, "forked-run", "revision touches completed history")
            self.assertEqual(result["run_id"], "forked-run")
            fork_events = load_events(RuntimePaths("forked-run").events)
            self.assertEqual([event["event_type"] for event in fork_events], ["run_started", "plan_adopted", "run_forked"])
            self.assertEqual(fork_events[-1]["payload"]["parent_run_id"], parent)

    def test_fork_can_resume_at_validated_parent_cursor_without_copying_history(self) -> None:
        with plan_world() as world:
            parent = world.adopt_run()
            phase_key = "phase_1"

            result = fork_plan(
                parent,
                world.scaffold.config_path,
                "resumed-fork",
                "clean recovery lineage",
                resume_phase_key=phase_key,
                resume_turn="single_prompt",
            )

            self.assertEqual(
                result["resume_cursor"],
                {"phase_key": phase_key, "turn": "single_prompt"},
            )
            projection = refresh_projection_for_run("resumed-fork")
            self.assertEqual(projection["current"], {"phase": 1, "turn": "single_prompt"})
            self.assertEqual(
                [event["event_type"] for event in load_events(RuntimePaths("resumed-fork").events)],
                ["run_started", "plan_adopted", "run_forked"],
            )

    def test_fork_resume_requires_phase_and_turn_together(self) -> None:
        with plan_world() as world:
            parent = world.adopt_run()

            with self.assertRaisesRegex(ValueError, "requires both"):
                fork_plan(
                    parent,
                    world.scaffold.config_path,
                    "invalid-resume-fork",
                    "missing turn",
                    resume_phase_key="phase_1",
                )

    def test_fork_rejects_missing_parent(self) -> None:
        with plan_world() as world:
            revised = world.write_revised_config(lambda config: config["phases"].append(world.phase(2, "fork work")))

            with self.assertRaisesRegex(ValueError, "parent run 'missing-parent' does not exist"):
                fork_plan("missing-parent", revised, "forked-run", "bad parent")

    def test_fork_initialization_failure_leaves_no_partial_child_and_can_retry(self) -> None:
        with plan_world() as world:
            parent = world.adopt_run()
            revised = world.write_revised_config(lambda config: config["phases"].append(world.phase(2, "fork work")))
            child_paths = RuntimePaths("atomic-child")

            with patch("runner.authority.events.event_log.os.replace", side_effect=OSError("injected crash")):
                with self.assertRaisesRegex(OSError, "injected crash"):
                    fork_plan(parent, revised, "atomic-child", "crash pressure")

            self.assertFalse(child_paths.events.exists())
            fork_plan(parent, revised, "atomic-child", "retry after crash")
            self.assertEqual(
                [event["event_type"] for event in load_events(child_paths.events)],
                ["run_started", "plan_adopted", "run_forked"],
            )

    def test_resumed_fork_accepts_revised_config_at_parent_path(self) -> None:
        with plan_world() as world:
            parent = world.adopt_run_with_phases(1, 2)
            world.mutate_config(lambda config: config["session_defaults"].update(model="revised-model"))

            result = fork_plan(
                parent,
                world.scaffold.config_path,
                "same-path-child",
                "provider revision",
                resume_phase_key="phase_2",
                resume_turn="single_prompt",
            )

            self.assertEqual(result["resume_cursor"], {"phase_key": "phase_2", "turn": "single_prompt"})

    def test_config_rejects_duplicate_phase_keys(self) -> None:
        with plan_world() as world:
            config_path = world.write_revised_config(
                lambda config: config["phases"].append({**world.phase(2, "duplicate key"), "phase_key": "phase_1"})
            )

            errors = validate_config(load_config(config_path), config_path)
            self.assertTrue(any("duplicates phase key 'phase_1'" in error for error in errors))

    def test_stale_revision_compare_and_append_cannot_create_duplicate_version(self) -> None:
        with plan_world() as world:
            run_id = world.adopt_run()
            revised = world.write_revised_config(lambda config: config["phases"].append(world.phase(2, "winner")))
            stale_diff = diff_plan(run_id, revised)
            revise_plan(run_id, revised, allow_current_restart=False, reason="winner")
            payload = adopt_plan_payload(revised, load_config(revised), stale_diff["to_plan_version"])
            payload.update(
                from_plan_version=stale_diff["from_plan_version"],
                revision_class=stale_diff["revision_class"],
                changes=stale_diff["changes"],
                reason="stale contender",
            )

            with self.assertRaisesRegex(ValueError, "stale plan revision"):
                append_runtime_event_if_plan_version(
                    RuntimePaths(run_id),
                    "plan_revised",
                    payload,
                    stale_diff["from_plan_version"],
                )

            versions = [
                event["payload"]["plan_version"]
                for event in load_events(RuntimePaths(run_id).events)
                if event["event_type"] in {"plan_adopted", "plan_revised"}
            ]
            self.assertEqual(versions, [1, 2])


class plan_world:
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

    def adopt_run(self, run_id: str = "plan-run") -> str:
        config = json.loads(self.scaffold.config_path.read_text(encoding="utf-8"))
        config["_config_path"] = str(self.scaffold.config_path.resolve())
        append_runtime_event(
            RuntimePaths(run_id),
            "run_started",
            payload={"config_path": str(self.scaffold.config_path.resolve())},
        )
        append_runtime_event(RuntimePaths(run_id), "plan_adopted", payload=adopt_plan_payload(self.scaffold.config_path, config))
        refresh_projection_for_run(run_id)
        return run_id

    def adopt_run_with_phases(self, *phase_ids: int, run_id: str = "plan-run") -> str:
        phases = [self.phase(phase_id, f"phase {phase_id}") for phase_id in phase_ids]
        config = json.loads(self.scaffold.config_path.read_text(encoding="utf-8"))
        config["phases"] = phases
        self.scaffold.config_path.write_text(json.dumps(config, indent=2) + "\n", encoding="utf-8")
        return self.adopt_run(run_id)

    def phase(self, phase_id: int, title: str) -> dict:
        config = json.loads(self.scaffold.config_path.read_text(encoding="utf-8"))
        phase = copy.deepcopy(config["phases"][0])
        phase["id"] = phase_id
        phase["title"] = title
        phase["phase_key"] = f"phase_{phase_id}"
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


if __name__ == "__main__":
    unittest.main()
