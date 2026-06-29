from __future__ import annotations

from dataclasses import dataclass
from enum import Enum


class FailureKind(str, Enum):
    STATE_LOAD = "state_load"
    STATE_VALIDATION = "state_validation"
    PROMPT_RENDER = "prompt_render"
    CODEX_INVOCATION = "codex_invocation"
    CODEX_EXIT = "codex_exit"
    POST_TURN_MERGE = "post_turn_merge"


class RecoveryKind(str, Enum):
    LOCAL_NORMALIZE = "local_normalize"
    BACKUP_RESTORE = "backup_restore"
    CODEX_RECOVERY = "codex_recovery"
    TERMINAL_STOP = "terminal_stop"


@dataclass
class RunnerFault:
    kind: FailureKind
    reason: str
    details: str


class StateValidationFailure(Exception):
    pass


class PromptRenderFailure(Exception):
    pass


class CodexExitFailure(Exception):
    pass


class PostTurnMergeFailure(Exception):
    pass
