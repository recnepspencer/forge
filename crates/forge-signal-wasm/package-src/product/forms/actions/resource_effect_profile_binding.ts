export function resolveResourceEffectProfileBinding(declaration, resourceSource, actionId) {
  const declared = declaration.resourceEffectProfile === null
    ? null
    : profileDigest(declaration.resourceEffectProfile);
  const effective = resourceSource?.effectProfile.profile ?? null;
  if (declared === null && effective === null) {
    return binding("none", declared, effective, null);
  }
  if (declared === null) {
    return binding("inheritedFromResourceLine", declared, effective, resourceSource.effectProfile.closeoutMatrixDigest);
  }
  if (resourceSource === null) {
    return binding(
      "declaredWithoutResourceLine",
      declared,
      null,
      null,
      blocker("resource:profileUnavailable", actionId, "action declares a resource effect profile without a resource line source"),
    );
  }
  if (effective === null) {
    return binding(
      "declaredWithoutLineEffectProfile",
      declared,
      null,
      null,
      blocker("resource:profileUnavailable", actionId, "resource line request has no inherited resource effect profile"),
    );
  }
  if (!sameProfileDigest(declared, effective)) {
    return binding(
      "declaredMismatchedResourceLine",
      declared,
      effective,
      resourceSource.effectProfile.closeoutMatrixDigest,
      blocker("resource:profileMismatch", actionId, "declared resource effect profile does not match the backing resource line request profile"),
    );
  }
  return binding(
    "declaredMatchesResourceLine",
    declared,
    effective,
    resourceSource.effectProfile.closeoutMatrixDigest,
  );
}

export function isResolvedResourceEffectProfile(resourceEffectProfile) {
  return (
    resourceEffectProfile !== null &&
    typeof resourceEffectProfile === "object" &&
    Object.hasOwn(resourceEffectProfile, "declared") &&
    Object.hasOwn(resourceEffectProfile, "effective") &&
    Object.hasOwn(resourceEffectProfile, "source")
  );
}

export function profileDigest(profile) {
  return Object.freeze({
    name: profile.name,
    optimism: profile.optimism,
    confirmation: profile.confirmation,
    rollback: profile.rollback,
    rebase: profile.rebase,
    preimage: profile.preimage,
  });
}

function binding(source, declared, effective, closeoutMatrixDigest, blockerArtifact = null) {
  return Object.freeze({
    declared,
    effective,
    source,
    closeoutMatrixDigest,
    blockers: blockerArtifact === null ? Object.freeze([]) : Object.freeze([blockerArtifact]),
  });
}

function blocker(kind, actionId, reason) {
  return Object.freeze({
    kind,
    action: actionId,
    reason,
  });
}

function sameProfileDigest(left, right) {
  return left.name === right.name
    && left.optimism === right.optimism
    && left.confirmation === right.confirmation
    && left.rollback === right.rollback
    && left.rebase === right.rebase
    && left.preimage === right.preimage;
}
