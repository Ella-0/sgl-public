use std::{iter::Map, iter::Peekable, slice::Iter};

use crate::err::ErrHandler;
use crate::util::*;

use crate::err::Result;

/// Matches the next token against a pattern. If it matches, Ok(..) is returned
/// with either none or an expression if the call is in the form `next!(.. => ..)`
/// this consumes that token from the token stream if it matches. It does not if
/// it does not match.
macro_rules! next {
    // Matches `next!(tokens, pattern)` where tokens is the input token stream
    // and pattern is the pattern to match the token against `Token::Word(id)`.
    ($toks:expr, $pattern:pat) => {
        match $toks.peek() {
            Some($pattern) => {
                $toks.next().1;
                Ok(())
            }
            Some((loc, t)) => Err((
                loc,
                format!("Unexpected Token: {} at {}:{} expected {}", t, loc.line, loc.col, stringify!($pattern)),
            )),
            None => Err((TokenLoc::eof(), "Unexpected end of file".into())),
        }
    };

    // Matches `next!(tokens, pattern => out)` where tokens is the input token stream
    // pattern is the pattern to match the token against `Token::Word(id)`. And
    // out is the expression to yeild. For example `next!(tokens, Token::Word(id) => id.clone())`
    // would match the next token against a word token. if it matches it will clone the string from
    // the word token and return it as an `Ok(id)` structure.
    ($toks:expr, $pattern:pat => $out:expr) => {
        match $toks.peek() {
            Some((loc, $pattern)) => match $toks.next().unwrap() {
                (loc, $pattern) => Ok((loc, $out)),
                _ => unreachable!(),
            },
            Some((loc, t)) => Err((
                *loc,
                format!("Unexpected Token: {} at {}:{} expected {}", t, loc.line, loc.col, stringify!($pattern)),
            )),
            None => Err((TokenLoc::eof(), "Unexpected end of file".into())),
        }
    };

    // Matches `next!(tokens, pattern if cond)` where tokens is the input token stream
    // pattern is the pattern to match the token against `Token::Word(id)`. And `cond`
    // is the condition to match against such as `if id == "while"`. For example, the
    // `next!(tokens, Token::Word(id) if id == "while")` would return `Ok(())` whenever
    // the word token is a while key word.
    ($toks:expr, $pattern:pat if $cond:expr) => {
        match $toks.peek() {
            Some((_, $pattern)) if $cond => {
                $toks.next();
                Ok(())
            }
            Some((loc, t)) => Err((
                *loc,
                format!("Unexpected Token: {} at {}:{} expected {}", t, loc.line, loc.col, stringify!($pattern)),
            )),
            None => Err((TokenLoc::eof(), "Unexpected end of file".into())),
        }
    };

    // This combines the last two cases and allows one to yeild a value and match with a condition.
    // for example `next!(tokens, Token::Word(id) if id.starts_with('_') => id)` which would yeild
    // the id from the word token if it starts with an underscore.
    ($toks:expr, $pattern:pat if $cond:expr => $out:expr) => {
        match $toks.peek() {
            Some((_, $pattern)) if $cond => {
                $toks.next();
                Ok($out)
            }
            Some((loc, t)) => Err((
                *loc,
                format!("Unexpected Token: {} at {}:{} expected {}", t, loc.line, loc.col, stringify!($pattern)),
            )),
            None => Err((TokenLoc::eof(), "Unexpected end of file".into())),
        }
    };
}

/// Tests if the next token matches a pattern, returns true if it does
/// returns false otherwise. This will not consume a token from the input streem.
macro_rules! peek {
    ($toks:expr, $pattern:pat) => {
        match $toks.peek().map(|t| t.1.clone()) {
            Some($pattern) => true,
            _ => false,
        }
    };

    ($toks:expr, $pattern:pat if $cond:expr) => {
        match $toks.peek().map(|t| t.1.clone()) {
            Some($pattern) if $cond => true,
            _ => false,
        }
    };
}

/// Tries to construct a `Type` sub tree node from the token stream
/// returns `Err` if failed.
fn parse_ty<I>(toks: &mut Peekable<I>) -> Result<Type>
where
    // Polymorphism: as long as the type `I` implements
    // the iterator trait (an abstract class like concept) we can use it.
    I: Iterator<Item = (TokenLoc, Token)>,
{
    // Match the next token against a series of patterns. and do 
    // different things depending on what it matches with.
    // If there aren't any tokens left return an end of file error.
    match toks
        .next()
        .ok_or((TokenLoc::eof(), "unexpected EOF".into()))?
    {
        // Match against the numeric types and return their corresponding types in
        // the tree. for example `f32` will return `Type::F32` and `u8` will return `Type::U8`
        (_, Token::Word(s)) if s == "i8" => Ok(Type::I8),
        (_, Token::Word(s)) if s == "u8" => Ok(Type::U8),
        (_, Token::Word(s)) if s == "i16" => Ok(Type::I16),
        (_, Token::Word(s)) if s == "u16" => Ok(Type::U16),
        (_, Token::Word(s)) if s == "i32" => Ok(Type::I32),
        (_, Token::Word(s)) if s == "u32" => Ok(Type::U32),
        (_, Token::Word(s)) if s == "i64" => Ok(Type::I64),
        (_, Token::Word(s)) if s == "u64" => Ok(Type::U64),
        (_, Token::Word(s)) if s == "bool" => Ok(Type::Bool),
        (_, Token::Word(s)) if s == "vec2" => Ok(Type::Vec2),
        (_, Token::Word(s)) if s == "vec3" => Ok(Type::Vec3),
        (_, Token::Word(s)) if s == "vec4" => Ok(Type::Vec4),
        (_, Token::Word(s)) if s == "f32" => Ok(Type::F32),

        // If the next token is an `*` we are parsing a pointer type in the form
        // `* <type>` and as such we need to recursively call the `parse_ty` function
	// Then use its result to construct an `Type::Ptr(Box(<whatever we just parsed>))`.
        // Boxes are used to prevent types recursively including eachother which would result
        // in an infinitely sized type. Instead we store an owned (most likely heap allocated)
        // pointer to the data.
        (_, Token::Punct(s)) if s == "*" => Ok(Type::Ptr(Box::new(parse_ty(toks)?))),

        // If the next token is an `(` we are parsing a tuple type in the form
        // `( <type> , <type> , <type> ... )`. To parse this we recursively call parse type
        // all the while we find, when peeking, the next token is not a `)`.
        (_, Token::Punct(s)) if s == "(" => {
            let mut a_types = Vec::new();
            while !next!(toks, Token::Punct(s) if s == ")").is_ok() {
                if !a_types.is_empty() {
                    next!(toks, Token::Punct(s) if s == ",")?;
                }
                let (_, n) = next!(toks, Token::Word(s) => s)?.clone();
                next!(toks, Token::Punct(s) if s == ":" => ":")?;
                let ty = parse_ty(toks)?;
                a_types.push((n, ty));
            }
            let ret_t = if next!(toks, Token::Punct(s) if s == ":").is_ok() {
                parse_ty(toks)?
            } else {
                Type::Void
            };
            Ok(Type::NFunc(Box::new(ret_t), a_types))
        }
        (loc, Token::Word(s)) => Ok(Type::NId(loc, s.to_string())),
        (_, Token::Punct(s)) if s == "{" => {
            let mut tys = vec![];
            while !next!(toks, Token::Punct(s) if s == "}").is_ok() {
                if !tys.is_empty() {
                    next!(toks, Token::Punct(s) if s == ",")?;
                }
                let (_, n) = next!(toks, Token::Word(s) => s)?.clone();
                next!(toks, Token::Punct(s) if s == ":")?;
                let ty = parse_ty(toks)?;
                tys.push((n, ty));
            }
            Ok(Type::NStruct(tys))
        }
        (_, Token::Punct(s)) if s == "[" => {
            next!(toks, Token::Punct(s) if s == "]")?;
            let ty = parse_ty(toks)?;
            Ok(Type::Arr(box ty))
        }
        (loc, t) => Err((
            loc,
            format!("Unexpected Token: {} at {}:{} caught in {}:{}:{}", t, loc.line, loc.col, file!(), line!(), column!()),
        )),
    }
}

fn parse_factor<I>(toks: &mut Peekable<I>) -> Result<Expr>
where
    I: Iterator<Item = (TokenLoc, Token)>,
{
    match next!(toks, s => s)? {
        (_, Token::Punct(s)) if s == "(" => {
            let r = parse_expr(toks)?;
            next!(toks, Token::Punct(s) if s == ")")?;
            Ok(r)
        }
        (_, Token::Punct(s)) if s == "&" => Ok(Expr::Ref(box parse_expr(toks)?)),
        (_, Token::Punct(s)) if s == "*" => Ok(Expr::Deref(box parse_expr(toks)?)),
        (_, Token::Word(s)) if s == "true" || s == "false" => Ok(Expr::Bool(s == "true")),
        (_, Token::Word(s)) if s == "null" => Ok(Expr::Null),
        (_, Token::Word(s)) if s == "shader" => {
            let (ty, mut tlds) = match next!(toks, Token::Word(s) => s)?.1.as_str() {
                // "vert" => (ShaderType::Vert, vec![TLD::NBuiltinVertPos, TLD::NBuiltinVertClipDistance, TLD::NBuiltinVertCullDistance, TLD::NBuiltinVertPointSize]),
                "vert" => (ShaderType::Vert, vec![TLD::NBuiltinVertPos, TLD::NBuiltinVertPointSize]),
                "frag" => (ShaderType::Frag, vec![]),
                s => todo!("unimplemented shader type {}", s),
            };
            next!(toks, Token::Punct(s) if s == "{")?;
            tlds.extend_from_slice(&parse_all(toks)?);
            next!(toks, Token::Punct(s) if s == "}")?;
            Ok(Expr::Shader(ty, tlds))
        }

        (loc, Token::Word(s)) => Ok(Expr::NId(loc, s.to_string())),
        (_, Token::Char(c)) => Ok(Expr::U8(u32::from(c) as u8)),
        (_, Token::Str(c)) => {
            let mut b: Vec<Expr> = c.bytes().map(|x| Expr::U8(x)).collect();
            b.push(Expr::U8(0));
            Ok(Expr::Arr(Type::U8, b))
        }
        /* parse vector literals */
        (_, Token::Punct(s)) if s == "<" => {
            let ty = parse_ty(toks)?;
            next!(toks, Token::Punct(s) if s == ">")?;
            next!(toks, Token::Punct(s) if s == "(")?;
            match ty {
                Type::Vec2 => {
                    let v0 = parse_expr(toks)?;
                    next!(toks, Token::Punct(s) if s == ",")?;
                    let v1 = parse_expr(toks)?;
                    next!(toks, Token::Punct(s) if s == ")")?;
                    Ok(Expr::Vec2(box v0, box v1))
                }
                Type::Vec3 => {
                    let v0 = parse_expr(toks)?;
                    next!(toks, Token::Punct(s) if s == ",")?;
                    let v1 = parse_expr(toks)?;
                    next!(toks, Token::Punct(s) if s == ",")?;
                    let v2 = parse_expr(toks)?;
                    next!(toks, Token::Punct(s) if s == ")")?;
                    Ok(Expr::Vec3(box v0, box v1, box v2))
                }
                Type::Vec4 => {
                    let v0 = parse_expr(toks)?;
                    next!(toks, Token::Punct(s) if s == ",")?;
                    let v1 = parse_expr(toks)?;
                    next!(toks, Token::Punct(s) if s == ",")?;
                    let v2 = parse_expr(toks)?;
                    next!(toks, Token::Punct(s) if s == ",")?;
                    let v3 = parse_expr(toks)?;
                    next!(toks, Token::Punct(s) if s == ")")?;
                    Ok(Expr::Vec4(box v0, box v1, box v2, box v3))
                }
                t => panic!("Expected vector type not {:#?}", t),
            }
        }
        /* parse array literals */
        (_, Token::Punct(s)) if s == "[" => {
            let ty = parse_ty(toks)?;
            next!(toks, Token::Punct(s) if s == "]")?;
            let mut e = vec![];
            next!(toks, Token::Punct(s) if s == "(")?;
            while !next!(toks, Token::Punct(s) if s == ")").is_ok() {
                if !e.is_empty() {
                    next!(toks, Token::Punct(s) if s == ",")?;
                }
                e.push(parse_expr(toks)?)
            }
            Ok(Expr::Arr(ty, e))
        }
        /* parse struct literals */
        (_, Token::Punct(s)) if s == "{" => {
            let ty = parse_ty(toks)?;
            next!(toks, Token::Punct(s) if s == "}")?;
            let mut e = vec![];
            next!(toks, Token::Punct(s) if s == "(")?;
            while !next!(toks, Token::Punct(s) if s == ")").is_ok() {
                if !e.is_empty() {
                    next!(toks, Token::Punct(s) if s == ",")?;
                }
                e.push(parse_expr(toks)?)
            }
            Ok(Expr::Struct(e))
        }
        (_, Token::I8(x)) => Ok(Expr::I8(x)),
        (_, Token::U8(x)) => Ok(Expr::U8(x)),
        (_, Token::I16(x)) => Ok(Expr::I16(x)),
        (_, Token::U16(x)) => Ok(Expr::U16(x)),
        (_, Token::I32(x)) => Ok(Expr::I32(x)),
        (_, Token::U32(x)) => Ok(Expr::U32(x)),
        (_, Token::I64(x)) => Ok(Expr::I64(x)),
        (_, Token::U64(x)) => Ok(Expr::U64(x)),

        (_, Token::Int(x)) => Ok(Expr::Int(x)),

        // (loc, t) => Err("{:#?}"),
        (loc, t) => Err((
            loc,
            format!("Unexpected Token: {} at {}:{}", t, loc.line, loc.col),
        )),
    }
}

fn parse_member<I>(toks: &mut Peekable<I>) -> Result<Expr>
where
    I: Iterator<Item = (TokenLoc, Token)>,
{
    let e = parse_factor(toks)?;
    if next!(toks, Token::Punct(s) if s == ".").is_ok() {
        let (loc, name) = next!(toks, Token::Word(s) => s)?.clone();
        Ok(Expr::NMember(loc, box e, name))
    } else {
        Ok(e)
    }
}

fn parse_call<I>(toks: &mut Peekable<I>) -> Result<Expr>
where
    I: Iterator<Item = (TokenLoc, Token)>,
{
    let e = parse_member(toks)?;
    if next!(toks, Token::Punct(s) if s == "(").is_ok() {
        let mut args = Vec::new();
        while !next!(toks, Token::Punct(s) if s == ")").is_ok() {
            if !args.is_empty() {
                next!(toks, Token::Punct(s) if s == ",")?;
            }

            args.push(parse_expr(toks)?);
        }
        Ok(Expr::Call(Box::new(e), args))
    } else {
        Ok(e)
    }
}

fn parse_arrsub<I>(toks: &mut Peekable<I>) -> Result<Expr>
where
    I: Iterator<Item = (TokenLoc, Token)>,
{
    let e = parse_call(toks)?;
    if next!(toks, Token::Punct(s) if s == "[").is_ok() {
        let s = parse_expr(toks)?;
        next!(toks, Token::Punct(s) if s == "]")?;
        Ok(Expr::ArrSub(Box::new(e), Box::new(s)))
    } else {
        Ok(e)
    }
}

fn parse_unary<I>(toks: &mut Peekable<I>) -> Result<Expr>
where
    I: Iterator<Item = (TokenLoc, Token)>,
{
    if next!(toks, Token::Punct(s) if s == "!").is_ok() {
        Ok(Expr::Not(box parse_arrsub(toks)?))
    } else {
        parse_arrsub(toks)
    }
}

fn parse_term<I>(toks: &mut Peekable<I>) -> Result<Expr>
where
    I: Iterator<Item = (TokenLoc, Token)>,
{
    let e = parse_unary(toks)?;
    if next!(toks, Token::Punct(s) if s == "*").is_ok() {
        let rhs = parse_expr(toks)?;
        Ok(Expr::Mul(box e, box rhs))
    } else if next!(toks, Token::Punct(s) if s == "/").is_ok() {
        let rhs = parse_expr(toks)?;
        Ok(Expr::Div(box e, box rhs))
    } else if next!(toks, Token::Punct(s) if s == "%").is_ok() {
        let rhs = parse_expr(toks)?;
        Ok(Expr::Mod(box e, box rhs))
    } else {
        Ok(e)
    }
}

fn parse_arith<I>(toks: &mut Peekable<I>) -> Result<Expr>
where
    I: Iterator<Item = (TokenLoc, Token)>,
{
    let e = parse_term(toks)?;
    if next!(toks, Token::Punct(s) if s == "+").is_ok() {
        let rhs = parse_expr(toks)?;
        Ok(Expr::Add(box e, box rhs))
    } else if next!(toks, Token::Punct(s) if s == "-").is_ok() {
        let rhs = parse_expr(toks)?;
        Ok(Expr::Sub(box e, box rhs))
    } else {
        Ok(e)
    }
}

fn parse_relational<I>(toks: &mut Peekable<I>) -> Result<Expr>
where
    I: Iterator<Item = (TokenLoc, Token)>,
{
    let e = parse_arith(toks)?;
    if next!(toks, Token::Punct(s) if s == "<").is_ok() {
        let rhs = parse_expr(toks)?;
        Ok(Expr::LT(box e, box rhs))
    } else if next!(toks, Token::Punct(s) if s == ">").is_ok() {
        let rhs = parse_expr(toks)?;
        Ok(Expr::GT(box e, box rhs))
    } else {
        Ok(e)
    }
}

fn parse_expr<I>(toks: &mut Peekable<I>) -> Result<Expr>
where
    I: Iterator<Item = (TokenLoc, Token)>,
{
    parse_relational(toks)
}

fn parse_stmt<I>(toks: &mut Peekable<I>) -> Result<Stmt>
where
    I: Iterator<Item = (TokenLoc, Token)>,
{
    match toks.peek().unwrap().1.clone() {
        Token::Punct(s) if s == ";" => {
            toks.next().unwrap();
            Ok(Stmt::Noop)
        }
        Token::Punct(s) if s == "{" => {
            toks.next().unwrap();
            let mut stmts = Vec::new();
            while !next!(toks, Token::Punct(s) if s == "}").is_ok() {
                stmts.push(parse_stmt(toks)?);
            }
            Ok(Stmt::Block(stmts))
        }
        Token::Word(s) if s == "if" => {
            toks.next().unwrap();
            next!(toks, Token::Punct(s) if s == "(")?;
            let e = parse_expr(toks)?;
            next!(toks, Token::Punct(s) if s == ")")?;
            let s = parse_stmt(toks)?;
            let t = if next!(toks, Token::Word(s) if s == "else").is_ok() {
                Some(Box::new(parse_stmt(toks)?))
            } else {
                None
            };
            Ok(Stmt::If(e, Box::new(s), t))
        }

        Token::Word(s) if s == "while" => {
            toks.next().unwrap();
            next!(toks, Token::Punct(s) if s == "(")?;
            let e = parse_expr(toks)?;
            next!(toks, Token::Punct(s) if s == ")")?;
            let s = parse_stmt(toks)?;
            Ok(Stmt::While(e, Box::new(s)))
        }
        Token::Word(s) if s == "var" => {
            toks.next().unwrap();
            let (_, n) = next!(toks, Token::Word(s) => s)?.clone();
            next!(toks, Token::Punct(s) if s == ":")?;
            let ty = parse_ty(toks)?;
            next!(toks, Token::Punct(s) if s == "=")?;
            let e = parse_expr(toks)?;
            next!(toks, Token::Punct(s) if s == ";")?;
            Ok(Stmt::NVarDecl {
                name: n,
                ty,
                val: e,
            })
        }
        Token::Word(s) if s == "ret" => {
            toks.next().unwrap();
            let e = if !next!(toks, Token::Punct(s) if s == ";").is_ok() {
                let x = Some(parse_expr(toks)?);
                next!(toks, Token::Punct(s) if s == ";")?;
                x
            } else {
                None
            };
            Ok(Stmt::Ret(e))
        }
        _ => {
            let e = parse_expr(toks)?;
            if next!(toks, Token::Punct(s) if s == ";").is_ok() {
                Ok(Stmt::Expr(e))
            } else {
                let le = e;
                next!(toks, Token::Punct(s) if s == "=")?;
                let e = parse_expr(toks)?;
                Ok(Stmt::Ass { loc: le, val: e })
            }
        }
    }
}

fn parse_tld<I>(toks: &mut Peekable<I>) -> Result<TLD>
where
    I: Iterator<Item = (TokenLoc, Token)>,
{
    let is_pub = next!(toks, Token::Word(s) if s == "pub").is_ok();
    match toks.next().unwrap() {
        (loc, Token::Word(s)) if s == "fn" => {
            let (_, s) = next!(toks, Token::Word(s) => s)?.clone();

            let t = parse_ty(toks)?;

            if peek!(toks, Token::Word(s) if s == "fn" || s == "ty" || s == "in" || s == "out" || s == "uniform")
                || peek!(toks, Token::Punct(s) if s == "}")
                || toks.peek().is_none()
            {
                Ok(TLD::NFuncDecl {
                    loc,
                    name: s,
                    ty: t,
                })
            } else {
                Ok(TLD::NFunc {
                    loc,
                    name: s,
                    ty: t,
                    stmt: parse_stmt(toks)?,
                })
            }
        }
        (loc, Token::Word(s)) if s == "ty" => {
            let (_, s) = next!(toks, Token::Word(s) => s)?.clone();
            next!(toks, Token::Punct(s) if s == ":")?;
            Ok(TLD::NTypeDecl {
                loc,
                name: s,
                ty: parse_ty(toks)?,
            })
        }
        (tloc, Token::Word(s)) if s == "in" => {
            next!(toks, Token::Punct(s) if s == "[")?;
            let (_, loc) = next!(toks, Token::Int(i) => i)?;
            let loc = loc as usize;
            next!(toks, Token::Punct(s) if s == "]")?;
            let (_, s) = next!(toks, Token::Word(s) => s)?.clone();
            next!(toks, Token::Punct(s) if s == ":")?;
            let ty = parse_ty(toks)?;
            Ok(TLD::NIn(tloc, loc, s, ty))
        }

        (tloc, Token::Word(s)) if s == "out" => {
            next!(toks, Token::Punct(s) if s == "[")?;
            let (_, loc) = next!(toks, Token::Int(i) => i)?;
            let loc = loc as usize;
            next!(toks, Token::Punct(s) if s == "]")?;
            let (_, s) = next!(toks, Token::Word(s) => s)?.clone();
            next!(toks, Token::Punct(s) if s == ":")?;
            let ty = parse_ty(toks)?;
            Ok(TLD::NOut(tloc, loc, s, ty))
        }

        (loc, t) => Err((
            loc,
            format!("Unexpected Token: {} at {}:{}", t, loc.line, loc.col),
        )),
    }
}

pub fn parse_all<I>(toks: &mut Peekable<I>) -> Result<Vec<TLD>>
where
    I: Iterator<Item = (TokenLoc, Token)>,
{
    let mut r = Vec::new();
    while toks.peek().is_some() && !peek!(toks, Token::Punct(s) if s == "}") {
        match parse_tld(toks) {
            Ok(t) => {
                r.push(t);
            }
            e => {
                e?;
            }
        }
    }
    Ok(r)
}

pub fn parse(handler: &ErrHandler, toks: Vec<(TokenLoc, Token)>) -> Vec<TLD> {
    let mut toks = toks.into_iter().peekable();
    match parse_all(&mut toks) {
        Ok(t) => t,

        e => handler.handle(e),
    }
}
