import {
  createSignals,
  resourceProcessingResult,
  resourceUploadResult,
} from "../../../index.js";

const signals = await createSignals({ deployment: "mainThreadCompatibility" });

const api = signals.api({
  baseUrl: "/api",
  headers: {
    authorization: "Bearer shared",
  },
});

const createUser = api.url("/users").create({
  load: ({ body }: { body: { userId: string; name: string } }) => ({
    id: body.userId,
    name: body.name,
  }),
});
const updateUser = api.url("/users/:userId").update({
  load: ({ userId, body }: { userId: string; body: { name: string } }) => ({
    id: userId,
    name: body.name,
  }),
});
const removeUser = api.url("/users/:userId").remove({
  load: ({ userId }) => ({ removed: userId }),
});
const prepareReceiptUpload = api.url("/receipts/:receiptId/upload")
  .signedUpload({
    method: "POST",
    finalizeRequired: true,
  })
  .processing("poll")
  .create({
    load: ({ receiptId, body }: { receiptId: string; body: { fileName: string } }) =>
      resourceUploadResult.prepared({
        uploadId: `upload:${receiptId}`,
        descriptor: {
          kind: "signed",
          url: `https://uploads.example/${receiptId}`,
          method: "POST",
          headers: { "x-upload-token": body.fileName },
          fields: {},
          objectKey: `receipts/${receiptId}`,
          expiresAt: null,
        },
        finalizeRequired: true,
        message: "ready",
      }),
  });
const reportStatus = api.url("/reports/:reportId")
  .processing("callback", {
    callbackId: "report-ready",
  })
  .detail({
    load: ({ reportId }) =>
      resourceProcessingResult.accepted({
        jobId: `job:${reportId}`,
        message: "queued",
      }),
  });
const exportUsers = api.url("/users/export").create({
  load: ({ body }: { body: { jobId: string } }) => ({ jobId: body.jobId }),
});
const responseOwnedPartialUpdate = api.url("/user-status/:userId")
  .response(signals.resource.response.detail<{ status: string }>()({ status: "status" }))
  .update({
    atomicity: "partialAllowed",
    reconciles: [],
    load: ({ userId }: { userId: string; body: {} }) => ({
      id: userId,
      status: "active",
    }),
  });

const createUserLine = createUser.line({
  body: {
    userId: "u2",
    name: "Ada",
  },
});
const updateUserLine = updateUser.line({
  userId: "u1",
  body: {
    name: "Grace",
  },
});
const removeUserLine = removeUser.line({ userId: "u1" });
const prepareReceiptUploadLine = prepareReceiptUpload.line({
  receiptId: "r1",
  body: {
    fileName: "receipt.png",
  },
});
const reportStatusLine = reportStatus.line({ reportId: "report-1" });
const exportUsersLine = exportUsers.line({
  body: {
    jobId: "job-1",
  },
});
const createUserRequestMethod = createUserLine.request().method;
const createUserRequestBody = createUserLine.request().body;
const prepareReceiptUploadTransportKind = prepareReceiptUploadLine.request().uploadTransport.kind;
const prepareReceiptProcessingKind = prepareReceiptUploadLine.request().processingJob.kind;
const reportStatusProcessingKind = reportStatusLine.request().processingJob.kind;

void createUserLine.value();
void updateUserLine.value();
void removeUserLine.value();
void prepareReceiptUploadLine.upload();
void prepareReceiptUploadLine.processing();
void reportStatusLine.processing();
void exportUsersLine.value();
void responseOwnedPartialUpdate;
void createUserRequestMethod;
void createUserRequestBody;
void prepareReceiptUploadTransportKind;
void prepareReceiptProcessingKind;
void reportStatusProcessingKind;
