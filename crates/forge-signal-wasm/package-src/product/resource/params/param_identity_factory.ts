import {
  brandCanonicalParamIdentity,
  isCanonicalParamIdentity,
} from "./canonical_param_identity.js";
import { createCanonicalParamSnapshot } from "./canonical_param_snapshot.js";

function resourceParamIdentity(params, canonicalKey) {
  if (typeof canonicalKey !== "string" || canonicalKey.length === 0) {
    throw new TypeError(
      "resourceParamIdentity(...) requires a non-empty canonicalKey",
    );
  }
  return brandCanonicalParamIdentity(
    createCanonicalParamSnapshot(params),
    canonicalKey,
  );
}

function requireCanonicalParamIdentity(value, family) {
  if (!isCanonicalParamIdentity(value)) {
    throw new TypeError(
      `${family} normalizeParams(...) must return resourceParamIdentity(...)`,
    );
  }
  return value;
}

export { requireCanonicalParamIdentity, resourceParamIdentity };
