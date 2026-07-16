function releaseLine(materialization) {
  const openEffects = materialization.effectBranchDag?.openEffects() ?? [];
  if (openEffects.length > 0) {
    const effectIds = Object.freeze(openEffects.map((effect) => effect.effectId));
    const error = new Error(
      `resource line free() denied because ${effectIds.length} resource effect branch${effectIds.length === 1 ? " is" : "es are"} still open`,
    );
    error.name = "ResourceLineReleaseDenied";
    error.code = "openEffects";
    error.effectIds = effectIds;
    throw error;
  }
  materialization.release();
}

export { releaseLine };
