export interface DemoMetadata {
  id: number;
  title: string;
  purpose: string;
  preface: string;
  difficulty: "Beginner" | "Intermediate" | "Advanced";
  primaryMessage: string;
  WORTHCode: string;
  alternativeCode: string;
  alternativeName: string;
  explanationAlternative: string;
  explanationWORTH: string;
  whatYouGet: string[];
  relatedDocsPath: string;
}

export const demoRegistry: DemoMetadata[] = [
  {
    id: 1,
    title: "Explainable Transfer Decision",
    purpose: "Move one transfer amount and watch the runtime itself report what recomputed, what changed, and what stayed the same Ã¢â‚¬â€ an audit trail read from the runtime, not kept by the UI.",
    preface: "Drag the amount across the $10,000 policy threshold. Every committed change lands as one transaction, and every row in the audit trail is read back from signals.diagnostics() after the commit Ã¢â‚¬â€ including the moments where the review lane recomputed but its answer did not change. Click any value to ask the runtime why.",
    difficulty: "Beginner",
    primaryMessage: "One input. Two decisions. A runtime that shows its work.",
    WORTHCode: `import { createSignals } from "worth-signals-wasm";
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
    explanationWORTH: "Worth owns the input, both derived decisions, and the evidence. After each transaction the runtime reports what recomputed, what changed, and what stayed the same Ã¢â‚¬â€ the same trail this page exports as JSON.",
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
    preface: "Every resource read here materializes as a line: a query result with a flight recorder attached. When server truth changes Ã¢â‚¬â€ whether you asked for it or not Ã¢â‚¬â€ the line keeps the receipt: status, freshness, provenance, and a lifecycle tape you can export.",
    difficulty: "Intermediate",
    primaryMessage: "When a value changes behind your back, the read itself can tell you why.",
    WORTHCode: `const api = signals.api({ baseUrl: "/api/storefront" });

const product = api.url("/products/:productId").detail({
  reconcile: signals.resource.detailFields({
    price: { read: (v) => v.price, write: (v, price) => ({ ...v, price }) },
  }),
  load: ({ productId }) => fetchProduct(productId),
});

const line = product.line({ productId: "p-204" });

// the server pushes new truth Ã¢â‚¬â€ no user action involved
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
// there is nothing to consult Ã¢â‚¬â€ the cache keeps no record.`,
    explanationAlternative: "Server-state hooks can fetch the data, and a push can rewrite the cache Ã¢â‚¬â€ but nothing records why a value changed. When users ask what happened, the answer lives in log aggregation, if anywhere.",
    explanationWORTH: "Worth materializes the read as a line. A server delivery lands with provenance, the lifecycle tape records it, and the value on screen can testify about its own history Ã¢â‚¬â€ exportable as JSON.",
    whatYouGet: [
      "A real backend push landing as a delivery effect with provenance",
      "Status and freshness as runtime truth, not app conventions",
      "A lifecycle tape read from line.history(), entry by entry",
      "Per-line recorders Ã¢â‚¬â€ switch products, each read has its own",
      "An exportable JSON line history"
    ],
    relatedDocsPath: "resources/index"
  },
  {
    id: 3,
    title: "The Form That Knows It Has Company",
    purpose: "A payout policy under dual control. Two simulated coworkers edit, comment, and lock fields around you Ã¢â‚¬â€ and the form itself decides, per person, who can write what and whether submit is allowed.",
    preface: "Three people hold the same form: you and two simulated coworkers, each with their own runtime-owned form controller over shared server truth. When Dana leases the limit field, one report produces three different verdicts Ã¢â‚¬â€ she can write, you cannot, and Priya's submit blocks only because her draft touches the leased field. Mostly watch; reach in whenever you like.",
    difficulty: "Beginner",
    primaryMessage: "One collaboration report in. One verdict out Ã¢â‚¬â€ per actor, with reasons.",
    WORTHCode: `const form = signals.form({
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

// your transport relays whatever is happeningÃ¢â‚¬Â¦
channel.on("collaboration", (event) => {
  form.reportCollaboration({
    posture: event.posture,
    leasedFields: event.leases,
    presence: event.presence,
  });
});

// The runtime decides what it means for this client.
form.fieldWritePosture("limit");
// => { canWrite: false, collaborator: "...", reason: "..." }
form.readiness();
// => blocked only if YOUR patch plan touches a leased field`,
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
    explanationAlternative: "The form library owns field state; the lock state lives in app code beside it. Nothing can verify that the disabled props, submit guards, and socket handlers agree Ã¢â‚¬â€ and nothing records why a submit was refused.",
    explanationWORTH: "Collaboration posture lives inside the form controller. One reported lease produces per-actor write posture and patch-plan-aware submit verdicts, each carrying the blocker and the collaborator's name Ã¢â‚¬â€ inspectable and exportable.",
    whatYouGet: [
      "Three real form controllers over one shared source Ã¢â‚¬â€ per-actor truth",
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
    purpose: "A manufacturing execution portal where opening a batch step is an admission decision Ã¢â‚¬â€ checked against role, training, and the effective SOP revision, with the audit trail as a by-product.",
    preface: "You are an operator trained on SOP-042 rev B. Partway through the session, document control makes rev C effective Ã¢â‚¬â€ and the step you executed minutes ago now denies you, with the reason naming the revision. Proceed under a recorded deviation, then replay the whole session under different facts: the inspector's question, answered by the runtime.",
    difficulty: "Intermediate",
    primaryMessage: "Execution is an access decision. The router keeps the receipts.",
    WORTHCode: `const stepExecution = signals.router.prerequisite(
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
// middleware, log shipping, retention, correlationÃ¢â‚¬Â¦
logAccess(user.id, request.url, "allowed?");`,
    explanationAlternative: "Guards are booleans scattered across loaders, and the audit trail is a separate logging pipeline that has to be kept honest by convention. Replaying a session under different facts Ã¢â‚¬â€ the question an auditor actually asks Ã¢â‚¬â€ has no runtime answer at all.",
    explanationWORTH: "Admission is declared on the route and evaluated against live session facts. Every attempt Ã¢â‚¬â€ admitted, denied, or under deviation Ã¢â‚¬â€ lands as a recorded decision with its reason, and the same session can be re-asked under different facts in one call.",
    whatYouGet: [
      "Admission prerequisites that return decisions with reasons, not booleans",
      "Live facts Ã¢â‚¬â€ role, training, effective revision Ã¢â‚¬â€ checked at every ingress",
      "A recorded audit trail where denials are records too",
      "Deviation-based override as one prerequisite branch, permanently recorded",
      "Session replay under different facts Ã¢â‚¬â€ the inspector's question, answered",
      "Route-owned resource lines with intent prefetch"
    ],
    relatedDocsPath: "router/index"
  },
  {
    id: 5,
    title: "Every Write Is a Branch",
    purpose: "Concurrent optimistic writes Ã¢â‚¬â€ an independent sibling, a failing parent, and its dependent child Ã¢â‚¬â€ settle out of order while a server-truth referee judges both screens live.",
    preface: "Optimistic UI puts something on screen the server has not confirmed yet. That is fine Ã¢â‚¬â€ until several of those guesses overlap and one of them fails. The left window is the callback model exactly as TanStack Query's documentation recommends: snapshot in onMutate, restore in onError, invalidate on settle. In the right window every optimistic write forks its own effect branch: rejection retires one branch, confirmation merges one branch, and a dependent write is a child branch that closes out with its parent. A server-truth strip referees both screens, and each wears a live badge saying whether it still agrees with the server.",
    difficulty: "Intermediate",
    primaryMessage: "One write, one branch. Rejection retires a branch Ã¢â‚¬â€ it never restores a shared snapshot.",
    WORTHCode: `const po = signals.api({
  baseUrl: "/api/procurement",
  effects: signals.resource.effects.branchNative(),
});

const poLines = po.url("/orders/:orderId/lines")
  .response(signals.resource.response.array({ itemId: (line) => line.id }))
  .list({ load: ({ orderId }) => client.fetchLines(orderId) });

// each admitted write owns a native branch; dependencies are declared
const admission = await line.patch(resourcePatch.dependsOn(
  poLines.patch.insert({ itemId, placement: "append", nextItem }),
  [parentEffectId],
));

// settlement is per-effect: merge one branch, or retire one branch
await line.effects().confirm(admission.effectId, { serverPatch });
await line.effects().reject(admission.effectId, { responseId });

line.effects().get(effectId);   // branch, dependencies, terminal receipt
line.effects().projection();    // the derived visible fold Ã¢â‚¬â€ rebuildable`,
    alternativeName: "React Query (TanStack Query)",
    alternativeCode: `const addLine = useMutation({
  mutationFn: saveLine,
  onMutate: async (line) => {
    await queryClient.cancelQueries({ queryKey: ["po", "lines"] });
    const previous = queryClient.getQueryData(["po", "lines"]);
    queryClient.setQueryData(["po", "lines"], (cur = []) => [...cur, line]);
    return { previous };
  },
  // restores a whole-cache snapshot Ã¢â‚¬â€ including anything
  // that was confirmed after the snapshot was taken
  onError: (_err, _line, ctx) =>
    queryClient.setQueryData(["po", "lines"], ctx?.previous),
  onSettled: () => {
    // even the recommended fix needs a concurrency guard
    if (queryClient.isMutating() === 1)
      queryClient.invalidateQueries({ queryKey: ["po", "lines"] });
  },
});`,
    explanationAlternative: "The rollback is your code restoring your closure variable. The cache cannot tell speculative rows from confirmed ones, so a failed write's rollback silently un-confirms whatever settled after its snapshot Ã¢â‚¬â€ and no record remains that the screen ever changed.",
    explanationWORTH: "Every admitted write owns a native effect branch with an explicit fork basis and declared dependencies. Rejection retires exactly one branch (and closes out its dependents by policy); confirmation reconciles one resource locus and merges one branch. The visible value is a derived projection Ã¢â‚¬â€ rebuildable from canonical truth plus open effects Ã¢â‚¬â€ and every claim on screen is a runtime-issued receipt.",
    whatYouGet: [
      "One effect branch per optimistic write, drawn live as a graph",
      "Declared parent/child dependencies with typed closeout",
      "A server-truth referee and live agreement badges on both screens",
      "Arbitrary-order settlement: ten branches converge with zero residue",
      "Clickable runtime receipts Ã¢â‚¬â€ branch, dependencies, terminal outcome"
    ],
    relatedDocsPath: "resources/index"
  },
  {
    id: 6,
    title: "Merge Aspects, Not Objects",
    purpose: "Two branches edit one gear. Worth merges them aspect by aspect against the fork basis — disjoint edits compose themselves, and a real collision becomes one decision.",
    preface: "Branching is the easy part. Deciding what a merge means is where most state management gives up and hands you last-write-wins. Here, every commit names the exact aspects it changed — thickness, gear count, hole size — so the runtime can compare each aspect to the basis both branches forked from. Different aspects? They merge without a question. The same aspect on both sides? You get a review with both values, and nothing is overwritten silently. Every commit stays inspectable afterward, and the code on this page is the production source, excerpted live.",
    difficulty: "Advanced",
    primaryMessage: "Declared aspects make merges mechanical: disjoint edits compose, collisions become one decision.",
    WORTHCode: `const branch = await gearTruth.forkBranch({
  parentBranchId: main.id,
  expectedParentBasis: main.basis,
  name: "Design branch",
});

await gearTruth.commit({
  branchId: branch.id,
  expectedBasis: branch.basis,
  operations: gearChanges,
});

const review = await gearTruth.previewMerge(mergeRequest);
await gearTruth.resolveMerge({ reviewId: review.id, selections: [] });`,
    alternativeName: "UI-owned object merge",
    alternativeCode: `// The component becomes the truth authority.
const merged = {
  ...target,
  teeth: chooseSource ? source.teeth : target.teeth,
};
setState(merged);`,
    explanationAlternative: "The component decides the merged values itself. It has no record of the basis either side started from, so it cannot tell a deliberate change from a stale overwrite — and after setState, no evidence remains that a merge happened at all.",
    explanationWORTH: "A process-local TypeScript authority owns the branches, the aspect commits, merge admission, and the atomic merge commit. Everything on the page — values, conflicts, history, the signal projection strip — is read back from that authority, not composed in React.",
    whatYouGet: [
      "Two writable branches forked from one basis",
      "One-aspect commits: release a slider, commit one locus",
      "Merges that only ask about true collisions — with both values shown",
      "A commit graph where every node is a sealed, inspectable snapshot",
      "A live Signal projection receipt — its native basis digest advances with every commit"
    ],
    relatedDocsPath: "local-truth/branch-merge"
  }
];
