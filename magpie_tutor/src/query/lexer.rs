//! Implementation of the Query Lexer
//!
//! The lexer is a simple lexer using regex to separate into large chunk to then be use to split
//! off into smaller token that the parser can use later.
//!
//! You can check the the regex is [`QUERY_REGEX`]

use crate::QUERY_REGEX;

#[derive(Debug, PartialEq)]
/// Enum containing variant that can be emitted by the lexer. For each tokens uses look at
/// [`Filters`][magpie_engine::query::Filters]
pub enum Token {
    /// End of input token. Named EOF for convention.
    Eof,

    /// Token for an opening parenthesis `(` use for grouping.
    OpenParen,
    /// Token for a closeing parenthesis `)` use for grouping.
    CloseParen,

    /// Token for a string maining used in keyword that require a string.
    Str(String),
    /// Token for a number maining used in keyword that require a number. Can be negative.
    Num(isize),

    /// Token for the name keyword.
    /// Alias: `name`, `n`
    Name,
    /// Token for the desciption keyword.
    /// Alias: `description`, `d`
    Desc,

    /// Token for the rarity keyword.
    /// Alias: `rarity`, `r`
    Rarity,
    /// Token for the temple keyword.
    /// Alias: `temple`, `tp`
    Temple,
    /// Token for the tribe keyword.
    /// Alias: `tribe`, `tb`
    Tribe,

    /// Token for the attack keyword.
    /// Alias: `attack`, `a`
    Attack,
    /// Token for the health keyword.
    /// Alias: `health`, `h`
    Health,

    /// Token for the sigil keyword.
    /// Alias: `sigil`, `s`
    Sigil,
    /// Token for the special attack keyword.
    /// Alias: `spatk`, `sp`
    SpAtk,

    /// Token for the special attack keyword.
    /// Alias: `cost`, `c`
    Costs,
    /// Token for the cost type keyword.
    /// Alias: `costtype`, `ct`
    CostType,

    /// Token for the trait keyword.
    /// Alias: `trait`, `tr`
    Trait,

    /// Token for the or operator. Usually require grouping.
    Or,
    /// Token for the not operator.
    Not,

    /// Token for the colon use to seprate between keyword and argument.
    Colon,

    /// Token for the equal sign use to separate keyword and argument.
    Equal,
    /// Token for the greater than sign use to separate keyword and argument.
    Greater,
    /// Token for the greater than ad equal sign use to separate keyword and argument.
    GreaterEq,
    /// Token for the lesser than sign use to separate keyword and argument.
    Less,
    /// Token for the lesser than and equal sign use to separate keyword and argument.
    LessEq,
}

/// Tokenize a given query. Fail on unrecognized token.
pub fn tokenize_query(query: &str) -> Result<Vec<Token>, String> {
    let mut tokens = vec![];
    for tk in QUERY_REGEX.captures_iter(query).map(|c| {
        (
            c.get(1).map(|m| m.as_str()), // string: ".+"
            c.get(2).map(|m| m.as_str()), // singular word: [-\w]+
            c.get(3).map(|m| m.as_str()), // symbol matches: [^\s\w"-]*
        )
    }) {
        tokens.push(match tk {
            // Simple string macthes
            (Some(str), ..) => Token::Str(str.to_owned()),
            // Single word matches. To reduce complexicity these are also responsible for number
            // matching so we try to convert to number first before sending out a string token
            (_, Some(sing), ..) => match sing {
                str if matches!(tokens.last().unwrap_or(&Token::Equal), Token::Colon) => str
                    .parse()
                    .map(Token::Num)
                    .unwrap_or(Token::Str(str.to_owned())),

                "name" | "n" => Token::Name,
                "description" | "d" => Token::Desc,
                "rarity" | "r" => Token::Rarity,
                "temple" | "tp" => Token::Temple,
                "tribe" | "tb" => Token::Tribe,
                "attack" | "a" => Token::Attack,
                "health" | "h" => Token::Health,
                "sigil" | "s" => Token::Sigil,
                "spatk" | "sp" => Token::SpAtk,
                "cost" | "c" => Token::Costs,
                "costtype" | "ct" => Token::CostType,
                "trait" | "tr" => Token::Trait,

                "or" => Token::Or,

                str => str
                    .parse()
                    .map(Token::Num)
                    .unwrap_or(Token::Str(str.to_owned())),
            },
            // Other symbol token, if they are not multi simple we try to separate them into simple
            // token and parse them.
            //
            // TODO: FIX THIS, BECAUSE IT GET CAUGHT ON "(<=" AND PRODUCE 3 TOKENS INSTEAD OF 2.
            (.., Some(sym)) => {
                tokens.extend(match_sym(sym)?);
                continue;
            }

            _ => unreachable!(),
        });
    }

    tokens.push(Token::Eof);

    Ok(tokens)
}

fn match_sym(sym: &str) -> Result<Vec<Token>, String> {
    Ok(vec![match sym {
        "(" => Token::OpenParen,
        ")" => Token::CloseParen,

        "!" => Token::Not,

        ":" => Token::Colon,
        "=" => Token::Equal,
        ">" => Token::Greater,
        "<" => Token::Less,

        ">=" => Token::GreaterEq,
        "<=" => Token::LessEq,

        sym if sym.len() > 1 => {
            let mut vec = vec![];
            for s in sym.chars() {
                vec.push(match_sym(&s.to_string())?.into_iter().next().unwrap());
            }
            return Ok(vec);
        }

        tk => return Err(format!("Unrecognized token: {tk}")),
    }])
}
