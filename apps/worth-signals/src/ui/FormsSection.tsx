import React from "react";
import { createSignals } from "forge-signal-wasm";
import { DxCorner } from "./DxCorner";
import { FormsSectionCodeSample } from "./FormsSectionCodeSample";

const FORMS_DX_SAMPLE = `const form = useSignalsForm({
  source: localFormSource.source,
  fields: ({ field }) => ({
    email: field("email"),
    role: field("role"),
  }),
  actions: { submit: updateUserLine },
});

return (
  <form>
    <input {...form.field("email")} />
    <select {...form.select("role", roleOptions)} />

    <button onClick={() => form.reset()}>Cancel</button>
    <button
      disabled={form.actions.submit.disabled}
      onClick={() => form.actions.submit.execute()}
    >
      {form.actions.submit.pending ? "Saving…" : "Save"}
    </button>
  </form>
);`;
import {
  ACTOR_META,
  DANA_COMMENT_BODIES,
  FEED_KIND_LABEL,
  INITIAL_COLLAB,
  PRIYA_JUSTIFICATIONS,
  PRIYA_LIMITS,
  SOURCE_POLICY,
  currency,
  type Actor,
  type CollabShape,
  type FeedEntry,
  type FeedKind,
  type PolicyDraft,
} from "./formsCollabSupport";
import "./formsSection.css";
import "./formsCollabSection.css";

interface FormsSectionProps {
  onNavigate: (path: string) => void;
}

type SignalsRuntime = Awaited<ReturnType<typeof createSignals>>;

type FieldId = "limit" | "justification" | "notes";

interface FieldWritePosture {
  canWrite: boolean;
  reason: string;
  blockers: readonly { kind: string; collaborator?: string; reason: string }[];
}

interface FormReadiness {
  canSubmit: boolean;
  blockers: readonly { kind: string; fields?: readonly string[]; reason: string }[];
}

interface FormDirty {
  isDirty: boolean;
  fields: readonly { field: string }[];
}

interface FormLike {
  fields: Record<FieldId, { set: (value: unknown) => unknown }>;
  source: () => PolicyDraft;
  effective: () => PolicyDraft;
  dirty: () => FormDirty;
  readiness: () => FormReadiness;
  fieldWritePosture: (fieldId: string) => FieldWritePosture;
  reportCollaboration: (update: Record<string, unknown>) => unknown;
  collaboration: () => { events: unknown[]; counters: Record<string, unknown>; posture: string };
  reset: () => unknown;
}

interface Office {
  signals: SignalsRuntime;
  sourceValue: () => PolicyDraft;
  setSource: (value: PolicyDraft) => void;
  forms: Record<Actor, FormLike>;
}

function buildOffice(signals: SignalsRuntime): Office {
  const source = signals.input(SOURCE_POLICY, { debugName: "payout.policy" }) as unknown as {
    (): PolicyDraft;
    set: (value: PolicyDraft) => unknown;
  };

  const makeForm = (actorId: Actor): FormLike =>
    (signals as unknown as { form: (declaration: Record<string, unknown>) => FormLike }).form({
      source,
      collaboration: {
        mode: "fieldLease",
        actorId,
        supportsPresence: true,
        supportsComments: true,
      },
      fields: ({ field }: { field: (path: string) => unknown }) => ({
        limit: field("limit"),
        justification: field("justification"),
        notes: field("notes"),
      }),
    });

  return {
    signals,
    sourceValue: () => source(),
    setSource: (value) => source.set(value),
    forms: {
      you: makeForm("you"),
      priya: makeForm("priya"),
      dana: makeForm("dana"),
    },
  };
}

function broadcastCollaboration(office: Office, collab: CollabShape): void {
  const update = {
    posture: "active",
    reason: collab.reason,
    leasedFields: collab.leaseOwner ? [{ field: "limit", ownerId: collab.leaseOwner }] : [],
    presence: Object.entries(collab.presence).map(([actorId, status]) => ({ actorId, status })),
    comments: collab.comment
      ? [{ id: collab.comment.id, authorId: collab.comment.authorId, target: collab.comment.target }]
      : [],
  };
  for (const form of Object.values(office.forms)) {
    form.reportCollaboration(update);
  }
}

interface DirectorContext {
  office: Office;
  collab: React.MutableRefObject<CollabShape>;
  cycle: React.MutableRefObject<number>;
  narrate: (actor: Actor, kind: FeedKind, text: string) => void;
  sync: () => void;
}

/** Each scene returns true when done; returning false replays it next tick (adaptive waits). */
type Scene = (ctx: DirectorContext) => boolean;

const DIRECTOR_SCENES: Scene[] = [
  (ctx) => {
    ctx.collab.current = {
      ...ctx.collab.current,
      presence: { priya: "active", dana: "viewing" },
      comment: null,
      reason: "collaboration posture is settled",
    };
    ctx.sync();
    ctx.narrate("priya", "presence", "Priya opened the payout policy. Dana is viewing.");
    return true;
  },
  (ctx) => {
    const justification = PRIYA_JUSTIFICATIONS[ctx.cycle.current % PRIYA_JUSTIFICATIONS.length];
    try {
      ctx.office.forms.priya.fields.justification.set(justification);
      ctx.narrate("priya", "edit", "Priya is drafting the justification — her draft, nobody else's.");
    } catch {
      ctx.narrate("priya", "info", "Priya could not edit the justification right now.");
    }
    return true;
  },
  (ctx) => {
    const limit = PRIYA_LIMITS[ctx.cycle.current % PRIYA_LIMITS.length];
    try {
      ctx.office.forms.priya.fields.limit.set(limit);
      ctx.narrate("priya", "edit", `Priya raised the limit to ${currency.format(limit)}. Her patch plan now touches limit.`);
    } catch {
      ctx.narrate("priya", "lease", "Priya tried to edit the limit, but someone else holds the lease.");
    }
    return true;
  },
  (ctx) => {
    const body = DANA_COMMENT_BODIES[ctx.cycle.current % DANA_COMMENT_BODIES.length];
    ctx.collab.current = {
      ...ctx.collab.current,
      presence: { ...ctx.collab.current.presence, dana: "active" },
      comment: { id: `comment-${Date.now()}`, authorId: "dana", target: "limit", body },
      reason: "dana flagged the limit for review",
    };
    ctx.sync();
    ctx.narrate("dana", "comment", `Dana commented on the limit: “${body}”`);
    return true;
  },
  (ctx) => {
    if (ctx.collab.current.leaseOwner === "you") {
      ctx.narrate("dana", "lease", "Dana wants to review the limit, but you hold the lease. She is waiting.");
      return false;
    }
    ctx.collab.current = {
      ...ctx.collab.current,
      leaseOwner: "dana",
      reason: "dana-r is reviewing the payout limit",
    };
    ctx.sync();
    ctx.narrate("dana", "lease", "Dana took the lease on the limit. One report — three different verdicts below.");
    return true;
  },
  (ctx) => {
    const reviewed = ctx.office.forms.dana.effective().limit >= 40_000 ? 38_000 : 30_000;
    try {
      ctx.office.forms.dana.fields.limit.set(reviewed);
      ctx.narrate("dana", "edit", `Dana adjusted the limit to ${currency.format(reviewed)} — she owns the lease, so her write is admitted.`);
    } catch {
      ctx.narrate("dana", "info", "Dana could not adjust the limit.");
    }
    return true;
  },
  (ctx) => {
    if (ctx.collab.current.leaseOwner !== "dana") return true;
    ctx.collab.current = {
      ...ctx.collab.current,
      leaseOwner: null,
      comment: null,
      presence: { ...ctx.collab.current.presence, dana: "idle" },
      reason: "collaboration posture is settled",
    };
    ctx.sync();
    ctx.narrate("dana", "lease", "Dana released the limit and resolved her comment. Everyone can write again.");
    return true;
  },
  (ctx) => {
    const priya = ctx.office.forms.priya;
    const readiness = priya.readiness();
    if (!readiness.canSubmit) {
      ctx.narrate("priya", "info", `Priya tried to submit, but the runtime blocked it: ${readiness.blockers[0]?.reason ?? "not ready"}.`);
      return true;
    }
    ctx.office.setSource(priya.effective());
    ctx.narrate("priya", "submit", "Priya submitted. Server truth moved underneath every draft — including yours.");
    return true;
  },
  (ctx) => {
    ctx.cycle.current += 1;
    ctx.narrate("you", "info", "Quiet moment. Edit a field, or take the limit lease and watch Dana wait.");
    return true;
  },
];

const SCENE_INTERVAL_MS = 3_400;

function presenceDotClass(status: string | undefined): string {
  if (status === "active") return "fc-dot fc-dot-active";
  if (status === "idle") return "fc-dot fc-dot-idle";
  return "fc-dot fc-dot-viewing";
}

function SubmitPill({ readiness }: { readiness: FormReadiness }): React.ReactElement {
  if (readiness.canSubmit) {
    return <span className="fc-pill fc-pill-ready">submit ready</span>;
  }
  // "unchanged" is the runtime saying there is nothing to submit — not a conflict
  if (readiness.blockers.length === 1 && readiness.blockers[0]?.kind === "unchanged") {
    return <span className="fc-pill fc-pill-idle">nothing to submit</span>;
  }
  return (
    <span className="fc-pill fc-pill-blocked" title={readiness.blockers[0]?.reason}>
      submit blocked
    </span>
  );
}

function ActorCard({
  actor,
  collab,
  form,
  lastLine,
}: {
  actor: Actor;
  collab: CollabShape;
  form: FormLike;
  lastLine: string | null;
}): React.ReactElement {
  const meta = ACTOR_META[actor];
  const readiness = form.readiness();
  const dirty = form.dirty();
  const holdsLease = collab.leaseOwner === actor;

  return (
    <article className="fc-actor-card">
      <header>
        <span className={presenceDotClass(collab.presence[actor])} aria-hidden="true" />
        <strong>{meta.name}</strong>
        <em>{meta.role}{meta.simulated ? " · simulated" : ""}</em>
        <SubmitPill readiness={readiness} />
      </header>
      <p className="fc-actor-line">
        {holdsLease ? "holds the lease on limit — canWrite: true for her alone" : lastLine ?? "watching the form"}
      </p>
      <footer>
        <span>
          {dirty.fields.length > 0
            ? `draft touches: ${dirty.fields.map((entry) => entry.field).join(", ")}`
            : "no local edits"}
        </span>
        {!readiness.canSubmit && readiness.blockers[0]?.kind !== "unchanged" ? (
          <span className="fc-blocker-line">{readiness.blockers[0]?.reason}</span>
        ) : null}
      </footer>
    </article>
  );
}

function FormsWorkbench({ office }: { office: Office }): React.ReactElement {
  const [, setRenderTick] = React.useState(0);
  const forceRender = React.useCallback(() => setRenderTick((tick) => tick + 1), []);
  const collabRef = React.useRef<CollabShape>(INITIAL_COLLAB);
  const cycleRef = React.useRef(0);
  const sceneRef = React.useRef(0);
  const feedIdRef = React.useRef(0);
  const [feed, setFeed] = React.useState<FeedEntry[]>([]);

  const narrate = React.useCallback((actor: Actor, kind: FeedKind, text: string) => {
    feedIdRef.current += 1;
    const id = feedIdRef.current;
    setFeed((current) => {
      if (current[0]?.text === text) return current;
      return [{ id, actor, kind, text }, ...current].slice(0, 7);
    });
  }, []);

  const sync = React.useCallback(() => {
    broadcastCollaboration(office, collabRef.current);
  }, [office]);

  React.useEffect(() => {
    broadcastCollaboration(office, collabRef.current);
    const handle = window.setInterval(() => {
      const scene = DIRECTOR_SCENES[sceneRef.current % DIRECTOR_SCENES.length];
      const done = scene({ office, collab: collabRef, cycle: cycleRef, narrate, sync });
      if (done) sceneRef.current += 1;
      forceRender();
    }, SCENE_INTERVAL_MS);
    return () => window.clearInterval(handle);
  }, [office, narrate, sync]);

  const you = office.forms.you;
  const collab = collabRef.current;
  const effective = you.effective();
  const dirtyFields = new Set(you.dirty().fields.map((entry) => entry.field));
  const readiness = you.readiness();
  const limitPosture = you.fieldWritePosture("limit");
  const youHoldLease = collab.leaseOwner === "you";

  const setField = (field: FieldId, value: unknown): void => {
    try {
      you.fields[field].set(value);
    } catch {
      // the runtime denied the write; the posture chip already explains why
    }
    forceRender();
  };

  const toggleLease = (): void => {
    if (collab.leaseOwner && collab.leaseOwner !== "you") return;
    collabRef.current = {
      ...collabRef.current,
      leaseOwner: youHoldLease ? null : "you",
      reason: youHoldLease ? "collaboration posture is settled" : "you are holding the payout limit",
    };
    sync();
    narrate("you", "lease", youHoldLease ? "You released the limit lease." : "You took the lease on the limit. Dana will have to wait.");
    forceRender();
  };

  const submitYours = (): void => {
    if (!readiness.canSubmit) return;
    office.setSource(you.effective());
    narrate("you", "submit", "You submitted. Your draft became server truth; your dirty state cleared itself.");
    forceRender();
  };

  const lastLineFor = (actor: Actor): string | null =>
    feed.find((entry) => entry.actor === actor)?.text ?? null;

  const liveCodeLine = `// → { canWrite: ${limitPosture.canWrite}${
    limitPosture.blockers[0]?.collaborator ? `, collaborator: "${limitPosture.blockers[0].collaborator}"` : ""
  } }`;

  const exportCollaboration = (): void => {
    const artifact = {
      exportedAt: new Date().toISOString(),
      scenario: "payout-limit-dual-control",
      source: "form.collaboration() + form.readiness(), read from the Worth runtime per actor",
      serverTruth: office.sourceValue(),
      actors: Object.fromEntries(
        (Object.keys(office.forms) as Actor[]).map((actor) => [
          actor,
          {
            readiness: office.forms[actor].readiness(),
            dirty: office.forms[actor].dirty(),
            collaboration: office.forms[actor].collaboration(),
          },
        ]),
      ),
    };
    const blob = new Blob([JSON.stringify(artifact, null, 2)], { type: "application/json" });
    const url = URL.createObjectURL(blob);
    const anchor = document.createElement("a");
    anchor.href = url;
    anchor.download = "worth-collaboration-report.json";
    anchor.click();
    URL.revokeObjectURL(url);
  };

  return (
    <>
      <section className="fc-stage" aria-label="Shared payout policy form">
        <article className="fc-your-panel">
          <header className="fc-panel-head">
            <div>
              <h3>Payout limit change</h3>
              <span className="fc-caption-line">dual control · actorId: "you"</span>
            </div>
            <SubmitPill readiness={readiness} />
          </header>

          <label className="fc-field">
            <span>
              Payout limit
              {dirtyFields.has("limit") ? <b className="fc-dirty-tag">draft</b> : null}
            </span>
            <input
              disabled={!limitPosture.canWrite}
              min="0"
              onChange={(event) => setField("limit", event.currentTarget.valueAsNumber)}
              step="1000"
              type="number"
              value={Number.isFinite(effective.limit) ? effective.limit : ""}
            />
            {!limitPosture.canWrite ? (
              <small className="fc-lock-line">
                locked · {limitPosture.blockers[0]?.collaborator ?? "collaboration"} — {limitPosture.reason}
              </small>
            ) : collab.comment?.target === "limit" ? (
              <small className="fc-comment-line">
                {ACTOR_META[collab.comment.authorId].name} commented: “{collab.comment.body}”
              </small>
            ) : (
              <small className="fc-field-hint">server truth: {currency.format(office.sourceValue().limit)}</small>
            )}
          </label>

          <label className="fc-field">
            <span>
              Justification
              {dirtyFields.has("justification") ? <b className="fc-dirty-tag">draft</b> : null}
            </span>
            <input
              onChange={(event) => setField("justification", event.currentTarget.value)}
              type="text"
              value={effective.justification}
            />
            <small className="fc-field-hint">shared field — Priya edits her own draft of it, not yours</small>
          </label>

          <label className="fc-field">
            <span>
              Notes
              {dirtyFields.has("notes") ? <b className="fc-dirty-tag">draft</b> : null}
            </span>
            <input
              onChange={(event) => setField("notes", event.currentTarget.value)}
              placeholder="Add reviewer context"
              type="text"
              value={effective.notes}
            />
            <small className="fc-field-hint">edits here never block on the limit lease</small>
          </label>

          <div className="forms-submit-row fc-actions">
            <button
              className="forms-secondary-button"
              disabled={collab.leaseOwner !== null && collab.leaseOwner !== "you"}
              onClick={toggleLease}
              title={collab.leaseOwner === "dana" ? "Dana holds the lease right now" : undefined}
              type="button"
            >
              {youHoldLease ? "Release the limit lease" : "Hold the limit lease"}
            </button>
            <button className="forms-secondary-button" onClick={() => { you.reset(); forceRender(); }} type="button">
              Reset draft
            </button>
            <button
              className="forms-primary-button"
              disabled={!readiness.canSubmit}
              onClick={submitYours}
              title={readiness.blockers[0]?.reason}
              type="button"
            >
              Submit change
            </button>
          </div>
        </article>

        <div className="fc-side-column">
          <ActorCard actor="priya" collab={collab} form={office.forms.priya} lastLine={lastLineFor("priya")} />
          <ActorCard actor="dana" collab={collab} form={office.forms.dana} lastLine={lastLineFor("dana")} />
          <p className="fc-honesty-note">
            The coworkers are scripted. Everything they cause — locks, verdicts, reasons — is derived by the runtime,
            per actor, from the same report.
          </p>
        </div>
      </section>

      <section className="fc-evidence" aria-label="Activity and form truth">
        <div className="fc-feed-panel">
          <header className="signals-panel-head">
            <h3>Activity</h3>
            <code>form.collaboration().events</code>
            <button className="signals-export-button" onClick={exportCollaboration} type="button">
              Export collaboration report (JSON)
            </button>
          </header>
          <ul className="fc-feed">
            {feed.map((entry) => (
              <li className={`fc-feed-row fc-feed-${entry.kind}`} key={entry.id}>
                <span className="fc-feed-kind">{FEED_KIND_LABEL[entry.kind]}</span>
                <span className="fc-feed-text">{entry.text}</span>
              </li>
            ))}
            {feed.length === 0 ? <li className="fc-feed-row"><span className="fc-feed-text">The office is waking up…</span></li> : null}
          </ul>
          <details className="signals-audit-payload fc-recorder-raw">
            <summary>raw collaboration events + counters</summary>
            <pre>
              {JSON.stringify(
                { counters: you.collaboration().counters, events: you.collaboration().events },
                null,
                2,
              )}
            </pre>
          </details>
        </div>

        <aside className="fc-truth-panel" aria-label="Your form truth">
          <header className="signals-panel-head">
            <h3>Your form truth</h3>
            <code>form.dirty() · form.readiness()</code>
          </header>
          <dl className="signals-why-grid">
            <div>
              <dt>server truth</dt>
              <dd>{currency.format(office.sourceValue().limit)} · “{office.sourceValue().justification.slice(0, 32)}…”</dd>
            </div>
            <div>
              <dt>your dirty fields</dt>
              <dd>{dirtyFields.size > 0 ? [...dirtyFields].join(", ") : "none — draft matches source"}</dd>
            </div>
            <div>
              <dt>limit write posture</dt>
              <dd className={limitPosture.canWrite ? "" : "is-changed"}>
                {limitPosture.canWrite ? "admitted" : `blocked by ${limitPosture.blockers[0]?.collaborator ?? "collaboration"}`}
              </dd>
            </div>
            <div>
              <dt>submit verdict</dt>
              <dd className={readiness.canSubmit ? "" : "is-changed"}>
                {readiness.canSubmit ? "ready" : readiness.blockers[0]?.reason ?? "blocked"}
              </dd>
            </div>
          </dl>
          <details className="signals-audit-payload">
            <summary>raw readiness payload</summary>
            <pre>{JSON.stringify(readiness, null, 2)}</pre>
          </details>
        </aside>
      </section>

      <section className="signals-code-section" aria-labelledby="fc-code-title">
        <h2 id="fc-code-title">One report in, one verdict out — per actor</h2>
        <FormsSectionCodeSample liveLine={liveCodeLine} />
      </section>

      <DxCorner
        code={FORMS_DX_SAMPLE}
        filename="edit-user-dialog.tsx"
        subtitle="Dual-control forms sound like enterprise sludge. This is regulation-grade behavior with one mental model — not React plus React Query plus Formik plus Zustand agreeing by convention."
        receipts={[
          {
            claim: "A field binding is a spread.",
            api: '<input {...form.field("email")} />',
          },
          {
            claim: "The submit button wires itself.",
            api: "form.actions.submit.disabled · .pending · .execute()",
          },
          {
            claim: "Even closing the form is a signal read.",
            api: "form.exit() — dirty- and pending-aware",
          },
        ]}
      />
    </>
  );
}

export function FormsSection({ onNavigate }: FormsSectionProps): React.ReactElement {
  const [office, setOffice] = React.useState<Office | null>(null);
  const [bootError, setBootError] = React.useState<string | null>(null);

  React.useEffect(() => {
    let active = true;
    createSignals({ deployment: "mainThreadCompatibility" })
      .then((signals) => {
        if (!active) return;
        setOffice(buildOffice(signals));
      })
      .catch((error: unknown) => {
        if (active) setBootError(error instanceof Error ? error.message : "Could not start the Worth runtime.");
      });
    return () => {
      active = false;
    };
  }, []);

  return (
    <div className="accent-forms fc-section">
      {bootError ? <div className="signals-runtime-message">{bootError}</div> : null}
      {!office && !bootError ? <div className="signals-runtime-message">Connecting to the Worth runtime…</div> : null}
      {office ? <FormsWorkbench office={office} /> : null}

      <div className="signals-docs-row">
        <button onClick={() => onNavigate("#/docs/forms/index")} type="button">
          Explore forms in the documentation <span aria-hidden="true">→</span>
        </button>
      </div>
    </div>
  );
}
