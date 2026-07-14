# Downloads

Use this page when a resource exposes structured value plus downloadable
artifacts.

## What This Covers

- builder-owned `.downloads(...)`
- plain ready downloads
- unavailable and incompatible descriptors
- multipart download handoff
- `line.download()`

## Builder-Owned Download Happy Path

```ts
const manualDetail = api.url("/assets/:assetId")
  .downloads(({ assetId }, _value: { id: string }, download) => [
    download.file("manual-pdf", {
      fileName: `${assetId}.pdf`,
      mediaType: "application/pdf",
      download: download.ready({
        url: `https://downloads.example/${assetId}.pdf`,
      }),
    }),
  ])
  .detail({
    load: ({ assetId }) => ({ id: assetId }),
  });

const line = manualDetail.line({ assetId: "asset-1" });

console.log(line.value());
console.log(line.download());
```

In this pleasant lane, `load(...)` returns the structured value. The builder
owns wrapping it into the binary descriptor surface.

## Multipart Download Happy Path

Use `download.multipart(...)` when the host needs a direct multipart handoff
instead of a plain GET or POST URL.

```ts
const exportDetail = api.url("/exports/:exportId")
  .downloads(({ exportId }, _value: { id: string }, download) => [
    download.export("export-bundle", {
      fileName: `${exportId}.zip`,
      mediaType: "application/zip",
      download: download.multipart({
        url: `https://downloads.example/${exportId}`,
        fields: { token: exportId },
        objectKey: `exports/${exportId}.zip`,
      }),
    }),
  ])
  .detail({
    load: ({ exportId }) => ({ id: exportId }),
  });
```

Multipart download still shows up as a ready download, but with
`transportKind: "directMultipart"` so the host can see the richer handoff
truth.

## Where To Go Next

- upload and processing posture:
  [Transfers](./transfers.md)
- line reads and diagnostics:
  [Line Inspection](./line-inspection.md)
- binary/download reference details:
  [Resource Binary And Download Reference](../api-reference/resource-binary-and-download.md)
