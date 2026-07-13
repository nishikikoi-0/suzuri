
# [12 Jul 2026] v0.1.2

Added to crates.io under suzuri-cli

### Added

    - SQLite-backed override system allowing for unoptimal query results to be filtered/replaced. DB_VERSION is used to ensure a full rebuild of the database occurs only when new entries are added.
    - --db-version flag to check local vs source database version and prompt for updates
    - --update-db flag to manually update or generate override database
    - --no-override flag to bypass the override database entirely
    - Colored terminal output (inspired by https://github.com/brunnerh/jisho-cli)

### Changed

    - is_common and parts_of_speech now Option<bool> instead of required fields to prevent api inconsistency-related crashes
    - Grouped consecutive senses under a single part-of-speech header instead of repeating it (Similarly inspired by https://github.com/brunnerh/jisho-cli)

### Fixed

    - args.version now maps to a real cli field rather than clap's built in flag. Cargo.toml's package name field needed to be switched to suzuri-cli to allow it to be published to crates.io, and clap's --version flag reflected this.


# [08 Jul 2026] v0.1.1

### Added

    - Support for multiple queries in one usage of suzuri command
    - --common flag to filter results only to words marked common by Jisho
    - "No search results" message as opposed to crash
    - URL encoding of strings to allow for phrasal queries

### Changed

    - Split project into proper modules
    - Introduced JsonParsed wrapper struct as opposed to standalone functions accepting &Vec<Data>

### Fixed

    - Handle empty senses/japanese fields or index[0] via is_empty() call
    

# [07 Jul 2026] v0.1.0

Created suzuri

### Initial functionality included:

    - Basic query through Jisho API
    - --verbose flag to display all results
