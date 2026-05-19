import { stableValueDigest } from "./values/value_paths.js";
import { readAttachmentTransfersReport } from "./attachment_transfers/report.js";
import { readResourceDriftReport, noteResourceDrift } from "./resource_drift/report.js";
import { readResourceMergeReport } from "./resource_merge/report.js";
import { readResourceSourceReport } from "./sources/resource_source_report.js";
import { previewResourceMerge as materializeResourceMergePreview } from "./resource_merge/projection.js";

export function createResourceSurfaceBindings(context) {
  return Object.freeze({
    resourceSource() {
      return readResourceSourceReport(context.source);
    },
    resourceMerge() {
      return readResourceMergeReport(context.resourceMerges, context.source);
    },
    previewResourceMerge(request) {
      return materializeResourceMergePreview(
        context.signalNamespace,
        context.source,
        context.resourceMerges,
        context.resourceMergeRegistry,
        request,
      );
    },
    clearResourceMerge(reason = undefined) {
      return context.resourceMerges.clear(reason);
    },
    resourceDrift() {
      syncResourceDrift(context);
      return readResourceDriftReport(context.resourceDrifts, {
        currentSourceDigest: stableValueDigest(context.authoritativeSource()),
        latestCanonicalSourceDigest: context.latestCanonicalSourceDigest(),
      });
    },
    attachmentTransfers() {
      return readAttachmentTransfersReport(
        context.formRef(),
        context.fieldDeclarations,
        context.source,
      );
    },
  });
}

function syncResourceDrift(context) {
  const currentSource = context.authoritativeSource();
  const sourceCompatibility = context.syncSourceCompatibility(currentSource);
  noteResourceDrift(context.resourceDrifts, {
    currentSourceDigest: stableValueDigest(currentSource),
    draft: context.draft(),
    effective: context.effective(),
    resourceSource: readResourceSourceReport(context.source),
    resourceMerge: readResourceMergeReport(context.resourceMerges, context.source),
    sourceCompatibility,
  });
}
