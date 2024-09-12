use std::collections::HashMap;

use crate::err::{ErrHandler, Result};
use crate::util::*;

pub struct Resolver {
    /// Rust vectors are basic lifo structures provided by rust. In this context
    /// the vec is being used a as a stack of hashmaps. Each individual hashmap
    /// on the stack represents a single code scope such as the block within an
    /// if statement.
    expr_tbl: Vec<HashMap<String, (usize, Type)>>,
    /// Keep track of which id should be allocated to the next identifier
    expr_next: usize,
    type_tbl: Vec<HashMap<String, (usize, Type)>>,
    type_next: usize,
}

impl Resolver {
    fn new() -> Self {
        Self {
            // Initiate the symbol table stack with a single hashmap representing global
            // scope for the file. Here all globally defined identifiers such as function
            // definitions are stored.
            expr_tbl: vec![HashMap::new()],
            expr_next: 0,
            type_tbl: vec![HashMap::new()],
            type_next: 0,
        }
    }

    fn res_type(&mut self, ty: Type) -> Result<Type> {
        match ty {
            Type::NId(tloc, n) => {
                let mut ret = None;
                let mut i = 0;
                while ret.is_none() && i < self.type_tbl.len() {
                    ret = match self.type_tbl[self.type_tbl.len() - i - 1].get(&n) {
                        Some(s) => Some(s.clone()),
                        None => None,
                    };
                    i += 1;
                }
                let (id, ty) = ret.ok_or((
                    tloc,
                    format!("Undefined Type: '{}' at {}:{}", n, tloc.line, tloc.col),
                ))?;
                Ok(Type::TId(id, box ty))
            }
            Type::NStruct(tys) => {
                let tys: Result<Vec<_>> = tys.into_iter().map(|t| self.res_type(t.1)).collect();
                Ok(Type::Struct(tys?))
            }
            Type::Ptr(box p) => Ok(Type::Ptr(box self.res_type(p)?)),
            Type::Func(box r, args) => Ok(Type::Func(
                box self.res_type(r)?,
                args.into_iter()
                    .map(|(id, t)| Ok((id, self.res_type(t)?)))
                    .collect::<Result<Vec<_>>>()?,
            )),
            Type::Arr(box ty) => Ok(Type::Arr(box self.res_type(ty)?)),
            e => Ok(e),
        }
    }

    fn res_expr(&mut self, expr: Expr) -> Result<Expr> {
        match expr {
            Expr::NId(tloc, n) => {
                let mut ret = None;
                let mut i = 0;
                while ret.is_none() && i < self.expr_tbl.len() {
                    ret = match self.expr_tbl[self.expr_tbl.len() - i - 1].get(&n) {
                        Some(s) => Some(s.clone()),
                        None => None,
                    };
                    i += 1;
                }
                let (id, t) = ret.ok_or((
                    tloc,
                    format!(
                        "Undefined reference to '{}' at {}:{}",
                        n, tloc.line, tloc.col
                    ),
                ))?;
                let t = self.res_type(t)?;
                Ok(Expr::TId(t, id))
            }
            Expr::Call(e, es) => Ok(Expr::Call(
                Box::new(self.res_expr(*e)?),
                es.into_iter()
                    .map(|e| self.res_expr(e))
                    .collect::<Result<Vec<_>>>()?,
            )),
            Expr::TCall(ty, e, es) => Ok(Expr::TCall(
                self.res_type(ty)?,
                Box::new(self.res_expr(*e)?),
                es.into_iter()
                    .map(|e| self.res_expr(e))
                    .collect::<Result<Vec<_>>>()?,
            )),
            Expr::Struct(es) => Ok(Expr::Struct(
                es.into_iter()
                    .map(|e| self.res_expr(e))
                    .collect::<Result<Vec<_>>>()?,
            )),
            Expr::Arr(ty, es) => Ok(Expr::Arr(
                self.res_type(ty)?,
                es.into_iter()
                    .map(|e| self.res_expr(e))
                    .collect::<Result<Vec<_>>>()?,
            )),
            Expr::Ref(box e) => Ok(Expr::Ref(box self.res_expr(e)?)),
            Expr::Deref(box e) => Ok(Expr::Deref(box self.res_expr(e)?)),
            Expr::NMember(loc, box e, s) => {
                let e = self.res_expr(e)?;
                match e.ty() {
                    Type::Vec2 | Type::Vec3 | Type::Vec4 => {
                        let i = match s.as_str() {
                            "x" | "r" => 0,
                            "y" | "g" => 1,
                            "z" | "b" => 2,
                            "w" | "a" => 3,
                            s => Err((
                                TokenLoc::eof(),
                                format!(
                                    "Unexpected Identifier: {}, Expected x, y, z, w, r, g, b or a",
                                    s
                                ),
                            ))?,
                        };
                        Ok(Expr::Member(box e, i, s))
                    }
                    Type::Arr(..) => {
                        let i = match s.as_str() {
                            "len" => 0,
                            "ptr" => 1,
                            _ => Err((
                                TokenLoc::eof(),
                                format!("Unexpected Identifier: {}, Expected len or ptr", s),
                            ))?,
                        };
                        Ok(Expr::Member(box e, i, s))
                    }
                    _ => todo!(),
                }
            }
            Expr::Vec2(box e0, box e1) => {
                Ok(Expr::Vec2(box self.res_expr(e0)?, box self.res_expr(e1)?))
            }

            Expr::Vec3(box e0, box e1, box e2) => Ok(Expr::Vec3(
                box self.res_expr(e0)?,
                box self.res_expr(e1)?,
                box self.res_expr(e2)?,
            )),

            Expr::Vec4(box e0, box e1, box e2, box e3) => Ok(Expr::Vec4(
                box self.res_expr(e0)?,
                box self.res_expr(e1)?,
                box self.res_expr(e2)?,
                box self.res_expr(e3)?,
            )),

            Expr::Mul(box lhs, box rhs) => {
                Ok(Expr::Mul(box self.res_expr(lhs)?, box self.res_expr(rhs)?))
            }
            Expr::Shader(ty, tlds) => Ok(Expr::Shader(
                ty,
                tlds.into_iter()
                    .map(|t| self.res_tld(t))
                    .collect::<Result<Vec<_>>>()?,
            )),

            Expr::Not(box e) => Ok(Expr::Not(box self.res_expr(e)?)),
            Expr::GT(box a, box b) => Ok(Expr::GT(box self.res_expr(a)?, box self.res_expr(b)?)),
            Expr::LT(box a, box b) => Ok(Expr::LT(box self.res_expr(a)?, box self.res_expr(b)?)),
            Expr::Add(box a, box b) => Ok(Expr::Add(box self.res_expr(a)?, box self.res_expr(b)?)),
            Expr::Sub(box a, box b) => Ok(Expr::Sub(box self.res_expr(a)?, box self.res_expr(b)?)),
            Expr::Mod(box a, box b) => Ok(Expr::Mod(box self.res_expr(a)?, box self.res_expr(b)?)),

            Expr::ArrSub(box a, box b) => Ok(Expr::ArrSub(box self.res_expr(a)?, box self.res_expr(b)?)),
            e => Ok(e),
        }
    }
    fn res_stmt(&mut self, stmt: Stmt) -> Result<Stmt> {
        match stmt {
            Stmt::NVarDecl { val, name, ty } => {
                let id = self.expr_next;
                self.expr_tbl
                    .last_mut()
                    .unwrap()
                    .insert(name.clone(), (id, ty.clone()));
                self.expr_next += 1;

                let val = self.res_expr(val)?;

                let ty = self.res_type(ty)?;

                Ok(Stmt::VarDecl { id, val, name, ty })
            }
            Stmt::Block(stmts) => Ok(Stmt::Block(
                stmts
                    .into_iter()
                    .map(|s| self.res_stmt(s))
                    .collect::<Result<Vec<_>>>()?,
            )),
            Stmt::While(e, box s) => {
                let e = self.res_expr(e)?;
                let s = self.res_stmt(s)?;
                Ok(Stmt::While(e, box s))
            }
            Stmt::If(e, stmt, e_s) => {
                let e = self.res_expr(e)?;
                let s = self.res_stmt(*stmt)?;
                let e_s = match e_s {
                    Some(box e_s) => Some(box self.res_stmt(e_s)?),
                    None => None,
                };
                Ok(Stmt::If(e, Box::new(s), e_s))
            }
            Stmt::Ret(Some(e)) => Ok(Stmt::Ret(Some(self.res_expr(e)?))),
            Stmt::Ret(None) => Ok(Stmt::Ret(None)),
            Stmt::Expr(e) => Ok(Stmt::Expr(self.res_expr(e)?)),

            Stmt::Ass { loc, val } => Ok(Stmt::Ass {
                loc: self.res_expr(loc)?,
                val: self.res_expr(val)?,
            }),
            x => Ok(x),
        }
    }

    fn res_tld(&mut self, tld: TLD) -> Result<TLD> {
        match tld {
            TLD::NFuncDecl { loc, name, ty } => {
                let id = self.expr_next;
                self.expr_next += 1;

                // self.expr_tbl.push(HashMap::new());

                if let Type::NFunc(r_ty, a_tys) = ty {
                    let ty = Type::Func(
                        box self.res_type(*r_ty)?,
                        a_tys
                            .into_iter()
                            .map(|(n, t)| {
                                let t = self.res_type(t)?;
                                // let id = self.expr_next;
                                // self.expr_tbl.last_mut().unwrap().insert(n.clone(), (id, t.clone()));
                                // self.expr_next += 1;
                                Ok((id, t))
                            })
                            .collect::<Result<Vec<_>>>()?,
                    );

                    self.expr_tbl
                        .last_mut()
                        .unwrap()
                        .insert(name.clone(), (id, ty.clone()));

                    // self.expr_tbl.pop();

                    Ok(TLD::FuncDecl { loc, name, id, ty })
                } else {
                    Err((TokenLoc::eof(), format!("Expected function type")))
                }
            }
            TLD::NFunc {
                loc,
                name,
                ty,
                stmt,
            } => {
                self.expr_tbl.push(HashMap::new());

                if let Type::NFunc(r_ty, a_tys) = ty {
                    let ty = Type::Func(
                        r_ty,
                        a_tys
                            .into_iter()
                            .map(|(n, t)| {
                                let t = self.res_type(t)?;
                                let id = self.expr_next;
                                self.expr_tbl
                                    .last_mut()
                                    .unwrap()
                                    .insert(n.clone(), (id, t.clone()));
                                self.expr_next += 1;
                                Ok((id, t))
                            })
                            .collect::<Result<Vec<_>>>()?,
                    );
                    // Functions are allways declared at the top of the expression
                    // tabe and should always be inserted there
                    let id = if let Some(x) = self.expr_tbl.get_mut(0).unwrap().get(&name) {
                        x.0
                    } else {
                        let id = self.expr_next;
                        self.expr_tbl
                            .get_mut(0)
                            .unwrap()
                            .insert(name.clone(), (id, ty.clone()));
                        self.expr_next += 1;
                        id
                    };

                    let stmt = self.res_stmt(stmt)?;

                    self.expr_tbl.pop();

                    Ok(TLD::Func {
                        loc,
                        id,
                        name,
                        ty,
                        stmt,
                    })
                } else {
                    Err((TokenLoc::eof(), format!("Expected function type")))
                }
            }
            TLD::NTypeDecl { loc, name, ty } => {
                let id = self.type_next;
                let ty = self.res_type(ty)?;
                self.type_tbl
                    .last_mut()
                    .unwrap()
                    .insert(name.clone(), (id, ty.clone()));
                self.type_next += 1;
                Ok(TLD::TypeDecl { loc, id, name, ty })
            }

            TLD::NIn(tloc, loc, name, ty) => {
                let id = self.expr_next;
                self.expr_tbl
                    .last_mut()
                    .unwrap()
                    .insert(name.clone(), (id, ty.clone()));
                self.expr_next += 1;
                let ty = self.res_type(ty)?;
                Ok(TLD::In(tloc, loc, id, name, ty))
            }

            TLD::NOut(tloc, loc, name, ty) => {
                let id = self.expr_next;
                self.expr_tbl
                    .last_mut()
                    .unwrap()
                    .insert(name.clone(), (id, ty.clone()));
                self.expr_next += 1;
                let ty = self.res_type(ty)?;
                Ok(TLD::Out(tloc, loc, id, name, ty))
            }

            TLD::NBuiltinVertPos => {
                let id = self.expr_next;
                self.expr_tbl
                    .last_mut()
                    .unwrap()
                    .insert("sgl_vert_pos".into(), (id, Type::Vec4));
                self.expr_next += 1;
                Ok(TLD::BuiltinVertPos(id))
            }

            TLD::NBuiltinVertPointSize => {
                let id = self.expr_next;
                self.expr_tbl
                    .last_mut()
                    .unwrap()
                    .insert("sgl_vert_point_size".into(), (id, Type::F32));
                self.expr_next += 1;
                Ok(TLD::BuiltinVertPointSize(id))
            }
            x => Ok(x),
        }
    }
}

/// Polymorphysm is used here. This implements the Pass trait (rust's abstract class like
/// concepts) for Resolver objects.
impl Pass for Resolver {
    fn res(handler: &ErrHandler, tlds: Vec<TLD>) -> Vec<TLD> {
        let mut res = Self::new();
        tlds.into_iter()
            .map(|t| handler.handle(res.res_tld(t)))
            .collect()
    }
}
