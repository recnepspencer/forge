from __future__ import annotations

import hashlib
import json
import os
import subprocess
import sys
import time
from dataclasses import dataclass
from pathlib import Path

from worth_ui_3141_proof_plan import prepare_claim, proofs
from worth_ui_ledger_operational_successors import stage_execution_claim
from worth_ui_ledger_runner_authentication import authentication_tag
from worth_ui_ledger_shared_execution_lineage import (
    SharedExecutionLineageRequest,
    inherit_shared_receipt_lineage,
)
from worth_ui_ledger_verifier_rebinding import (
    REBINDABLE_SOURCE_IDENTITIES,
    bind_fresh_compile_artifact,
    bind_fresh_shared_world,
    bind_fresh_supporting_world,
)


@dataclass(frozen=True)
class RowExecutionOptions:
    shared_world_artifact: str | None
    candidate_ledger: Path | None
    supporting_world_artifact: str | None
    refresh_mode: str
    predecessor_handoff: str | None


@dataclass(frozen=True)
class PreparedRowExecution:
    current: dict[str, str]
    artifact: Path
    command: list[str]
    environment: dict[str, str]


class PortfolioRowExecutor:
    def __init__(self, root: Path, target: Path) -> None:
        self.root = root
        self.target = target

    def __call__(
        self, row: dict[str, str], artifact: Path, compile_artifact: str, **values: object
    ) -> dict[str, object]:
        options = RowExecutionOptions(
            shared_world_artifact=optional_string(values, "shared_world_artifact"),
            candidate_ledger=optional_path(values, "candidate_ledger"),
            supporting_world_artifact=optional_string(values, "supporting_world_artifact"),
            refresh_mode=optional_string(values, "refresh_mode") or "direct",
            predecessor_handoff=optional_string(values, "predecessor_handoff"),
        )
        started = time.perf_counter()
        print(f"[portfolio:start] {row['requirement']}", file=sys.stderr, flush=True)
        prepared = self.prepare(row, artifact, compile_artifact, options)
        payload = self.execute(row["requirement"], prepared)
        self.bind_mapping(payload, prepared.current, prepared.artifact, prepared.command)
        print(
            f"[portfolio:pass] {row['requirement']} "
            f"elapsed={time.perf_counter() - started:.2f}s",
            file=sys.stderr,
            flush=True,
        )
        return payload

    def prepare(
        self, row: dict[str, str], artifact: Path, compile_artifact: str,
        options: RowExecutionOptions,
    ) -> PreparedRowExecution:
        current = dict(row)
        proof = proofs().get(row["requirement"])
        if proof is not None and not preserve_historical_claim(current, options.refresh_mode):
            prepare_claim(current, proof)
        identity = artifact.resolve().relative_to(self.root).as_posix()
        command = self.command(current, identity, compile_artifact, options)
        if options.candidate_ledger is not None:
            stage_execution_claim(
                options.candidate_ledger,
                current,
                identity,
                command,
                preserve_claim=preserve_historical_claim(current, options.refresh_mode),
            )
        predecessor = (
            options.predecessor_handoff
            if options.predecessor_handoff is not None
            else predecessor_artifact(command)
        )
        return PreparedRowExecution(
            current, artifact, command, execution_environment(
                compile_artifact, options, predecessor
            )
        )

    def command(
        self, current: dict[str, str], artifact: str, compile_artifact: str,
        options: RowExecutionOptions,
    ) -> list[str]:
        command = current["exact_command"].split()
        command[command.index("--artifact") + 1] = artifact
        command = bind_fresh_compile_artifact(command, compile_artifact)
        if options.shared_world_artifact is not None:
            command = bind_fresh_shared_world(command, options.shared_world_artifact)
        if options.supporting_world_artifact is not None:
            command = bind_fresh_supporting_world(command, options.supporting_world_artifact)
        return command

    def execute(
        self, requirement: str, prepared: PreparedRowExecution
    ) -> dict[str, object]:
        command = list(prepared.command)
        if command and command[0] == "python":
            command[0] = sys.executable
        completed = subprocess.run(
            command, cwd=self.root, env=prepared.environment,
            stdout=subprocess.PIPE, text=True, check=False,
        )
        if completed.returncode != 0:
            sys.stderr.write(completed.stdout)
            raise RuntimeError(f"fresh governed execution failed for {requirement}")
        payload = json.loads(completed.stdout.splitlines()[-1])
        if payload.get("exit_posture") != "passed" or payload.get("requirement") != requirement:
            raise RuntimeError(f"fresh governed execution was not exact for {requirement}")
        return payload

    def bind_mapping(
        self, payload: dict[str, object], current: dict[str, str], artifact: Path,
        command: list[str],
    ) -> None:
        payload["production_entry"] = current["production_entry"]
        payload["independent_oracle"] = current["independent_oracle"]
        mapped = current["source_identity"].split(";")
        payload["mapping_source_identity"] = mapped
        payload["source_rebindings"] = source_rebindings(
            self.root, self.target, mapped, payload["source_identity"]
        )
        payload["executed_exact_command"] = " ".join(command)
        inherit_shared_receipt_lineage(SharedExecutionLineageRequest(
            row=current,
            payload=payload,
            root=self.root,
            revision=str(payload["source_revision"]),
            state_digest=str(payload["source_state_digest"]),
            current_claim=str(payload["claim_digest"]),
        ))
        retain_rebound_mapping(self.root, artifact, payload)


def execution_environment(
    compile_artifact: str, options: RowExecutionOptions, predecessor: str | None
) -> dict[str, str]:
    environment = dict(os.environ)
    environment["WORTH_UI_COMPILE_ARTIFACT"] = compile_artifact
    for key, value in (
        ("WORTH_UI_SHARED_WORLD_ARTIFACT", options.shared_world_artifact),
        ("WORTH_UI_SUPPORTING_WORLD_ARTIFACT", options.supporting_world_artifact),
        ("WORTH_UI_PREDECESSOR_ARTIFACT", predecessor),
    ):
        if value is not None:
            environment[key] = value
    if options.candidate_ledger is not None:
        environment["WORTH_UI_MILESTONE_3141_LEDGER"] = str(
            options.candidate_ledger.resolve()
        )
    return environment


def predecessor_artifact(command: list[str]) -> str | None:
    for index, word in enumerate(command[:-1]):
        if word == "--source" and command[index + 1].endswith("predecessor-handoff.json"):
            return command[index + 1]
    return None


def retain_rebound_mapping(
    root: Path, identity: Path, payload: dict[str, object]
) -> None:
    payload.pop("artifact_sha256", None)
    payload.pop("runner_authentication", None)
    payload["runner_authentication"] = authentication_tag(payload, root)
    identity.write_text(json.dumps(payload, indent=2) + "\n", encoding="utf-8")
    payload["artifact_sha256"] = hashlib.sha256(identity.read_bytes()).hexdigest()


def source_rebindings(
    root: Path, target: Path, canonical_sources: list[str], executed_sources: object
) -> list[dict[str, str]]:
    if not isinstance(executed_sources, list) or not all(
        isinstance(source, str) for source in executed_sources
    ) or len(canonical_sources) != len(executed_sources):
        raise RuntimeError("fresh evidence has invalid executed source identities")
    result = []
    for canonical, executed in zip(canonical_sources, executed_sources, strict=True):
        if canonical == executed:
            continue
        if canonical not in REBINDABLE_SOURCE_IDENTITIES:
            raise RuntimeError("fresh evidence substituted a production source")
        identity = (root / executed).resolve()
        relative = identity.relative_to(target.resolve())
        if not relative.parts or not relative.parts[0].startswith("worth-ui-3141-verify-"):
            raise RuntimeError("fresh evidence source is outside its governed temporary world")
        result.append({
            "canonical": canonical,
            "executed": executed,
            "sha256": hashlib.sha256(identity.read_bytes()).hexdigest(),
        })
    return result


def optional_string(values: dict[str, object], name: str) -> str | None:
    value = values.get(name)
    if value is not None and not isinstance(value, str):
        raise TypeError(f"{name} must be a string")
    return value


def optional_path(values: dict[str, object], name: str) -> Path | None:
    value = values.get(name)
    if value is not None and not isinstance(value, Path):
        raise TypeError(f"{name} must be a Path")
    return value


def preserve_historical_claim(row: dict[str, str], refresh_mode: str) -> bool:
    del row
    return refresh_mode.startswith("root-phase-")
