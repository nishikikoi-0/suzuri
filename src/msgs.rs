pub const VERSION_CMD: &str = concat!("
    Suzuri v", env!("CARGO_PKG_VERSION"));

pub const HELP_CMD: &str = concat!("
    Suzuri v", env!("CARGO_PKG_VERSION"), " - a CLI-based Japanese dictionary using the Jisho API
Made by nishikikoi

Usage:

    suzuri [OPTIONS] <query>

Arguments:

    <input>                 Input accepted in romaji, kana,
                            kanji, and english
    <\"phrasal input\">     Parenthesis allow for phrasal queries
    <multiple> <queries>    Words separated by spaces will be handled separately

Options:

    -v, --verbose <query>   Show all search results, rather
                            than only the most relevant one
    -h, --help              Show this help message and exit
    -c  --common            Limits results exclusively to those
                            marked as common words by jisho.
        --version           Show version information

Examples:

    suzuri bakuzen/ばくぜん/漠然
        All display the kanji, reading, and definition of 漠然。

    suzuri ocean
        Displays information for 海洋 【かいよう】

    suzuri -v disappointment
        Displays information for 失望, 残念, 失意, etc.

    suzuri \"don't care\"
        Displays information for 気にもしない 【きにもしない】

    suzuri north south east west
        Displays information for all of 北, 南, 東, and 西
        ");
