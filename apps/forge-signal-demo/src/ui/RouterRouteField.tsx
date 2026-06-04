import React from "react";
import { motion, useReducedMotion } from "motion/react";
import "./routerRouteField.css";

type RouterStage = "loadingLogin" | "login" | "loadingDenied" | "denied" | "loadingBalance" | "balance" | "loadingCrumbs" | "crumbs";

type RouterStageSpec = {
  duration: number;
  stage: RouterStage;
};

const routerStages: RouterStageSpec[] = [
  { duration: 850, stage: "loadingLogin" },
  { duration: 1350, stage: "login" },
  { duration: 850, stage: "loadingDenied" },
  { duration: 1500, stage: "denied" },
  { duration: 850, stage: "loadingBalance" },
  { duration: 1450, stage: "balance" },
  { duration: 850, stage: "loadingCrumbs" },
  { duration: 1700, stage: "crumbs" },
];

const nextRouterStage = (index: number) => (index + 1) % routerStages.length;
const isLoading = (stage: RouterStage) => stage.startsWith("loading");

const routeLabel = (stage: RouterStage) => {
  if (stage === "login" || stage === "loadingDenied") return "/login";
  if (stage === "denied" || stage === "loadingBalance") return "/admin/vault";
  if (stage === "balance" || stage === "loadingCrumbs") return "/accounts/primary";
  if (stage === "crumbs") return "/accounts/primary/ledger";
  return "/admin/vault";
};

export const RouterRouteField: React.FC = () => {
  const [stageIndex, setStageIndex] = React.useState(0);
  const prefersReducedMotion = useReducedMotion();
  const stage = routerStages[stageIndex].stage;

  React.useEffect(() => {
    const timeout = window.setTimeout(() => setStageIndex(nextRouterStage), routerStages[stageIndex].duration);
    return () => window.clearTimeout(timeout);
  }, [stageIndex]);

  const transition = { duration: prefersReducedMotion ? 0 : 0.45, ease: [0.19, 1, 0.22, 1] } as const;

  return (
    <div className="xai-router-scene" aria-hidden="true">
      <div className="xai-router-chrome">
        <span className="xai-router-dot" />
        <span className="xai-router-dot" />
        <span className="xai-router-dot" />
        <motion.span animate={{ opacity: isLoading(stage) ? 0.72 : 1 }} className="xai-router-path" transition={transition}>
          {routeLabel(stage)}
        </motion.span>
      </div>

      <div className="xai-router-stage">
        <motion.div
          animate={{ opacity: isLoading(stage) ? 1 : 0, scale: isLoading(stage) ? 1 : 0.94, y: isLoading(stage) ? 0 : -8 }}
          className="xai-router-loading"
          transition={transition}
        >
          <span className="xai-router-spinner" />
          <span>Resolving route</span>
        </motion.div>

        <motion.div
          animate={{ opacity: stage === "login" ? 1 : 0, scale: stage === "login" ? 1 : 0.96, y: stage === "login" ? 0 : 12 }}
          className="xai-router-login"
          transition={transition}
        >
          <span>Sign in required</span>
          <strong>Login</strong>
          <small>Session boundary intercepted navigation.</small>
        </motion.div>

        <motion.div
          animate={{ opacity: stage === "denied" ? 1 : 0, scale: stage === "denied" ? 1 : 0.96, y: stage === "denied" ? 0 : 12 }}
          className="xai-router-denied"
          transition={transition}
        >
          <strong>You don't have permission to access this page.</strong>
          <span>Route guard returned denied.</span>
        </motion.div>

        <motion.div
          animate={{ opacity: stage === "balance" ? 1 : 0, scale: stage === "balance" ? 1 : 0.96, y: stage === "balance" ? 0 : 12 }}
          className="xai-router-balance"
          transition={transition}
        >
          <span>Available balance</span>
          <strong>$1,000,000</strong>
        </motion.div>

        <motion.div
          animate={{ opacity: stage === "crumbs" ? 1 : 0, scale: stage === "crumbs" ? 1 : 0.96, y: stage === "crumbs" ? 0 : 12 }}
          className="xai-router-crumbs"
          transition={transition}
        >
          <span>Home</span>
          <i />
          <span>Accounts</span>
          <i />
          <span>Primary</span>
          <i />
          <strong>Ledger</strong>
        </motion.div>
      </div>
    </div>
  );
};
