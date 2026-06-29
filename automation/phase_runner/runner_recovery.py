from __future__ import annotations

import traceback
from dataclasses import dataclass
from pathlib import Path
from typing import Any

from codex_cli import CodexResult, run_codex
from runner_faults import (
    CodexExitFailure,
    FailureKind,
    PostTurnMergeFailure,
    PromptRenderFailure,
    RecoveryKind,
    RunnerFault,
    StateValidationFailure,
)
from state import append_history, backup_path, load_state, save_state
from state_normalization import normalize_state


@dataclass
class RecoveryDecision:
    kind: RecoveryKind
    detail: str


@dataclass
class RecoveryOutcome:
    should_continue: bool
    decision: RecoveryDecision


def classify_exception(error: Exception) -> RunnerFault:
    details = traceback.format_exc()
    kind = FailureKind.CODEX_INVOCATION
    name = type(error).__name__
    if isinstance(error, StateValidationFailure):
        kind = FailureKind.STATE_VALIDATION
    elif isinstance(error, PromptRenderFailure):
        kind = FailureKind.PROMPT_RENDER
    elif isinstance(error, CodexExitFailure):
        kind = FailureKind.CODEX_EXIT
    elif isinstance(error, PostTurnMergeFailure):
        kind = FailureKind.POST_TURN_MERGE
    elif isinstance(error, FileNotFoundError):
        kind = FailureKind.STATE_LOAD
    elif isinstance(error, OSError):
        kind = FailureKind.CODEX_INVOCATION
    return RunnerFault(kind=kind, reason=f"{name}: {error}", details=details)


def attempt_local_normalization(state_path: Path) -> bool:
    state = load_state(state_path)
    changed = normalize_state(state)
    if changed:
        append_history(
            state,
            "runner_state_normalized",
            "normalized malformed state buckets before validation",
        )
        save_state(state_path, state)
    return changed


def load_state_for_recovery(state_path: Path) -> dict[str, Any]:
    try:
        return load_state(state_path)
    except Exception:
        backup = backup_path(state_path)
        if not backup.exists():
            raise
        state = load_state(backup)
        state["_recovery_source_path"] = str(backup)
        return state


def choose_recovery(state_path: Path, fault: RunnerFault) -> RecoveryDecision:
    if fault.kind is FailureKind.STATE_VALIDATION and attempt_local_normalization(state_path):
        return RecoveryDecision(
            kind=RecoveryKind.LOCAL_NORMALIZE,
            detail="local normalization repaired canonical state",
        )
    if fault.kind is FailureKind.STATE_LOAD and backup_path(state_path).exists():
        return RecoveryDecision(
            kind=RecoveryKind.BACKUP_RESTORE,
            detail="live state unavailable; restore from backup",
        )
    if fault.kind in {
        FailureKind.STATE_LOAD,
        FailureKind.STATE_VALIDATION,
        FailureKind.PROMPT_RENDER,
        FailureKind.CODEX_EXIT,
        FailureKind.POST_TURN_MERGE,
    }:
        return RecoveryDecision(
            kind=RecoveryKind.CODEX_RECOVERY,
            detail=f"delegate {fault.kind.value} repair to persisted Codex thread",
        )
    return RecoveryDecision(
        kind=RecoveryKind.TERMINAL_STOP,
        detail=f"terminal {fault.kind.value} failure",
    )


def run_recovery(
    state_path: Path,
    log_path: Path | None,
    fault: RunnerFault,
    prompt_builder: Any,
) -> RecoveryOutcome:
    decision = choose_recovery(state_path, fault)
    if decision.kind is RecoveryKind.LOCAL_NORMALIZE:
        state = load_state(state_path)
        append_history(state, "runner_recovery_succeeded", decision.detail)
        save_state(state_path, state)
        return RecoveryOutcome(should_continue=True, decision=decision)
    if decision.kind is RecoveryKind.BACKUP_RESTORE:
        backup = backup_path(state_path)
        state = load_state(backup)
        append_history(state, "runner_recovery_restored_backup", decision.detail)
        save_state(state_path, state)
        return RecoveryOutcome(should_continue=True, decision=decision)
    if decision.kind is RecoveryKind.CODEX_RECOVERY:
        state = load_state_for_recovery(state_path)
        append_history(state, "runner_recovery_requested", fault.reason)
        save_state(state_path, state)
        prompt = prompt_builder(state, state_path, fault.reason, fault.details)
        result = run_codex(state, prompt, log_path)
        merge_recovery_result(state_path, result)
        if result.exit_code == 0:
            return RecoveryOutcome(should_continue=True, decision=decision)
        record_terminal_stop(
            state_path,
            f"codex recovery exited with {result.exit_code} after {fault.kind.value}",
            result.exit_code,
        )
        return RecoveryOutcome(should_continue=False, decision=decision)
    record_terminal_stop(state_path, decision.detail, None)
    return RecoveryOutcome(should_continue=False, decision=decision)


def merge_recovery_result(state_path: Path, result: CodexResult) -> None:
    state = load_state(state_path)
    session = state.setdefault("session", {})
    if result.capture.get("thread_id") and not session.get("thread_id"):
        session["thread_id"] = result.capture["thread_id"]
        session["thread_started_at"] = result.capture.get("thread_started_at")
    append_history(state, "codex_turn_completed", "runner recovery", result.exit_code)
    if result.exit_code == 0:
        append_history(state, "runner_recovery_succeeded", "codex recovery completed")
    save_state(state_path, state)


def record_terminal_stop(state_path: Path, detail: str, exit_code: int | None) -> None:
    state = load_state_for_recovery(state_path)
    append_history(state, "runner_terminal_stop", detail, exit_code)
    save_state(state_path, state)
