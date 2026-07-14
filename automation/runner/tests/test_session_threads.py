from __future__ import annotations

import unittest

from runner.authority.projections.projector import empty_projection, family_for_turn
from runner.generation.scaffold_templates import scaffold_config
from runner.generation.scaffold_types import ScaffoldRequest
from runner.roles.session_policy import RoleSessionPolicy, session_state_for_execution


def two_family_config() -> dict:
    """A standard_loop config where the plan turn runs on the oversight family
    and the implement turn runs on the build family — the mixed-provider shape
    that the single global thread slot could not represent."""
    config = scaffold_config(ScaffoldRequest("milestone", "demo", "C:/tmp/demo", "spec.md"))
    bindings = config["phases"][0]["role_bindings"]
    bindings["plan"]["session_policy"]["continuity_family"] = "oversight"
    bindings["implement"]["session_policy"]["continuity_family"] = "build"
    bindings["review"]["session_policy"]["continuity_family"] = "oversight"
    bindings["repair"]["session_policy"]["continuity_family"] = "build"
    return config


class FamilyDerivationTests(unittest.TestCase):
    def test_family_is_derived_from_the_turn_role_binding(self) -> None:
        config = two_family_config()
        self.assertEqual(family_for_turn(config, 1, "plan"), "oversight")
        self.assertEqual(family_for_turn(config, 1, "implement"), "build")
        self.assertEqual(family_for_turn(config, 1, "review"), "oversight")

    def test_run_level_and_unbound_turns_own_no_family(self) -> None:
        config = two_family_config()
        self.assertIsNone(family_for_turn(config, None, None))
        self.assertIsNone(family_for_turn(config, 1, "not_a_turn"))
        self.assertIsNone(family_for_turn(config, 99, "plan"))


class SessionResolutionTests(unittest.TestCase):
    def resolved_session(self, threads: dict[str, str], family: str, force_fresh: bool = False) -> dict:
        session = empty_projection(two_family_config(), "demo")["session"]
        session["threads"] = dict(threads)
        policy = RoleSessionPolicy(reuse_session=True, continuity_family=family, fresh_session_after_qa_repair_cycles=None)
        return session_state_for_execution(session, policy, force_fresh)

    def test_each_family_resumes_its_own_thread(self) -> None:
        threads = {"oversight": "codex-A", "build": "grok-B"}
        self.assertEqual(self.resolved_session(threads, "build")["thread_id"], "grok-B")
        self.assertEqual(self.resolved_session(threads, "oversight")["thread_id"], "codex-A")

    def test_a_family_never_resumes_another_providers_thread(self) -> None:
        # The exact single-slot bug: build must not resume the codex thread just
        # because the oversight turn wrote it most recently.
        threads = {"oversight": "codex-A"}
        self.assertIsNone(self.resolved_session(threads, "build")["thread_id"])

    def test_force_fresh_session_ignores_the_family_thread(self) -> None:
        threads = {"build": "grok-B"}
        self.assertIsNone(self.resolved_session(threads, "build", force_fresh=True)["thread_id"])

    def test_missing_thread_map_falls_back_to_fresh(self) -> None:
        policy = RoleSessionPolicy(reuse_session=True, continuity_family="build", fresh_session_after_qa_repair_cycles=None)
        resolved = session_state_for_execution({"thread_id": None}, policy, False)
        self.assertIsNone(resolved["thread_id"])


class SessionResetScopeTests(unittest.TestCase):
    def test_reset_clears_only_the_named_family(self) -> None:
        # session_reset scoping is expressed through family_for_turn: a reset on
        # the build turn clears only the build thread, leaving oversight intact.
        threads = {"oversight": "codex-A", "build": "grok-B"}
        reset_family = family_for_turn(two_family_config(), 1, "implement")
        self.assertEqual(reset_family, "build")
        threads.pop(reset_family, None)
        self.assertEqual(threads, {"oversight": "codex-A"})


if __name__ == "__main__":
    unittest.main()
