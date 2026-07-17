import {
  DEMO_FIVE_CODE,
  DEMO_FOUR_CODE,
  DEMO_ONE_CODE,
  DEMO_THREE_CODE,
  DEMO_TWO_CODE,
} from "./demoCodeSamples.ts";

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
    title: "Diagnostics No Other State System Has",
    purpose: "Worth has the strongest diagnostics model in web state management. Move one transfer amount, inspect the runtime's exact causal evidence, and ask any derived decision why it behaved that way.",
    preface: "Move a transfer below and across the $10,000 review line. Worth records what was touched, what recomputed, what actually changed, and which dependencies each decision read. Then click a value and ask the same runtime that made the decision to explain it.",
    difficulty: "Beginner",
    primaryMessage: "A derived decision should be able to show its work.",
    WORTHCode: DEMO_ONE_CODE,
    alternativeName: "React useState + useMemo",
    alternativeCode: `import { useState, useMemo } from "react";

const [amount, setAmount] = useState(8_000);
const fee = useMemo(() => amount * 0.004, [amount]);
const reviewLane = useMemo(
  () => amount >= 10_000 ? "Manual review" : "Automatic",
  [amount],
);`,
    explanationAlternative: "React can calculate the same answers. It cannot tell you which policy ran, whether its answer changed, or what input it read unless you build a second evidence system beside it. That is a lot of detective work for two tiny calculations.",
    explanationWORTH: "Worth owns the input, the decisions, and the evidence connecting them. React only keeps the visible comparison list. The useful opinion here is simple: if software makes an important decision, asking why should be a normal API call—not an incident-response exercise.",
    whatYouGet: [
      "One transaction—and one receipt—for each committed change",
      "A direct answer to ‘why did this value do that?’",
      "Proof that a policy can run without changing its answer",
      "An honest boundary between runtime evidence and the UI that displays it"
    ],
    relatedDocsPath: "core/diagnostics"
  },
  {
    id: 2,
    title: "The Read That Keeps a Record",
    purpose: "This looks like an ordinary product page backed by an ordinary fetch. It isn't. Give it a few seconds.",
    preface: "Every resource read here materializes as a line: a query result with a flight recorder attached. When server truth changes — whether you asked for it or not — the line keeps the receipt: status, freshness, provenance, and a lifecycle tape you can export.",
    difficulty: "Intermediate",
    primaryMessage: "When a value changes behind your back, the read itself can tell you why.",
    WORTHCode: DEMO_TWO_CODE,
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
    explanationWORTH: "Worth materializes the read as a line. A server delivery lands with provenance, the lifecycle tape records it, and the value on screen can testify about its own history — exportable as JSON.",
    whatYouGet: [
      "A real backend push landing as a delivery effect with provenance",
      "Status and freshness as runtime truth, not app conventions",
      "A lifecycle tape read from line.history(), entry by entry",
      "Per-line recorders — switch products, each read has its own",
      "An exportable JSON line history"
    ],
    relatedDocsPath: "resources/debugging/README"
  },
  {
    id: 3,
    title: "The Form That Knows It Has Company",
    purpose: "A payout policy under dual control. Two simulated coworkers edit, comment, and lock fields around you — and the form itself decides, per person, who can write what and whether submit is allowed.",
    preface: "Three people hold the same form: you and two simulated coworkers, each with their own runtime-owned form controller over shared server truth. When Dana leases the limit field, one report produces three different verdicts — she can write, you cannot, and Priya's submit blocks only because her draft touches the leased field. Mostly watch; reach in whenever you like.",
    difficulty: "Beginner",
    primaryMessage: "One collaboration report in. One verdict out — per actor, with reasons.",
    WORTHCode: DEMO_THREE_CODE,
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
    explanationWORTH: "Collaboration posture lives inside the form controller. One reported lease produces per-actor write posture and patch-plan-aware submit verdicts, each carrying the blocker and the collaborator's name — inspectable and exportable.",
    whatYouGet: [
      "Three real form controllers over one shared source — per-actor truth",
      "Field leases that block writes with the owner's name attached",
      "Patch-plan-aware submit verdicts: leases only block drafts that touch them",
      "Presence, comments, and a collaboration event recorder",
      "An exportable JSON collaboration report"
    ],
    relatedDocsPath: "forms/collaboration/README"
  },
  {
    id: 4,
    title: "The Route That Checks Your Training",
    purpose: "A manufacturing execution portal where opening a batch step is an admission decision — checked against role, training, and the effective SOP revision, with the audit trail as a by-product.",
    preface: "You are an operator trained on SOP-042 rev B. Partway through the session, document control makes rev C effective — and the step you executed minutes ago now denies you, with the reason naming the revision. Proceed under a recorded deviation, then replay the whole session under different facts: the inspector's question, answered by the runtime.",
    difficulty: "Intermediate",
    primaryMessage: "Execution is an access decision. The router keeps the receipts.",
    WORTHCode: DEMO_FOUR_CODE,
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
    explanationWORTH: "Admission is declared on the route and evaluated against live session facts. Every attempt — admitted, denied, or under deviation — lands as a recorded decision with its reason, and the same session can be re-asked under different facts in one call.",
    whatYouGet: [
      "Admission prerequisites that return decisions with reasons, not booleans",
      "Live facts — role, training, effective revision — checked at every ingress",
      "A recorded audit trail where denials are records too",
      "Deviation-based override as one prerequisite branch, permanently recorded",
      "Session replay under different facts — the inspector's question, answered",
      "Route-owned resource lines with intent prefetch"
    ],
    relatedDocsPath: "router/admission/admit"
  },
  {
    id: 5,
    title: "Every Write Is a Branch",
    purpose: "Concurrent optimistic writes — an independent sibling, a failing parent, and its dependent child — settle out of order while a server-truth referee judges both screens live.",
    preface: "Optimistic UI puts something on screen the server has not confirmed yet. That is fine — until several of those guesses overlap and one of them fails. The left window is the callback model exactly as TanStack Query's documentation recommends: snapshot in onMutate, restore in onError, invalidate on settle. In the right window every optimistic write forks its own effect branch: rejection retires one branch, confirmation merges one branch, and a dependent write is a child branch that closes out with its parent. A server-truth strip referees both screens, and each wears a live badge saying whether it still agrees with the server.",
    difficulty: "Intermediate",
    primaryMessage: "One write, one branch. Rejection retires a branch — it never restores a shared snapshot.",
    WORTHCode: DEMO_FIVE_CODE,
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
    explanationWORTH: "Every admitted write owns a native effect branch with an explicit fork basis and declared dependencies. Rejection retires exactly one branch (and closes out its dependents by policy); confirmation reconciles one resource locus and merges one branch. The visible value is a derived projection — rebuildable from canonical truth plus open effects — and every claim on screen is a runtime-issued receipt.",
    whatYouGet: [
      "One effect branch per optimistic write, drawn live as a graph",
      "Declared parent/child dependencies with typed closeout",
      "A server-truth referee and live agreement badges on both screens",
      "Arbitrary-order settlement: ten branches converge with zero residue",
      "Clickable runtime receipts — branch, dependencies, terminal outcome"
    ],
    relatedDocsPath: "resources/effects/README"
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
