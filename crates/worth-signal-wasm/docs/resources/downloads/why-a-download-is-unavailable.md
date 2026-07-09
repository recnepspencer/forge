# Why A Download Is Unavailable

Use this page when a descriptor exists but the host should not treat it as
actionable yet.

## Download States

Descriptor download truth can be:

- `ready`
- `unavailable`
- `incompatible`

## What They Mean

- `ready`
  The descriptor is actionable now.
- `unavailable`
  The artifact is not ready or otherwise unavailable.
- `incompatible`
  The descriptor exists, but the current transport or boundary cannot use it
  safely.

The shipped incompatible reasons are:

- `staleDescriptor`
- `transportBoundary`

## Stable Entry Points

- `line.download()`
- `line.diagnostics().download`
- `download.unavailable(...)`
- `download.incompatible(...)`

## Related Docs

- [Describe Binary Values](./describe-binary-values.md)
- [File, Media, And Export Downloads](./file-media-and-export-downloads.md)
