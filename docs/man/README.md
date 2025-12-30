# Manpages

This directory contains the manual pages for `rusttale`.

## Structure

- `en/`: English manual pages (Default)
- `fr/`: French manual pages
- `tr/`: Turkish manual pages

## Installation

To install all man pages to your system (Default):

```bash
sudo make install
```

To install specific languages (e.g., only Turkish):

```bash
sudo make install LANGS="tr"
```
or using the pattern target directly:
```bash
sudo make install-tr
```

To uninstall:

```bash
sudo make uninstall
```

## Viewing

To view a specific man page without installing:

```bash
man ./en/man1/rusttale.1
```
