import { createInitialLineBinding } from "../../lines/actions/initial_load_settlement.js";
import {
  createMutationResponseTargetBasisSnapshots,
} from "../../mutation/resource_mutation_response_target_basis.js";

function createBinding(
  params,
  lineScope,
  kind,
  load,
  policy,
  requestDescriptor,
  lineIdentity,
  mutationResponseDeclaration,
  lifecycle,
  lifecycleHistory,
) {
  return createInitialLineBinding(
    load,
    params,
    lineScope,
    kind,
    policy,
    requestDescriptor,
    lifecycle,
    lifecycleHistory,
    mutationResponseDeclaration === null
      ? null
      : Object.freeze({
          lineIdentity,
          requestDescriptor,
          declaration: mutationResponseDeclaration,
          submittedTargets: createMutationResponseTargetBasisSnapshots(
            mutationResponseDeclaration,
            params,
          ),
        }),
  );
}

export { createBinding };
