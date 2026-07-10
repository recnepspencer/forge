from __future__ import annotations

import json
import sys
import tempfile
import textwrap
import unittest
from contextlib import ExitStack
from pathlib import Path
from unittest.mock import patch

from runner.authority.config import load_config, validate_config
from runner.authority.events import load_events
from runner.authority.run_identity import RuntimePaths
from runner.facade.runtime_state import append_runtime_event, refresh_projection_for_run
from runner.facade.commands.status import status_view
from runner.graph_runtime.orchestrator import drive_graph_run
from runner.generation.scaffold_writer import operator_injection_overlay_prompt
from runner.telegram_bridge.cli import write_poller_health
from runner.telegram_bridge.routing import record_alert, route_reply, telegram_receipts_path


CHAT_ID = "9001"
RUN_ONE = "telegram-e2e-one"
RUN_TWO = "telegram-e2e-two"


class TelegramOperatorE2ETests(unittest.TestCase):
    def test_reply_is_routed_to_active_run_and_consumed_by_next_provider_prompt(self) -> None:
        with e2e_world() as world:
            run_id = world.start_run(RUN_ONE)
            record_alert(signal_payload(run_id, "sig-one"), telegram_message_id=101, chat_id=CHAT_ID)

            disposition = route_reply(reply_update(101, "Use the consumer prompt root only.", 501), CHAT_ID)
            self.assertEqual(disposition.status, "injected")

            result_code = drive_graph_run(world.config_path, run_id, loop=False, sleep_seconds=0, log_path=None)
            self.assertEqual(result_code, 0)
            observed_prompt = world.provider_observed_prompt(run_id)
            self.assertIn("Use the consumer prompt root only.", observed_prompt)
            self.assertEqual(operator_override_sources(run_id), ["telegram:501"])
            self.assertEqual(latest_receipt_statuses(), ["injected"])

    def test_duplicate_stale_and_cross_run_replies_do_not_create_false_injections(self) -> None:
        with e2e_world() as world:
            first_run_id = world.start_run(RUN_ONE)
            second_run_id = world.start_run(RUN_TWO)
            record_alert(signal_payload(first_run_id, "sig-one"), telegram_message_id=201, chat_id=CHAT_ID)
            record_alert(signal_payload(second_run_id, "sig-two"), telegram_message_id=202, chat_id=CHAT_ID)

            first = route_reply(reply_update(201, "instruction for first run", 601), CHAT_ID)
            duplicate = route_reply(reply_update(201, "different duplicate text", 601), CHAT_ID)
            second = route_reply(reply_update(202, "instruction for second run", 602), CHAT_ID)
            self.assertEqual((first.status, duplicate.status, second.status), ("injected", "duplicate_ignored", "injected"))
            self.assertEqual(operator_override_reasons(first_run_id), ["instruction for first run"])
            self.assertEqual(operator_override_reasons(second_run_id), ["instruction for second run"])

            self.assertEqual(drive_graph_run(world.config_path, first_run_id, False, 0, None), 0)
            stale = route_reply(reply_update(201, "too late", 603), CHAT_ID)
            unmapped = route_reply(reply_update(999, "wrong message", 604), CHAT_ID)
            self.assertEqual(stale.status, "rejected_stale")
            self.assertEqual(unmapped.status, "rejected_unmapped")
            self.assertEqual(operator_override_reasons(first_run_id), ["instruction for first run"])

    def test_poller_receipts_and_health_are_derived_bridge_state(self) -> None:
        with e2e_world() as world:
            run_id = world.start_run(RUN_ONE)
            self.assertEqual(route_reply(reply_update(303, "not mapped", 701), CHAT_ID).status, "rejected_unmapped")
            write_poller_health(False, "telegram getUpdates failed")
            self.assertEqual(telegram_receipts_path(), world.runtime_root / "telegram" / "inbound-receipts.jsonl")
            self.assertEqual(latest_receipt_statuses(), ["rejected_unmapped"])
            telegram = status_view(refresh_projection_for_run(run_id))["telegram"]
            self.assertEqual(telegram["poller_health"]["error"], "telegram getUpdates failed")
            self.assertEqual(telegram["latest_inbound_receipt"]["status"], "rejected_unmapped")


class e2e_world:
    def __enter__(self):
        self.tempdir = tempfile.TemporaryDirectory()
        self.root = Path(self.tempdir.name)
        self.runtime_root = self.root / "runtime"
        self.project_root = self.root / "project"
        self.config_path = self.project_root / "automation" / "runner" / "config" / "telegram-e2e.json"
        self.observed_root = self.root / "observed"
        self.create_project_files()
        self.stack = ExitStack()
        self.patch_runtime_roots()
        return self

    def __exit__(self, exc_type, exc, tb):
        self.stack.close()
        self.tempdir.cleanup()

    def patch_runtime_roots(self) -> None:
        patch_targets = (
            "runner.authority.run_identity.runtime_paths.CANONICAL_RUNTIME_ROOT",
            "runner.authority.run_identity.CANONICAL_RUNTIME_ROOT",
            "runner.telegram_bridge.routing.CANONICAL_RUNTIME_ROOT",
        )
        for target in patch_targets:
            self.stack.enter_context(patch(target, self.runtime_root))

    def create_project_files(self) -> None:
        prompt_root = self.project_root / "automation" / "project_prompts"
        (prompt_root / "assets" / "contracts").mkdir(parents=True)
        (prompt_root / "assets" / "recovery").mkdir(parents=True)
        (prompt_root / "assets" / "turns").mkdir(parents=True)
        (prompt_root / "assemblies" / "turns").mkdir(parents=True)
        self.config_path.parent.mkdir(parents=True)
        self.observed_root.mkdir(parents=True)
        (self.project_root / "spec.md").write_text("Telegram E2E spec", encoding="utf-8")
        (prompt_root / "assets" / "contracts" / "default.md").write_text("Emit a valid RUNNER_EVENT.", encoding="utf-8")
        (prompt_root / "assets" / "recovery" / "operator_injection_overlay.md").write_text(
            operator_injection_overlay_prompt(),
            encoding="utf-8",
        )
        (prompt_root / "assets" / "turns" / "default.md").write_text(
            "Run ID: {run_id}\nWork on {phase.title}.\n{contract}",
            encoding="utf-8",
        )
        (prompt_root / "assemblies" / "turns" / "default.json").write_text(
            '{"kind":"assembly","parts":[{"asset_id":"turns/default"}]}\n',
            encoding="utf-8",
        )
        self.provider_script = self.root / "fake_grok_provider.py"
        self.provider_script.write_text(fake_provider_source(), encoding="utf-8")
        self.config_path.write_text(json.dumps(self.config_payload(), indent=2), encoding="utf-8")
        errors = validate_config(load_config(self.config_path), self.config_path)
        if errors:
            raise AssertionError(errors)

    def config_payload(self) -> dict:
        return {
            "schema_version": 1,
            "project": {"name": "telegram-e2e", "cwd": str(self.project_root), "spec_file": "spec.md", "context_files": ["spec.md"]},
            "prompt_library_policy": {
                "runner_asset_roots": ["automation/project_prompts/assets"],
                "runner_assembly_roots": ["automation/project_prompts/assemblies"],
                "consumer_asset_roots": ["automation/consumer_prompts/assets"],
                "consumer_assembly_roots": ["automation/consumer_prompts/assemblies"],
                "allow_consumer_prompts": True,
                "allow_direct_file_binding": False,
            },
            "turn_templates": {"single_prompt": {"assembly_id": "turns/default"}},
            "contract_template": {"asset_id": "contracts/default"},
            "session_defaults": {
                "provider": "grok",
                "command": sys.executable,
                "command_args": [str(self.provider_script)],
                "model": "test",
                "reasoning_effort": "low",
                "config": {},
                "env": {"TELEGRAM_E2E_OBSERVED_DIR": str(self.observed_root)},
            },
            "loop_escalation": {
                "families": {
                    "review_family": {"turns": ["single_prompt"], "threshold": 4, "action": "start_fresh_session"}
                }
            },
            "escalation_policy": {"provider_crash": {"stages": [], "on_exhausted": "notify"}},
            "outcome_repair_policy": {
                "missing_runner_event": {"max_attempts": 1, "first_attempt": "same_agent_event_repair_prompt", "on_exhausted": "route_to_recovery"},
                "malformed_runner_event": {"max_attempts": 1, "first_attempt": "same_agent_event_repair_prompt", "on_exhausted": "route_to_recovery"},
            },
            "operator_intervention_policy": {
                "allow_live_injection": True,
                "allow_immediate_interrupt": False,
                "default_injection_mode": "next_turn_preface",
                "default_post_injection_route": "continue_current_phase",
                "record_as_authority_event": True,
            },
            "runner_control": {"turn_timeout_seconds": 30, "idle_timeout_seconds": 30},
            "phases": [single_prompt_phase()],
        }

    def start_run(self, run_id: str) -> str:
        append_runtime_event(RuntimePaths(run_id), "run_started", payload={"config_path": str(self.config_path.resolve())})
        refresh_projection_for_run(run_id)
        return run_id

    def provider_observed_prompt(self, run_id: str) -> str:
        return (self.observed_root / f"{run_id}.prompt.txt").read_text(encoding="utf-8")


def fake_provider_source() -> str:
    return textwrap.dedent(
        """
        import argparse
        import json
        import os
        import re
        from pathlib import Path

        parser = argparse.ArgumentParser()
        parser.add_argument("--prompt-file")
        args, _ = parser.parse_known_args()
        prompt = Path(args.prompt_file).read_text(encoding="utf-8")
        turn_id = re.search(r'Runner turn instance id: ([^\\n]+)', prompt).group(1)
        run_id = re.search(r'Run ID: ([^\\n]+)', prompt).group(1)
        observed = Path(os.environ["TELEGRAM_E2E_OBSERVED_DIR"]) / f"{run_id}.prompt.txt"
        observed.write_text(prompt, encoding="utf-8")
        event = {"event_type": "single_prompt_completed", "payload": {"summary": "done", "turn_instance_id": turn_id, "notes": {"done": ["provider consumed prompt"]}}}
        print(json.dumps({"type": "session.created", "session_id": "fake-thread"}), flush=True)
        print(json.dumps({"type": "assistant.message", "text": "RUNNER_EVENT: " + json.dumps(event, separators=(",", ":"))}), flush=True)
        """
    ).strip()


def single_prompt_phase() -> dict:
    return {
        "id": 1,
        "title": "Telegram E2E",
        "owner": "runner",
        "scope": ["."],
        "acceptance": ["reply reaches prompt"],
        "instructions": "prove telegram live injection",
        "qa_focus": "routing integrity",
        "program_id": "single_prompt",
        "prompt_template": {"asset_id": "turns/default"},
        "contract_template": {"asset_id": "contracts/default"},
        "success_event_type": "single_prompt_completed",
        "role_bindings": {
            "single_prompt": {
                "role_id": "implementer",
                "model_policy": {"provider": "grok", "model": "test", "reasoning_effort": "low"},
                "session_policy": {"continuity_family": "default"},
                "prompt_template": {"assembly_id": "turns/default"},
            }
        },
    }


def signal_payload(run_id: str, signal_id: str) -> dict:
    return {"signal_id": signal_id, "signal_kind": "blocker", "run_id": run_id, "phase_id": 1, "turn": "single_prompt"}


def reply_update(message_id: int, text: str, update_id: int) -> dict:
    return {
        "update_id": update_id,
        "message": {"chat": {"id": CHAT_ID}, "text": text, "reply_to_message": {"message_id": message_id}},
    }


def operator_override_sources(run_id: str) -> list[str]:
    return [event["payload"].get("source_id") for event in operator_override_events(run_id)]


def operator_override_reasons(run_id: str) -> list[str]:
    return [event["payload"]["reason"] for event in operator_override_events(run_id)]


def operator_override_events(run_id: str) -> list[dict]:
    return [event for event in load_events(RuntimePaths(run_id).events) if event["event_type"] == "operator_override"]


def latest_receipt_statuses() -> list[str]:
    return [json.loads(line)["status"] for line in telegram_receipts_path().read_text(encoding="utf-8").splitlines()]


if __name__ == "__main__":
    unittest.main()
