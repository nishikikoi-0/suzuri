
[08 Jul 2026] v0.1.1

Added

    - Support for multiple queries in one usage of suzuri command
    - --common flag to filter results only to words marked common by Jisho
    - "No search results" message as opposed to crash
    - URL encoding of strings to allow for phrasal queries

Changed

    - Split project into proper modules
    - Introduced JsonParsed wrapper struct as opposed to standalone functions accepting &Vec<Data>

Fixed

    - Handle empty senses/japanese fields or index[0] via is_empty() call
    

[07 Jul 2026] v0.1.0

Created suzuri

Initial functionality included:
    - Basic query through Jisho API
    - --verbose flag to display all results
