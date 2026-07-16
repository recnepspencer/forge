import {
  type Agreement,
  type PanelEvent,
  type PanelVariant,
  type PoLine,
  type ScenarioPhase,
  type ServerTruth,
} from "./resourcesSectionSupport";

export interface PanelAdmissionOptions {
  readonly dependsOnLineId?: string;
}

export type AgreementEvidence = "live" | "refetchCompleted";

export interface PanelController {
  addLine(line: PoLine, options?: PanelAdmissionOptions): void | Promise<void>;
  settle(lineId: string, accepted: boolean): void;
  reset(): void | Promise<void>;
}

export interface PanelProps {
  baseMs: number | null;
  highlightId: string | null;
  phase: ScenarioPhase;
  serverTruth: ServerTruth;
  onAgreement: (
    variant: PanelVariant,
    agreement: Agreement | null,
    evidence: AgreementEvidence,
  ) => void;
  onController: (controller: PanelController | null) => void;
}

export function pushPanelEvent(
  setter: (value: PanelEvent[] | ((current: PanelEvent[]) => PanelEvent[])) => void,
  event: PanelEvent,
): void {
  setter((current) => [event, ...current].slice(0, 4));
}
