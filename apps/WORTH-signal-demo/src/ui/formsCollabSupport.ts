export type Actor = "you" | "priya" | "dana";

export interface PolicyDraft {
  limit: number;
  justification: string;
  notes: string;
}

export const SOURCE_POLICY: PolicyDraft = Object.freeze({
  limit: 25_000,
  justification: "Vendor payment volume increased in Q3.",
  notes: "",
});

export const ACTOR_META: Record<Actor, { name: string; role: string; simulated: boolean }> = {
  you: { name: "You", role: "analyst", simulated: false },
  priya: { name: "Priya S.", role: "maker", simulated: true },
  dana: { name: "Dana R.", role: "reviewer", simulated: true },
};

export type PresenceStatus = "active" | "idle" | "viewing";

export interface CollabShape {
  leaseOwner: Actor | null;
  presence: Partial<Record<Actor, PresenceStatus>>;
  comment: { id: string; authorId: Actor; target: string; body: string } | null;
  reason: string;
}

export const INITIAL_COLLAB: CollabShape = {
  leaseOwner: null,
  presence: { priya: "active", dana: "viewing" },
  comment: null,
  reason: "collaboration posture is settled",
};

export type FeedKind = "presence" | "comment" | "lease" | "edit" | "submit" | "info";

export interface FeedEntry {
  id: number;
  actor: Actor;
  kind: FeedKind;
  text: string;
}

export const FEED_KIND_LABEL: Record<FeedKind, string> = {
  presence: "presenceChange",
  comment: "commentChange",
  lease: "leaseChange",
  edit: "draftWrite",
  submit: "sourceUpdate",
  info: "info",
};

export const currency = new Intl.NumberFormat("en-US", {
  currency: "USD",
  maximumFractionDigits: 0,
  style: "currency",
});

export const PRIYA_LIMITS = [40_000, 32_000, 48_000];
export const PRIYA_JUSTIFICATIONS = [
  "Vendor volume up 40% quarter over quarter.",
  "Two new logistics vendors onboarding in August.",
  "Seasonal inventory build ahead of Q4.",
];
export const DANA_COMMENT_BODIES = [
  "Anything above $30k needs the board threshold check.",
  "Confirm this stays under the delegated authority cap.",
  "Please attach the vendor risk score before submit.",
];
