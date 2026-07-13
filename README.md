# Suzuri 0.1.2

A CLI-based Japanese dictionary using the Jisho API

If in your usage you stumble across any inaccurate definitions, poor search results, or otherwise strange behavior, please submit an issue, so that I can manually override it. As useful as Jisho's API is, many searches return very strange results due to minor inconsistencies.

Suzuri is intended to give Japanese learners quick access to definitions, especially for Anki decks. It is still in the very early stages of development, and will need time before it is fully functional.

## Installation

### Cargo
Via crates:
```bash
cargo install suzuri-cli
```
Or from GitHub directly:
```bash
cargo install --git https://github.com/nishikikoi-0/suzuri
```

### From source
```bash
git clone https://github.com/nishikikoi-0/suzuri.git
cd suzuri
cargo install --path .
```

## Planned implementation

- Kanji diagrams sourced from https://kanjivg.tagaini.net
- Ankiconnect compatibility, card creator with autofill + edit capabilities
