# Provider Setup

## What This Feature Is

Provider setup explains how runner roles choose Codex, Cursor, or Grok for
agent turns. Use it before a long run so provider binaries, credentials, and
model policy are not discovered as failures mid-run.

## Stable Entry Points

Providers are configured in `session_defaults` and in each phase
`role_bindings.<turn>.model_policy`.

```json
"session_defaults": {
  "provider": "codex",
  "model": "gpt-5",
  "reasoning_effort": "medium",
  "config": {}
}
```

## Supported Providers

- `codex`
- `cursor`
- `grok`

## Codex

Codex requires a model and reasoning effort:

```json
"model_policy": {
  "provider": "codex",
  "model": "gpt-5",
  "reasoning_effort": "medium"
}
```

Check the local Codex CLI before a run:

```powershell
codex --help
```

## Cursor

Cursor uses the Cursor adapter and local Cursor agent entrypoint. A typical
binding still declares provider and model:

```json
"model_policy": {
  "provider": "cursor",
  "model": "default"
}
```

Cursor installations vary by environment. Check the local command or adapter
entrypoint your workspace expects before a long run.

## Grok

Grok can use the default `grok` command or an explicit command path:

```json
"model_policy": {
  "provider": "grok",
  "command": "C:/Users/Esther/.grok/bin/grok.exe",
  "command_args": [],
  "model": "grok-4.5",
  "reasoning_effort": "medium",
  "config": {}
}
```

Check local availability:

```powershell
grok --help
```

Use a model id available to your Grok CLI. If the configured model is not
available locally, the CLI may fail or fall back depending on local Grok
configuration.

## Environment And Process Customization

Provider policies may include:

- `command`: executable path.
- `command_args`: extra command arguments.
- `config`: provider-specific config object.
- `env`: string environment variables passed to the provider process.

Use `env` for local non-secret toggles. Prefer environment or local ignored
files for secrets.

## Anti-Patterns

- Do not start a long run before the provider CLI works by itself.
- Do not assume a generated config proves provider credentials are valid.
- Do not put provider secrets in committed config.

## Related Docs

- [Config Reference](config-reference.md)
- [First Run From Zero](first-run-from-zero.md)
