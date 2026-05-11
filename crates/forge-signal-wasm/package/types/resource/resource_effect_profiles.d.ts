declare const forgeSignalResourceEffectProfileBrand: unique symbol;

export type ResourceEffectProfileName =
  | "branchNative"
  | "serverCanonical"
  | "pessimistic"
  | "deliveryAuthoritative"
  | "nonReversible"
  | "sensitive"
  | string;

export type ResourceEffectOptimism = "branchSpeculative" | "none";
export type ResourceEffectConfirmation =
  | "exact"
  | "serverCanonical"
  | "acceptedPendingDelivery";
export type ResourceEffectRollback =
  | "branchRestore"
  | "branchRestoreOrInverse"
  | "unavailable";
export type ResourceEffectRebase = "nativeMergePlan" | "unavailable";
export type ResourceEffectPreimage =
  | "none"
  | "compactInverse"
  | "digestOnly"
  | "retainedFragment";

export interface ResourceEffectProfile {
  readonly name: ResourceEffectProfileName;
  readonly optimism: ResourceEffectOptimism;
  readonly confirmation: ResourceEffectConfirmation;
  readonly rollback: ResourceEffectRollback;
  readonly rebase: ResourceEffectRebase;
  readonly preimage: ResourceEffectPreimage;
  readonly [forgeSignalResourceEffectProfileBrand]: "resourceEffectProfile";
}

export interface ResourceEffectProfileOptions {
  readonly name: string;
  readonly optimism: ResourceEffectOptimism;
  readonly confirmation: ResourceEffectConfirmation;
  readonly rollback: ResourceEffectRollback;
  readonly rebase: ResourceEffectRebase;
  readonly preimage: ResourceEffectPreimage;
}

export interface ResourceEffects {
  branchNative(): ResourceEffectProfile;
  serverCanonical(): ResourceEffectProfile;
  pessimistic(): ResourceEffectProfile;
  deliveryAuthoritative(): ResourceEffectProfile;
  nonReversible(): ResourceEffectProfile;
  sensitive(): ResourceEffectProfile;
  custom(options: ResourceEffectProfileOptions): ResourceEffectProfile;
}

export const resourceEffects: ResourceEffects;
