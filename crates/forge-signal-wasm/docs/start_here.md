# Start Here

This is the shortest path through the shipped resource surface.

## Default Mental Model

For API-backed state, the normal lane is:

1. declare shared request posture with `signals.api(...)`
2. declare one route with `api.url(...)`
3. finish it with `.detail(...)`, `.list(...)`, `.paged(...)`, `.create(...)`,
   `.update(...)`, or `.remove(...)`
4. materialize a line with `family.line(...)`
5. start reading with `line.summary()`

Small example:

```ts
import { createSignals } from "forge-signal-wasm";

const signals = createSignals();

const api = signals.api({
  baseUrl: "/api",
});

const userDetail = api.url("/users/:userId").detail({
  load: ({ userId }) => ({ id: userId, name: `User ${userId}` }),
});

const line = userDetail.line({ userId: "u1" });

console.log(line.summary());
console.log(line.value());
```

## If You Already Know Your Task

- fetch or write ordinary resources:
  [feature_fetch_and_write_api_resources.md](./feature_fetch_and_write_api_resources.md)
- auth, policy, continuation, and request posture:
  [feature_request_posture_and_policy.md](./feature_request_posture_and_policy.md)
- collection patching, summaries, or delivery:
  [feature_collections_and_delivery.md](./feature_collections_and_delivery.md)
- signed or multipart upload, or deferred processing:
  [feature_transfers.md](./feature_transfers.md)
- downloads or multipart downloads:
  [feature_downloads.md](./feature_downloads.md)
- line reads, diagnostics, and history:
  [feature_line_inspection.md](./feature_line_inspection.md)
- exact restore, replay availability, and verification packages:
  [feature_history_and_restore.md](./feature_history_and_restore.md)
- external push packets and basis refresh:
  [feature_external_delivery_and_compatibility.md](./feature_external_delivery_and_compatibility.md)
- raw family declarations:
  [feature_raw_escape_hatch.md](./feature_raw_escape_hatch.md)

## What Not To Do First

Do not start with `signals.resource.*(...)` unless you already know you need
manual identity, raw request shape control, or compatibility-specific behavior.

The raw lane is real and supported. It just should not be the first stop for
ordinary app code.

## Next Reads

- [feature_index.md](./feature_index.md)
- [resource_recipes.md](./resource_recipes.md)
