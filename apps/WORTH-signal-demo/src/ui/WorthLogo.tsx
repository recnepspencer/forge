export function WorthLogo() {
  return (
    <svg
      className="worth-logo"
      viewBox="6 7 58 22"
      aria-hidden="true"
      focusable="false"
    >
      <defs>
        <marker
          id="worth-logo-point"
          viewBox="0 0 2.2 2.4"
          refX=".35"
          refY="1.2"
          markerWidth="2.2"
          markerHeight="2.4"
          orient="auto"
        >
          <path d="M0 .35 L2.2 1.2 L0 2.05 Z" fill="#ffce6b" />
        </marker>
        <linearGradient
          id="worth-logo-gold"
          x1="3"
          y1="25"
          x2="29"
          y2="6"
          gradientUnits="userSpaceOnUse"
        >
          <stop offset="0" stopColor="#ff9248" />
          <stop offset="1" stopColor="#ffce6b" />
        </linearGradient>
      </defs>
      <polyline
        className="worth-logo-line"
        markerEnd="url(#worth-logo-point)"
        points="8 20 12 24 16 15 20 23.35 24 10"
      />
      <text className="worth-logo-wordmark" x="25.5" y="24">
        orth
      </text>
    </svg>
  );
}
