import {
  invalidateAllFamilyLines,
  invalidateFamilyLine,
} from "./family_invalidation.js";
import { createFamilyIdentity } from "../../identity/family_identity.js";
import { createRuntimeLineIdentity } from "../../identity/runtime_line_identity.js";
import { createFamilyDefinitionRecord } from "../../lowering/family_definition_record.js";
import { createLineMaterializationRecord } from "../../lowering/line_materialization_record.js";
import { createLinePatchRecord } from "../../lowering/line_patch_record.js";
import { createLineReloadRecord } from "../../lowering/line_reload_record.js";
import { requireCanonicalParamIdentity } from "../../params/param_identity_factory.js";
import { resolveResourcePolicyProfile } from "../../policies/policy_profile_resolution.js";
import { resolveResourceProcessingJobPosture } from "../../processing/processing_job_resolution.js";
import { createResourceRequestDescriptor } from "../../requests/request_descriptor.js";
import { resolveResourceUploadTransportPosture } from "../../uploads/upload_transport_resolution.js";
import {
  resolveResourceAuthPosture,
  resolveResourceContinuationPosture,
  resolveResourceRequestContext,
} from "../../requests/request_posture_resolution.js";
import { createLineHandle } from "../../lines/line_handle.js";
import { createPatchCapableLineHandle } from "../../lines/line_patch_capable_handle.js";
import { createInitialLineBinding } from "../../lines/actions/initial_load_settlement.js";
import { recordLineHistoryEntry } from "../../lines/history/record_line_history_entry.js";
import { lookupOrCreateLine } from "../../lines/line_lookup.js";
import { createLineHistoryState } from "../../lines/history/line_history_state.js";
import { createLineLifecycleState } from "../../lines/state/line_lifecycle_state.js";
import { createLineBackingRef } from "../../lines/state/line_backing_ref.js";
import { createLineRegistryEntry } from "../../lines/state/line_registry_entry.js";
import { createLineDeliveryState } from "../../lines/state/line_delivery_state.js";
import { createLineRequestState } from "../../requests/line_request_state.js";

function createMaterializedFamily(
  kind,
  signalNamespace,
  resourceLineEpoch,
  familyId,
  declaration,
  compatibility,
) {
  const familyScope = signalNamespace.scope(familyId);
  const familyIdentity = createFamilyIdentity(kind, familyId);
  const policy = resolveResourcePolicyProfile(declaration.policy, kind);
  const familyRecord = createFamilyDefinitionRecord(
    familyIdentity,
    declaration,
    familyScope,
    policy,
    declaration.auth,
    declaration.requestContext,
    declaration.continuation,
    declaration.processingJob,
    declaration.uploadTransport,
    compatibility,
  );
  const linesByCanonicalKey = new Map();
  let lineCounter = 0;

  function createLine(rawParams) {
    const canonicalParamIdentity = requireCanonicalParamIdentity(
      familyRecord.declaration.normalizeParams(rawParams),
      kind,
    );
    return lookupOrCreateLine(
      linesByCanonicalKey,
      canonicalParamIdentity.canonicalKey,
      () =>
        createMaterializedLine(
          canonicalParamIdentity,
          linesByCanonicalKey,
          familyIdentity,
          familyRecord,
          familyScope,
          resourceLineEpoch,
          () => {
            lineCounter += 1;
            return lineCounter;
          },
        ),
    );
  }

  return Object.freeze({
    invalidate(rawParams) {
      const canonicalParamIdentity = requireCanonicalParamIdentity(
        familyRecord.declaration.normalizeParams(rawParams),
        kind,
      );
      return invalidateFamilyLine(
        linesByCanonicalKey,
        canonicalParamIdentity.canonicalKey,
      );
    },
    invalidateAll() {
      return invalidateAllFamilyLines(linesByCanonicalKey);
    },
    line: createLine,
  });
}

function createMaterializedLine(
  canonicalParamIdentity,
  linesByCanonicalKey,
  familyIdentity,
  familyRecord,
  familyScope,
  resourceLineEpoch,
  nextLineCounter,
) {
  let sharedRequestState = null;
  const sharedLifecycleHistory = createLineHistoryState();

  function snapshotContinuity(materialization) {
    try {
      return Object.freeze({
        status: materialization.binding.statusSignal(),
        freshness: materialization.binding.freshnessSignal(),
        diagnostics: materialization.binding.diagnosticsSignal(),
      });
    } catch {
      return null;
    }
  }

  function restoreContinuity(materialization, continuitySnapshot) {
    if (continuitySnapshot === null) {
      return;
    }
    materialization.binding.statusSignal.set(continuitySnapshot.status);
    materialization.binding.freshnessSignal.set(continuitySnapshot.freshness);
    materialization.binding.diagnosticsSignal.set(continuitySnapshot.diagnostics);
  }

  function createReplacementMaterialization(requestDescriptorOverride = null) {
    return createConcreteMaterialization(
      canonicalParamIdentity,
      familyIdentity,
      familyRecord,
      familyScope,
      resourceLineEpoch,
      nextLineCounter,
      releaseCurrentLine,
      rematerializeLine,
      sharedLifecycleHistory,
      sharedRequestState,
      requestDescriptorOverride,
    );
  }

  const lineBacking = createLineBackingRef(
    resourceLineEpoch,
    () => createReplacementMaterialization(),
    snapshotContinuity,
    restoreContinuity,
  );
  resourceLineEpoch.register(lineBacking);
  const releaseCurrentLine = createCurrentLineRelease(
    lineBacking,
    resourceLineEpoch,
    linesByCanonicalKey,
    canonicalParamIdentity.canonicalKey,
  );

  function rematerializeLine({
    requestDescriptorOverride = null,
    invalidateNamespace = false,
  } = {}) {
    if (invalidateNamespace) {
      resourceLineEpoch.invalidateAll();
    }
    return lineBacking.forceRematerialize(
      () => createReplacementMaterialization(requestDescriptorOverride),
    );
  }

  const materialization = rematerializeLine();
  sharedRequestState = materialization.requestState;
  recordLineHistoryEntry(
    materialization.lifecycleHistory,
    materialization.binding,
    "materialized",
  );
  const handle = createMaterializedLineHandle(lineBacking);
  return createLineRegistryEntry(lineBacking, handle);
}

function createMaterializedLineHandle(lineBacking) {
  const baseHandle = createLineHandle(lineBacking);
  if (lineBacking.current().patch.familyKind === "detail") {
    return baseHandle;
  }
  return createPatchCapableLineHandle(baseHandle, lineBacking);
}

function createConcreteMaterialization(
  canonicalParamIdentity,
  familyIdentity,
  familyRecord,
  familyScope,
  resourceLineEpoch,
  nextLineCounter,
  release,
  rematerialize,
  lifecycleHistory,
  requestStateOverride,
  requestDescriptorOverride = null,
) {
  const lineCounter = nextLineCounter();
  const lineScope = familyScope.scope(`line${lineCounter}`);
  const runtimeLineId = `${familyIdentity.familyId}.line${lineCounter}`;
  const lineIdentity = createRuntimeLineIdentity(
    familyIdentity,
    canonicalParamIdentity,
    runtimeLineId,
    lineScope.scopeId,
    familyRecord.compatibility ?? null,
  );
  const requestDescriptor =
    requestDescriptorOverride
    ?? createResourceRequestDescriptor(
      lineIdentity,
      resolveResourceAuthPosture(
        familyRecord.auth,
        canonicalParamIdentity.params,
        familyRecord.identity.kind,
      ),
      resolveResourceRequestContext(
        familyRecord.requestContext,
        canonicalParamIdentity.params,
        familyRecord.identity.kind,
      ),
      resolveResourceContinuationPosture(
        familyRecord.continuation,
        canonicalParamIdentity.params,
        familyRecord.identity.kind,
      ),
      resolveResourceProcessingJobPosture(
        familyRecord.processingJob,
        canonicalParamIdentity.params,
        familyRecord.identity.kind,
      ),
      resolveResourceUploadTransportPosture(
        familyRecord.uploadTransport,
        canonicalParamIdentity.params,
        familyRecord.identity.kind,
      ),
    );
  const lifecycle = createLineLifecycleState();
  const requestState =
    requestStateOverride ?? createLineRequestState(requestDescriptor);
  const binding = createBinding(
    canonicalParamIdentity.params,
    lineScope,
    familyRecord.identity.kind,
    familyRecord.declaration.load,
    familyRecord.policy,
    requestDescriptor,
    lifecycle,
    lifecycleHistory,
  );
  const reload = createLineReloadRecord(
    canonicalParamIdentity.params,
    familyRecord.identity.kind,
    familyRecord.declaration.load,
    familyRecord.policy,
    requestState,
  );
  const history = familyScope.history();
  const delivery = createLineDeliveryState();
  const patch = createLinePatchRecord(
    familyRecord.identity.kind,
    familyRecord.declaration.itemIdentity,
    familyRecord.declaration.reconcile ?? null,
  );
  return createLineMaterializationRecord(
    lineIdentity,
    requestDescriptor,
    requestState,
    binding,
    history,
    lifecycleHistory,
    delivery,
    patch,
    lineScope,
      lifecycle,
      reload,
      release,
      rematerialize,
      resourceLineEpoch,
    );
}

function createBinding(
  params,
  lineScope,
  kind,
  load,
  policy,
  requestDescriptor,
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
  );
}

function createCurrentLineRelease(
  lineBacking,
  resourceLineEpoch,
  linesByCanonicalKey,
  canonicalKey,
) {
  let released = false;
  return () => {
    if (released) {
      return;
    }
    released = true;
    linesByCanonicalKey.delete(canonicalKey);
    resourceLineEpoch.unregister(lineBacking);
    const materialization = lineBacking.current();
    if (materialization !== null) {
      disposeLineMaterialization(materialization);
    }
    for (const retiredMaterialization of lineBacking.retired()) {
      disposeLineMaterialization(retiredMaterialization);
    }
  };
}

function disposeLineMaterialization(materialization) {
  materialization.lifecycle.markReleased();
  materialization.lifecycle.releaseOwnedViews();
  materialization.binding.valueSignal.free();
  materialization.binding.readableValueSignal.free();
  materialization.binding.processingSignal.free();
  materialization.binding.uploadSignal.free();
  materialization.binding.downloadSignal.free();
  materialization.binding.statusSignal.free();
  materialization.binding.freshnessSignal.free();
  materialization.binding.diagnosticsSignal.free();
}

export { createMaterializedFamily };
