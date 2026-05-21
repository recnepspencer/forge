import {
  createSignals,
  resourceParamIdentity,
  resourceParams,
  resourcePolicyProfiles,
  resourceProcessingJob,
  resourceProcessingResult,
  resourceUploadResult,
  resourceUploadTransport,
} from "../index.js";

const signals = await createSignals({ deployment: "mainThreadCompatibility" });

const retryingDetail = signals.resource.detail({
  params: resourceParams<{ productId: string }>(),
  policy: resourcePolicyProfiles.retryOnce(),
  normalizeParams: ({ productId }) =>
    resourceParamIdentity({ productId }, productId),
  load: ({ productId }) => ({ id: productId }),
});

const timeoutDetail = signals.resource.detail({
  params: resourceParams<{ productId: string }>(),
  policy: resourcePolicyProfiles.timeoutFast(),
  normalizeParams: ({ productId }) =>
    resourceParamIdentity({ productId }, productId),
  load: ({ productId }) => ({ id: productId }),
});

const report = signals.resource.detail({
  params: resourceParams<{ reportId: string }>(),
  processingJob: resourceProcessingJob.poll(),
  normalizeParams: ({ reportId }) =>
    resourceParamIdentity({ reportId }, reportId),
  load: ({ reportId }) =>
    reportId === "ready"
      ? { id: reportId, status: "ready" as const }
      : resourceProcessingResult.accepted({
          jobId: `job:${reportId}`,
          message: "queued",
        }),
});

const reportLine = report.line({ reportId: "queued" });
const reportValue = reportLine.value();
const reportProcessing = reportLine.processing();

const asyncDetail = signals.resource.detail({
  params: resourceParams<{ productId: string }>(),
  policy: resourcePolicyProfiles.retryOnce(),
  normalizeParams: ({ productId }) =>
    resourceParamIdentity({ productId }, productId),
  load: async ({ productId }) => ({ id: productId }),
});

const asyncDetailLine = asyncDetail.line({ productId: "queued" });
const asyncDetailValue = asyncDetailLine.value();
const asyncDetailSignal = asyncDetailLine.signal();
const asyncDetailHistory = asyncDetailLine.history();

const receiptUpload = signals.resource.detail({
  params: resourceParams<{ receiptId: string }>(),
  uploadTransport: resourceUploadTransport.signed({
    method: "PUT",
    finalizeRequired: true,
  }),
  normalizeParams: ({ receiptId }) =>
    resourceParamIdentity({ receiptId }, receiptId),
  load: ({ receiptId }) =>
    resourceUploadResult.prepared({
      uploadId: `upload:${receiptId}`,
      descriptor: {
        kind: "signed",
        url: `https://uploads.example/${receiptId}`,
        method: "PUT",
        headers: { "x-upload-token": "demo" },
        fields: {},
        objectKey: `receipts/${receiptId}.png`,
        expiresAt: "2026-05-04T12:00:00Z",
      },
      finalizeRequired: true,
      message: "ready to upload",
    }),
});

const receiptLine = receiptUpload.line({ receiptId: "r1" });
const receiptValue = receiptLine.value();
const receiptUploadState = receiptLine.upload();

const receiptPipeline = signals.resource.detail({
  params: resourceParams<{ receiptId: string }>(),
  processingJob: resourceProcessingJob.poll(),
  uploadTransport: resourceUploadTransport.signed({
    method: "POST",
    finalizeRequired: true,
  }),
  normalizeParams: ({ receiptId }) =>
    resourceParamIdentity({ receiptId }, receiptId),
  load: ({ receiptId }) =>
    receiptId === "done"
      ? { id: receiptId, status: "ready" as const }
      : resourceUploadResult.uploaded({
          uploadId: `upload:${receiptId}`,
          finalizeRequired: true,
          awaitingProcessing: true,
          message: "processing upload",
        }),
});

const receiptPipelineLine = receiptPipeline.line({ receiptId: "queued" });
const receiptPipelineValue = receiptPipelineLine.value();
const receiptPipelineProcessing = receiptPipelineLine.processing();
const receiptPipelineUpload = receiptPipelineLine.upload();

void retryingDetail;
void timeoutDetail;
void reportValue;
void reportProcessing;
void asyncDetailValue;
void asyncDetailSignal;
void asyncDetailHistory.lifecycle;
void receiptValue;
void receiptUploadState;
void receiptPipelineValue;
void receiptPipelineProcessing;
void receiptPipelineUpload;
