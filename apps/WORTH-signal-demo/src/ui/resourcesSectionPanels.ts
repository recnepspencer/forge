import {
  type PanelEvent,
  type PoLine,
  type ScenarioPhase,
  type ServerTruth,
} from "./resourcesSectionSupport";

export interface PanelAdmissionOptions {
  readonly dependsOnLineId?: string;
}

export interface PanelController {
  addLine(line: PoLine, options?: PanelAdmissionOptions): void | Promise<void>;
  settle(lineId: string, accepted: boolean): Promise<void>;
  reset(): void | Promise<void>;
}

export interface PanelProps {
  highlightId: string | null;
  phase: ScenarioPhase;
  serverTruth: ServerTruth;
  onController: (controller: PanelController | null) => void;
}

export function pushPanelEvent(
  setter: (value: PanelEvent[] | ((current: PanelEvent[]) => PanelEvent[])) => void,
  event: PanelEvent,
): void {
  setter((current) => [event, ...current].slice(0, 4));
}
