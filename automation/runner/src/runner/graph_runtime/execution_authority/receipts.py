from __future__ import annotations

import json
import os
from dataclasses import dataclass
from pathlib import Path
from typing import Any

from runner.authority.run_identity import RuntimePaths, now_iso


@dataclass(frozen=True)
class ExecutionReceipt:
    execution_id: str
    turn_instance_id: str
    state: str
    runner_pid: int
    provider_pid: int | None = None
    exit_code: int | None = None
    capture: dict[str, Any] | None = None


def load_execution(paths: RuntimePaths, execution_id: str) -> ExecutionReceipt | None:
    path = receipt_path(paths, execution_id)
    if not path.exists():
        return None
    return decode_receipt(json.loads(path.read_text(encoding="utf-8")))


def claim_execution(paths: RuntimePaths, execution_id: str, turn_instance_id: str) -> tuple[ExecutionReceipt, bool]:
    existing = load_execution(paths, execution_id)
    if existing is not None:
        return existing, False
    receipt = ExecutionReceipt(execution_id, turn_instance_id, "claimed", os.getpid())
    write_receipt(paths, receipt)
    return receipt, True


def record_process_launch(paths: RuntimePaths, execution_id: str, provider_pid: int) -> ExecutionReceipt:
    current = required_receipt(paths, execution_id)
    if current.state == "finished":
        return current
    launched = ExecutionReceipt(execution_id, current.turn_instance_id, "launched", current.runner_pid, provider_pid)
    write_receipt(paths, launched)
    return launched


def finish_execution(
    paths: RuntimePaths,
    execution_id: str,
    exit_code: int,
    capture: dict[str, Any],
) -> ExecutionReceipt:
    current = required_receipt(paths, execution_id)
    finished = ExecutionReceipt(
        execution_id,
        current.turn_instance_id,
        "finished",
        current.runner_pid,
        current.provider_pid,
        exit_code,
        capture,
    )
    write_receipt(paths, finished)
    return finished


def receipt_path(paths: RuntimePaths, execution_id: str) -> Path:
    return paths.executions / f"{execution_id}.json"


def required_receipt(paths: RuntimePaths, execution_id: str) -> ExecutionReceipt:
    receipt = load_execution(paths, execution_id)
    if receipt is None:
        raise ValueError(f"execution receipt {execution_id!r} does not exist")
    return receipt


def write_receipt(paths: RuntimePaths, receipt: ExecutionReceipt) -> None:
    paths.executions.mkdir(parents=True, exist_ok=True)
    target = receipt_path(paths, receipt.execution_id)
    temporary = target.with_name(f"{target.name}.{os.getpid()}.tmp")
    payload = {
        "execution_id": receipt.execution_id,
        "turn_instance_id": receipt.turn_instance_id,
        "state": receipt.state,
        "runner_pid": receipt.runner_pid,
        "provider_pid": receipt.provider_pid,
        "exit_code": receipt.exit_code,
        "capture": receipt.capture,
        "updated_at": now_iso(),
    }
    temporary.write_text(json.dumps(payload, separators=(",", ":")), encoding="utf-8")
    os.replace(temporary, target)


def decode_receipt(payload: dict[str, Any]) -> ExecutionReceipt:
    return ExecutionReceipt(
        execution_id=payload["execution_id"],
        turn_instance_id=payload["turn_instance_id"],
        state=payload["state"],
        runner_pid=payload["runner_pid"],
        provider_pid=payload.get("provider_pid"),
        exit_code=payload.get("exit_code"),
        capture=payload.get("capture"),
    )
