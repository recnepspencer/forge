# Resource Binary And Download Reference

If your question is "how do I add downloads?" or "how do I do multipart
download handoff?", start with [Downloads](../resources/downloads.md)
before using this lower-level reference page.

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

Recommended declaration lane:

- `signals.api(...)`
- `api.url(...)`
- `.downloads((params, value, download) => [...])`
- then `.detail(...)`, `.list(...)`, or `.paged(...)`

Helpers:

- `resourceBinaryValue(...)`
- `resourceBinaryDescriptor.file(...)`
- `resourceBinaryDescriptor.media(...)`
- `resourceBinaryDescriptor.export(...)`
- `resourceDownload.ready(...)`
- `resourceDownload.multipart(...)`
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

In the pleasant lane, `downloads(...)` owns descriptor wrapping.

That means:

- `load(...)` returns the structured value
- `.downloads(...)` declares descriptors from that value
- the runtime wraps both into the same binary/download substrate underneath

The raw escape hatch can still return `resourceBinaryValue(...)` directly.

The runtime then:

1. unwraps the structured value into `line.value()`
2. stores descriptor state for `line.download()`
3. records descriptor counts and status in diagnostics and history

## Small Example

```ts
import {
  createSignals,
} from "forge-signal-wasm";

const signals = await createSignals();

const reportDetail = signals.api({
  baseUrl: "/api",
}).url("/reports/:reportId")
  .downloads(({ reportId }, _value, download) => [
    download.file("report-pdf", {
      fileName: `${reportId}.pdf`,
      mediaType: "application/pdf",
      download: download.ready({
        url: `https://downloads.example/${reportId}.pdf`,
        method: "GET",
      }),
    }),
  ])
  .detail({
    load: ({ reportId }) => ({
      id: reportId,
      title: "Quarterly Report",
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
} from "forge-signal-wasm";

const signals = await createSignals();
let downloadReady = false;

const manualDetail = signals.api({
  baseUrl: "/api",
}).url("/assets/:assetId")
  .downloads(({ assetId }, _value, download) => [
    download.file("manual-pdf", {
      fileName: `${assetId}.pdf`,
      mediaType: "application/pdf",
      download: downloadReady
        ? download.ready({
            url: `https://downloads.example/${assetId}.pdf`,
            method: "GET",
          })
        : download.unavailable({
            reason: "notReady",
            detail: "manual is still generating",
          }),
    }),
    download.export("manual-export", {
      fileName: `${assetId}.zip`,
      mediaType: "application/zip",
      download: download.incompatible({
        reason: "transportBoundary",
        detail: "host handoff required",
      }),
    }),
  ])
  .detail({
    load: ({ assetId }) => ({
      id: assetId,
      title: "Manual",
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
- the route-first lane still keeps descriptor declaration separate from the
  structured value

Multipart-ready handoff is also part of the surface. Use it when the host must
submit a ready descriptor through a direct multipart form post instead of a
plain GET or POST URL:

```ts
const exportDetail = signals.api({ baseUrl: "/api" })
  .url("/exports/:exportId")
  .downloads(({ exportId }, _value, download) => [
    download.export("bundle", {
      fileName: `${exportId}.zip`,
      mediaType: "application/zip",
      download: download.multipart({
        url: `https://downloads.example/${exportId}`,
        headers: { authorization: "Bearer host-token" },
        fields: { token: String(exportId) },
        objectKey: `exports/${exportId}.zip`,
      }),
    }),
  ])
  .detail({
    load: ({ exportId }) => ({ id: exportId, status: "ready" }),
  });
```

## How It Relates To Other Features

- API route docs explain the default declaration lane before you add download
  descriptors.
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

- [Route Authoring Reference](./route-authoring.md)
- [Resource Transfers Reference](./resource-transfers.md)
- [Resource Line Reference](./resource-line.md)
- [Resource Request And Policy Reference](./resource-request-and-policy.md)
