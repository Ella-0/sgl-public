use std::collections::HashMap;

use crate::util::*;
use crate::err::*;

pub struct Typer {}

impl Typer {
    fn new() -> Self {
        Self {}
    }

    fn res_type(&mut self, ty: Type) -> Type {
        match ty {
            t => t,
        }
    }

    fn res_expr(&mut self, suggested: Option<Type>, expr: Expr) -> Expr {
        match expr {
            Expr::Null => Expr::TNull(suggested.expect("Could not infer type")),

            Expr::Int(v) => match suggested {
                Some(Type::U8) => Expr::U8(v as u8),
                Some(Type::U16) => Expr::U16(v as u16),
                Some(Type::U32) => Expr::U32(v as u32),
                Some(Type::U64) => Expr::U64(v as u64),
                Some(Type::I8) => Expr::I8(v as i8),
                Some(Type::I16) => Expr::I16(v as i16),
                Some(Type::I32) => Expr::I32(v as i32),
                Some(Type::I64) => Expr::I64(v as i64),
                Some(Type::F32) => Expr::F32(v as f32),
                None => Expr::I32(v as i32),
                e => panic!("Expected integer or real value: {:#?} {}", e, v),
            },

            Expr::Call(e, v) => {
                let e = self.res_expr(None, *e);
                if let Type::Func(r, p) = e.ty() {
                    Expr::TCall(
                        *r,
                        box e,
                        v.into_iter()
                            .zip(p.iter())
                            .map(|(ex, (_, ty))| self.res_expr(Some(ty.clone()), ex))
                            .collect(),
                    )
                } else {
                    panic!()
                }
            }

            Expr::Struct(e) => match suggested.clone() {
                Some(Type::Struct(t)) | Some(Type::TId(_, box Type::Struct(t))) => Expr::TStruct(
                    suggested.unwrap(),
                    e.into_iter()
                        .zip(t)
                        .map(|(e, t)| self.res_expr(Some(t), e))
                        .collect(),
                ),
                _ => panic!(),
            },

            Expr::Arr(ty, es) => Expr::Arr(
                self.res_type(ty.clone()),
                es.into_iter()
                    .map(|e| self.res_expr(Some(ty.clone()), e))
                    .collect(),
            ),

            Expr::Vec2(box e0, box e1) => Expr::Vec2(
                box self.res_expr(Some(Type::F32), e0),
                box self.res_expr(Some(Type::F32), e1),
            ),

            Expr::Vec3(box e0, box e1, box e2) => Expr::Vec3(
                box self.res_expr(Some(Type::F32), e0),
                box self.res_expr(Some(Type::F32), e1),
                box self.res_expr(Some(Type::F32), e2),
            ),

            Expr::Vec4(box e0, box e1, box e2, box e3) => Expr::Vec4(
                box self.res_expr(Some(Type::F32), e0),
                box self.res_expr(Some(Type::F32), e1),
                box self.res_expr(Some(Type::F32), e2),
                box self.res_expr(Some(Type::F32), e3),
            ),

            Expr::Mul(box lhs, box rhs) => Expr::Mul(
                box self.res_expr(suggested.clone(), lhs),
                box self.res_expr(suggested, rhs),
            ),

            Expr::Shader(ty, tlds) => {
                Expr::Shader(ty, tlds.into_iter().map(|tld| self.res_tld(tld)).collect::<Result<_>>().unwrap())
            }

            Expr::Add(box lhs, box rhs) => Expr::Add(
                box self.res_expr(suggested.clone(), lhs),
                box self.res_expr(suggested, rhs),
            ),

            Expr::Sub(box lhs, box rhs) => Expr::Sub(
                box self.res_expr(suggested.clone(), lhs),
                box self.res_expr(suggested, rhs),
            ),

            Expr::Mod(box lhs, box rhs) => Expr::Mod(
                box self.res_expr(suggested.clone(), lhs),
                box self.res_expr(suggested, rhs),
            ),

            Expr::LT(box lhs, box rhs) => {
                let lhs = self.res_expr(None, lhs);
                let rhs = self.res_expr(Some(lhs.ty()), rhs);
                Expr::LT(box lhs, box rhs)
            }

            Expr::ArrSub(box arr, box idx) => {
		let arr = self.res_expr(None, arr);
		let idx = self.res_expr(None, idx);
		Expr::ArrSub(box arr, box idx)
            }

            Expr::Ref(box e) => {
                Expr::Ref(box self.res_expr(None, e))
            }

            e => e,
        }
    }
    fn res_stmt(&mut self, stmt: Stmt) -> Result<Stmt> {
        match stmt {
            Stmt::VarDecl { id, val, name, ty } => {
                let val = self.res_expr(Some(ty.clone()), val);
                Ok(Stmt::VarDecl { id, val, name, ty })
            }

            Stmt::Expr(e) => Ok(Stmt::Expr(self.res_expr(None, e))),

            Stmt::Block(s) => Ok(Stmt::Block(s.into_iter().map(|s| self.res_stmt(s)).collect::<Result<Vec<_>>>()?)),


            Stmt::Ass { loc, val } => {
                let loc = self.res_expr(None, loc);
                let val = self.res_expr(Some(loc.ty()), val);
                Ok(Stmt::Ass { loc, val })
            }

            Stmt::While(e, box s) => {
                let e = self.res_expr(None, e);
                let s = self.res_stmt(s)?;
                Ok(Stmt::While(e, box s))
            }

            s => Ok(s),
        }
    }

    fn res_tld(&mut self, tld: TLD) -> Result<TLD> {
        match tld {
            TLD::Func {
                loc,
                id,
                name,
                ty,
                stmt,
            } => {
                let stmt = self.res_stmt(stmt)?;
                Ok(TLD::Func {
                    loc,
                    id,
                    name,
                    ty,
                    stmt,
                })
            }
            d => Ok(d),
        }
    }
}

impl Pass for Typer {
    fn res(handler: &ErrHandler, tlds: Vec<TLD>) -> Vec<TLD> {
        let mut res = Self::new();
        handler.handle(tlds.into_iter().map(|t| res.res_tld(t)).collect())
    }
}
