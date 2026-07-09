import { requireResourceEffectProfile } from "./resource_effect_profile.js";
import {
  readTaggedRequestSourceResolution,
} from "../requests/request_source_metadata.js";

function resolveResourceEffectProfile(input, params, family) {
  const tagged = readTaggedRequestSourceResolution(input, params);
  if (tagged !== null) {
    return Object.freeze({
      value: requireResourceEffectProfile(tagged.value, family),
      source: tagged.source,
    });
  }
  if (input === undefined) {
    return Object.freeze({
      value: null,
      source: Object.freeze({ source: "default.effects", overridden: false }),
    });
  }
  return Object.freeze({
    value: requireResourceEffectProfile(
      typeof input === "function" ? input(params) : input,
      family,
    ),
    source: Object.freeze({ source: "endpoint.effects", overridden: false }),
  });
}

export { resolveResourceEffectProfile };
