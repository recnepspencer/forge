# File, Media, And Export Downloads

Use this page when you need to choose the right descriptor kind or understand
multipart download handoff.

## Descriptor Kinds

- `file`
  A normal downloadable file artifact.
- `media`
  A media-oriented artifact such as an image preview.
- `export`
  A generated export artifact, often archive-shaped.

## Stable Entry Points

- `download.file(...)`
- `download.media(...)`
- `download.export(...)`
- `download.ready(...)`
- `download.multipart(...)`

## Example

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

## Multipart Download Truth

Multipart downloads still show up as `ready`, but with
`transportKind: "directMultipart"` so the host can see that it is a richer
handoff than a simple GET or POST URL.

## Related Docs

- [Offer Downloads](./offer-downloads.md)
- [Why A Download Is Unavailable](./why-a-download-is-unavailable.md)
