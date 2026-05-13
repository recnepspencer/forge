import { createInitialLineBinding } from "../../lines/actions/initial_load_settlement.js";
import {
  createMutationResponseTargetBasisSnapshots,
} from "../../mutation/resource_mutation_response_target_basis.js";
import {
  createSubmittedMutationResponseIdentityMigration,
} from "../../mutation/identity/resource_mutation_response_identity_migration.js";

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
          submittedIdentityMigration:
            mutationResponseDeclaration.identityMigration === null
              ? null
              : createSubmittedMutationResponseIdentityMigration(
                  mutationResponseDeclaration.identityMigration,
                  params,
                ),
        }),
  );
}

function createSeededBinding(
  params,
  lineScope,
  kind,
  seededValue,
  policy,
  requestDescriptor,
  lifecycle,
  lifecycleHistory,
) {
  return createInitialLineBinding(
    () => seededValue,
    params,
    lineScope,
    kind,
    policy,
    requestDescriptor,
    lifecycle,
    lifecycleHistory,
    null,
  );
}

export { createBinding, createSeededBinding };
