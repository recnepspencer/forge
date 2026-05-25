# Compatibility And Readiness

Use this page when you want to know when transfer results are admitted and when
the runtime rejects them.

## The Main Rule

Upload and processing result values are only admitted when the family declared
the matching posture first.

That means:

- upload results need declared upload transport
- processing results need declared processing posture

The posture also has to come from the real helper factories. A hand-built plain
object that merely looks like upload or processing posture is still denied.

## Why This Matters

Without that rule, a route could start returning upload or processing truth
that the declaration never promised.

## Example

If a family returns an upload result without `.signedUpload(...)` or
`.multipartUpload(...)`, the line is denied instead of pretending the transfer
lane existed all along.

The same rule applies to processing results and `.processing(...)`.

The same helper-factory rule applies too:

- use `resourceUploadTransport.*(...)` for raw families
- use `resourceProcessingJob.*(...)` for raw families
- use `.signedUpload(...)`, `.multipartUpload(...)`, and `.processing(...)` on
  the route-first lane

## Related Docs

- [Upload Files](./upload-files.md)
- [Track Processing Jobs](./track-processing-jobs.md)
