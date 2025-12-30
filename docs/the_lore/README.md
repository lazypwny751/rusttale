# Rusttale Lore Book

This directory contains the official lore and documentation for the **Rusttale** universe, formatted as a professional book using LaTeX.

## Structure

The lore is maintained in multiple languages, with a shared theme:

*   **en/**: English (Primary Source)
*   **tr/**: Turkish
*   **fr/**: French
*   **theme/**: Shared LaTeX style definitions (`general.sty`)

## Requirements

To build the PDFs, you need a standard LaTeX distribution (TeX Live is recommended).

### Debian/Ubuntu
```bash
sudo apt install texlive-latex-base texlive-latex-extra texlive-lang-european texlive-fonts-recommended
```

## Build Instructions

A `Makefile` is included to handle the build process automatically.

### Build All Versions
Generates PDFs for all supported languages.
```bash
make
```

### Build Specific Language
You can build a specific language version individually:
```bash
make en   # Generates rusttale-en.pdf
make tr   # Generates rusttale-tr.pdf
make fr   # Generates rusttale-fr.pdf
```

### Clean Workspace
Removes all temporary build artifacts (`.aux`, `.log`, `.toc`, etc.) and the generated PDFs.
```bash
make clean
```

## Output

The generated PDF files will be placed in the root of this directory:
*   `rusttale-en.pdf`
*   `rusttale-tr.pdf`
*   `rusttale-fr.pdf`
