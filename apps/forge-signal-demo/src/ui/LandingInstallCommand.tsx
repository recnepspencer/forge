import React from "react";
import "./landingInstallCommand.css";

export function LandingInstallCommand() {
  const [copied, setCopied] = React.useState(false);
  const command = "npm install forge-signal-wasm";

  return (
    <div className="xai-install-command" aria-label="Install Forge Signal WASM">
      <code>{command}</code>
      <button
        onClick={() => {
          void navigator.clipboard.writeText(command);
          setCopied(true);
          setTimeout(() => setCopied(false), 1600);
        }}
        type="button"
      >
        {copied ? "Copied" : "Copy"}
      </button>
    </div>
  );
}
