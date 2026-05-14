import type { FormCollaborationArtifact } from "./collaboration.js";
import type { FormExitPresentationArtifact } from "./exit.js";
import type { FormHandoffPresentationArtifact } from "./handoff.js";
import type { FormAttachmentPresentationArtifact } from "./attachments.js";
import type { FormMediaPresentationArtifact } from "./media.js";
import type {
  FormPresentationHistoryArtifact,
  FormPresentationLaneUpdateArtifact,
  FormPresentationLifecycleArtifact,
  FormPresentationReport,
  FormPresentationSettlementArtifact,
} from "./presentation.js";
import type { FormMessagePresentationArtifact } from "./messages.js";

type FormScopedMessagePresentationUpdate =
  | {
      readonly status: "pending" | "busy" | "settling" | "ready" | "failed" | "unavailable";
      readonly target?: string | null;
      readonly reason: string;
      readonly token?: string | null;
      readonly scope?: "wholeForm";
      readonly channel?: "inline" | "summary" | "banner" | "toast";
      readonly audience?: "user" | "developer" | "system";
      readonly visibleCount?: number;
      readonly operation?: "show" | "update" | "dismiss" | "clear";
    }
  | {
      readonly status: "pending" | "busy" | "settling" | "ready" | "failed" | "unavailable";
      readonly target: string;
      readonly reason: string;
      readonly token?: string | null;
      readonly scope: "field" | "section" | "action" | "control" | "step";
      readonly channel?: "inline" | "summary" | "banner" | "toast";
      readonly audience?: "user" | "developer" | "system";
      readonly visibleCount?: number;
      readonly operation?: "show" | "update" | "dismiss" | "clear";
    };

export interface FormControllerPresentationBindings {
  presentation(): FormPresentationReport;
  presentationLifecycle(laneId?: string): FormPresentationReport | FormPresentationLifecycleArtifact | null;
  reportPresentationLane(
    laneId: "collaboration",
    update: {
      readonly status: "pending" | "busy" | "settling" | "ready" | "failed" | "unavailable";
      readonly target?: string | null;
      readonly reason: string;
      readonly token?: string | null;
    },
  ): FormPresentationLaneUpdateArtifact;
  reportPresentationLane(
    laneId: "attachments",
    update: {
      readonly status: "pending" | "busy" | "settling" | "ready" | "failed" | "unavailable";
      readonly target?: string | null;
      readonly reason: string;
      readonly token?: string | null;
      readonly section: string;
    },
  ): FormPresentationLaneUpdateArtifact;
  reportPresentationLane(
    laneId: "media",
    update: {
      readonly status: "pending" | "busy" | "settling" | "ready" | "failed" | "unavailable";
      readonly target?: string | null;
      readonly reason: string;
      readonly token?: string | null;
      readonly surfaceId: string;
      readonly scopeKind?: "modal" | null;
    },
  ): FormPresentationLaneUpdateArtifact;
  reportPresentationLane(
    laneId: "handoff" | "exit",
    update: {
      readonly status: "pending" | "busy" | "settling" | "ready" | "failed" | "unavailable";
      readonly target?: string | null;
      readonly reason: string;
      readonly token?: string | null;
      readonly scopeKind: "route" | "modal" | "external";
      readonly surfaceId: string;
    },
  ): FormPresentationLaneUpdateArtifact;
  clearPresentationLane(
    laneId: "collaboration" | "exit" | "attachments" | "media" | "handoff",
    options?: { readonly reason?: string },
  ): FormPresentationLaneUpdateArtifact;
  reportExit(update: {
    readonly status: "pending" | "busy" | "settling" | "ready" | "failed" | "unavailable";
    readonly target?: string | null;
    readonly reason: string;
    readonly token?: string | null;
    readonly scopeKind: "route" | "modal" | "external";
    readonly surfaceId: string;
    readonly operation?: "generic" | "block" | "confirm" | "dismiss" | "leave" | "stay" | "close";
    readonly unsupportedReason?: string | null;
  }): FormExitPresentationArtifact;
  clearExit(options?: { readonly reason?: string }): FormExitPresentationArtifact;
  reportHandoff(update: {
    readonly status: "pending" | "busy" | "settling" | "ready" | "failed" | "unavailable";
    readonly target?: string | null;
    readonly reason: string;
    readonly token?: string | null;
    readonly scopeKind: "route" | "modal" | "external";
    readonly surfaceId: string;
    readonly operation?: "generic" | "open" | "handoff" | "dismiss" | "return" | "close";
    readonly unsupportedReason?: string | null;
  }): FormHandoffPresentationArtifact;
  clearHandoff(options?: { readonly reason?: string }): FormHandoffPresentationArtifact;
  reportAttachments(update: {
    readonly status: "pending" | "busy" | "settling" | "ready" | "failed" | "unavailable";
    readonly target?: string | null;
    readonly reason: string;
    readonly token?: string | null;
    readonly section: string;
    readonly selectedCount?: number;
    readonly stagedCount?: number;
    readonly failedCount?: number;
    readonly operation?: "generic" | "select" | "stage" | "preview" | "remove" | "clear";
  }): FormAttachmentPresentationArtifact;
  clearAttachments(options?: { readonly reason?: string }): FormAttachmentPresentationArtifact;
  reportMedia(update: {
    readonly status: "pending" | "busy" | "settling" | "ready" | "failed" | "unavailable";
    readonly target?: string | null;
    readonly reason: string;
    readonly token?: string | null;
    readonly surfaceId: string;
    readonly scopeKind?: "modal" | null;
    readonly mode?: "preview" | "capture" | "crop" | "annotate" | null;
    readonly operation?: "generic" | "open" | "replace" | "annotate" | "close";
  }): FormMediaPresentationArtifact;
  clearMedia(options?: { readonly reason?: string }): FormMediaPresentationArtifact;
  reportMessages(update: FormScopedMessagePresentationUpdate): FormMessagePresentationArtifact;
  clearMessages(options?: { readonly reason?: string }): FormMessagePresentationArtifact;
  reportCollaboration(update: {
    readonly posture?: "active" | "blocked" | "settling" | "unavailable";
    readonly reason?: string;
    readonly lockOwnerId?: string | null;
    readonly leasedFields?: ReadonlyArray<{ readonly field: string; readonly ownerId: string }>;
    readonly branchId?: string | null;
    readonly readOnly?: boolean;
    readonly remoteUpdateDigest?: string | null;
    readonly presence?: ReadonlyArray<{ readonly actorId: string; readonly status: "active" | "idle" | "viewing" }>;
    readonly comments?: ReadonlyArray<{ readonly id: string; readonly authorId: string; readonly target?: string | null }>;
  }): FormCollaborationArtifact;
  clearCollaboration(options?: { readonly reason?: string }): FormCollaborationArtifact;
  acknowledgePresentation(laneId: string): FormPresentationSettlementArtifact;
  timeoutPresentation(laneId: string, options?: { readonly reason?: string }): FormPresentationSettlementArtifact;
  presentationHistory(): ReadonlyArray<FormPresentationHistoryArtifact>;
}
