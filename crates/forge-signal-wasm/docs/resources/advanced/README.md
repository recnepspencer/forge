# Advanced Resource Modeling

Use this section when the pleasant `signals.api(...).url(...)...` lane is not
quite enough and you need to model the resource surface directly.

## Start Here

- [Resource Family Identity](./resource-family-identity.md)
  Canonical keys, line reuse, and family identity boundaries.
- [Request Targets And Identity](./request-targets-and-identity.md)
  Raw family declarations, request posture fields, and canonical param targets.
- [Detail Fields, Regions, And Json Paths](./detail-fields-regions-and-json-paths.md)
  Narrow detail updates beyond broad replacement.
- [Item Aspects And Value Summaries](./item-aspects-and-value-summaries.md)
  Narrow list and paged updates plus summary declarations.
- [Raw Resource Lines](./raw-resource-lines.md)
  The escape hatch when you need direct `signals.resource.*(...)` control.

## Main Mental Model

The advanced lane is not a different runtime. It is the same runtime with more
authoring responsibility.

## Related Docs

- [Raw Escape Hatch](../raw-escape-hatch.md)
- [Reconciliation Contract](../../resource-contracts/reconciliation.md)
