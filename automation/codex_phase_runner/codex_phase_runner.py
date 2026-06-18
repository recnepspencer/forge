#!/usr/bin/env python3
"""Run a JSON-backed phase plan through a persistent Codex CLI session."""

from __future__ import annotations

import argparse
import datetime as dt
import json
import subprocess
import sys
import time
from pathlib import Path
from typing import Any


TERMINAL_PHASE_STATUSES = {"blocked"}
WORK_STATUSES = {"not_started", "in_progress", "regressed"}
MAX_HISTORY_ENTRIES = 80


def load_state(path: Path) -> dict[str, Any]:
    with path.open("r", encoding="utf-8-sig") as state_file:
        return json.load(state_file)


def save_state(path: Path, state: dict[str, Any]) -> None:
    trim_history(state)
    path.write_text(json.dumps(state, indent=2) + "\n", encoding="utf-8")


def trim_history(state: dict[str, Any]) -> None:
    history = state.get("history")
    if isinstance(history, list) and len(history) > MAX_HISTORY_ENTRIES:
        state["history"] = history[-MAX_HISTORY_ENTRIES:]


def now_iso() -> str:
    return dt.datetime.now(dt.timezone.utc).isoformat()


def phase_by_id(state: dict[str, Any], phase_id: int) -> dict[str, Any]:
    for phase in state["phases"]:
        if phase["id"] == phase_id:
            return phase
    raise ValueError(f"phase id {phase_id} is not present in {state['state_file']}")


def current_phase(state: dict[str, Any]) -> dict[str, Any] | None:
    current_id = state.get("current_phase")
    if current_id is None:
        return None
    return phase_by_id(state, int(current_id))


def next_phase_id(state: dict[str, Any], phase_id: int) -> int | None:
    phase_ids = [int(phase["id"]) for phase in state["phases"]]
    try:
        index = phase_ids.index(phase_id)
    except ValueError as error:
        raise ValueError(f"phase id {phase_id} is not present") from error

    if index + 1 >= len(phase_ids):
        return None
    return phase_ids[index + 1]


def select_prompt(state: dict[str, Any]) -> tuple[str | None, str]:
    phase = current_phase(state)
    if phase is None:
        return None, "all phases are complete"

    status = phase["status"]
    qa_status = phase["qa_status"]

    if status in TERMINAL_PHASE_STATUSES:
        return None, f"phase {phase['id']} is {status}"

    if status in WORK_STATUSES:
        key = "start" if status == "not_started" else "continue"
        return build_prompt(state, phase, key), f"phase {phase['id']} {key}"

    if status == "complete" and qa_status != "passed":
        return build_prompt(state, phase, "qa"), f"phase {phase['id']} qa"

    if status == "complete" and qa_status == "passed":
        next_id = next_phase_id(state, int(phase["id"]))
        if next_id is None:
            state["current_phase"] = None
            state["completed_at"] = now_iso()
            return None, "all phases are complete"

        state["current_phase"] = next_id
        next_phase = phase_by_id(state, next_id)
        if next_phase["status"] == "not_started":
            return build_prompt(state, next_phase, "start"), f"phase {next_id} start"
        return build_prompt(state, next_phase, "continue"), f"phase {next_id} continue"

    raise ValueError(
        f"unsupported phase state for phase {phase['id']}: "
        f"status={status!r}, qa_status={qa_status!r}"
    )


def build_prompt(state: dict[str, Any], phase: dict[str, Any], prompt_key: str) -> str:
    state_path = Path(state["state_file"]).resolve()
    spec_path = Path(state["spec_file"]).resolve()
    prompt = phase["prompts"][prompt_key]
    qa_skepticism = ""
    if prompt_key == "qa":
        qa_skepticism = """
QA skepticism posture:
- Spec is authority; do not redesign or widen.
- Bring Sam Harris-level skepticism: calm, precise, skeptical of closure.
- Passing tests are weak evidence unless structure proves the claim.
- Find fake proof, authority leaks, and blockers to this phase.
"""
    return f"""You are running the Worth Query-Native Hardening Gate automation.

State file: {state_path}
Spec file: {spec_path}
Current phase: {phase["id"]} - {phase["title"]}
Prompt kind: {prompt_key}
{qa_skepticism}

{prompt}

Rules for this automated turn:
- Work in the repository named by the state file.
- Keep following AGENTS.md, arch laws, perf laws, and the phase spec.
- At the end of the turn, update the JSON state file truthfully.
- Before writing the JSON state file, read it fresh from disk in the same command or script that writes it.
- Mutate only the current phase row, current_phase, completed_at, and history entries needed to describe this turn.
- Preserve all unrelated phase rows, session fields, prompt text, and existing history.
- Use only these phase status values: not_started, in_progress, complete, regressed, blocked.
- Use only these QA status values: not_started, needed, in_progress, passed, failed.
- If implementation passes but QA has not run, set status complete and qa_status needed.
- If QA passes, set status complete and qa_status passed.
- If QA finds a real issue, set status regressed and qa_status failed or needed, then record the gap in history.
- You may set this phase back to in_progress or regressed if QA or implementation finds a gap.
- Do not advance current_phase yourself unless the current phase status is complete and qa_status is passed.
- If you are blocked, mark the phase status blocked and record the blocker in history.
"""


def codex_command(state: dict[str, Any]) -> list[str]:
    session = state["session"]
    codex_executable = session.get("codex_command", "codex")
    model = session["model"]
    effort = session["reasoning_effort"]
    thread_id = session.get("thread_id")
    config_args: list[str] = []

    for key, value in session.get("config", {}).items():
        config_args.extend(["-c", f"{key}={json.dumps(value)}"])

    if thread_id:
        command = [
            codex_executable,
            "exec",
            "resume",
            "--json",
            "-m",
            model,
            "-c",
            f'model_reasoning_effort="{effort}"',
            *config_args,
            thread_id,
            "-",
        ]
    else:
        command = [
            codex_executable,
            "exec",
            "--json",
            "-m",
            model,
            "-c",
            f'model_reasoning_effort="{effort}"',
            *config_args,
            "-C",
            session["cwd"],
            "-",
        ]

    return command


def run_codex(state: dict[str, Any], prompt: str, log_path: Path | None) -> int:
    command = codex_command(state)
    process = subprocess.Popen(
        command,
        cwd=state["session"]["cwd"],
        stdin=subprocess.PIPE,
        stdout=subprocess.PIPE,
        stderr=subprocess.STDOUT,
        text=True,
        encoding="utf-8",
        errors="replace",
    )

    assert process.stdin is not None
    process.stdin.write(prompt)
    process.stdin.close()

    assert process.stdout is not None
    for line in process.stdout:
        print(line, end="")
        if log_path:
            with log_path.open("a", encoding="utf-8") as log_file:
                log_file.write(line)
        capture_thread_id(state, line)

    return process.wait()


def capture_thread_id(state: dict[str, Any], line: str) -> None:
    try:
        event = json.loads(line)
    except json.JSONDecodeError:
        return

    if not isinstance(event, dict):
        return

    if event.get("type") == "thread.started":
        state["session"]["thread_id"] = event["thread_id"]
        state["session"]["thread_started_at"] = now_iso()


def append_history(
    state: dict[str, Any],
    event: str,
    detail: str,
    exit_code: int | None = None,
) -> None:
    state.setdefault("history", []).append(
        {
            "at": now_iso(),
            "event": event,
            "detail": detail,
            "exit_code": exit_code,
        }
    )


def run_once(state_path: Path, dry_run: bool, log_path: Path | None) -> bool:
    state = load_state(state_path)
    state["state_file"] = str(state_path)

    prompt, reason = select_prompt(state)

    if dry_run:
        if prompt is None:
            print(reason)
            return False
        print(prompt)
        return True

    mark_dispatch_started(state, reason)
    save_state(state_path, state)

    if prompt is None:
        print(reason)
        append_history(state, "runner_stop", reason)
        save_state(state_path, state)
        return False

    append_history(state, "runner_prompt_selected", reason)
    save_state(state_path, state)

    exit_code = run_codex(state, prompt, log_path)

    if exit_code != 0:
        print(f"codex exited with {exit_code}", file=sys.stderr)
        return False

    return True


def mark_dispatch_started(state: dict[str, Any], reason: str) -> None:
    phase = current_phase(state)
    if phase is None:
        return

    if reason.endswith(" start") and phase["status"] == "not_started":
        phase["status"] = "in_progress"
        phase["attempts"] = int(phase.get("attempts", 0)) + 1

    if reason.endswith(" qa") and phase["qa_status"] == "needed":
        phase["qa_status"] = "in_progress"


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("state_file", type=Path)
    parser.add_argument("--loop", action="store_true", help="run until blocked or complete")
    parser.add_argument(
        "--max-turns",
        type=int,
        default=None,
        help="optional safety cap; when omitted with --loop, run until complete or blocked",
    )
    parser.add_argument("--sleep-seconds", type=int, default=10)
    parser.add_argument("--dry-run", action="store_true")
    parser.add_argument("--log", type=Path)
    return parser.parse_args()


def main() -> int:
    if hasattr(sys.stdout, "reconfigure"):
        sys.stdout.reconfigure(encoding="utf-8", errors="replace")
    if hasattr(sys.stderr, "reconfigure"):
        sys.stderr.reconfigure(encoding="utf-8", errors="replace")

    args = parse_args()
    turns = 0

    while True:
        should_continue = run_once(args.state_file, args.dry_run, args.log)
        turns += 1

        if not args.loop or not should_continue:
            return 0

        if args.max_turns is not None and turns >= args.max_turns:
            print(f"stopped after max turns: {args.max_turns}")
            return 0

        time.sleep(args.sleep_seconds)


if __name__ == "__main__":
    raise SystemExit(main())
