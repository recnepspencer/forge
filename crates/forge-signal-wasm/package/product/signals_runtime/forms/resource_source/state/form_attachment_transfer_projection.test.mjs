import assert from "node:assert/strict";
import test from "node:test";

import { withSignals } from "../../action_execution_test_helpers.mjs";
import {
  createAttachmentTransferLineFixture,
  createDownloadDescriptors,
} from "../fixtures/resource_attachment_transfer_line_fixture.mjs";

test("signals.form projects resource-owned attachment transfer posture onto a declared attachment field", async () => {
  await withSignals((signals) => {
    const line = createAttachmentTransferLineFixture({
      value: { evidence: { digest: "file-1", name: "audit.pdf" } },
      upload: {
        kind: "uploaded",
        transportKind: "signed",
        uploadId: "upload-1",
        descriptor: null,
        finalizeRequired: true,
        awaitingProcessing: true,
        message: "upload complete",
      },
      processing: {
        kind: "accepted",
        completionKind: "replaceVisibleValue",
        jobId: "job-1",
        message: "queued for processing",
      },
      download: createDownloadDescriptors("file-1"),
    });
    const form = signals.form({
      source: signals.form.source.resourceLine(line, { id: "attachment-transfer" }),
      fields: ({ evidence }) => ({
        evidence: evidence("evidence", {
          attachmentIdentity: "digest",
          metadata: { required: true },
        }),
      }),
    });

    const report = form.attachmentTransfers();
    assert.equal(report.fields.length, 1);
    assert.equal(report.fields[0].fieldFamily, "evidence");
    assert.equal(report.fields[0].bindingKind, "resourceTransfer");
    assert.equal(report.fields[0].status, "busy");
    assert.equal(report.fields[0].attachmentDigest, "file-1");
    assert.equal(report.fields[0].processing?.kind, "accepted");
    assert.equal(report.fields[0].upload?.kind, "uploaded");
    assert.equal(report.fields[0].readyDescriptorCount, 1);
    assert.equal(report.summary.transferSurfaceFields, 1);
    assert.equal(form.resourceSource()?.transfer.download.readyCount, 1);
    assert.equal(
      form.readiness().blockers.some((blocker) => blocker.kind === "resource:processingPending"),
      true,
    );
    assert.equal(form.diagnostics().attachmentTransfers.digest, report.digest);
    assert.equal(form.verification().digests.attachmentTransferDigest, report.digest);
    assert.equal(
      form.verification().digests.resourceTransferDigest,
      form.resourceSource()?.transfer.digest ?? null,
    );
    assert.equal(
      form.verification().performanceEnvelope.attachmentTransfers.attachmentFields,
      0,
    );
    assert.equal(
      form.verification().performanceEnvelope.attachmentTransfers.evidenceFields,
      1,
    );
  });
});

test("signals.form keeps ambiguous line-scoped transfer posture explicit instead of inventing per-field ownership", async () => {
  await withSignals((signals) => {
    const line = createAttachmentTransferLineFixture({
      value: {
        evidence: { digest: "file-1", name: "audit.pdf" },
        appendix: { digest: "file-2", name: "appendix.pdf" },
      },
      upload: {
        kind: "prepared",
        transportKind: "signed",
        uploadId: "upload-1",
        descriptor: {
          kind: "signed",
          url: "https://example.test/upload",
          method: "PUT",
          headers: {},
          fields: {},
          objectKey: "object-1",
          expiresAt: null,
        },
        finalizeRequired: true,
        awaitingProcessing: false,
        message: "ready to upload",
      },
      processing: { kind: "ready", completionKind: "none", jobId: null, message: null },
      download: { count: 0, readyCount: 0, unavailableCount: 0, incompatibleCount: 0, descriptors: [] },
    });
    const form = signals.form({
      source: signals.form.source.resourceLine(line, { id: "attachment-transfer-ambiguous" }),
      fields: ({ evidence, attachment }) => ({
        evidence: evidence("evidence", { attachmentIdentity: "digest" }),
        appendix: attachment("appendix", { attachmentIdentity: "digest" }),
      }),
    });

    const report = form.attachmentTransfers();
    assert.equal(report.summary.evidenceFields, 1);
    assert.equal(report.summary.attachmentFields, 1);
    assert.equal(report.summary.mappingUnavailableFields, 2);
    assert.equal(report.fields[0].bindingKind, "mappingUnavailable");
    assert.equal(report.fields[1].bindingKind, "mappingUnavailable");
    assert.equal(
      form.readiness().blockers.some((blocker) => blocker.kind === "resource:transferMappingUnavailable"),
      true,
    );
  });
});

test("signals.form leaves non-resource attachment fields explicitly outside the resource transfer surface", async () => {
  await withSignals((signals) => {
    const form = signals.form({
      source: { evidence: { digest: "file-1", name: "audit.pdf" } },
      fields: ({ evidence }) => ({
        evidence: evidence("evidence", { attachmentIdentity: "digest" }),
      }),
    });

    const report = form.attachmentTransfers();
    assert.equal(report.fields[0].fieldFamily, "evidence");
    assert.equal(report.fields[0].bindingKind, "outsideTransferSurface");
    assert.equal(report.fields[0].status, "unavailable");
    assert.equal(
      report.fields[0].reason,
      "evidence field evidence is outside the resource transfer surface because the form source is not a resource line",
    );
    assert.equal(report.summary.transferSurfaceFields, 0);
    assert.equal(form.readiness().blockers.some((blocker) => blocker.kind === "resource:uploadPending"), false);
  });
});

test("signals.form keeps resource transfer posture non-blocking when the effective attachment value is absent", async () => {
  await withSignals((signals) => {
    const line = createAttachmentTransferLineFixture({
      value: { evidence: null },
      upload: {
        kind: "uploaded",
        transportKind: "signed",
        uploadId: "upload-detach",
        descriptor: null,
        finalizeRequired: true,
        awaitingProcessing: true,
        message: "upload complete",
      },
      processing: {
        kind: "accepted",
        completionKind: "replaceVisibleValue",
        jobId: "job-detach",
        message: "queued for processing",
      },
      download: createDownloadDescriptors("file-0"),
    });
    const form = signals.form({
      source: signals.form.source.resourceLine(line, { id: "attachment-transfer-empty" }),
      fields: ({ evidence }) => ({
        evidence: evidence("evidence", {
          attachmentIdentity: "digest",
          metadata: { required: true },
        }),
      }),
    });

    const report = form.attachmentTransfers();
    assert.equal(report.fields[0].fieldFamily, "evidence");
    assert.equal(report.fields[0].bindingKind, "noAttachment");
    assert.equal(report.fields[0].status, "ready");
    assert.equal(report.fields[0].attachmentPresent, false);
    assert.equal(report.fields[0].attachmentDigest, null);
    assert.equal(report.fields[0].upload, null);
    assert.equal(report.fields[0].processing, null);
    assert.equal(
      form.readiness().blockers.some((blocker) => blocker.kind === "resource:processingPending"),
      false,
    );
    assert.equal(
      form.readiness().blockers.some((blocker) => blocker.kind === "resource:uploadPending"),
      false,
    );
  });
});
