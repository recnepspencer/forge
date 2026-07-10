from __future__ import annotations


def drive_graph_run(*args, **kwargs):
    from runner.graph_runtime.orchestrator import drive_graph_run as _drive_graph_run

    return _drive_graph_run(*args, **kwargs)


__all__ = ["drive_graph_run"]
