import {
  isResourcePolicyProfile,
  resourcePolicyProfiles,
} from "./resource_policy_profiles.js";

function resolveResourcePolicyProfile(value, family) {
  if (value === undefined) {
    return createResolvedPolicyProfile("stable");
  }
  if (!isResourcePolicyProfile(value)) {
    throw new TypeError(
      `${family} resources require policy created with resourcePolicyProfiles.*()`,
    );
  }
  if (value === resourcePolicyProfiles.immediatelyStale()) {
    return createResolvedPolicyProfile("immediatelyStale");
  }
  if (value === resourcePolicyProfiles.retryOnce()) {
    return createResolvedPolicyProfile("retryOnce");
  }
  if (value === resourcePolicyProfiles.timeoutFast()) {
    return createResolvedPolicyProfile("timeoutFast");
  }
  return createResolvedPolicyProfile("stable");
}

function createResolvedPolicyProfile(name) {
  return Object.freeze({
    profileName: name,
    staleAfterSettle: name === "immediatelyStale",
    preserveVisibleValueOnReject: true,
    retryLimit: name === "retryOnce" ? 1 : 0,
    timeoutMs: name === "timeoutFast" ? 0 : null,
  });
}

export { resolveResourcePolicyProfile };
