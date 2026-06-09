# ScriptureGlide 📖✨

**ScriptureGlide** is a professional, high-performance scripture presentation system designed for churches, study groups, speakers, and personal study. It features a modern, responsive React-based control interface and a robust Tauri/Rust backend designed for zero-latency, multi-display visual projections.

![ScriptureGlide Banner](https://github.com/user-attachments/assets/0aa67016-6eaf-458a-adb2-6e31a0763ed6)

---

## 📖 Table of Contents
1. [How It Works](#-how-it-works)
2. [Key Features](#-key-features)
   - [Operation Modes](#1-operation-modes)
   - [The Library Sidebar](#2-the-library-sidebar-multi-scripture)
   - [Fetching & Custom Content](#3-fetching--custom-content)
   - [Live Markup System](#4-live-markup-system)
   - [Keyboard Shortcuts](#5-keyboard-shortcuts)
   - [Saved Studies (.glide)](#6-saved-studies-glide)
3. [Settings & Customization](#-settings--customization)
4. [Getting Started](#-getting-started)
   - [Prerequisites](#prerequisites)
   - [Installation](#installation)
   - [Development](#development)
   - [Building Installers](#building-installers)

---

## 🛠️ How It Works

ScriptureGlide operates as a **dual-state presentation layout**. It separates the **Operator Dashboard** (where you search, edit, markup, and manage scriptures) from the **Presentation Screen** (which displays distraction-free, beautifully centered scripture to the audience).

1. **Setup:** The operator launches the app, selecting either Single Monitor or Multiple Monitor mode.
2. **Retrieve:** The operator fetches a bible passage using the built-in search or imports custom text.
3. **Markup & Edit:** The operator highlights, underlines, or frames words using the markup toolbar. They can also directly edit the text on the fly.
4. **Project:** As the operator navigates through verses, the active text automatically scrolls and centers in the visual field of the presentation window.

---

## 🌟 Key Features

### 1. Operation Modes
- **Single Monitor Mode:** Classic mode where both controls and the visual presentation live in one unified window. Ideal for personal study, sermon preparation, or screen recording.
- **Multiple Monitor Mode (Dual Screen):** Professional mode. The control dashboard stays on the operator's monitor, while the scripture displays in a borderless, full-screen presentation window on the secondary display (projector or TV).

### 2. The Library Sidebar (Multi-Scripture)
You can load **multiple scriptures in the program at the same time** using the collapsible library sidebar on the left side of the dashboard.
- **Toggle Sidebar:** Click the chevron next to "Back to Welcome Screen" in the top header to collapse or expand the library.
- **Add Scripture:** Click the `+ Add Scripture` button to create a new session slot.
- **Switch Sessions:** Click any item in the list to switch active scriptures instantly. The main viewer and presentation screen will update immediately.
- **Discard Sessions:** Click the close `X` button on any sidebar item hover to remove it.

### 3. Fetching & Custom Content
- **Bible Search:** Enter a passage reference (e.g., `John 3:16`, `Romans 12:1-5`, or `Psalm 23`) and click search.
- **Multiple Translations:** Select from the dropdown. Supported sources include:
  - **Online APIs:** ESV (requires key), NET Bible, KJV, WEB, BBE, Clementine Latin Vulgate, SBL Greek New Testament.
  - **Original Languages:** Hebrew Old Testament (WLC), Greek Septuagint (LXX), SBL Greek New Testament (SBLGNT).
  - **YouVersion Integration:** Access popular versions (NIV, NASB, ESV) via API.
- **Custom Text:** Click the `Custom Text` button to paste quotes, sermon slides, or custom study notes. Each paragraph becomes an interactive segment.

### 4. Live Markup System
Highlight, format, or mark up scripture in real-time. Any changes to the text or formatting in the control window sync instantly with zero latency to the presentation view.
- **Highlighter Tool:** Highlight text selections with a configurable background color.
- **Text Color Tool:** Change text color for emphasis.
- **Underline Tool:** Draw colored underlines under selections.
- **Shape Tools:** Circle or Box specific words or phrases.
- **Eraser Tool:** Remove formatting from the selected text.
- **Clear All:** Wipe all markups and reset the passage back to clean text.
- **Direct Text Editing:** Click directly into any verse on the control screen to correct, annotate, or translate words in real-time.

### 5. Keyboard Shortcuts
Navigate the presentation smoothly without needing to click:
- **Arrow Down:** Move selection to the next active verse group (scrolls presentation to center it).
- **Arrow Up:** Move selection to the previous active verse group.
- **Enter (in Search Box):** Fetch the entered scripture reference.

### 6. Saved Studies (.glide)
You can save and load your prepared passages, including all live markups, active verse positions, and translation choices.
- **Save Study:** Click the Save icon. If the session was loaded from a file, it will write directly back. If not, it will prompt you for a file location.
- **Save Study As:** Choose to save the active study as a new file.
- **Load Study:** Open `.glide` files directly. Opening a `.glide` file appends it as a **new session in the library sidebar** so you can keep multiple pre-saved studies open at once.

---

## ⚙️ Settings & Customization

Click the **Settings Cog** in the top right to customize:
- **Typography:** Choose between Academic Serif, Modern Sans, Bold Display, or Study Monospace.
- **Text Sizing & Line Spacing:** Adjust the presentation layout for maximum readability.
- **Scroll Speed:** Control the transition speed when centering new verses.
- **Active Verse Count:** Display between 1 and 10 active verses highlighted at a time.
- **Highlight Intensity:** Set transparency levels for markup highlights.
- **Colors:** Adjust background color, theme (Dark/Light), reference box color, and verse number color.
- **Target Display:** Select which monitor projects the presentation.

---

## 🚀 Getting Started

### Prerequisites
- [Node.js](https://nodejs.org/) (Latest LTS)
- [Rust & Cargo](https://www.rust-lang.org/) (For Tauri compilation)

### Installation

1. **Clone the repository:**
   ```bash
   git clone https://github.com/gregvruggink/ScriptureGlide.git
   cd scriptureglide
   ```

2. **Install dependencies:**
   ```bash
   npm install
   ```

3. **Optional ESV API Key:**
   To use the ESV translation, register for an API key at [api.esv.org](https://api.esv.org/) and enter it in the settings dashboard of the app.

---

## 🛠️ Development

Run the application in development mode:
```bash
npm run tauri:dev
```
This launches the backend dev server and opens the hot-reloaded desktop application.

---

## 🏗️ Building Installers

To compile production-ready installers for your current platform:
```bash
npm run tauri:build
```
Compiled setups can be found in `src-tauri/target/release/bundle/`.

---

*Developed with ❤️ for the global church.*
