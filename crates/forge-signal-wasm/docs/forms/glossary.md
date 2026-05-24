# Forms Glossary

- `source`
  The authoritative input value the form started from.
- `draft`
  The user's in-progress edits, stored separately from source truth.
- `effective`
  The value the form currently reads after layering draft on top of source.
- `field locus`
  The stable path and field identity the runtime uses for one form field.
- `semantic dirty`
  The form changed in meaning, not just in touch or object identity.
- `patch plan`
  The derived list of write operations needed to turn source into effective.
- `broad replacement`
  A patch plan that replaces a wider value because a safe narrow patch is not
  honest.
- `parse failure`
  Raw input failed to cross the parse boundary into admitted draft truth.
- `visible message`
  A message artifact that is currently admitted to the user-facing lane.
- `source compatibility`
  The form's posture when source schema versions drift and the draft must be
  preserved, migrated, or denied.
