# Understand Transfer Results

Use this page when you need to read what an upload or processing endpoint
returned.

## Upload Results

`line.upload()` can be:

- `ready`
- `prepared`
- `uploaded`

## Processing Results

`line.processing()` can be:

- `ready`
- `accepted`
- `processing`

## Example

```ts
console.log(line.upload());
console.log(line.processing());
```

## What The Common States Mean

- `prepared`
  The server returned an upload descriptor the host can use.
- `uploaded`
  The upload already completed, and the route may now be waiting on downstream
  processing.
- `accepted`
  The work was queued but not finished.
- `processing`
  The work is still running.
- `ready`
  There is no current pending transfer handoff or processing job.

## Related Docs

- [Upload Files](./upload-files.md)
- [Track Processing Jobs](./track-processing-jobs.md)
