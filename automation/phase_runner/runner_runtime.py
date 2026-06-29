from __future__ import annotations

from pathlib import Path
from typing import Any

from codex_cli import run_codex
from prompts import render_prompt
from runner_faults import (
    CodexExitFailure,
    PostTurnMergeFailure,
    PromptRenderFailure,
    StateValidationFailure,
)
from runner_recovery import classify_exception, run_recovery
from state import append_history, cursor_reason, load_state, save_state
from state_normalization import normalize_state
from validation import validate_state

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

    try:
        prompt = render_prompt(state, state_path)
    except Exception as error:
        raise PromptRenderFailure(str(error)) from error
    if dry_run:
        print(prompt)
        return True

    append_history(state, "runner_prompt_selected", reason)
    save_state(state_path, state)

    result = run_codex(state, prompt, log_path)
    try:
        merge_after_codex(state_path, reason, result.exit_code, result.capture)
    except Exception as error:
        raise PostTurnMergeFailure(str(error)) from error
    if result.exit_code != 0:
        raise CodexExitFailure(f"codex exited with {result.exit_code}")
    return True


def run_once_with_recovery(
    state_path: Path,
    dry_run: bool,
    log_path: Path | None,
    forced_phase: str | None,
    forced_turn: str | None,
    recover: bool,
    recovery_prompt_builder: Any,
) -> bool:
    try:
        return run_once(state_path, dry_run, log_path, forced_phase, forced_turn)
    except Exception as error:
        if not recover or dry_run:
            raise
        fault = classify_exception(error)
        outcome = run_recovery(state_path, log_path, fault, recovery_prompt_builder)
        return outcome.should_continue


def apply_forced_cursor(
    state: dict[str, Any], forced_phase: str | None, forced_turn: str | None
) -> None:
    if forced_phase is not None or forced_turn is not None:
        if not isinstance(state.get("current"), dict):
            state["current"] = {}
    if forced_phase is not None:
        state["current"]["phase"] = int(forced_phase)
    if forced_turn is not None:
        state["current"]["turn"] = forced_turn


def persist_normalized_state(state_path: Path, state: dict[str, Any]) -> None:
    if normalize_state(state):
        append_history(
            state,
            "runner_state_normalized",
            "normalized malformed state buckets before validation",
        )
        save_state(state_path, state)


def stop_without_turn(
    state_path: Path,
    dry_run: bool,
    state: dict[str, Any],
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


def validate_or_raise(state: dict[str, Any], state_path: Path) -> None:
    errors = validate_state(state, state_path)
    if errors:
        raise StateValidationFailure(
            "\n".join(f"validation error: {error}" for error in errors)
        )


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


def merge_after_codex(
    state_path: Path,
    reason: str,
    exit_code: int,
    capture: dict[str, str],
) -> None:
    state = load_state(state_path)
    session = state.setdefault("session", {})
    if capture.get("thread_id") and not session.get("thread_id"):
        session["thread_id"] = capture["thread_id"]
        session["thread_started_at"] = capture.get("thread_started_at")
    append_history(state, "codex_turn_completed", reason, exit_code)
    save_state(state_path, state)


def should_stop_before_phase(state: dict[str, Any]) -> bool:
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


def stop_before_phase_reason(state: dict[str, Any]) -> str:
    current = state.get("current", {})
    stop_before_phase = state.get("runner_control", {}).get("stop_before_phase")
    label = state.get("runner_control", {}).get("stop_reason")
    if not label:
        label = "configured stop-before-phase gate"
    return (
        f"{label}: current phase {current.get('phase')} "
        f"{current.get('turn')} reached stop_before_phase {stop_before_phase}"
    )


def recovery_prompt(
    state: dict[str, Any],
    state_path: Path,
    reason: str,
    details: str,
) -> str:
    current = state.get("current")
    return f"""The automated phase runner failed before it could send the next normal turn.

This is a recovery turn in the same persistent Codex session. Fix the cause, then
leave the JSON state valid so the runner can continue automatically.

State file: {state_path.resolve()}
Recovery state source: {state.get("_recovery_source_path", str(state_path.resolve()))}
Current cursor before recovery: {current}
Failure reason: {reason}

Failure details:
```text
{details[-6000:]}
```

Recovery rules:
- Read the state file fresh immediately before writing it.
- Preserve the current cursor unless the state already contains completed work
  that objectively requires a cursor correction.
- Preserve all existing phase notes, history, session, project, and template
  fields except the malformed field or directly related recovery history.
- If a notes field is supposed to be one of the standard note buckets, make it a
  JSON array: plan, done, remaining, findings, verification.
- Do not mark QA passed during recovery. Recovery only makes the runner able to
  continue; normal review/repair/close turns do the certification work.
- After fixing, run:
  python automation\\phase_runner\\runner.py {state_path} --validate
  and record the result in history.
"""
