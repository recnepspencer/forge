import {
  createSignals,
  resourceProcessingJob,
  resourceUploadTransport,
} from "../../../index.js";

const signals = await createSignals({ deployment: "mainThreadCompatibility" });

signals.api({}).url("/receipts/:receiptId/upload")
  .signedUpload({
    method: "POST",
    finalizeRequired: true,
  })
  .create({
    // @ts-expect-error signedUpload(...) owns uploadTransport(...) in the route-first lane
    uploadTransport: resourceUploadTransport.signed({
      method: "POST",
      finalizeRequired: true,
    }),
    load: ({ receiptId, body }: { receiptId: string; body: { fileName: string } }) => ({
      receiptId,
      fileName: body.fileName,
    }),
  });

signals.api({}).url("/reports/:reportId")
  .processing("poll")
  .detail({
    // @ts-expect-error processing(...) owns processingJob(...) in the route-first lane
    processingJob: resourceProcessingJob.poll(),
    load: ({ reportId }) => ({ id: reportId }),
  });

signals.api({}).url("/receipts")
  .signedUpload()
  .items((item: { id: string }) => item.id)
  .list({
    // @ts-expect-error collection-owned signedUpload(...) owns uploadTransport(...) in the final declaration too
    uploadTransport: resourceUploadTransport.signed(),
    load: () => [{ id: "t1" }],
  });

signals.api({}).url("/reports")
  .items((item: { id: string }) => item.id)
  .processing("poll")
  .paged({
    accumulatePage: (existing: Array<{ id: string }>, next: Array<{ id: string }>) =>
      [...existing, ...next],
    // @ts-expect-error collection-owned processing(...) owns processingJob(...) in the final declaration too
    processingJob: resourceProcessingJob.poll(),
    load: () => [{ id: "t1" }],
  });

signals.api({}).url("/receipts")
  .signedUpload()
  // @ts-expect-error upload builders must not stack in one route lane
  .multipartUpload();

signals.api({}).url("/reports")
  .processing("poll")
  // @ts-expect-error processing(...) must not stack in one route lane
  .processing("callback", {
    callbackId: "report-ready",
  });
