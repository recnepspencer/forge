const RESOURCE_POLICY_PROFILE_BRAND = Symbol(
  "forgeSignal.resourcePolicyProfile",
);

const STABLE_PROFILE = Object.freeze({
  name: "stable",
  [RESOURCE_POLICY_PROFILE_BRAND]: "resourcePolicyProfile",
});

const IMMEDIATELY_STALE_PROFILE = Object.freeze({
  name: "immediatelyStale",
  [RESOURCE_POLICY_PROFILE_BRAND]: "resourcePolicyProfile",
});

const RETRY_ONCE_PROFILE = Object.freeze({
  name: "retryOnce",
  [RESOURCE_POLICY_PROFILE_BRAND]: "resourcePolicyProfile",
});

const TIMEOUT_FAST_PROFILE = Object.freeze({
  name: "timeoutFast",
  [RESOURCE_POLICY_PROFILE_BRAND]: "resourcePolicyProfile",
});

const resourcePolicyProfiles = Object.freeze({
  stable() {
    return STABLE_PROFILE;
  },
  immediatelyStale() {
    return IMMEDIATELY_STALE_PROFILE;
  },
  retryOnce() {
    return RETRY_ONCE_PROFILE;
  },
  timeoutFast() {
    return TIMEOUT_FAST_PROFILE;
  },
});

function isResourcePolicyProfile(value) {
  return (
    Boolean(value) &&
    value[RESOURCE_POLICY_PROFILE_BRAND] === "resourcePolicyProfile"
  );
}

export { isResourcePolicyProfile, resourcePolicyProfiles };
