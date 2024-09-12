use crate::util::*;

/// Represents states in the state machine,
/// Some states come with a payload, this is because they require multiple characters
/// and the state machine must keep track what the previous characters were in order
/// to construct the relevant token
#[derive(Debug)]
enum State {
    /// Represents the start state
    Start,

    /// Represents the end state
    End,

    /// Represents an in progress punctuation token such as `==`
    Punct(String),

    /// Represents an in progress word such as `while`
    Word(String),

    /// Represents the start of a character literal
    CharStart,
    /// Represents the end of a character literal
    /// with the consumed character ready to construct a token
    CharEnd(char),

    /// Represents an in progress string literall such as `"hello"`
    InStr(String),

    /// Represents when building an escape character within a string literal
    /// such as `\n` for new line
    InEscape(String),

    /// Represents the `0` integer
    Zero,

    /// Represents an in progress binary integer literal such as `0b0110`
    InBin(u8, u64),
    /// Represents an in progress octal integer literal such as `0763`
    InOct(u8, u64),
    /// Represents an in progress decimal integer literal such as `1000`
    InDec(u8, u64),
    /// Represents an in progress hexadecimal integer literal such as `0xff`
    InHex(u8, u64),
    /// Represents an in progress size extension on the end of an integer literal
    /// such as `10u32`
    InBits(u8, u64, bool, u8),

    /// Machine is in a line comment such as `// test`
    Comment,

    /// Machine is in a comment block such as `/* test */`
    CommentBlock,
    /// Machine has hit an `*` within a comment block such as `/* * */`
    CommentInStar,
}

/// Macro to help with pattern matching for the next token
macro_rules! next {
    ($token_line:expr,$token_col:expr,$expression:expr, $pattern:pat => $out:expr) => {{
        if let Some('\n') = $expression.peek() {
            $token_line += 1;
            $token_col = 1;
        } else {
            $token_col += 1;
        }

        if let Some($pattern) = $expression.next() {
            $out
        } else {
            panic!()
        }
    }};
}

/// Takes an input string (the contents of the source file) and returns a list
/// of tokens along with their location in the source file.
pub fn lex(src: &str) -> Vec<(TokenLoc, Token)> {
    // The current state of the state machine
    let mut state = State::Start;

    // The output tokens
    let mut ret = Vec::new();

    // a peekable iterator over the source file, allowing the lexer to look
    // ahead one character and make decisions based on that.
    let mut chars = src.chars().peekable();

    // the current line and column for constructing the token location
    // descriptions
    let mut line = 1;
    let mut col = 1;

    while !matches!(state, State::End) {
        state = if let Some(c) = chars.peek() {
            match state {
                // Handle the starting state. depending on the next tken
                // we either switch to the `CharStart`, `InStr`, `InDec`,
                // `Zero`, `Punct` or `Word`. If the character is whitespace
                // we stay at the start state.
                State::Start => match c {
                    '\'' => next!(line, col, chars, _ => State::CharStart),
                    '"' => next!(line, col, chars, _ => State::InStr(String::new())),
                    '1'..='9' => {
                        next!(line, col, chars, c => State::InDec(1, c.to_digit(10).unwrap() as u64))
                    }
                    '0' => next!(line, col, chars, _ => State::Zero),
                    _ if c.is_ascii_whitespace() => next!(line, col, chars, _ => state),
                    _ if c.is_ascii_punctuation() && *c != '_' => {
                        let mut s = String::new();
                        next!(line, col, chars, c => s.push(c));
                        State::Punct(s)
                    }
                    _ => {
                        let mut s = String::new();
                        next!(line, col, chars, c => s.push(c));
                        State::Word(s)
                    }
                },

                // Handles building a punctuation token, most punctuation
                // tokens are only a single character such as `+` however
                // some such as `==` are two characters; here we handle that.
                //
                // The `/` character may look like a divide token but could
                // be the start of a comment so this is also handled.
                //
                // In all other cases we push a punctuation token to the
                // output token stream.
                State::Punct(p_v) => match (p_v.as_str(), c) {
                    ("=", '=') | ("-", '>') => {
                        let c = *c;
                        next!(line, col, chars, _ => ());
                        ret.push((
                            TokenLoc {
                                line,
                                col: col - 2,
                                len: 2,
                            },
                            Token::Punct(format!("{}{}", p_v, c)),
                        ));
                        State::Start
                    }
                    ("/", '/') => {
                        next!(line, col, chars, _ => ());
                        State::Comment
                    }
                    ("/", '*') => {
                        next!(line, col, chars, _ => ());
                        State::CommentBlock
                    }
                    _ => {
                        ret.push((
                            TokenLoc {
                                line,
                                col: col - 1,
                                len: 1,
                            },
                            Token::Punct(p_v),
                        ));
                        State::Start
                    }
                },

                // Handles word tokens such as keywords and identifiers.
                // To simplify lexical analysis and parsing they are are not
                // distinguished until the parser.
                //
                // If we hit whitespace we move back to the `Start` state
                // and push a new `Word` token to the output token stream.
                //
                // We do the same for punctuation unless the punctuation
                // is an `_` in which case it is considered part of the word.
                State::Word(p_v) => match c {
                    _ if c.is_ascii_whitespace() => {
                        ret.push((
                            TokenLoc {
                                line,
                                col: col - p_v.len(),
                                len: p_v.len(),
                            },
                            Token::Word(p_v),
                        ));
                        State::Start
                    }
                    _ if c.is_ascii_punctuation() && *c != '_' => {
                        ret.push((
                            TokenLoc {
                                line,
                                col: col - p_v.len(),
                                len: p_v.len(),
                            },
                            Token::Word(p_v),
                        ));
                        State::Start
                    }

                    _ => {
                        let mut s = p_v;
                        next!(line, col, chars, c => s.push(c));
                        State::Word(s)
                    }
                },

                // Handles any token starting with a `0`, this could be various different
                // types of integers or floats.
                //
                // if the `0` is followed by any other number the token is an integer encoded
                // in octal and thus we move to the `InOct` state.
                //
                // if the `0` is followed by a `b` or `x`, the token is an integer encoded
                // as binary or decimal respectively. We thus move to the `InBin` or `InHex`
                // states.
                //
                // `u` and `i` denote signed and unsigned integer bit extensions. These tell
                // the compiler how large the interger type is. For example, `0u32` is a 32
                // bit unsigned integer.
                State::Zero => match c {
                    '0'..='7' => {
                        next!(line, col, chars, c => State::InOct(2, c.to_digit(8).unwrap() as u64))
                    }
                    'b' => next!(line, col, chars, _ => State::InBin(2, 0)),
                    'x' => next!(line, col, chars, _ => State::InHex(2, 0)),
                    'u' => next!(line, col, chars, _ => State::InBits(2, 0, false, 0)),
                    'i' => next!(line, col, chars, _ => State::InBits(2, 0, true, 0)),
                    'f' => todo!(),

                    _ => {
                        ret.push((TokenLoc { line, col, len: 1 }, Token::Int(0)));
                        State::Start
                    }
                },

                // Handles binary integer literals
                //
                // handles `u` and `i` bit extensions
                State::InBin(n, v) => match c {
                    '0' | '1' => {
                        next!(line, col, chars, c => State::InBin(n + 1, v << 1 + c.to_digit(2).unwrap() as u64))
                    }
                    'u' => next!(line, col, chars, _ => State::InBits(n + 1, v, false, 0)),
                    'i' => next!(line, col, chars, _ => State::InBits(n + 1, v, true, 0)),
                    _ => {
                        ret.push((
                            TokenLoc {
                                line,
                                col: col - n as usize,
                                len: n as usize,
                            },
                            Token::Int(v),
                        ));
                        State::Start
                    }
                },

                // Handles octal integer literals
                //
                // handles `u` and `i` bit extensions
                State::InOct(n, v) => match c {
                    '0'..='7' => {
                        next!(line, col, chars, c => State::InOct(n + 1, v << 3 + c.to_digit(8).unwrap() as u64))
                    }
                    'u' => next!(line, col, chars, _ => State::InBits(n + 1, v, false, 0)),
                    'i' => next!(line, col, chars, _ => State::InBits(n + 1, v, true, 0)),
                    _ => {
                        ret.push((
                            TokenLoc {
                                line,
                                col: col - n as usize,
                                len: n as usize,
                            },
                            Token::Int(v),
                        ));
                        State::Start
                    }
                },

                // Handles decimal integer literals
                //
                // handles `u` and `i` bit extensions
                //
                // also handles `f` as what looks like a base 10 integer could also be a float.
                State::InDec(n, v) => match c {
                    '0'..='9' => {
                        next!(line, col, chars, c => State::InDec(n + 1, v * 10 + c.to_digit(10).unwrap() as u64))
                    }
                    'u' => next!(line, col, chars, _ => State::InBits(n + 1, v, false, 0)),
                    'i' => next!(line, col, chars, _ => State::InBits(n + 1, v, true, 0)),
                    _ => {
                        ret.push((
                            TokenLoc {
                                line,
                                col: col - n as usize,
                                len: n as usize,
                            },
                            Token::Int(v),
                        ));
                        State::Start
                    }
                },

                // Handles hexadecimal integer literals
                //
                // handles `u` and `i` bit extensions
                State::InHex(n, v) => match c {
                    '0'..='9' | 'a'..='f' | 'A'..='F' => {
                        next!(line, col, chars, c => State::InHex(n + 1, v << 4 + c.to_digit(16).unwrap() as u64))
                    }
                    'u' => next!(line, col, chars, _ => State::InBits(n + 1, v, false, 0)),
                    'i' => next!(line, col, chars, _ => State::InBits(n + 1, v, true, 0)),
                    _ => {
                        ret.push((
                            TokenLoc {
                                line,
                                col: col - n as usize,
                                len: n as usize,
                            },
                            Token::Int(v),
                        ));
                        State::Start
                    }
                },

                // Handles bit extension, weather the integer is signed or unsigned and how many bits are used
                // to represent it
                State::InBits(n, v, s, b) => match c {
                    '0'..='9' => {
                        next!(line, col, chars, c => State::InBits(n + 1, v, s, b * 10 + c.to_digit(10).unwrap() as u8))
                    }
                    _ => {
                        ret.push((
                            TokenLoc {
                                line,
                                col: col - n as usize,
                                len: n as usize,
                            },
                            match (s, b) {
                                (false, 8) => Token::U8(v as u8),
                                (false, 16) => Token::U16(v as u16),
                                (false, 32) => Token::U32(v as u32),
                                (false, 64) => Token::U64(v as u64),
                                (true, 8) => Token::I8(v as i8),
                                (true, 16) => Token::I16(v as i16),
                                (true, 32) => Token::I32(v as i32),
                                (true, 64) => Token::I64(v as i64),
                                _ => panic!(
                                    "unknown int type: signed={}, bits={} with value {}",
                                    s, b, v
                                ),
                            },
                        ));
                        State::Start
                    }
                },

                // Handles the start of a line comment, it is terminated by a new line,
                // upon which we move back to the `Start` state
                State::Comment => match c {
                    '\n' => next!(line, col, chars, _ => State::Start),
                    _ => next!(line, col, chars, _ => state),
                },

                // Handles multi-line comments. if we encounter a `*` character
                // we move into the `CommentInStar` state to decide if to terminate
                // the comment and move back to the `Start` state
                State::CommentBlock => match c {
                    '*' => next!(line, col, chars, _ => State::CommentInStar),
                    _ => next!(line, col, chars, _ => state),
                },

                // Decides if to terminate the comment block depending on the next
                // character
                State::CommentInStar => match c {
                    '/' => next!(line, col, chars, _ => State::Start),
                    '*' => next!(line, col, chars, _ => state),
                    _ => next!(line, col, chars, _ => State::CommentBlock),
                },

                // Handles a character literal.
                State::CharStart => next!(line, col, chars, c => State::CharEnd(c)),

                // Decides if to terminate the literal depending on next character.
                State::CharEnd(a) => match c {
                    '\'' => {
                        next!(line, col, chars, _ => ());
                        ret.push((TokenLoc { line, col, len: 1 }, Token::Char(a)));
                        State::Start
                    }
                    _ => panic!("{}", a),
                },

                // Handles string literal tokens, if we encounter a `\`, we move into
                // the `InEscape` state to handle string escapes such as `\n`.
                //
                // If we encounter a `"` it's the end of the string so move back to the
                // `Start` state and push a new `String` token to the token stream.
                State::InStr(s) => match c {
                    '\\' => next!(line, col, chars, _ => State::InEscape(s)),
                    '"' => {
                        next!(line, col, chars, _ => ());
                        ret.push((
                            TokenLoc {
                                line,
                                col: col - s.len() - 2,
                                len: s.len() + 2,
                            },
                            Token::Str(s),
                        ));
                        State::Start
                    }
                    c => {
                        let mut s = s;
                        s.push(*c);
                        next!(line, col, chars, _ => State::InStr(s))
                    }
                },
                State::InEscape(s) => match c {
                    '\\' => {
                        let mut s = s;
                        s.push('\\');
                        next!(line, col, chars, _ => State::InStr(s))
                    }
                    'n' => {
                        let mut s = s;
                        s.push('\n');
                        next!(line, col, chars, _ => State::InStr(s))
                    }
                    _ => panic!(),
                },
                State::End => unreachable!(),
            }
        } else {
            State::End
        };
    }

    ret
}

#[cfg(test)]
mod test {
    use super::*;
    use std::assert_matches::assert_matches;

    #[test]
    fn test_int() {
        assert_matches!(lex("10")[0].1, Token::Int(10));
    }
}
