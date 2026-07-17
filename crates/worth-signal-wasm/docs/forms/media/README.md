# Attachments And Media

Attachment and evidence fields give files stable form identity and produce
attach/detach patch posture. They do not upload bytes.

```ts
const form = signals.form({
  source: { evidence: { digest: "sha256:old", name: "draft.pdf" } },
  fields: ({ evidence }) => ({
    evidence: evidence<{ digest: string; name: string }>("evidence", {
      digest: "digest",
    }),
  }),
});
```

A transfer service or resource line owns upload/download lifecycle. The form
can project resource-owned progress and visibility only when the attachment
identity and resource binding are unambiguous.

Read next:

- [Attachments](./attachments.md)
- [Evidence Fields](./evidence-fields.md)
- [Attachment Transfers](./attachment-transfers.md)
- [Media Visibility](./media-visibility.md)
- [Resource-Backed Forms](../resource-backed/README.md)
