function createResourceEffectProfileDigest(profile) {
  if (profile === null) {
    return null;
  }
  return Object.freeze({
    name: profile.name,
    optimism: profile.optimism,
    confirmation: profile.confirmation,
    rollback: profile.rollback,
    rebase: profile.rebase,
    preimage: profile.preimage,
  });
}

export { createResourceEffectProfileDigest };
