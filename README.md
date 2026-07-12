# Suzuri 0.1.1

A CLI-based Japanese dictionary using the Jisho API

Suzuri is intended to give Japanese learners quick access to definitions, especially for Anki decks. More features are currently planned

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

- Overrides for problematic entries (i.e. "suzuri dog" returns トックリ形 before 犬, "suzuri the" should return an explanation of definiteness in Japanese rather than simply return no results)
- Kanji diagrams sourced from https://kanjivg.tagaini.net
- Ankiconnect compatibility, card creator with autofill + edit capabilities
