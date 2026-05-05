# Resource Binary And Download Reference

## What This Feature Is

This is the resource surface for values that also have downloadable files or
media.

Use it when the resource value is still normal structured data, but the UI also
needs download descriptors such as:

- a PDF
- an image or video
- an export archive

## Why You Use It

- keep structured data separate from downloadable files
- model "ready", "not ready yet", and "cannot download this here" explicitly
- inspect download state from the line without flattening everything into one
  app-specific object
- let download readiness change without pretending the whole value changed

## Stable Entry Points

Helpers:

- `resourceBinaryValue(...)`
- `resourceBinaryDescriptor.file(...)`
- `resourceBinaryDescriptor.media(...)`
- `resourceBinaryDescriptor.export(...)`
- `resourceDownload.ready(...)`
- `resourceDownload.unavailable(...)`
- `resourceDownload.incompatible(...)`

Line inspection:

- `line.download()`
- `line.diagnostics().download`
- `line.diagnosticsSummary().download`

## Core Mental Model

There are two separate things here:

- the structured resource value
- the downloadable files attached to that value

So:

- `line.value()` gives you the structured value
- `line.download()` gives you the download view

That split matters because "the export is ready now" is not the same thing as
"the business data changed".

## How It Executes

`load(...)` can return `resourceBinaryValue(...)`.

The runtime then:

1. unwraps the structured value into `line.value()`
2. stores descriptor state for `line.download()`
3. records descriptor counts and status in diagnostics and history

## Small Example

```ts
import {
  createSignals,
  resourceBinaryDescriptor,
  resourceBinaryValue,
  resourceDownload,
  resourceParamIdentity,
  resourceParams,
} from "forge-signal-wasm";

const signals = createSignals();

const reportDetail = signals.resource.detail({
  params: resourceParams<{ reportId: string }>(),
  normalizeParams: ({ reportId }) =>
    resourceParamIdentity({ reportId }, reportId),
  load: ({ reportId }) =>
    resourceBinaryValue({
      value: { id: reportId, title: "Quarterly Report" },
      descriptors: [
        resourceBinaryDescriptor.file({
          id: "report-pdf",
          fileName: `${reportId}.pdf`,
          mediaType: "application/pdf",
          download: resourceDownload.ready({
            url: `https://downloads.example/${reportId}.pdf`,
            method: "GET",
          }),
        }),
      ],
    }),
});

const line = reportDetail.line({ reportId: "q1" });
console.log(line.value());
console.log(line.download());
```

## Real Example

```ts
import {
  createSignals,
  resourceBinaryDescriptor,
  resourceBinaryValue,
  resourceDownload,
  resourceParamIdentity,
  resourceParams,
} from "forge-signal-wasm";

const signals = createSignals();
let downloadReady = false;

const manualDetail = signals.resource.detail({
  params: resourceParams<{ assetId: string }>(),
  normalizeParams: ({ assetId }) =>
    resourceParamIdentity({ assetId }, assetId),
  load: ({ assetId }) =>
    resourceBinaryValue({
      value: { id: assetId, title: "Manual" },
      descriptors: [
        resourceBinaryDescriptor.file({
          id: "manual-pdf",
          fileName: `${assetId}.pdf`,
          mediaType: "application/pdf",
          download: downloadReady
            ? resourceDownload.ready({
                url: `https://downloads.example/${assetId}.pdf`,
                method: "GET",
              })
            : resourceDownload.unavailable({
                reason: "notReady",
                detail: "manual is still generating",
              }),
        }),
        resourceBinaryDescriptor.export({
          id: "manual-export",
          fileName: `${assetId}.zip`,
          mediaType: "application/zip",
          download: resourceDownload.incompatible({
            reason: "transportBoundary",
            detail: "host handoff required",
          }),
        }),
      ],
    }),
});

const line = manualDetail.line({ assetId: "asset-1" });
console.log(line.download());

downloadReady = true;
line.refresh();
console.log(line.value());
console.log(line.download());
```

This example shows two common cases:

- a file that is not ready yet
- a file that exists but cannot be downloaded through the current transport

## How It Relates To Other Features

- Transfer docs explain upload and processing before the final download state
  exists.
- Line docs explain where `download()` sits in the full line surface.

## Inspection And Debugging

Use:

- `line.download()`
- `line.diagnostics().download`
- `line.diagnosticsSummary().download`

These tell you:

- how many descriptors exist
- how many are ready, unavailable, or incompatible
- whether download state changed without the main value changing

## Anti-Patterns

- mixing download descriptor state into the structured value just because the UI
  reads them together
- treating `incompatible` as just another way to say "not ready yet"
- wrapping upload or processing result objects in `resourceBinaryValue(...)`

## Current Limits

- this surface models downloadable artifacts
- it is not a general-purpose file transfer system

## Related Docs

- [resource_transfers_reference.md](./resource_transfers_reference.md)
- [resource_line_reference.md](./resource_line_reference.md)
- [resource_request_and_policy_reference.md](./resource_request_and_policy_reference.md)
