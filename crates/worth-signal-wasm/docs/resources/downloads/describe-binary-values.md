# Describe Binary Values

Use this page when you need the mental model for structured value plus binary
descriptors.

## The Main Idea

The line's structured value stays ordinary app data.

Downloadable artifacts live beside it as descriptors that you read through
`line.download()`.

## Stable Entry Points

- `line.value()`
- `line.download()`
- `line.diagnostics().download`

## Example

```ts
console.log(line.value());
console.log(line.download().descriptors);
```

## Why This Separation Helps

It lets the runtime change download readiness without pretending the business
value changed, and it lets narrow patches preserve download descriptors when
only the structured part changed.

## Related Docs

- [Offer Downloads](./offer-downloads.md)
- [Why A Download Is Unavailable](./why-a-download-is-unavailable.md)
