export interface DemoMetadata {
  id: number;
  title: string;
  purpose: string;
  preface: string;
  difficulty: "Beginner" | "Intermediate" | "Advanced";
  primaryMessage: string;
  forgeCode: string;
  alternativeCode: string;
  alternativeName: string;
  explanationAlternative: string;
  explanationForge: string;
  whatYouGet: string[];
  relatedDocsPath: string;
}

export const demoRegistry: DemoMetadata[] = [
  {
    id: 1,
    title: "Explainable Transfer Decision",
    purpose: "Move one transfer amount and watch the runtime itself report what recomputed, what changed, and what stayed the same — an audit trail read from the runtime, not kept by the UI.",
    preface: "Drag the amount across the $10,000 policy threshold. Every committed change lands as one transaction, and every row in the audit trail is read back from signals.diagnostics() after the commit — including the moments where the review lane recomputed but its answer did not change. Click any value to ask the runtime why.",
    difficulty: "Beginner",
    primaryMessage: "One input. Two decisions. A runtime that shows its work.",
    forgeCode: `import { createSignals } from "forge-signal-wasm";
const signals = await createSignals();

const amount = signals.input(8_000);
const fee = signals.computed(() => amount() * 0.004);
const reviewLane = signals.computed(() =>
  amount() >= 10_000 ? "Manual review" : "Automatic"
);

// land the change as one unit
signals.transaction((tx) => tx.set(amount, 12_000));

// ask the runtime what actually happened
signals.diagnostics().why(reviewLane.id);
`,
    alternativeName: "React useState + useMemo",
    alternativeCode: `import { useState, useMemo } from "react";

const [amount, setAmount] = useState(8_000);
const fee = useMemo(() => amount * 0.004, [amount]);
const reviewLane = useMemo(
  () => amount >= 10_000 ? "Manual review" : "Automatic",
  [amount],
);`,
    explanationAlternative: "React can compute the same two values, but it cannot testify about them: useMemo keeps no record of what recomputed, what was skipped, or why. The audit trail on this page has no React equivalent.",
    explanationForge: "Worth owns the input, both derived decisions, and the evidence. After each transaction the runtime reports what recomputed, what changed, and what stayed the same — the same trail this page exports as JSON.",
    whatYouGet: [
      "Transactional commits with one propagation record per change",
      "An audit trail read from signals.diagnostics(), not app state",
      "Per-value explanations via why(): reads, state, output change",
      "An exportable JSON decision trail backed by runtime history"
    ],
    relatedDocsPath: "learn/feature-index"
  },
  {
    id: 2,
    title: "The Read That Keeps a Record",
    purpose: "This looks like an ordinary product page backed by an ordinary fetch. It isn't. Give it a few seconds.",
    preface: "Every resource read here materializes as a line: a query result with a flight recorder attached. When server truth changes — whether you asked for it or not — the line keeps the receipt: status, freshness, provenance, and a lifecycle tape you can export.",
    difficulty: "Intermediate",
    primaryMessage: "When a value changes behind your back, the read itself can tell you why.",
    forgeCode: `const api = signals.api({ baseUrl: "/api/storefront" });

const product = api.url("/products/:productId").detail({
  reconcile: signals.resource.detailFields({
    price: { read: (v) => v.price, write: (v, price) => ({ ...v, price }) },
  }),
  load: ({ productId }) => fetchProduct(productId),
});

const line = product.line({ productId: "p-204" });

// the server pushes new truth — no user action involved
line.deliver(product.delivery.field({
  packetId: "pkt-08", basisId: "srv-v1", nextBasisId: "srv-v2",
  field: "price", value: 188,
}));

// the line kept the receipt
line.diagnostics().lastEffect.provenance; // "deliveredPatch"
line.history().lifecycle;                 // the full tape`,
    alternativeName: "Generic server-state query",
    alternativeCode: `const detail = useQuery({
  queryKey: ["product", productId],
  queryFn: () => loadProduct(productId),
});

// websocket pushes new truth into the cache
queryClient.setQueryData(["product", productId], next);

// the value changed on screen. why? when? from where?
// there is nothing to consult — the cache keeps no record.`,
    explanationAlternative: "Server-state hooks can fetch the data, and a push can rewrite the cache — but nothing records why a value changed. When users ask what happened, the answer lives in log aggregation, if anywhere.",
    explanationForge: "Worth materializes the read as a line. A server delivery lands with provenance, the lifecycle tape records it, and the value on screen can testify about its own history — exportable as JSON.",
    whatYouGet: [
      "A real backend push landing as a delivery effect with provenance",
      "Status and freshness as runtime truth, not app conventions",
      "A lifecycle tape read from line.history(), entry by entry",
      "Per-line recorders — switch products, each read has its own",
      "An exportable JSON line history"
    ],
    relatedDocsPath: "resources/index"
  },
  {
    id: 3,
    title: "The Form That Knows It Has Company",
    purpose: "A payout policy under dual control. Two simulated coworkers edit, comment, and lock fields around you — and the form itself decides, per person, who can write what and whether submit is allowed.",
    preface: "Three people hold the same form: you and two simulated coworkers, each with their own runtime-owned form controller over shared server truth. When Dana leases the limit field, one report produces three different verdicts — she can write, you cannot, and Priya's submit blocks only because her draft touches the leased field. Mostly watch; reach in whenever you like.",
    difficulty: "Beginner",
    primaryMessage: "One collaboration report in. One verdict out — per actor, with reasons.",
    forgeCode: `const form = signals.form({
  source: payoutPolicy,
  collaboration: {
    mode: "fieldLease",
    actorId: session.userId,
    supportsPresence: true,
    supportsComments: true,
  },
  fields: ({ field }) => ({
    limit: field("limit"),
    justification: field("justification"),
  }),
});

// your transport relays whatever is happening…
channel.on("collaboration", (event) => {
  form.reportCollaboration({
    posture: event.posture,
    leasedFields: event.leases,
    presence: event.presence,
  });
});

// …the runtime decides what it means for this client
form.fieldWritePosture("limit");
// → { canWrite: false, collaborator: "...", reason: "..." }
form.readiness();
// → blocked only if YOUR patch plan touches a leased field`,
    alternativeName: "React Hook Form + socket glue",
    alternativeCode: `const form = useForm({ values: policy });

// hand-rolled lock state beside the form library
const [locks, setLocks] = useState({});
socket.on("lock", (msg) => setLocks(...));

// disabled props, submit guards, and tooltips
// are all app conventions the form cannot verify
<input disabled={locks.limit && locks.limit !== me} />
<button disabled={
  form.formState.isDirty && locks[touchedField]
} />`,
    explanationAlternative: "The form library owns field state; the lock state lives in app code beside it. Nothing can verify that the disabled props, submit guards, and socket handlers agree — and nothing records why a submit was refused.",
    explanationForge: "Collaboration posture lives inside the form controller. One reported lease produces per-actor write posture and patch-plan-aware submit verdicts, each carrying the blocker and the collaborator's name — inspectable and exportable.",
    whatYouGet: [
      "Three real form controllers over one shared source — per-actor truth",
      "Field leases that block writes with the owner's name attached",
      "Patch-plan-aware submit verdicts: leases only block drafts that touch them",
      "Presence, comments, and a collaboration event recorder",
      "An exportable JSON collaboration report"
    ],
    relatedDocsPath: "forms/index"
  },
  {
    id: 4,
    title: "The Route That Checks Your Training",
    purpose: "A manufacturing execution portal where opening a batch step is an admission decision — checked against role, training, and the effective SOP revision, with the audit trail as a by-product.",
    preface: "You are an operator trained on SOP-042 rev B. Partway through the session, document control makes rev C effective — and the step you executed minutes ago now denies you, with the reason naming the revision. Proceed under a recorded deviation, then replay the whole session under different facts: the inspector's question, answered by the runtime.",
    difficulty: "Intermediate",
    primaryMessage: "Execution is an access decision. The router keeps the receipts.",
    forgeCode: `const stepExecution = signals.router.prerequisite(
  "stepExecution",
  async ({ facts, allow, forbidden }) => {
    if (facts.trainedRev !== facts.effectiveRev) {
      return forbidden({
        reason: "trainingSupersededByRevision",
        detail: \`Trained on rev \${facts.trainedRev}; effective is \${facts.effectiveRev}.\`,
      });
    }
    return allow({ reason: "trainingCurrent" });
  },
);

const routes = signals.router.define({
  stepExecute: signals.router.route("/batches/:batchId/steps/:stepId", {
    admission: [stepExecution],
    resources: {
      page: signals.router.resourceLine(stepFamily, { prefetch: "intent" }),
    },
  }),
});

// every navigation is an admission decision with a recorded outcome
const ingress = signals.router.browserHistory.push(href);
const report = await routes.admitBrowserHistoryIngress(ingress, session.facts);
story.record(report);

// the inspector's question, answered by the runtime
const replay = routes.simulateSequence(story.events().map((e) => e.targetHref));
await replay.run({ facts: { role: "operator", trainedRev: "B", effectiveRev: "C" } });`,
    alternativeName: "React Router v6 + audit pipeline",
    alternativeCode: `// guards scattered across loaders
export async function loader({ request }) {
  const user = await getUser(request);
  if (!user.trainedOn("SOP-042")) {
    // which revision? checked where? logged how?
    return redirect("/denied");
  }
  return fetchStep(params);
}

// the audit trail is a separate project:
// middleware, log shipping, retention, correlation…
logAccess(user.id, request.url, "allowed?");`,
    explanationAlternative: "Guards are booleans scattered across loaders, and the audit trail is a separate logging pipeline that has to be kept honest by convention. Replaying a session under different facts — the question an auditor actually asks — has no runtime answer at all.",
    explanationForge: "Admission is declared on the route and evaluated against live session facts. Every attempt — admitted, denied, or under deviation — lands as a recorded decision with its reason, and the same session can be re-asked under different facts in one call.",
    whatYouGet: [
      "Admission prerequisites that return decisions with reasons, not booleans",
      "Live facts — role, training, effective revision — checked at every ingress",
      "A recorded audit trail where denials are records too",
      "Deviation-based override as one prerequisite branch, permanently recorded",
      "Session replay under different facts — the inspector's question, answered",
      "Route-owned resource lines with intent prefetch"
    ],
    relatedDocsPath: "router/index"
  },
  {
    id: 5,
    title: "The Write That Admits It's Guessing",
    purpose: "Two optimistic writes overlap and one fails — watch the callback model and the runtime disagree about what is true.",
    preface: "Optimistic UI puts something on screen the server has not confirmed yet. That is fine — until two of those guesses overlap and one of them fails. The left window is the callback model exactly as TanStack Query's documentation recommends: snapshot in onMutate, restore in onError, invalidate on settle. The right window is the Worth runtime, where every optimistic patch is an admitted effect with provenance, and a server confirmation reconciles one item instead of overwriting the screen. Same clicks, same server, different truths.",
    difficulty: "Intermediate",
    primaryMessage: "Worth writes are effects with provenance — confirmations are item-scoped and cannot clobber the screen.",
    forgeCode: `const po = signals.api({
  baseUrl: "/api/procurement",
  effects: signals.resource.effects.branchNative(),
});

const poLines = po.url("/orders/:orderId/lines")
  .response(signals.resource.response.array({ itemId: (line) => line.id }))
  .list({ load: ({ orderId }) => client.fetchLines(orderId) });

const saveLine = po.url("/orders/:orderId/lines/:lineId")
  .update({
    // a confirmation replaces one item — nothing else on screen is touched
    reconciles: [{ family: poLines, params: ({ orderId }) => ({ orderId }),
      collection: { kind: "item" } }],
    load: ({ orderId, lineId, body }) => client.saveLine(orderId, lineId, body),
  });

line.diagnostics().lastEffect;  // provenance, confirmation, rollback posture
line.history().lifecycle;       // what the screen showed, and when`,
    alternativeName: "React Query (TanStack Query)",
    alternativeCode: `const addLine = useMutation({
  mutationFn: saveLine,
  onMutate: async (line) => {
    await queryClient.cancelQueries({ queryKey: ["po", "lines"] });
    const previous = queryClient.getQueryData(["po", "lines"]);
    queryClient.setQueryData(["po", "lines"], (cur = []) => [...cur, line]);
    return { previous };
  },
  // restores a whole-cache snapshot — including anything
  // that was confirmed after the snapshot was taken
  onError: (_err, _line, ctx) =>
    queryClient.setQueryData(["po", "lines"], ctx?.previous),
  onSettled: () => {
    // even the recommended fix needs a concurrency guard
    if (queryClient.isMutating() === 1)
      queryClient.invalidateQueries({ queryKey: ["po", "lines"] });
  },
});`,
    explanationAlternative: "The rollback is your code restoring your closure variable. The cache cannot tell speculative rows from confirmed ones, so a failed write's rollback silently un-confirms whatever settled after its snapshot — and no record remains that the screen ever changed.",
    explanationForge: "Every optimistic patch is an admitted effect with an envelope: provenance, confirmation kind, rollback posture. Server confirmations reconcile exactly one item, so overlapping writes cannot clobber each other, and the whole incident stays inspectable in line.history().lifecycle.",
    whatYouGet: [
      "Effect envelopes with provenance on every write",
      "Item-scoped server confirmations that preserve pending rows",
      "Recorded lifecycle history of what the screen showed",
      "useManagedResourceWrite — the whole lifecycle in one hook"
    ],
    relatedDocsPath: "resources/index"
  },
  {
    id: 6,
    title: "Route-Coupled Resource Form",
    purpose: "Show the first stacked composed example linking routes, resources, and forms.",
    preface: "This is the adapter-tax demo. TanStack, Formik, and a router are not the problem; the problem is the layer you write to translate query status into form status, mutation status into route-leave rules, server results into cache patches, and lifecycle events into toasts. The Worth block shows the same workflow when those contracts already line up.",
    difficulty: "Advanced",
    primaryMessage: "Worth primitives compose into workflows without requiring a separate orchestration layer.",
    forgeCode: `// Combine Routing + Resources + Forms in a single flow
const routes = signals.router.define({
  detail: signals.router.route("/tasks/:taskId"),
  edit: signals.router.route("/tasks/:taskId/edit")
});

const taskDetail = api.url("/tasks/:taskId").detail({
  load: ({ taskId }) => fetchTask(taskId)
});

// Backing form directly with the resource line
const form = signals.form({
  source: taskDetail.line({ taskId }).toSource(),
  fields: ({ field }) => ({
    title: field("title"),
    status: field("status")
  })
});`,
    alternativeName: "React Router + Formik + React Query",
    alternativeCode: `// Requires complex useEffect chains to map query fields
// into Formik initial values, along with search param triggers
useEffect(() => {
  if (query.data) {
    formik.setValues(query.data);
  }
}, [query.data]);

// Dynamic route checking needed to prevent unsaved changes`,
    explanationAlternative: "High coordination debt. Demands fragile useEffect syncer loops, manual dirty caches inside routers to blocks exits, and context bridging.",
    explanationForge: "Primitives align naturally. Form binds natively to Resource Lines, and the route admission process checks form.readiness to intercept unsaved departures.",
    whatYouGet: [
      "Zero-glue form + resource + router bindings",
      "Auto-warming resource lines on route transition",
      "Preserved draft continuity between routes",
      "Dynamic route exit validation guards"
    ],
    relatedDocsPath: "forms/route-coupling/route-authority-handoff"
  }
];
