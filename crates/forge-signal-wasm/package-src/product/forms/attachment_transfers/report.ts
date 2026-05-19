import { readResourceSourceReport } from "../sources/resource_source_report.js";
import { stableValueDigest } from "../values/value_paths.js";

export function readAttachmentTransfersReport(form, fieldDeclarations, source) {
  const attachmentFields = fieldDeclarations.filter((field) => (
    field.family === "attachment" || field.family === "evidence"
  ));
  const resourceSource = readResourceSourceReport(source);
  const fields = Object.freeze(
    attachmentFields.map((field) => readAttachmentTransferFieldReport(
      form,
      field,
      resourceSource,
      attachmentFields.length,
    )),
  );
  const summary = Object.freeze({
    totalFields: fields.length,
    binaryFields: fields.length,
    attachmentFields: fields.filter((field) => field.fieldFamily === "attachment").length,
    evidenceFields: fields.filter((field) => field.fieldFamily === "evidence").length,
    transferSurfaceFields: fields.filter((field) => field.bindingKind !== "outsideTransferSurface").length,
    busyFields: fields.filter((field) => field.status === "busy").length,
    unavailableFields: fields.filter((field) => field.status === "unavailable").length,
    mappingUnavailableFields: fields.filter((field) => field.bindingKind === "mappingUnavailable").length,
  });
  const counters = Object.freeze({
    costBasis: "attachmentTransferDerivedScan",
    incrementalStatus: "notIncremental",
    binaryFields: fields.length,
    attachmentFields: summary.attachmentFields,
    evidenceFields: summary.evidenceFields,
    transferSurfaceFields: summary.transferSurfaceFields,
    mappedFields: fields.filter((field) => (
      field.bindingKind === "resourceTransfer" || field.bindingKind === "noAttachment"
    )).length,
    busyFields: summary.busyFields,
    unavailableFields: summary.unavailableFields,
  });
  return Object.freeze({
    fields,
    summary,
    counters,
    digest: stableValueDigest({ fields, summary, counters }),
  });
}

export function attachmentTransferReadinessBlockers(report, actionId = undefined) {
  const shared = actionId === undefined ? {} : { action: actionId };
  return Object.freeze(
    report.fields.flatMap((field) => {
      if (field.bindingKind === "mappingUnavailable" && field.transferActive) {
        return [Object.freeze({
          kind: "resource:transferMappingUnavailable",
          field: field.field,
          ...shared,
          reason: field.reason,
        })];
      }
      if (field.bindingKind !== "resourceTransfer") {
        return [];
      }
      if (field.processing !== null && field.processing.kind !== "ready") {
        return [Object.freeze({
          kind: "resource:processingPending",
          field: field.field,
          ...shared,
          reason: `attachment field ${field.field} is waiting on resource processing`,
        })];
      }
      if (field.upload !== null && field.upload.kind !== "ready") {
        return [Object.freeze({
          kind: "resource:uploadPending",
          field: field.field,
          ...shared,
          reason: `attachment field ${field.field} is waiting on resource upload`,
        })];
      }
      return [];
    }),
  );
}

function readAttachmentTransferFieldReport(form, field, resourceSource, attachmentFieldCount) {
  const attachment = form.fields[field.name].attachmentIdentity();
  const fieldLabel = binaryFieldLabel(field);
  if (attachment === null) {
    if (resourceSource === null) {
      return createAttachmentTransferFieldReport(field, attachment, {
        bindingKind: "outsideTransferSurface",
        status: "unavailable",
        reason: `${fieldLabel} is outside the resource transfer surface because the form source is not a resource line`,
        transferActive: false,
        upload: null,
        processing: null,
        matchingDownloadDescriptors: Object.freeze([]),
        matchingDescriptorCount: 0,
        readyDescriptorCount: 0,
        unavailableDescriptorCount: 0,
        incompatibleDescriptorCount: 0,
      });
    }
    return createAttachmentTransferFieldReport(field, attachment, {
      bindingKind: "noAttachment",
      status: "ready",
      reason: `${fieldLabel} has no attached value to project onto resource transfer posture`,
      transferActive: false,
      upload: null,
      processing: null,
      matchingDownloadDescriptors: Object.freeze([]),
      matchingDescriptorCount: 0,
      readyDescriptorCount: 0,
      unavailableDescriptorCount: 0,
      incompatibleDescriptorCount: 0,
    });
  }
  if (resourceSource === null) {
    return createAttachmentTransferFieldReport(field, attachment, {
      bindingKind: "outsideTransferSurface",
      status: "unavailable",
      reason: `${fieldLabel} is outside the resource transfer surface because the form source is not a resource line`,
      transferActive: false,
      upload: null,
      processing: null,
      matchingDownloadDescriptors: Object.freeze([]),
      matchingDescriptorCount: 0,
      readyDescriptorCount: 0,
      unavailableDescriptorCount: 0,
      incompatibleDescriptorCount: 0,
    });
  }
  const transfer = resourceSource.transfer;
  const matchingDownloadDescriptors = Object.freeze(
    transfer.download.descriptors.filter((descriptor) => descriptor.id === attachment.attachmentDigest),
  );
  const matchingDescriptorCount = matchingDownloadDescriptors.length;
  const readyDescriptorCount = matchingDownloadDescriptors.filter(
    (descriptor) => descriptor.download.kind === "ready",
  ).length;
  const unavailableDescriptorCount = matchingDownloadDescriptors.filter(
    (descriptor) => descriptor.download.kind === "unavailable",
  ).length;
  const incompatibleDescriptorCount = matchingDownloadDescriptors.filter(
    (descriptor) => descriptor.download.kind === "incompatible",
  ).length;
  const transferActive = transfer.summary.uploadActive || transfer.summary.processingActive;
  if (transferActive && attachmentFieldCount !== 1) {
    return createAttachmentTransferFieldReport(field, attachment, {
      bindingKind: "mappingUnavailable",
      status: "unavailable",
      reason: "resource line transfer posture is line-scoped and cannot be projected onto multiple binary fields",
      transferActive,
      upload: transfer.upload,
      processing: transfer.processing,
      matchingDownloadDescriptors,
      matchingDescriptorCount,
      readyDescriptorCount,
      unavailableDescriptorCount,
      incompatibleDescriptorCount,
    });
  }
  if (transfer.download.count > 0 && matchingDescriptorCount === 0) {
    return createAttachmentTransferFieldReport(field, attachment, {
      bindingKind: "mappingUnavailable",
      status: "unavailable",
      reason: `resource line binary descriptors do not include the declared identity for ${fieldLabel}`,
      transferActive,
      upload: transfer.upload,
      processing: transfer.processing,
      matchingDownloadDescriptors,
      matchingDescriptorCount,
      readyDescriptorCount,
      unavailableDescriptorCount,
      incompatibleDescriptorCount,
    });
  }
  return createAttachmentTransferFieldReport(field, attachment, {
    bindingKind: "resourceTransfer",
    status: resolveAttachmentTransferStatus(transfer, {
      readyDescriptorCount,
      unavailableDescriptorCount,
      incompatibleDescriptorCount,
    }),
    reason: resolveAttachmentTransferReason(fieldLabel, transfer, {
      readyDescriptorCount,
      unavailableDescriptorCount,
      incompatibleDescriptorCount,
    }),
    transferActive,
    upload: transfer.upload,
    processing: transfer.processing,
    matchingDownloadDescriptors,
    matchingDescriptorCount,
    readyDescriptorCount,
    unavailableDescriptorCount,
    incompatibleDescriptorCount,
  });
}

function createAttachmentTransferFieldReport(field, attachment, report) {
  const artifact = {
    field: field.name,
    path: field.path,
    fieldFamily: field.family,
    attachmentDigest: attachment?.attachmentDigest ?? null,
    attachmentPresent: attachment !== null,
    metadata: attachment?.metadata ?? field.attachment.metadata,
    bindingKind: report.bindingKind,
    status: report.status,
    reason: report.reason,
    transferActive: report.transferActive,
    upload: report.upload,
    processing: report.processing,
    matchingDownloadDescriptors: report.matchingDownloadDescriptors,
    matchingDescriptorCount: report.matchingDescriptorCount,
    readyDescriptorCount: report.readyDescriptorCount,
    unavailableDescriptorCount: report.unavailableDescriptorCount,
    incompatibleDescriptorCount: report.incompatibleDescriptorCount,
  };
  return Object.freeze({
    ...artifact,
    digest: stableValueDigest(artifact),
  });
}

function resolveAttachmentTransferStatus(transfer, counts) {
  if (transfer.processing.kind === "accepted" || transfer.processing.kind === "processing") {
    return "busy";
  }
  if (transfer.upload.kind === "prepared" || transfer.upload.kind === "uploaded") {
    return "busy";
  }
  if (counts.readyDescriptorCount > 0 || transfer.download.count === 0) {
    return "ready";
  }
  return "unavailable";
}

function resolveAttachmentTransferReason(fieldId, transfer, counts) {
  if (transfer.processing.kind === "accepted" || transfer.processing.kind === "processing") {
    return `${fieldId} is waiting on resource processing`;
  }
  if (transfer.upload.kind === "prepared" || transfer.upload.kind === "uploaded") {
    return `${fieldId} is waiting on resource upload`;
  }
  if (counts.unavailableDescriptorCount > 0 && counts.readyDescriptorCount === 0) {
    return `${fieldId} has no ready resource download descriptor`;
  }
  if (counts.incompatibleDescriptorCount > 0 && counts.readyDescriptorCount === 0) {
    return `${fieldId} is backed by an incompatible resource download descriptor`;
  }
  return `${fieldId} is aligned with resource-owned transfer posture`;
}

function binaryFieldLabel(field) {
  return field.family === "evidence"
    ? `evidence field ${field.name}`
    : `attachment field ${field.name}`;
}
