# Aspect-Native Recovery

Some Query stops are not just "denied." They are denied because the required
aspect slice no longer fits the retained declaration or continuation story.

That is what `ForgeQueryRecoveryAspectPosture` is for.

## The Useful Cases

- `None`
  - aspect truth is not the important recovery axis here
- `RequiredContract`
  - the source lane required a contract and the stop is aspect-sensitive
- `AspectSensitiveReadmission`
  - continuation readmission depends on retained aspect fit
- `CategoryScopedAspectComposition`
  - contribution composition was bound against a declaration-scoped aspect story
- `RetainedContractAndCoverage`
  - proof-visible recovery retained both the declaration contract and what was
    actually covered or published

## Why This Matters

If you flatten aspect-native stops into generic denial, you lose the real fix.

For example:

- a signal `MissingRequiredAspect` stop usually means "repair the declaration
  meaning"
- a continuation `Stale` stop usually means "refresh basis and revalidate the
  retained continuation witness"
- a contribution-composed proof can carry the specific contribution intent that
  failed under retained declaration aspect truth

That is why the recovery brief exposes both:

- `recommended_action()`
- `aspect_posture()`

Use them together.
