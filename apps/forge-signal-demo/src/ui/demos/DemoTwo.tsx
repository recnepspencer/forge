import React, { useState } from "react";

import { DemoShell } from "../DemoShell";
import { useSignal } from "../Demos";

interface DemoTwoProps {
  signals: any;
  demo: any;
  onNavigate: any;
}

const fieldFrameStyle: React.CSSProperties = {
  display: "flex",
  flexDirection: "column",
  gap: "0.35rem",
};

const labelStyle: React.CSSProperties = {
  fontSize: "0.85rem",
  fontWeight: "600",
  color: "var(--text-secondary)",
};

const inputBaseStyle: React.CSSProperties = {
  width: "100%",
  padding: "0.7rem 0.85rem",
  borderRadius: "8px",
  background: "rgba(15, 23, 42, 0.72)",
  color: "var(--text-primary)",
  fontSize: "0.92rem",
  outline: "none",
  transition: "border-color 0.2s ease, box-shadow 0.2s ease, background 0.2s ease",
};

function messageStyle(tone: "error" | "info" | "success" | "warning"): React.CSSProperties {
  if (tone === "error") {
    return {
      color: "var(--state-danger-text)",
      background: "var(--state-danger-soft)",
      border: "1px solid var(--state-danger-border)",
    };
  }
  if (tone === "success") {
    return {
      color: "var(--state-success-text)",
      background: "var(--state-success-bg)",
      border: "1px solid var(--state-success-border)",
    };
  }
  if (tone === "warning") {
    return {
      color: "var(--state-warning-text)",
      background: "var(--state-warning-bg)",
      border: "1px solid var(--state-warning-border)",
    };
  }
  return {
    color: "var(--state-info-text)",
    background: "var(--state-info-bg)",
    border: "1px solid var(--state-info-border)",
  };
}

export const DemoTwo: React.FC<DemoTwoProps> = ({ signals, demo, onNavigate }) => {
  const formRef = React.useRef<any>(null);
  const [submitSuccess, setSubmitSuccess] = useState(false);
  const [terminalLog, setTerminalLog] = useState<string[]>([]);
  const [, setTick] = useState(0);

  const triggerRender = () => setTick((t) => t + 1);

  const api = React.useMemo(
    () => signals.api({
      baseUrl: "/api",
      effects: signals.resource.effects.branchNative(),
    }),
    [signals],
  );

  const articleFields = React.useMemo(
    () => signals.resource.detailFields({
      title: {
        read: (value: any) => value.title,
        write: (value: any, title: string) => ({ ...value, title }),
      },
      status: {
        read: (value: any) => value.status,
        write: (value: any, status: string) => ({ ...value, status }),
      },
      contactMethod: {
        read: (value: any) => value.contactMethod,
        write: (value: any, contactMethod: string) => ({ ...value, contactMethod }),
      },
      email: {
        read: (value: any) => value.email,
        write: (value: any, email: string) => ({ ...value, email }),
      },
      phone: {
        read: (value: any) => value.phone,
        write: (value: any, phone: string) => ({ ...value, phone }),
      },
    }),
    [signals],
  );

  const articleDetail = React.useMemo(
    () =>
      api.url("/articles/:articleId").detail({
        reconcile: articleFields,
        load: async ({ articleId }: any) => {
          const response = await fetch(`/api/articles/${articleId}.json`);
          if (!response.ok) {
            throw new Error(`Could not fetch article metadata: ${response.status}`);
          }
          return response.json();
        },
      }),
    [api, articleFields],
  );

  const line = React.useMemo(
    () => articleDetail.line({ articleId: "article-12" }),
    [articleDetail],
  );

  const sourceValue = useSignal<any>(signals, line.signal());
  const lineStatus = line.status().kind;
  const isLoading = lineStatus === "pending";

  const form = React.useMemo(() => {
    const nextForm = signals.form({
      source: signals.form.source.resourceLine(line, { id: "article-form" }),
      fields: ({ field }: any) => ({
        title: field("title"),
        status: field("status"),
        contactMethod: field("contactMethod"),
        email: field("email"),
        phone: field("phone"),
      }),
      validation: ({ field }: any) => ({
        titleRequired: field("title", (title: string) => (
          title
            ? { kind: "valid", field: "title", digest: title }
            : {
                kind: "invalid",
                field: "title",
                message: {
                  code: "title.required",
                  message: "Title is required",
                  severity: "error",
                  target: "title",
                  audience: "user",
                  visibility: "visible",
                },
              }
        )),
        emailRequired: field("email", (email: string) => {
          const method = formRef.current?.fields.contactMethod.value();
          if (method !== "Email") {
            return { kind: "valid", field: "email", digest: "inactive" };
          }
          if (!email) {
            return {
              kind: "invalid",
              field: "email",
              message: {
                code: "email.required",
                message: "Email is required when contact method is Email",
                severity: "error",
                target: "email",
                audience: "user",
                visibility: "visible",
              },
            };
          }
          if (!email.includes("@")) {
            return {
              kind: "invalid",
              field: "email",
              message: {
                code: "email.invalid",
                message: "Email must include @",
                severity: "error",
                target: "email",
                audience: "user",
                visibility: "visible",
              },
            };
          }
          return { kind: "valid", field: "email", digest: email };
        }),
        phoneRequired: field("phone", (phone: string) => {
          const method = formRef.current?.fields.contactMethod.value();
          if (method !== "Phone") {
            return { kind: "valid", field: "phone", digest: "inactive" };
          }
          if (!phone) {
            return {
              kind: "invalid",
              field: "phone",
              message: {
                code: "phone.required",
                message: "Phone is required when contact method is Phone",
                severity: "error",
                target: "phone",
                audience: "user",
                visibility: "visible",
              },
            };
          }
          if (!/^\+?[\d\s-]{7,15}$/.test(phone)) {
            return {
              kind: "invalid",
              field: "phone",
              message: {
                code: "phone.invalid",
                message: "Phone must be numeric and between 7 and 15 characters",
                severity: "error",
                target: "phone",
                audience: "user",
                visibility: "visible",
              },
            };
          }
          return { kind: "valid", field: "phone", digest: phone };
        }),
      }),
      actions: ({ submit }: any) => ({
        submit: submit(),
      }),
    });
    formRef.current = nextForm;
    return nextForm;
  }, [line, signals]);

  const titleVal = form.fields.title.value();
  const statusVal = form.fields.status.value();
  const contactMethodVal = form.fields.contactMethod.value();
  const emailVal = form.fields.email.value();
  const phoneVal = form.fields.phone.value();

  const sourceSnapshot = form.source();
  const draftSnapshot = form.draft();
  const effectiveSnapshot = form.effective();
  const dirtyState = form.dirty();
  const validation = form.validation();
  const readiness = form.readiness();
  const patchPlan = form.patchPlan();
  const submitPlan = form.actionPlan("submit");
  const executionHistory = form.actionExecutionHistory();
  const latestExecution = executionHistory.at(-1);

  const titleError = validation.artifacts.find(
    (artifact: any) => artifact.field === "title" && artifact.kind === "invalid",
  )?.message?.message;
  const emailError = validation.artifacts.find(
    (artifact: any) => artifact.field === "email" && artifact.kind === "invalid",
  )?.message?.message;
  const phoneError = validation.artifacts.find(
    (artifact: any) => artifact.field === "phone" && artifact.kind === "invalid",
  )?.message?.message;

  const canSubmitBtn = submitPlan.status === "accepted" && !isLoading;

  const authoredFieldsCode = `const articleFields = signals.resource.detailFields({
  title: {
    read: (value) => value.title,
    write: (value, title) => ({ ...value, title }),
  },
  status: {
    read: (value) => value.status,
    write: (value, status) => ({ ...value, status }),
  },
  contactMethod: {
    read: (value) => value.contactMethod,
    write: (value, contactMethod) => ({ ...value, contactMethod }),
  },
  email: {
    read: (value) => value.email,
    write: (value, email) => ({ ...value, email }),
  },
  phone: {
    read: (value) => value.phone,
    write: (value, phone) => ({ ...value, phone }),
  },
});

const api = signals.api({
  baseUrl: "/api",
  effects: signals.resource.effects.branchNative(),
});

const articleDetail = api.url("/articles/:articleId").detail({
  reconcile: articleFields,
  load: async ({ articleId }) => {
    const response = await fetch(\`/api/articles/\${articleId}.json\`);
    return response.json();
  },
});

const form = signals.form({
  source: signals.form.source.resourceLine(line, { id: "article-form" }),
  fields: ({ field }) => ({
    title: field("title"),
    status: field("status"),
    contactMethod: field("contactMethod"),
    email: field("email"),
    phone: field("phone"),
  }),
  actions: ({ submit }) => ({
    submit: submit(),
  }),
});`;

  const handleSubmit = (event: React.FormEvent) => {
    event.preventDefault();
    if (!canSubmitBtn) {
      return;
    }

    const execution = form.executeAction("submit");
    const nextLogs = [
      `[submit] result = ${execution.resultKind}`,
      "[resource] synthetic static JSON source; no live backend write is performed in this demo",
    ];

    if (execution.resourceSubmission) {
      nextLogs.push(
        `[resource] patchCount = ${execution.resourceSubmission.patchCount}`,
        `[resource] sourceKind = ${execution.resourceSubmission.sourceKind}`,
      );
      if (execution.resourceSubmission.mutationResponse) {
        nextLogs.push(
          `[resource] confirmation = ${execution.resourceSubmission.mutationResponse.confirmationKind}`,
          `[resource] fallbackTargets = ${execution.resourceSubmission.mutationResponse.fallbackTargetCount}`,
        );
      }
      for (const patch of execution.resourceSubmission.patches ?? []) {
        nextLogs.push(
          `[patch] ${patch.field} -> ${patch.operationKind} (${patch.patchKind})`,
        );
      }
    }

    setTerminalLog((prev) => [...prev, ...nextLogs]);
    setSubmitSuccess(execution.resultKind === "fulfilled");
    window.setTimeout(() => setSubmitSuccess(false), 4000);
    triggerRender();
  };

  const handleCancel = () => {
    form.fields.title.clearDraft();
    form.fields.status.clearDraft();
    form.fields.contactMethod.clearDraft();
    form.fields.email.clearDraft();
    form.fields.phone.clearDraft();
    setSubmitSuccess(false);
    triggerRender();
  };

  const clearTerminal = () => setTerminalLog([]);

  return (
    <DemoShell
      demo={demo}
      onNavigate={onNavigate}
      customCodeBlock={
        <div style={{ display: "flex", flexDirection: "column", gap: "1rem" }}>
          <div
            style={{
              background: "rgba(8, 12, 22, 0.95)",
              border: "1px solid rgba(255, 255, 255, 0.08)",
              borderRadius: "10px",
              overflow: "hidden",
              boxShadow: "0 8px 32px rgba(0,0,0,0.5)",
            }}
          >
            <div
              style={{
                borderBottom: "1px solid rgba(255,255,255,0.06)",
                padding: "0.85rem 1rem",
                display: "flex",
                justifyContent: "space-between",
                alignItems: "center",
              }}
            >
              <span
                style={{
                  color: "var(--accent-forms)",
                  fontWeight: "600",
                  fontSize: "0.75rem",
                  letterSpacing: "1px",
                }}
              >
                RESOURCE-BACKED FIELD CONSTRUCTION
              </span>
              <span style={{ color: "var(--text-muted)", fontSize: "0.72rem" }}>
                authored Forge surface
              </span>
            </div>
            <pre
              style={{
                margin: 0,
                padding: "1rem 1.1rem",
                overflowX: "auto",
                whiteSpace: "pre-wrap",
                color: "#cffafe",
                fontFamily: '"Fira Code", "JetBrains Mono", monospace',
                fontSize: "0.78rem",
                lineHeight: "1.65",
              }}
            >
              <code>{authoredFieldsCode}</code>
            </pre>
          </div>

          <div
            style={{
              background: "rgba(8, 12, 22, 0.95)",
              border: "1px solid rgba(255, 255, 255, 0.08)",
              borderRadius: "10px",
              padding: "1rem 1.1rem",
              fontFamily: '"Fira Code", "JetBrains Mono", monospace',
              fontSize: "0.78rem",
              lineHeight: "1.65",
              color: "var(--text-secondary)",
              boxShadow: "0 8px 32px rgba(0,0,0,0.5)",
            }}
          >
            <div
              style={{
                display: "flex",
                justifyContent: "space-between",
                alignItems: "center",
                marginBottom: "0.85rem",
              }}
            >
              <span
                style={{
                  color: "var(--accent-forms)",
                  fontWeight: "600",
                  fontSize: "0.75rem",
                  letterSpacing: "1px",
                }}
              >
                LIVE FORM + RESOURCE READOUT
              </span>
              <span style={{ color: "var(--text-muted)", fontSize: "0.72rem" }}>
                synthetic data, honest state
              </span>
            </div>
            <div><span style={{ color: "var(--state-info-text)" }}>[source]</span> {JSON.stringify(sourceSnapshot)}</div>
            <div><span style={{ color: "var(--accent-forms)" }}>[draft]</span> {JSON.stringify(draftSnapshot)}</div>
            <div><span style={{ color: "var(--state-success-text)" }}>[effective]</span> {JSON.stringify(effectiveSnapshot)}</div>
            <div><span style={{ color: dirtyState.isDirty ? "var(--state-warning-text)" : "var(--text-muted)" }}>[dirty]</span> {String(dirtyState.isDirty)}</div>
            <div><span style={{ color: canSubmitBtn ? "var(--state-success-text)" : "var(--state-danger-text)" }}>[submit plan]</span> {submitPlan.status}</div>
            <div><span style={{ color: "var(--state-info-text)" }}>[patch plan]</span> {JSON.stringify(patchPlan.operations)}</div>
            <div><span style={{ color: readiness.canSubmit ? "var(--state-success-text)" : "var(--state-danger-text)" }}>[readiness]</span> {JSON.stringify({ canSubmit: readiness.canSubmit, blockers: readiness.blockers.map((blocker: any) => blocker.kind) })}</div>
            {latestExecution && (
              <div><span style={{ color: "var(--state-success-text)" }}>[last execution]</span> {JSON.stringify({ resultKind: latestExecution.resultKind, patchCount: latestExecution.resourceSubmission?.patchCount ?? 0 })}</div>
            )}
          </div>
        </div>
      }
    >
      <form onSubmit={handleSubmit} style={{ display: "flex", flexDirection: "column", gap: "1.25rem" }}>
        <div
          style={{
            ...messageStyle("info"),
            borderRadius: "10px",
            padding: "0.85rem 0.95rem",
            fontSize: "0.82rem",
            lineHeight: "1.55",
          }}
        >
          This demo is backed by a synthetic static JSON resource at
          <code style={{ marginLeft: "0.35rem", color: "var(--text-primary)" }}>
            /public/api/articles/article-12.json
          </code>
          . The submit action is a real Forge resource-backed submit, but it
          patches the resident resource line locally for the demo instead of
          pretending to perform a live backend write.
        </div>

        <h4
          style={{
            margin: 0,
            fontSize: "1rem",
            fontWeight: 600,
            color: "var(--text-primary)",
            borderBottom: "1px solid rgba(255,255,255,0.06)",
            paddingBottom: "0.5rem",
          }}
        >
          Edit Article Details
        </h4>

        <div style={fieldFrameStyle}>
          <label style={labelStyle}>
            Article Title <span style={{ color: "var(--state-danger-text)" }}>*</span>
          </label>
          <input
            type="text"
            className="form-input"
            placeholder={isLoading ? "Loading article details..." : "e.g. Boilerplate Article"}
            disabled={isLoading}
            value={titleVal || ""}
            onChange={(event) => {
              form.fields.title.set(event.target.value);
              triggerRender();
            }}
            style={{
              ...inputBaseStyle,
              border: `1px solid ${titleError ? "var(--state-danger-border)" : "rgba(255,255,255,0.15)"}`,
              boxShadow: titleError ? "0 0 0 1px rgba(251, 113, 133, 0.15)" : "none",
            }}
          />
          {titleError && (
            <span
              style={{
                ...messageStyle("error"),
                fontSize: "0.76rem",
                borderRadius: "8px",
                padding: "0.45rem 0.6rem",
              }}
            >
              {titleError}
            </span>
          )}
        </div>

        <div style={fieldFrameStyle}>
          <label style={labelStyle}>Publication Status</label>
          <select
            className="form-select"
            disabled={isLoading}
            value={statusVal || ""}
            onChange={(event) => {
              form.fields.status.set(event.target.value);
              triggerRender();
            }}
            style={{
              ...inputBaseStyle,
              border: "1px solid rgba(255,255,255,0.15)",
            }}
          >
            <option value="Draft">Draft</option>
            <option value="Review">In Review</option>
            <option value="Published">Published</option>
          </select>
        </div>

        <div style={fieldFrameStyle}>
          <label style={labelStyle}>Preferred Contact Method</label>
          <select
            className="form-select"
            disabled={isLoading}
            value={contactMethodVal || "None"}
            onChange={(event) => {
              form.fields.contactMethod.set(event.target.value);
              triggerRender();
            }}
            style={{
              ...inputBaseStyle,
              border: "1px solid rgba(255,255,255,0.15)",
            }}
          >
            <option value="None">None</option>
            <option value="Email">Email</option>
            <option value="Phone">Phone</option>
          </select>
        </div>

        {contactMethodVal === "Email" && (
          <div style={fieldFrameStyle}>
            <label style={labelStyle}>
              Contact Email <span style={{ color: "var(--state-danger-text)" }}>*</span>
            </label>
            <input
              type="text"
              className="form-input"
              placeholder="e.g. dev@forge.sh"
              disabled={isLoading}
              value={emailVal || ""}
              onChange={(event) => {
                form.fields.email.set(event.target.value);
                triggerRender();
              }}
              style={{
                ...inputBaseStyle,
                border: `1px solid ${emailError ? "var(--state-danger-border)" : "rgba(255,255,255,0.15)"}`,
                boxShadow: emailError ? "0 0 0 1px rgba(251, 113, 133, 0.15)" : "none",
              }}
            />
            {emailError && (
              <span
                style={{
                  ...messageStyle("error"),
                  fontSize: "0.76rem",
                  borderRadius: "8px",
                  padding: "0.45rem 0.6rem",
                }}
              >
                {emailError}
              </span>
            )}
          </div>
        )}

        {contactMethodVal === "Phone" && (
          <div style={fieldFrameStyle}>
            <label style={labelStyle}>
              Contact Phone <span style={{ color: "var(--state-danger-text)" }}>*</span>
            </label>
            <input
              type="text"
              className="form-input"
              placeholder="e.g. 555 0199"
              disabled={isLoading}
              value={phoneVal || ""}
              onChange={(event) => {
                form.fields.phone.set(event.target.value);
                triggerRender();
              }}
              style={{
                ...inputBaseStyle,
                border: `1px solid ${phoneError ? "var(--state-danger-border)" : "rgba(255,255,255,0.15)"}`,
                boxShadow: phoneError ? "0 0 0 1px rgba(251, 113, 133, 0.15)" : "none",
              }}
            />
            {phoneError && (
              <span
                style={{
                  ...messageStyle("error"),
                  fontSize: "0.76rem",
                  borderRadius: "8px",
                  padding: "0.45rem 0.6rem",
                }}
              >
                {phoneError}
              </span>
            )}
          </div>
        )}

        <div
          style={{
            ...messageStyle(readiness.canSubmit ? "success" : "warning"),
            borderRadius: "10px",
            padding: "0.8rem 0.95rem",
            fontSize: "0.8rem",
            lineHeight: "1.55",
          }}
        >
          <strong style={{ color: "var(--text-primary)" }}>Readiness:</strong>{" "}
          {readiness.canSubmit
            ? "submit is admitted through the resource-backed patch plan"
            : `blocked by ${readiness.blockers.map((blocker: any) => blocker.kind).join(", ") || "current form state"}`}
        </div>

        <div
          style={{
            display: "flex",
            justifyContent: "flex-end",
            gap: "0.75rem",
            borderTop: "1px solid rgba(255,255,255,0.06)",
            paddingTop: "1rem",
          }}
        >
          <button
            type="button"
            className="btn"
            disabled={!dirtyState.isDirty || isLoading}
            onClick={handleCancel}
            style={{
              padding: "0.55rem 1.25rem",
              borderRadius: "8px",
              background: "transparent",
              border: "1px solid rgba(255,255,255,0.15)",
              color: dirtyState.isDirty && !isLoading ? "var(--text-primary)" : "var(--text-muted)",
              fontSize: "0.85rem",
              fontWeight: "600",
              cursor: dirtyState.isDirty && !isLoading ? "pointer" : "not-allowed",
              opacity: dirtyState.isDirty && !isLoading ? 1 : 0.4,
              transition: "all 0.2s ease",
            }}
          >
            Reset Draft
          </button>
          <button
            type="submit"
            className="btn btn-primary"
            disabled={!canSubmitBtn}
            style={{
              padding: "0.55rem 1.25rem",
              borderRadius: "8px",
              background: canSubmitBtn ? "var(--accent-forms)" : "rgba(255,255,255,0.05)",
              color: canSubmitBtn ? "#0f172a" : "var(--text-muted)",
              border: "none",
              fontSize: "0.85rem",
              fontWeight: "700",
              cursor: canSubmitBtn ? "pointer" : "not-allowed",
              opacity: canSubmitBtn ? 1 : 0.5,
              transition: "all 0.2s ease",
            }}
          >
            Submit Resource Patch
          </button>
        </div>

        {submitSuccess && (
          <div
            style={{
              ...messageStyle("success"),
              borderRadius: "10px",
              padding: "0.7rem 0.85rem",
              textAlign: "center",
              fontSize: "0.82rem",
            }}
          >
            Resource-backed submit executed. The resident line source was
            reconciled and the draft cleared.
          </div>
        )}

        {terminalLog.length > 0 && (
          <div
            style={{
              background: "rgba(8, 12, 22, 0.92)",
              border: "1px solid rgba(255,255,255,0.08)",
              borderRadius: "10px",
              padding: "0.95rem 1rem",
              fontFamily: '"JetBrains Mono", monospace',
              fontSize: "0.78rem",
              lineHeight: "1.6",
              color: "var(--text-secondary)",
            }}
          >
            <div
              style={{
                display: "flex",
                justifyContent: "space-between",
                alignItems: "center",
                marginBottom: "0.65rem",
              }}
            >
              <span
                style={{
                  color: "var(--accent-forms)",
                  fontWeight: "600",
                  fontSize: "0.75rem",
                  letterSpacing: "1px",
                }}
              >
                SUBMIT EXECUTION LOG
              </span>
              <button
                onClick={clearTerminal}
                style={{
                  background: "transparent",
                  border: "none",
                  color: "var(--text-muted)",
                  cursor: "pointer",
                  fontSize: "0.72rem",
                  textDecoration: "underline",
                  padding: 0,
                }}
              >
                Clear Logs
              </button>
            </div>
            <div style={{ display: "flex", flexDirection: "column", gap: "0.3rem" }}>
              {terminalLog.map((log, index) => (
                <div key={`${log}-${index}`} style={{ color: "var(--state-success-text)" }}>
                  {log}
                </div>
              ))}
            </div>
          </div>
        )}

        <div
          style={{
            color: "var(--text-muted)",
            fontSize: "0.75rem",
            lineHeight: "1.55",
          }}
        >
          Current resource status: <strong style={{ color: "var(--text-secondary)" }}>{lineStatus}</strong>
          {" • "}
          source title: <strong style={{ color: "var(--text-secondary)" }}>{sourceValue?.title ?? "unavailable"}</strong>
        </div>
      </form>
    </DemoShell>
  );
};
