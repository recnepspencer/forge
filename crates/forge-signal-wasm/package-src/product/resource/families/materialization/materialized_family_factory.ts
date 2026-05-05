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
import { createLineRegistryEntry } from "../../lines/state/line_registry_entry.js";
import { createLineDeliveryState } from "../../lines/state/line_delivery_state.js";
import { createLineRequestState } from "../../requests/line_request_state.js";

function createMaterializedFamily(
  kind,
  signalNamespace,
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
  nextLineCounter,
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
  const requestDescriptor = createResourceRequestDescriptor(
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
  const lifecycleHistory = createLineHistoryState();
  const requestState = createLineRequestState(requestDescriptor);
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
  const release = createLineRelease(
    binding,
    lifecycle,
    linesByCanonicalKey,
    canonicalParamIdentity.canonicalKey,
  );
  const materialization = createLineMaterializationRecord(
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
  );
  recordLineHistoryEntry(lifecycleHistory, binding, "materialized");
  const handle = createMaterializedLineHandle(materialization);
  return createLineRegistryEntry(materialization, handle);
}

function createMaterializedLineHandle(materialization) {
  const baseHandle = createLineHandle(materialization);
  if (materialization.patch.familyKind === "detail") {
    return baseHandle;
  }
  return createPatchCapableLineHandle(baseHandle, materialization);
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

function createLineRelease(
  binding,
  lifecycle,
  linesByCanonicalKey,
  canonicalKey,
) {
  let released = false;
  return () => {
    if (released) {
      return;
    }
    released = true;
    lifecycle.markReleased();
    linesByCanonicalKey.delete(canonicalKey);
    lifecycle.releaseOwnedViews();
    binding.valueSignal.free();
    binding.readableValueSignal.free();
    binding.processingSignal.free();
    binding.uploadSignal.free();
    binding.downloadSignal.free();
    binding.statusSignal.free();
    binding.freshnessSignal.free();
    binding.diagnosticsSignal.free();
  };
}

export { createMaterializedFamily };
