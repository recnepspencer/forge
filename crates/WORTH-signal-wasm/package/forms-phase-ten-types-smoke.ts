import { createSignals } from "./index.js";

const signals = await createSignals({ deployment: "mainThreadCompatibility" });
const state = signals.input({
  title: "Ship docs",
  slug: "ship-docs",
  status: "draft",
  owner: "me",
  evidence: { digest: "file-1", name: "audit.pdf" },
});

const phaseTenForm = signals.form({
  source: {
    value: state,
    sourceAdmission: { status: "ready", reason: "source admitted" },
    draftRestore: { status: "ready", reason: "draft restored" },
  },
  fields: ({ field, evidence }) => ({
    title: field<string>("title", {
      label: "Title",
      row: "hero",
      density: "comfortable",
      accessibility: {
        readingOrder: 1,
        focusOrder: 1,
      },
    }),
    slug: field<string>("slug", {
      adapter: {
        tier: "externalImperative",
        reportsRawInput: true,
      },
    }),
    status: field<string>("status"),
    evidence: evidence<{ digest: string; name: string }>("evidence", {
      attachmentIdentity: "digest",
      metadata: { required: true },
    }),
  }),
  validation: ({ field, form, asyncField }) => ({
    titleRequired: field<string>("title", (value) => (
      value.length > 0
        ? { kind: "valid", field: "title", digest: value }
        : {
            kind: "invalid",
            field: "title",
            message: {
              code: "title.required",
              severity: "error",
              audience: "user",
              visibility: "visible",
            },
          }
    )),
    slugLifecycle: asyncField("slug", {
      id: "slugUnique",
      triggers: ["submit"],
      debounceMs: 250,
    }),
    reviewPolicy: form("reviewPolicy", ["status", "owner"], (values) => (
      values.status === "draft"
        ? true
        : {
            kind: "warning",
            message: {
              code: "status.review",
              severity: "warning",
              audience: "user",
              visibility: "summary",
            },
          }
    )),
  }),
  availability: ({ field, section, action }) => ({
    ownerReadonly: field("owner", ["status"], (values) => (
      values.status === "published"
        ? { state: "readonly", draftPolicy: "freeze" }
        : "enabled"
    )),
    evidenceSection: section("evidence", ["evidence"], ["status"], () => "enabled"),
    submitVisible: action("submit", ["title"], () => "enabled"),
  }),
  admission: ({ field, action }) => ({
    ownerReview: field("owner", "review", ["status"], () => ({
      posture: "requiresReview",
      reason: "owner edits require review",
    })),
    submitApproval: action("submit", "approval", ["status"], () => ({
      posture: "requiresApproval",
      actorDigest: "reviewer-1",
      reason: "publish requires reviewer approval",
    })),
  }),
  actions: ({ submit, action, step }) => ({
    submit: submit({
      hostRequirements: ["online", "credentials"],
    }),
    saveDraft: action("saveDraft", {
      patchPolicy: "allowEmpty",
      hostEffect: "draft.store",
    }),
    nextReview: step("nextReview", "review", "next", {
      routeCoupled: false,
    }),
  }),
  host: {
    focus: "title",
    visibility: "visible",
    viewport: { width: 1280, height: 720 },
    online: "online",
    persistence: true,
    credentials: true,
    autofill: false,
  },
  measurement: {
    observe: ["animationFrame", "contentGrowth"],
    maxRetainedSnapshots: 3,
  },
  presentation: {
    entry: {
      bootstrap: {
        sourceAdmission: true,
        draftRestore: true,
        validation: true,
        readiness: true,
      },
    },
    action: {
      settleOn: ["messages", "layout"],
    },
  },
});

const phaseTenActionReadiness = phaseTenForm.actionReadiness("submit").canRun;
const phaseTenWritePosture = phaseTenForm.fieldWritePosture("owner").reason;
const phaseTenVerificationDigest = phaseTenForm.verification().digests.readinessDigest;
const phaseTenPresentation = phaseTenForm.presentationLifecycle("entry");

signals.form({
  source: { title: "Ship docs" },
  fields: ({ field }) => ({
    title: field("title", {
      // @ts-expect-error field density must be one of the declared layout densities
      density: "dense",
    }),
  }),
});

signals.form({
  source: { title: "Ship docs" },
  fields: ({ field }) => ({
    title: field("title", {
      // @ts-expect-error reading order must be numeric
      accessibility: { readingOrder: "first" },
    }),
  }),
});

signals.form({
  source: { title: "Ship docs" },
  fields: ({ field }) => ({
    title: field("title"),
  }),
  validation: ({ field }) => ({
    // @ts-expect-error validation severity must be info warning or error
    invalidSeverity: field("title", () => ({
      kind: "invalid",
      message: {
        code: "bad",
        severity: "fatal",
        audience: "user",
        visibility: "visible",
      },
    })),
  }),
});

signals.form({
  source: { title: "Ship docs" },
  fields: ({ field }) => ({
    title: field("title"),
  }),
  availability: ({ field }) => ({
    // @ts-expect-error availability states must be declared
    invalidAvailability: field("title", ["title"], () => "collapsed"),
  }),
});

signals.form({
  source: { title: "Ship docs" },
  fields: ({ field }) => ({
    title: field("title"),
  }),
  admission: ({ action }) => ({
    invalidAdmission: action(
      "submit",
      // @ts-expect-error admission capability must be declared
      "publish",
      ["title"],
      () => "admitted",
    ),
  }),
});

signals.form({
  source: { title: "Ship docs" },
  fields: ({ field }) => ({
    title: field("title"),
  }),
  actions: ({ submit }) => ({
    submit: submit({
      // @ts-expect-error host requirements must be declared form host capabilities
      hostRequirements: ["clipboard"],
    }),
  }),
});

signals.form({
  source: { title: "Ship docs" },
  fields: ({ field }) => ({
    title: field("title"),
  }),
  actions: ({ step }) => ({
    invalidStep: step(
      "invalidStep",
      "review",
      // @ts-expect-error step commands must be declared controller-local commands
      "advance",
    ),
  }),
});

signals.form({
  source: { title: "Ship docs" },
  fields: ({ field }) => ({
    title: field("title"),
  }),
  host: {
    // @ts-expect-error viewport bindings require width and height
    viewport: { width: 1280 },
  },
});

signals.form({
  source: { title: "Ship docs" },
  fields: ({ field }) => ({
    title: field("title"),
  }),
  measurement: {
    // @ts-expect-error measurement causes must be declared renderer causes
    observe: ["scroll"],
  },
});

signals.form({
  source: { title: "Ship docs" },
  fields: ({ field }) => ({
    title: field("title"),
  }),
  presentation: {
    entry: {
      bootstrap: {
        // @ts-expect-error bootstrap flags must be booleans
        hostFacts: "required",
      },
    },
  },
});

signals.form({
  source: { title: "Ship docs" },
  fields: ({ field }) => ({
    title: field("title"),
  }),
  actions: ({ submit }) => ({
    submit: submit({
      // @ts-expect-error submit resource effect profiles must be branded runtime profiles
      resourceEffectProfile: { name: "WORTHd-profile" },
    }),
  }),
});

void phaseTenForm;
void phaseTenActionReadiness;
void phaseTenWritePosture;
void phaseTenVerificationDigest;
void phaseTenPresentation;
