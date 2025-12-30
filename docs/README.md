# Rusttale Documentation

This directory serves as the central hub for all documentation related to the **Rusttale** project. It is organized into specific subdirectories based on the format and purpose of the documentation.

## Contents

### 1. Manpages (`man/`)
Standard Unix manual pages for the game's command-line interface and technical usage.
*   **Format:** Roff (Troff)
*   **Languages:** English, French, Turkish
*   **Build:** Managed via Makefile to install to system paths.

### 2. The Lore (`the_lore/`)
The official "Lore Book" and narrative guide for the Rusttale universe.
*   **Format:** LaTeX (compiles to PDF)
*   **Languages:** English, French, Turkish
*   **Content:** World rules, character backstories, and philosophical concepts.

## Future Additions

This structure is designed to be extensible. Future documentation modules may include:
*   **Static Site:** HTML/CSS documentation or a wiki generated from markdown.
*   **API Reference:** Auto-generated code documentation (rustdoc).
*   **Assets Guide:** Specifications for the `resourcetale` repository.
