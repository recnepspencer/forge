from __future__ import annotations


COMPILE_ARTIFACT = "_docs/worth-ui/milestone-3.14.1-evidence/compile-contracts.json"
NATIVE_WORLD_ARTIFACT = "_docs/worth-ui/milestone-3.14.1-evidence/p2-world-01.json"
MOUNTED_WORLD_ARTIFACT = "_docs/worth-ui/milestone-3.14.1-evidence/p1-worlds-01.json"
P3_WORLD_ARTIFACT = "_docs/worth-ui/milestone-3.14.1-evidence/p3-hp02-world-01.json"
P3_DELTA_ARTIFACT = "_docs/worth-ui/milestone-3.14.1-evidence/p3-delta-source-01.json"
P3_PREDECESSOR_HANDOFF = (
    "_docs/worth-ui/milestone-3.14.1-evidence/p3-predecessor-handoff.json"
)
P4_PREDECESSOR_HANDOFF = (
    "_docs/worth-ui/milestone-3.14.1-evidence/p4-predecessor-handoff.json"
)
P5_PREDECESSOR_HANDOFF = (
    "_docs/worth-ui/milestone-3.14.1-evidence/p5-predecessor-handoff.json"
)
P5_ATLAS_ARTIFACT = "_docs/worth-ui/milestone-3.14.1-evidence/p5-atlas-01.json"
REBINDABLE_SOURCE_IDENTITIES = {
    COMPILE_ARTIFACT,
    NATIVE_WORLD_ARTIFACT,
    MOUNTED_WORLD_ARTIFACT,
    P3_WORLD_ARTIFACT,
    P3_DELTA_ARTIFACT,
    P3_PREDECESSOR_HANDOFF,
    P4_PREDECESSOR_HANDOFF,
    P5_PREDECESSOR_HANDOFF,
    P5_ATLAS_ARTIFACT,
}


def bind_fresh_compile_artifact(command: list[str], artifact: str) -> list[str]:
    return rebind_sources(command, {COMPILE_ARTIFACT}, artifact)


def bind_fresh_shared_world(command: list[str], artifact: str) -> list[str]:
    return rebind_sources(
        command,
        {NATIVE_WORLD_ARTIFACT, MOUNTED_WORLD_ARTIFACT, P3_WORLD_ARTIFACT, P3_DELTA_ARTIFACT},
        artifact,
    )


def bind_fresh_supporting_world(command: list[str], artifact: str) -> list[str]:
    return rebind_sources(command, {P3_DELTA_ARTIFACT, P5_ATLAS_ARTIFACT}, artifact)


def bind_fresh_predecessor_handoff(
    command: list[str], artifact: str, phase: int = 3
) -> list[str]:
    canonical = {
        3: P3_PREDECESSOR_HANDOFF,
        4: P4_PREDECESSOR_HANDOFF,
        5: P5_PREDECESSOR_HANDOFF,
    }[phase]
    return rebind_sources(command, {canonical}, artifact)


def rebind_sources(
    command: list[str], canonical: set[str], artifact: str
) -> list[str]:
    rebound = list(command)
    for index, word in enumerate(rebound[:-1]):
        if word == "--source" and rebound[index + 1] in canonical:
            rebound[index + 1] = artifact
    return rebound
