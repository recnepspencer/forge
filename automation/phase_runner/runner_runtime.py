from __future__ import annotations

from pathlib import Path

from codex_cli import run_codex
from prompts import render_prompt
from state import append_history, cursor_reason, load_state, save_state
from state_normalization import normalize_state
from validation import validate_state


class RunnerFailure(Exception):
    pass


def run_once(
    state_path: Path,
    dry_run: bool,
    log_path: Path | None,
    forced_phase: str | None,
    forced_turn: str | None,
) -> bool:
    state = load_state(state_path)
    persist_normalized_state(state_path, state)
    validate_or_raise(state, state_path)

    apply_forced_cursor(state, forced_phase, forced_turn)
    reason = cursor_reason(state)
    if state.get("current") is None:
        return stop_without_turn(state_path, dry_run, state, "runner_stop", reason)
    if should_stop_before_phase(state):
        return stop_without_turn(
            state_path,
            dry_run,
            state,
            "runner_stop_before_phase",
            stop_before_phase_reason(state),
        )

    prompt = render_prompt(state, state_path)
    if dry_run:
        print(prompt)
        return True

    append_history(state, "runner_prompt_selected", reason)
    save_state(state_path, state)

    result = run_codex(state, prompt, log_path)
    merge_after_codex(state_path, result.capture)
    if result.exit_code != 0:
        raise RunnerFailure(f"codex exited with {result.exit_code}")
    return True


def apply_forced_cursor(
    state: dict, forced_phase: str | None, forced_turn: str | None
) -> None:
    if forced_phase is not None or forced_turn is not None:
        if not isinstance(state.get("current"), dict):
            state["current"] = {}
    if forced_phase is not None:
        state["current"]["phase"] = int(forced_phase)
    if forced_turn is not None:
        state["current"]["turn"] = forced_turn


def persist_normalized_state(state_path: Path, state: dict) -> None:
    if normalize_state(state):
        append_history(
            state,
            "runner_state_normalized",
            "normalized malformed note buckets before validation",
        )
        save_state(state_path, state)


def stop_without_turn(
    state_path: Path,
    dry_run: bool,
    state: dict,
    event: str,
    reason: str,
) -> bool:
    if dry_run:
        print(reason)
        return False
    append_history(state, event, reason)
    save_state(state_path, state)
    print(reason)
    return False


def validate_or_raise(state: dict, state_path: Path) -> None:
    errors = validate_state(state, state_path)
    if errors:
        raise RunnerFailure("\n".join(f"validation error: {error}" for error in errors))


def validate_command(state_path: Path) -> int:
    state = load_state(state_path)
    persist_normalized_state(state_path, state)
    errors = validate_state(state, state_path)
    if errors:
        for error in errors:
            print(f"validation error: {error}")
        return 2
    print("state file is valid")
    return 0


def merge_after_codex(state_path: Path, capture: dict[str, str]) -> None:
    state = load_state(state_path)
    session = state.setdefault("session", {})
    if capture.get("thread_id") and not session.get("thread_id"):
        session["thread_id"] = capture["thread_id"]
        session["thread_started_at"] = capture.get("thread_started_at")
        save_state(state_path, state)


def should_stop_before_phase(state: dict) -> bool:
    current = state.get("current")
    if not isinstance(current, dict):
        return False
    stop_before_phase = state.get("runner_control", {}).get("stop_before_phase")
    if stop_before_phase is None:
        return False
    try:
        return int(current.get("phase")) >= int(stop_before_phase)
    except (TypeError, ValueError):
        return False


def stop_before_phase_reason(state: dict) -> str:
    current = state.get("current", {})
    stop_before_phase = state.get("runner_control", {}).get("stop_before_phase")
    label = state.get("runner_control", {}).get("stop_reason")
    if not label:
        label = "configured stop-before-phase gate"
    return (
        f"{label}: current phase {current.get('phase')} "
        f"{current.get('turn')} reached stop_before_phase {stop_before_phase}"
    )
