# Sharp System: UI/UX Guide ◈ ✨

This guide defines the design principles, aesthetics, and interaction patterns that make Sharp System feel like a premium, high-performance terminal.

## ◈ The "Sharp Terminal" Aesthetic
The UI is built on a "Glass-Terminal" metaphor: high-fidelity, semi-transparent layers floating over a deep, obsidian background.

### 1. Glassmorphism (The Foundation)
Every primary container utilizes "Glass-Card-Premium" styling:
- **Background**: Semi-transparent dark surfaces (`rgba(20, 20, 25, 0.7)`).
- **Blur**: `backdrop-filter: blur(20px)` for depth.
- **Borders**: Razor-thin 1px borders (`rgba(255, 255, 255, 0.05)`) with subtle top-lights.

### 2. Color Palette
- **Obsidian (Base)**: `#0a0a0c`
- **Vibrant Violet (Primary)**: `#8b5cf6` (used for calls-to-action and highlights).
- **Electric Cyan (Accent)**: `#06b6d4` (used for system status and health).
- **Muted Steel (Secondary)**: `#94a3b8` (used for labels and metadata).

---

## ◈ Animation & Interactivity
Motion is not just decoration in Sharp System; it provides feedback and reinforces the "high-speed" promise.

### 1. Staggered Entry (`scaleInUp`)
Used in the Catalog and Dashboards to make data "pop" into existence.
- Each card enters with a slight delay (`0.04s` per item).
- **Cubic Bezier**: `cubic-bezier(0.2, 0.8, 0.2, 1)`.

### 2. Micro-Interactions
- **Hover Radiance**: Cards emit a subtle 10% saturation glow and 2px lift when hovered.
- **Button Pulsing**: Primary actions have a slow, breathing pulse on the border to guide the user.

---

## ◈ Component Usage
- **Glass-Card**: Main content containers.
- **Stat-Mini-Card**: High-level numerical insights (Total Count, Inventory Value).
- **Input-Field-Premium**: Floating label inputs with focus-glow borders.

## ◈ Typography
Focused on high readability and a "technical" feel:
- **Headings**: Inter or Outfit (Bold, tracking -0.02em).
- **Metadata/Numbers**: JetBrains Mono or Fira Code (Semibold).
