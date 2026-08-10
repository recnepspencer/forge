pub(super) const EXPECTED_COST_USD_TARGET_BASIS: &str = "committed-patch-target|locator=version=bridge.committed-patch-target.v1;domain=locator;entries=[locus=named:aspect_field.aspect_key,kind=locator,value=exact-text:cost;locus=named:aspect_field.authority,kind=locator,value=exact-text:authoritative;locus=named:aspect_field.field_path,kind=locator,value=exact-text:usd;locus=named:aspect_field.kind,kind=locator,value=exact-text:aspect]|mutation-mask=version=bridge.committed-patch-target.v1;domain=aspect-mask;entries=[locus=named:cost.mutation.field.usd,kind=mask,value=exact-text:usd]|projection-mask=version=bridge.committed-patch-target.v1;domain=aspect-mask;entries=[locus=named:cost.projection.field.usd,kind=mask,value=exact-text:usd]|kind=entity-field";

mod aspect_truth;
mod certification_matrix;
mod workload_bundle;
