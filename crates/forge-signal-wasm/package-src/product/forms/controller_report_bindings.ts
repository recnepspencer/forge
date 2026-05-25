import { readAccessibilityReport } from "./accessibility/artifacts.js";
import { readAttachmentPresentationReport } from "./attachments/report.js";
import { readCollaborationReport } from "./collaboration/artifacts.js";
import { readExitPresentationReport, deriveExitPresentationBasis } from "./exit/report.js";
import { readHandoffReport } from "./handoff/report.js";
import { readInputCapabilitiesReport } from "./input_capabilities/report.js";
import { readInteractionReport } from "./interaction/report.js";
import { createFormBoundInput } from "./input_binding.js";
import { readLayoutReport } from "./layout/artifacts.js";
import { readMediaPresentationReport } from "./media/report.js";
import { readMessagePresentationReport } from "./messages/report.js";
import { readNavigationReport } from "./navigation/report.js";
import { readControlAvailability, readControlAvailabilities } from "./availability/control_reads.js";
import { applyReportedRouteAuthority } from "./route_authority/apply.js";
import { resolveRouteAuthorityBinding } from "./route_authority/binding.js";
import { readRouteAuthorityReport } from "./route_authority/report.js";
import { readHostReport } from "./host/artifacts.js";

export function createFormReportBindings({
  formRef,
  fieldDeclarations,
  requireRouteFormsAuthorityArtifact,
  hostBindings,
  syncSourceCompatibility,
  authoritativeSource,
  exits,
  handoffs,
  routeAuthority,
  writeDraft,
  recordDraftWrite,
  attachments,
  media,
  messages,
  collaborationDeclaration,
  collaborations,
  interactions,
  navigation,
  layoutMeasurements,
}) {
  return Object.freeze({
    host() {
      return readHostReport(hostBindings);
    },
    inputCapabilities() {
      return readInputCapabilitiesReport(fieldDeclarations);
    },
    inputCapability(fieldId) {
      return formRef().inputCapabilities().fields.find((entry) => entry.field === fieldId) ?? null;
    },
    bindInput(fieldId, options = {}) {
      return createFormBoundInput(formRef().field(fieldId), options);
    },
    exit() {
      return readExitPresentationReport(exits, deriveExitPresentationBasis(formRef()));
    },
    handoff() {
      return readHandoffReport(handoffs);
    },
    routeAuthority() {
      return readRouteAuthorityReport(routeAuthority);
    },
    reportRouteAuthority(authority) {
      return applyReportedRouteAuthority(
        requireRouteFormsAuthorityArtifact(authority),
        formRef(),
        routeAuthority,
        writeDraft,
        recordDraftWrite,
      );
    },
    bindRouteAuthority(routeOrAuthority) {
      const binding = resolveRouteAuthorityBinding(routeOrAuthority);
      if (binding.kind === "missing") {
        throw new TypeError(binding.reason);
      }
      return applyReportedRouteAuthority(
        requireRouteFormsAuthorityArtifact(binding.authority),
        formRef(),
        routeAuthority,
        writeDraft,
        recordDraftWrite,
      );
    },
    clearRouteAuthority(options = {}) {
      return routeAuthority.clear(options.reason ?? null);
    },
    controlAvailabilities() {
      return readControlAvailabilities(formRef().availability());
    },
    controlAvailability(controlId) {
      return readControlAvailability(formRef().availability(), controlId);
    },
    attachments() {
      return readAttachmentPresentationReport(attachments);
    },
    media() {
      return readMediaPresentationReport(media);
    },
    messages() {
      return readMessagePresentationReport(messages, formRef().visibleMessages());
    },
    collaboration() {
      return readCollaborationReport(
        collaborationDeclaration,
        collaborations,
        formRef().resourceSource(),
      );
    },
    interaction() {
      return readInteractionReport(fieldDeclarations, readHostReport(hostBindings), interactions);
    },
    navigation() {
      syncSourceCompatibility(authoritativeSource());
      return readNavigationReport(navigation, formRef().steps().artifacts);
    },
    accessibility() {
      syncSourceCompatibility(authoritativeSource());
      return readAccessibilityReport(fieldDeclarations, formRef());
    },
    layout() {
      syncSourceCompatibility(authoritativeSource());
      return readLayoutReport(fieldDeclarations, formRef());
    },
    layoutField(fieldId) {
      syncSourceCompatibility(authoritativeSource());
      return formRef().layout().fields.find((entry) => entry.field === fieldId) ?? null;
    },
    layoutMeasurement() {
      syncSourceCompatibility(authoritativeSource());
      return layoutMeasurements.report();
    },
  });
}
