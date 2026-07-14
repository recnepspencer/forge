from __future__ import annotations

import tempfile
from pathlib import Path

from runner.graph_runtime.execution_authority.receipts import claim_execution, finish_execution
from runner.graph_runtime.nodes.execution_nodes import execution_identity


class CanaryPaths:
    def __init__(self, root: Path) -> None:
        self.executions = root / "executions"


def run_stability_canary() -> dict[str, object]:
    with tempfile.TemporaryDirectory(prefix="runner-stability-canary-") as temporary:
        paths = CanaryPaths(Path(temporary))
        completed_id = execution_identity("canary-turn-1", "complete")
        _, completed_claim = claim_execution(paths, completed_id, "canary-turn-1")
        finish_execution(paths, completed_id, 0, {"agent_messages": ["complete"]})
        completed_replay, completed_reclaimed = claim_execution(paths, completed_id, "canary-turn-1")

        interrupted_id = execution_identity("canary-turn-2", "interrupt")
        _, interrupted_claim = claim_execution(paths, interrupted_id, "canary-turn-2")
        interrupted_replay, interrupted_reclaimed = claim_execution(paths, interrupted_id, "canary-turn-2")

        failed_id = execution_identity("canary-turn-3", "failed prompt")
        recovery_id = execution_identity("canary-turn-3", "recovery prompt")
        _, failed_claim = claim_execution(paths, failed_id, "canary-turn-3")
        _, recovery_claim = claim_execution(paths, recovery_id, "canary-turn-3")

        checks = {
            "completed_result_is_reused": completed_claim
            and not completed_reclaimed
            and completed_replay.state == "finished",
            "interrupted_execution_is_not_reclaimed": interrupted_claim
            and not interrupted_reclaimed
            and interrupted_replay.state == "claimed",
            "recovery_prompt_gets_distinct_execution": failed_claim
            and recovery_claim
            and failed_id != recovery_id,
        }
        return {"healthy": all(checks.values()), "checks": checks}
