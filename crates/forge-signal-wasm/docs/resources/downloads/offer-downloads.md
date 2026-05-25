# Offer Downloads

Use this page when a route should expose downloadable artifacts beside normal
structured resource value.

## Stable Entry Points

- `.downloads(...)`
- `download.file(...)`
- `download.media(...)`
- `download.export(...)`
- `line.download()`

## Example

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

## Why This Matters

The route keeps:

- the structured resource value
- the binary descriptor surface

as two separate first-class reads instead of mixing them into one opaque blob.

## Related Docs

- [Describe Binary Values](./describe-binary-values.md)
- [File, Media, And Export Downloads](./file-media-and-export-downloads.md)
