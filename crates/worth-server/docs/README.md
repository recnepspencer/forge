# worth-server docs

These docs are organized by what you are trying to do, not by internal phase
names.

If you are new here, start with one of these:

- [Read data](./read-data.md)
- [Write data](./write-data.md)
- [Stream results](./stream-results.md)
- [Upload files](./upload-files.md)
- [Download files](./download-files.md)
- [Connect an agent](./connect-an-agent.md)
- [Connect another backend](./connect-another-backend.md)
- [Runtime-backed vs durable](./runtime-backed-vs-durable.md)

How to think about `worth-server`:

- `WorthServer::worth_native()` is the direct Rust surface.
- `WorthServer::compat_http()` is the compatibility boundary that normalizes
  HTTP-shaped requests into canonical WORTH execution.
- The compatibility surface is strict on purpose. It denies dishonest cache,
  basis, validator, upload, and resume claims before they become authority.

The compatibility route families used throughout these docs are:

```text
/compat/reads/{operation}
/compat/mutations/{operation}
/compat/streams/{operation}
/compat/uploads/{operation}
/compat/downloads/{operation}
```

Every task page calls out:

- the stable Rust entry points
- the normalized route family
- the mental model
- what the server refuses to fake
- the current limits you need to design around
