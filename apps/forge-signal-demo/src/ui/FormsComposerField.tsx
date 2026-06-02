import React from "react";
import { motion, useReducedMotion } from "motion/react";
import formBeachUpload from "../assets/forms-miami-upload.png";
import "./formsComposerField.css";

type FormSceneStage =
  | "typing"
  | "uploadFocus"
  | "uploadPressed"
  | "imageLoading"
  | "imageAttached"
  | "submitReady"
  | "submitting"
  | "sent";

type StageSpec = {
  duration: number;
  stage: FormSceneStage;
};

const formPrompt = "Forms are first class";

const stageSequence: StageSpec[] = [
  { duration: 3200, stage: "typing" },
  { duration: 900, stage: "uploadFocus" },
  { duration: 700, stage: "uploadPressed" },
  { duration: 1100, stage: "imageLoading" },
  { duration: 1100, stage: "imageAttached" },
  { duration: 1100, stage: "submitReady" },
  { duration: 900, stage: "submitting" },
  { duration: 1200, stage: "sent" },
];

const rowFocus = {
  imageAttached: "image",
  imageLoading: "image",
  sent: "submit",
  submitReady: "submit",
  submitting: "submit",
  typing: "input",
  uploadFocus: "upload",
  uploadPressed: "upload",
} as const satisfies Record<FormSceneStage, "image" | "input" | "submit" | "upload">;

const isUploadPressed = (stage: FormSceneStage) => stage === "uploadPressed";
const isImageLoading = (stage: FormSceneStage) => stage === "imageLoading";
const isImageVisible = (stage: FormSceneStage) =>
  stage === "imageAttached" || stage === "submitReady" || stage === "submitting" || stage === "sent";
const isSubmitLoading = (stage: FormSceneStage) => stage === "submitting";
const isSubmitSent = (stage: FormSceneStage) => stage === "sent";

const nextStageIndex = (index: number) => (index + 1) % stageSequence.length;

const typedLengthForProgress = (progress: number) => Math.min(formPrompt.length, Math.floor(progress * (formPrompt.length + 1)));

export const FormsComposerField: React.FC = () => {
  const [stageIndex, setStageIndex] = React.useState(0);
  const [typedLength, setTypedLength] = React.useState(0);
  const [trackOffset, setTrackOffset] = React.useState(0);
  const prefersReducedMotion = useReducedMotion();
  const viewportRef = React.useRef<HTMLDivElement>(null);
  const inputRef = React.useRef<HTMLDivElement>(null);
  const uploadRef = React.useRef<HTMLButtonElement>(null);
  const imageRef = React.useRef<HTMLDivElement>(null);
  const submitRef = React.useRef<HTMLButtonElement>(null);
  const stage = stageSequence[stageIndex].stage;

  React.useEffect(() => {
    const timeout = window.setTimeout(() => setStageIndex(nextStageIndex), stageSequence[stageIndex].duration);
    return () => window.clearTimeout(timeout);
  }, [stageIndex]);

  React.useEffect(() => {
    if (stage !== "typing") {
      setTypedLength(formPrompt.length);
      return;
    }

    const startedAt = performance.now();
    setTypedLength(0);

    const interval = window.setInterval(() => {
      const elapsed = performance.now() - startedAt;
      setTypedLength(typedLengthForProgress(elapsed / 2300));
    }, 55);

    return () => window.clearInterval(interval);
  }, [stage]);

  React.useLayoutEffect(() => {
    const viewport = viewportRef.current;
    if (!viewport) return;

    const rowRefs = {
      image: imageRef,
      input: inputRef,
      submit: submitRef,
      upload: uploadRef,
    };

    const measureTargetOffset = () => {
      const target = rowRefs[rowFocus[stage]].current;
      if (!target) return;

      const rowCenter = target.offsetTop + target.offsetHeight / 2;
      const viewportAnchor = rowFocus[stage] === "submit" ? viewport.clientHeight * 0.68 : viewport.clientHeight * 0.5;
      setTrackOffset(viewportAnchor - rowCenter);
    };

    measureTargetOffset();

    const observer = new ResizeObserver(measureTargetOffset);
    observer.observe(viewport);
    Object.values(rowRefs).forEach((ref) => {
      if (ref.current) observer.observe(ref.current);
    });

    return () => observer.disconnect();
  }, [stage, typedLength]);

  const typedText = stage === "typing" ? formPrompt.slice(0, typedLength) : formPrompt;
  const showCaret = stage === "typing" && typedLength < formPrompt.length;

  return (
    <div className="xai-form-cinema" aria-hidden="true">
      <div className="xai-form-flow-viewport" ref={viewportRef}>
        <motion.div
          animate={{ y: prefersReducedMotion ? 0 : trackOffset }}
          className="xai-form-flow-track"
          transition={{ duration: prefersReducedMotion ? 0 : 0.72, ease: [0.19, 1, 0.22, 1] }}
        >
          <div className="xai-form-input-row" data-active={rowFocus[stage] === "input"} ref={inputRef}>
            <div className="xai-form-input-shell">
              <span className="xai-form-placeholder" data-hidden={typedText.length > 0}>
                Ask the form to do the work
              </span>
              <span className="xai-form-message">
                {typedText}
                <span className="xai-form-caret" data-visible={showCaret} />
              </span>
            </div>
          </div>

          <motion.button
            animate={{
              background: isUploadPressed(stage)
                ? "linear-gradient(135deg, color-mix(in srgb, var(--section-accent) 68%, #fff1bf), color-mix(in srgb, var(--section-accent) 42%, #5b310e))"
                : "rgba(12, 14, 20, 0.96)",
              scale: isUploadPressed(stage) ? 0.985 : 1,
              y: isUploadPressed(stage) ? 2 : 0,
            }}
            className="xai-form-upload-button"
            ref={uploadRef}
            tabIndex={-1}
            transition={{ duration: 0.22, ease: [0.19, 1, 0.22, 1] }}
            type="button"
          >
            <span className="xai-form-image-picker-icon">
              <span className="xai-form-image-sun" />
              <span className="xai-form-image-mountain" />
              <span className="xai-form-image-plus" />
            </span>
            <span>Add image</span>
          </motion.button>

          <div className="xai-form-attachment" ref={imageRef}>
            <motion.span
              animate={{ opacity: isImageLoading(stage) ? 1 : 0, rotate: isImageLoading(stage) ? 540 : 0 }}
              className="xai-form-image-loading"
              transition={{ duration: isImageLoading(stage) ? 0.9 : 0.18, ease: "linear" }}
            />
            <motion.img
              animate={{ opacity: isImageVisible(stage) ? 1 : 0, scale: isImageVisible(stage) ? 1 : 0.94 }}
              alt=""
              src={formBeachUpload}
              transition={{ duration: 0.45, ease: [0.19, 1, 0.22, 1] }}
            />
          </div>

          <button className="xai-form-submit" ref={submitRef} tabIndex={-1} type="button">
            <span className="xai-form-submit-label" data-hidden={isSubmitLoading(stage) || isSubmitSent(stage)}>
              Submit
            </span>
            <span className="xai-form-spinner" data-visible={isSubmitLoading(stage)} />
            <span className="xai-form-sent" data-visible={isSubmitSent(stage)} />
          </button>
        </motion.div>
      </div>
    </div>
  );
};
