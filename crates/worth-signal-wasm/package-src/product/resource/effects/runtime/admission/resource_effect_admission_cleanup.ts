import {
  retireAcquiredEffectBranches,
} from "../../branches/resource_effect_branch_acquisition_execution.js";

async function cleanupFailedResourceEffectAdmission(
  history,
  index,
  registered,
  admissionError,
) {
  try {
    await retireAcquiredEffectBranches(
      history,
      registered.branch,
      "superseded",
    );
    index.withdraw(registered.effectId);
  } catch (cleanupError) {
    const failure = new AggregateError(
      [admissionError, cleanupError],
      `resource effect ${registered.effectId} admission and cleanup both failed`,
    );
    failure.effectId = registered.effectId;
    throw failure;
  }
}

export { cleanupFailedResourceEffectAdmission };
