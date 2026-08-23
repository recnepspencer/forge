from __future__ import annotations

import re
from dataclasses import dataclass
from enum import Enum
from pathlib import Path
from typing import Mapping


EVIDENCE_DIRECTORY = "_docs/worth-ui/milestone-3.14.1-evidence"
REQUIREMENT_PATTERN = re.compile(r"^P(?P<phase>[1-9][0-9]*)-[A-Z0-9-]+-01$")
SHA256_PATTERN = re.compile(r"^[0-9a-f]{64}$")


class ArtifactKind(Enum):
    ROW_EVIDENCE = "row-evidence"
    PREDECESSOR_HANDOFF = "predecessor-handoff"
    PHASE_INVALIDATION = "phase-invalidation"
    SUPERSEDED_ROW_EVIDENCE = "superseded-row-evidence"
    ARTIFACT_DRIFT_INVENTORY = "artifact-drift-inventory"
    INCIDENT_ROW_EVIDENCE = "incident-row-evidence"
    EXECUTION_OBSERVATION = "execution-observation"


@dataclass(frozen=True)
class ArtifactIdentity:
    kind: ArtifactKind
    relative_path: str
    phase: int
    requirement: str | None = None

    def destination(self, root: Path) -> Path:
        destination = (root / self.relative_path).resolve()
        try:
            destination.relative_to(root.resolve())
        except ValueError as error:
            raise RuntimeError("typed artifact identity escapes the repository") from error
        return destination

    def validate_json_payload(self, payload: Mapping[str, object]) -> None:
        if self.kind is ArtifactKind.ROW_EVIDENCE:
            if payload.get("schema_version") not in {5, 6, 7}:
                raise RuntimeError("row evidence has the wrong schema version")
            if payload.get("requirement") != self.requirement:
                raise RuntimeError("row evidence requirement does not match its identity")
            return
        if self.kind is ArtifactKind.PREDECESSOR_HANDOFF:
            expected_schema = predecessor_schema(self.phase)
            if payload.get("schema") != expected_schema:
                raise RuntimeError("predecessor handoff has the wrong schema")
            if payload.get("through_phase") != self.phase - 1:
                raise RuntimeError("predecessor handoff phase does not match its identity")
            return
        if self.kind is ArtifactKind.EXECUTION_OBSERVATION:
            if payload.get("observation_sha256") != self.requirement:
                raise RuntimeError("execution observation digest does not match its identity")
            record = payload.get("record")
            if not isinstance(record, Mapping) or record.get("schema") != (
                "worth-ui-ledger-execution-observation-v1"
            ):
                raise RuntimeError("execution observation has the wrong schema")
            return
        if self.kind is ArtifactKind.PHASE_INVALIDATION:
            if payload.get("schema") not in {
                "worth-ui-ledger-phase-invalidation-v1",
                "worth-ui-ledger-phase-invalidation-v2",
            }:
                raise RuntimeError("phase invalidation has the wrong schema")
            if payload.get("phase") != self.phase:
                raise RuntimeError("phase invalidation phase does not match its identity")
            return
        if self.kind is ArtifactKind.ARTIFACT_DRIFT_INVENTORY:
            if payload.get("schema") != "worth-ui-ledger-artifact-drift-inventory-v1":
                raise RuntimeError("artifact drift inventory has the wrong schema")
            return
        raise RuntimeError("superseded evidence is published as exact retained bytes")


def row_evidence(requirement: str) -> ArtifactIdentity:
    phase = requirement_phase(requirement)
    filename = requirement.lower() + ".json"
    return ArtifactIdentity(
        ArtifactKind.ROW_EVIDENCE,
        f"{EVIDENCE_DIRECTORY}/{filename}",
        phase,
        requirement,
    )


def execution_observation(observation_sha256: str) -> ArtifactIdentity:
    require_sha256(observation_sha256, "execution observation digest")
    return ArtifactIdentity(
        ArtifactKind.EXECUTION_OBSERVATION,
        f"{EVIDENCE_DIRECTORY}/execution-observations/"
        f"{observation_sha256[:2]}/{observation_sha256}.json",
        0,
        observation_sha256,
    )


def require_row_evidence_identity(requirement: str, declared: str) -> ArtifactIdentity:
    identity = row_evidence(requirement)
    if declared != identity.relative_path:
        raise ValueError(
            f"{requirement} artifact must be {identity.relative_path}"
        )
    return identity


def predecessor_handoff(
    phase: int, verification_state_digest: str | None = None
) -> ArtifactIdentity:
    require_positive_phase(phase)
    filename = f"p{phase}-predecessor-handoff.json"
    if verification_state_digest is None:
        relative_path = f"{EVIDENCE_DIRECTORY}/{filename}"
    else:
        require_sha256(verification_state_digest, "verification state digest")
        relative_path = (
            "workspaces/worth-ui/target/"
            f"worth-ui-3141-verify-predecessor-{verification_state_digest}/{filename}"
        )
    return ArtifactIdentity(
        ArtifactKind.PREDECESSOR_HANDOFF, relative_path, phase
    )


def declared_predecessor_handoff(declared: str, phase: int) -> ArtifactIdentity:
    canonical = predecessor_handoff(phase)
    if declared == canonical.relative_path:
        return canonical
    prefix = "workspaces/worth-ui/target/worth-ui-3141-verify-predecessor-"
    suffix = f"/p{phase}-predecessor-handoff.json"
    if declared.startswith(prefix) and declared.endswith(suffix):
        digest = declared[len(prefix) : -len(suffix)]
        return predecessor_handoff(phase, digest)
    raise ValueError("predecessor handoff is not a typed canonical identity")


def phase_invalidation(phase: int, incident_digest: str) -> ArtifactIdentity:
    require_positive_phase(phase)
    require_sha256(incident_digest, "invalidation incident digest")
    return ArtifactIdentity(
        ArtifactKind.PHASE_INVALIDATION,
        f"{EVIDENCE_DIRECTORY}/invalidations/p{phase}/{incident_digest}.json",
        phase,
    )


def superseded_row_evidence(requirement: str, digest: str) -> ArtifactIdentity:
    phase = requirement_phase(requirement)
    require_sha256(digest, "superseded artifact digest")
    return ArtifactIdentity(
        ArtifactKind.SUPERSEDED_ROW_EVIDENCE,
        f"{EVIDENCE_DIRECTORY}/superseded/{requirement.lower()}/{digest}.json",
        phase,
        requirement,
    )


def artifact_drift_inventory(incident_digest: str) -> ArtifactIdentity:
    require_sha256(incident_digest, "artifact drift incident digest")
    return ArtifactIdentity(
        ArtifactKind.ARTIFACT_DRIFT_INVENTORY,
        f"{EVIDENCE_DIRECTORY}/invalidations/incidents/{incident_digest}.json",
        0,
    )


def incident_row_evidence(
    incident_digest: str, requirement: str, observed_digest: str
) -> ArtifactIdentity:
    phase = requirement_phase(requirement)
    require_sha256(incident_digest, "artifact drift incident digest")
    require_sha256(observed_digest, "incident row evidence digest")
    return ArtifactIdentity(
        ArtifactKind.INCIDENT_ROW_EVIDENCE,
        f"{EVIDENCE_DIRECTORY}/invalidations/observed-row-evidence/"
        f"{observed_digest}.json",
        phase,
        requirement,
    )


def predecessor_schema(phase: int) -> str:
    require_positive_phase(phase)
    return "worth-ui-phase-predecessor-handoff-v4"


def requirement_phase(requirement: str) -> int:
    matched = REQUIREMENT_PATTERN.fullmatch(requirement)
    if matched is None:
        raise ValueError(f"invalid Worth UI requirement identity: {requirement}")
    return int(matched.group("phase"))


def require_positive_phase(phase: int) -> None:
    if phase < 1:
        raise ValueError("artifact phase must be positive")


def require_sha256(value: str, name: str) -> None:
    if SHA256_PATTERN.fullmatch(value) is None:
        raise ValueError(f"{name} must be a lowercase SHA-256 digest")
