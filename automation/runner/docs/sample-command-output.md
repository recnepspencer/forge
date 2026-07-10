# Sample Command Output

## What This Feature Is

This page shows representative output shapes for runner inspection commands.
Use it when wiring scripts or when a new operator wants to know what a healthy
or unhealthy response looks like.

## `report`

Text mode:

```text
Run store-m1-run-1: active
Events: 42
Current: phase 6 review
Latest: provider crash during review
Next: continue monitoring current run
```

JSON mode:

```json
{
  "run_id": "store-m1-run-1",
  "state": "active",
  "current": {"phase": 6, "turn": "review"},
  "completed_at": null,
  "stopped": false,
  "latest_summary": "provider crash during review",
  "event_count": 42,
  "telegram": {
    "poller_health": {"healthy": true, "error": null, "at": 1783654996.0},
    "latest_inbound_receipt": {"status": "injected", "run_id": "store-m1-run-1"}
  },
  "notification_delivery_failure": null,
  "next_operator_action": "continue monitoring current run"
}
```

## `doctor`

Healthy:

```json
{
  "run_id": "store-m1-run-1",
  "healthy": true,
  "state": "active",
  "findings": [
    {"severity": "info", "code": "no_findings", "message": "no obvious runner health issue found"}
  ]
}
```

Unhealthy Telegram poller:

```json
{
  "run_id": "store-m1-run-1",
  "healthy": false,
  "state": "active",
  "findings": [
    {"severity": "error", "code": "telegram_poller_unhealthy", "message": "poll failed"}
  ]
}
```

`doctor` exits nonzero when `healthy` is false.

## `artifacts`

```json
{
  "run_id": "store-m1-run-1",
  "artifacts": [
    {
      "lane": "events",
      "retention_class": "authority",
      "path": "C:/workspace/automation/runner/runtime/events/store-m1-run-1.jsonl",
      "exists": true,
      "bytes": 12044
    },
    {
      "lane": "logs",
      "retention_class": "observation",
      "path": "C:/workspace/automation/runner/runtime/logs/store-m1-run-1.jsonl",
      "exists": true,
      "bytes": 884120
    }
  ]
}
```

## `archive`

```json
{
  "run_id": "store-m1-run-1",
  "archive_root": "C:/workspace/automation/runner/runtime/archives/store-m1-run-1",
  "copied": {
    "events": "C:/workspace/automation/runner/runtime/archives/store-m1-run-1/events.jsonl",
    "projection": "C:/workspace/automation/runner/runtime/archives/store-m1-run-1/projection.json",
    "config": "C:/workspace/automation/runner/runtime/archives/store-m1-run-1/config.json"
  },
  "pruned": []
}
```

## Related Docs

- [Runner Reporting](runner-reporting.md)
- [Runtime Artifacts And Retention](runtime-artifacts-and-retention.md)
