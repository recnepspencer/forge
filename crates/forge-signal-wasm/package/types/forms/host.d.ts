import type {
  CallableFormSignal,
  FormSourceValue,
} from "./core.js";
import type {
  HostOnlineCapability,
  HostPersistenceCapability,
  HostViewportCapability,
  HostVisibilityCapability,
} from "../callable_surface.js";

export type FormHostRequiredCapability =
  | "online"
  | "persistence"
  | "credentials"
  | "autofill";

export type FormFocusHostBinding =
  | string
  | null
  | (() => string | null)
  | CallableFormSignal<string | null>;

export type FormVisibilityHostBinding =
  | "visible"
  | "hidden"
  | (() => "visible" | "hidden")
  | CallableFormSignal<"visible" | "hidden">
  | HostVisibilityCapability;

export type FormViewportHostBinding =
  | {
    readonly width: number;
    readonly height: number;
  }
  | (() => { readonly width: number; readonly height: number })
  | CallableFormSignal<{ readonly width: number; readonly height: number }>
  | HostViewportCapability;

export type FormOnlineHostBinding =
  | boolean
  | "online"
  | "offline"
  | (() => boolean | "online" | "offline")
  | CallableFormSignal<boolean | "online" | "offline">
  | HostOnlineCapability;

export type FormAvailabilityHostBinding =
  | boolean
  | (() => boolean)
  | CallableFormSignal<boolean>;

export type FormPersistenceHostBinding<T = unknown> =
  | FormAvailabilityHostBinding
  | HostPersistenceCapability<T>;

export interface FormHostBindings {
  readonly focus?: FormFocusHostBinding;
  readonly visibility?: FormVisibilityHostBinding;
  readonly viewport?: FormViewportHostBinding;
  readonly online?: FormOnlineHostBinding;
  readonly persistence?: FormPersistenceHostBinding<FormSourceValue>;
  readonly credentials?: FormAvailabilityHostBinding;
  readonly autofill?: FormAvailabilityHostBinding;
}

export interface FormHostFocusFact {
  readonly fact: "focus";
  readonly declared: boolean;
  readonly posture: "supported" | "unavailable";
  readonly focusedField?: string | null;
  readonly reason: string | null;
  readonly digest: string;
}

export interface FormHostVisibilityFact {
  readonly fact: "visibility";
  readonly declared: boolean;
  readonly posture: "supported" | "unavailable";
  readonly state?: "visible" | "hidden" | null;
  readonly reason: string | null;
  readonly digest: string;
}

export interface FormHostViewportFact {
  readonly fact: "viewport";
  readonly declared: boolean;
  readonly posture: "supported" | "unavailable";
  readonly size?: {
    readonly width: number;
    readonly height: number;
  } | null;
  readonly reason: string | null;
  readonly digest: string;
}

export interface FormHostOnlineFact {
  readonly fact: "online";
  readonly declared: boolean;
  readonly posture: "supported" | "unavailable";
  readonly state?: "online" | "offline" | null;
  readonly reason: string | null;
  readonly digest: string;
}

export interface FormHostAvailabilityFact {
  readonly fact: "persistence" | "credentials" | "autofill";
  readonly declared: boolean;
  readonly posture: "supported" | "unavailable";
  readonly available?: boolean | null;
  readonly reason: string | null;
  readonly digest: string;
}

export interface FormHostReport {
  readonly facts: {
    readonly focus: FormHostFocusFact;
    readonly visibility: FormHostVisibilityFact;
    readonly viewport: FormHostViewportFact;
    readonly online: FormHostOnlineFact;
    readonly persistence: FormHostAvailabilityFact;
    readonly credentials: FormHostAvailabilityFact;
    readonly autofill: FormHostAvailabilityFact;
  };
  readonly summary: {
    readonly supported: number;
    readonly unavailable: number;
  };
  readonly counters: {
    readonly costBasis: "hostFactDerivedRead";
    readonly incrementalStatus: "notIncremental";
    readonly declaredFacts: number;
    readonly supportedFacts: number;
    readonly unavailableFacts: number;
    readonly hostHandleFacts: number;
  };
  readonly digest: string;
}
