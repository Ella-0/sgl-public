//! This file takes the tree after the semantic stages of the compiler and transforms
//! it into LLVM-IR, the intermediate representation processed by the LLVM compiler backend.

use llvm_sys::{core::*, prelude::*};
use std::collections::HashMap;
use crate::util::*;

/// Alias some helper types. These map type ids to type refs
type TypeSymTbl = HashMap<usize, LLVMTypeRef>; // TODO: change to Vec
type ExprSymTbl = HashMap<usize, LLVMValueRef>;

/// The GenLLVM structure stores state needed by the code generator as it processes the
/// tree.
pub struct GenLLVM {
    /// The LLVM module that the compiled file is building
    module: LLVMModuleRef,

    /// The LLVM builder object that LLVM uses to add instructions to
    /// blocks of code within a module.
    builder: LLVMBuilderRef,

    /// Maps of identifier ids to their currently assigned LLVM values
    type_tbl: TypeSymTbl,
    expr_tbl: ExprSymTbl,
}

/// Methods associated with the GenLLVM structure.
impl GenLLVM {

    /// Construct a new GenLLVM structure from the name
    /// of the file.
    pub fn new(n: &str) -> Self {
        let mut n: Vec<u8> = n.to_string().into();
        n.push(0u8);
        unsafe {
            // Initialise the LLVM module
            let module = LLVMModuleCreateWithName(n.as_ptr() as *const i8);

            // Set the target architecture for LLVM. Currently this is hard coded
            // but it should not be. My system is `x86_64-linux-musl` but to target
            // windows or glibc linux distros with aarch64 (64 bit arm) cpu architecture
            //  for example one would need `aarch64-windows-msvc` or `aarch64-linux-gnu`
            LLVMSetTarget(module, "x86_64-unknown-linux-musl\0".as_ptr() as *const i8);

            // Initialise the builder object for LLVM
            let builder = LLVMCreateBuilder();

            // Initialse the GenLLVM structure
            Self {
                module,
                builder,
                type_tbl: TypeSymTbl::new(),
                expr_tbl: ExprSymTbl::new(),
            }
        }
    }

    /// Converts a type node in the tree to an LLVM type
    fn gen_type(&self, ty: &Type) -> LLVMTypeRef {
        unsafe {
            match ty {
                // Bools are simply 1 bit integers
                Type::Bool => LLVMIntType(1),

                // LLVM uses the same integer type for signed and unsigned
                // and instead distinguishes signed and unsigned on an instruction
                // level.
                Type::I8 | Type::U8 => LLVMIntType(8),
                Type::I16 | Type::U16 => LLVMIntType(16),
                Type::I32 | Type::U32 => LLVMIntType(32),
                Type::I64 | Type::U64 => LLVMIntType(64),

                // LLVM uses Half, Float and Double to refer to 16 bit, 32 bit and 64 bit IEEE floats
                // respectivley
                Type::_F16 => LLVMHalfType(),
                Type::F32 => LLVMFloatType(),
                Type::_F64 => LLVMDoubleType(),

                // CPU code doesn't allow for vector types. These are GPU only types
                // TODO: better error handling here.
                Type::Vec2 => panic!("Type {:#?} not implemented for CPU side code", ty),
                Type::Vec3 => panic!("Type {:#?} not implemented for CPU side code", ty),
                Type::Vec4 => panic!("Type {:#?} not implemented for CPU side code", ty),

                // Converts a function type to an LLVM function type. Here higher order functions
                // are used to map the argument types to llvm types
                Type::Func(rty, tys) => {
                    let mut tys: Vec<LLVMTypeRef> =
                        tys.iter().map(|(_, t)| self.gen_type(t)).collect();
                    LLVMFunctionType(self.gen_type(rty), tys.as_mut_ptr(), tys.len() as u32, 0)
                }

                // Convert a struct type into an unamed LLVM struct type. Higher order functions
                // are used to map the types of each sturct member to llvm types.
                Type::Struct(types) => {
                    let mut ltypes: Vec<LLVMTypeRef> =
                        types.iter().map(|t| self.gen_type(t)).collect();
                    LLVMStructType(ltypes.as_mut_ptr(), ltypes.len() as u32, 0)
                }

                // Maps a pointer to an LLVM pointer through recursion
                Type::Ptr(ty) => LLVMPointerType(self.gen_type(ty), 0),

                // Maps an array type to an LLVM array type.
                Type::Arr(ty) => {
                    let mut ltypes = vec![LLVMIntType(64), LLVMPointerType(self.gen_type(ty), 0)];
                    LLVMStructType(ltypes.as_mut_ptr(), ltypes.len() as u32, 0)
                }

                // Map an identifier to its concrete type
                Type::TId(id, _ty) => *self.type_tbl.get(id).expect("undefined ref"),
                Type::Id(id) => *self.type_tbl.get(id).expect("undefined ref"),

                // Map the void type to the LLVM void type.
                Type::Void => LLVMVoidType(),

                // Named identifiers are not allowed or expected at this point in the compiler
                Type::NId(_, s) => unreachable!("{}", s),
                Type::NFunc(..) => unreachable!(),
                Type::NStruct(..) => unreachable!(),
            }
        }
    }

    /// lexprs or left expressions are expressions that occur on the left hand side of
    /// an assignment. For example an lexpr could be a identifier for a varable such as
    /// `foo = 10` or the lexpr could be an index into an array such as `foo[0] = 10` or
    /// an lexpr could be a dereferenced pointer such as `*foo = 10` to store a value into
    /// a pointer.
    fn gen_lexpr(&self, alloc_block: LLVMBasicBlockRef, expr: &Expr) -> LLVMValueRef {
        unsafe {
            match expr {
                // References to temporary values are not allowed
                Expr::Ref(_) => panic!("Can't reference a temporary"),

                // Resolve an identifier to it's contrete expression
                Expr::TId(_, id) => *self.expr_tbl.get(id).expect("undefined ref"),

                // A deref in an lexpr allows assigning to the result of a regular
                // expression
                Expr::Deref(box e) => self.gen_expr(alloc_block, e),

                // Get a reference to a struct member
                Expr::Member(box e, i, s) => {
                    let e = self.gen_lexpr(alloc_block, e);
                    LLVMBuildStructGEP(self.builder, e, *i as u32, [0i8].as_mut_ptr())
                }

                // Get a reference to a value in an array
                Expr::ArrSub(arr, idx) => {
                    let arr = self.gen_expr(alloc_block, arr);
                    let idx = self.gen_expr(alloc_block, idx);
                    let mut idx = [idx];
                    LLVMBuildGEP(
                        self.builder,
                        arr,
                        idx.as_mut_ptr(),
                        1,
                        [0i8].as_ptr()
                    )
                },

                // Unimplemented lexprs panic
                e => todo!("{:#?}", e),
            }
        }
    }

    /// Here we convert a regular right hand side expression to its LLVM equivalent
    fn gen_expr(&self, alloc_block: LLVMBasicBlockRef, expr: &Expr) -> LLVMValueRef {
        unsafe {
            match expr {
                // Map numeric constants to their LLVM equivalents.
                Expr::Bool(x) => LLVMConstInt(LLVMIntType(1), if *x { 1 } else { 0 }, 0),
                Expr::I8(x) => LLVMConstInt(LLVMIntType(8), *x as u64, 0),
                Expr::U8(x) => LLVMConstInt(LLVMIntType(8), *x as u64, 0),
                Expr::I16(x) => LLVMConstInt(LLVMIntType(16), *x as u64, 0),
                Expr::U16(x) => LLVMConstInt(LLVMIntType(16), *x as u64, 0),
                Expr::U32(x) => LLVMConstInt(LLVMIntType(32), *x as u64, 0),
                Expr::I32(x) => LLVMConstInt(LLVMIntType(32), *x as u64, 0),
                Expr::U64(x) => LLVMConstInt(LLVMIntType(64), *x as u64, 0),
                Expr::I64(x) => LLVMConstInt(LLVMIntType(64), *x as u64, 0),
                Expr::F32(x) => LLVMConstReal(LLVMFloatType(), *x as f64),

                // generate struct literals
                Expr::TStruct(ty, x) => {
                    // generate the type of the struct
                    let ty = self.gen_type(ty);

                    // Allocate enough space for types on the heap
                    let e_tys = std::alloc::alloc(
                        std::alloc::Layout::array::<LLVMTypeRef>(
                            LLVMCountStructElementTypes(ty) as usize
                        )
                        .unwrap(),
                    ) as *mut LLVMTypeRef;

                    // get the struct element types and store it in the e_tys ptr
                    LLVMGetStructElementTypes(ty, e_tys);

                    // convert to a rust slice to make e_tys easier to work with
                    let e_tys =
                        std::slice::from_raw_parts(e_tys, LLVMCountStructElementTypes(ty) as usize);

                    // list of non constant expressions we must add extra store code for
                    let mut non_constants = vec![];

                    // build the struct constant for all constant expresisons fill in their values
                    // for all non constant expressions assign nothing to (`LLVMConstNull`)
                    //
                    // higher order functions used here to map all LLVM expressions to either
                    // a constant LLVM expression or an LLVM null constant
                    let s = LLVMConstNamedStruct(
                        ty,
                        x.iter()
                            .enumerate()
                            .zip(e_tys)
                            .map(|((i, e), ty)| {
                                let e = self.gen_expr(alloc_block, e);
                                let a = if LLVMIsConstant(e) != 0 {
                                    e
                                } else {
                                    non_constants.push((i, e));
                                    LLVMConstNull(*ty)
                                };
                                a
                            })
                            .collect::<Vec<_>>()
                            .as_mut_ptr(),
                        e_tys.len() as u32,
                    );

                    // for all non constat types use higher order functions to
                    // generate code to insert values into the struct after they
                    // have been evaulated
                    non_constants.into_iter().fold(s, |s, (i, e)| {
                        LLVMBuildInsertValue(self.builder, s, e, i as u32, [0i8].as_ptr())
                    })
                }

                // at this point in the compiler we shouldn't have anything we don't know the type for
                Expr::Struct(..) => panic!(),

                // generate an array literal in a similar way to the struct.
                Expr::Arr(ty, v) => {
                    // save current position of the builder
                    let pos = LLVMGetInsertBlock(self.builder);

                    // position the builder into the alloc block to allow us to allocate the memory
                    // required for the array. This is required so we don't keep growing the stack
                    // if we end up in a loop
                    LLVMPositionBuilderAtEnd(self.builder, alloc_block);

                    // generate the array type
                    let ty = self.gen_type(ty);

                    // insert a stack allocation for that array
                    let r = LLVMBuildArrayAlloca(
                        self.builder,
                        ty,
                        LLVMConstInt(LLVMIntType(32), v.len() as u64, 0),
                        "arr\0".as_ptr() as *const i8,
                    );

                    // position builder back where it was to insert valuse into the array
                    LLVMPositionBuilderAtEnd(self.builder, pos);

                    // list of non constant values we have to add seperately
                    let mut non_constants = vec![];

                    // list of constant expressions we can add now
                    // higher order functions used here to map llvm expressions to
                    // either a constant expression or an llvm null constant
                    let mut constants: Vec<_> = v
                        .iter()
                        .enumerate()
                        .map(|(i, e)| {
                            let e = self.gen_expr(alloc_block, e);
                            if LLVMIsConstant(e) != 0 {
                                e
                            } else {
                                non_constants.push((i, e));
                                LLVMConstNull(ty)
                            }
                        })
                        .collect();

                    // for all the non constants seperately build insert instructions
                    // for them to insert them into the array
                    let arr = non_constants.into_iter().fold(
                        LLVMConstArray(ty, constants.as_mut_ptr(), constants.len() as u32),
                        |arr, (i, e)| {
                            LLVMBuildInsertValue(self.builder, arr, e, i as u32, [0i8].as_ptr())
                        },
                    );

                    // build one huge store instruction for the array into the alloc block
                    // location.
                    LLVMBuildStore(
                        self.builder,
                        arr,
                        LLVMBuildBitCast(
                            self.builder,
                            r,
                            LLVMPointerType(LLVMArrayType(ty, constants.len() as u32), 0),
                            [0i8].as_ptr(),
                        ),
                    );

                    // take the raw pointer from the alloc block and bundle it into a tuple with
                    // its length to create a slice structure.
                    let mut vals = vec![
                        LLVMConstInt(LLVMIntType(64), v.len() as u64, 0),
                        LLVMConstPointerNull(LLVMPointerType(ty, 0)),
                    ];
                    let s = LLVMConstStruct(vals.as_mut_ptr(), vals.len() as u32, 0);
                    LLVMBuildInsertValue(self.builder, s, r, 1, [0i8].as_ptr())
                }

                // generate binary expressions into their LLVM counter parts
                Expr::Add(a, b) => LLVMBuildAdd(
                    self.builder,
                    self.gen_expr(alloc_block, a),
                    self.gen_expr(alloc_block, b),
                    [0i8].as_ptr(),
                ),
                Expr::Sub(a, b) => LLVMBuildSub(
                    self.builder,
                    self.gen_expr(alloc_block, a),
                    self.gen_expr(alloc_block, b),
                    [0i8].as_ptr(),
                ),
                Expr::Mul(a, b) => LLVMBuildMul(
                    self.builder,
                    self.gen_expr(alloc_block, a),
                    self.gen_expr(alloc_block, b),
                    [0i8].as_ptr(),
                ),
                Expr::Div(a, b) => LLVMBuildSDiv(
                    self.builder,
                    self.gen_expr(alloc_block, a),
                    self.gen_expr(alloc_block, b),
                    [0i8].as_ptr(),
                ),
                Expr::Mod(a, b) => LLVMBuildSRem(
                    self.builder,
                    self.gen_expr(alloc_block, a),
                    self.gen_expr(alloc_block, b),
                    [0i8].as_ptr()
                ),
                Expr::LT(a, b) => LLVMBuildICmp(
                    self.builder,
                    llvm_sys::LLVMIntPredicate::LLVMIntSLT,
                    self.gen_expr(alloc_block, a),
                    self.gen_expr(alloc_block, b),
                    [0i8].as_ptr(),
                ),
                Expr::GT(a, b) => LLVMBuildICmp(
                    self.builder,
                    llvm_sys::LLVMIntPredicate::LLVMIntSGT,
                    self.gen_expr(alloc_block, a),
                    self.gen_expr(alloc_block, b),
                    [0i8].as_ptr(),
                ),

                // non typed identifiers should not occur at this point in the compiler as such
                // we panic.
                Expr::Id(_id) => panic!(),
                // All Named Identifiers should be replaced with integer references
                Expr::NId(..) => unreachable!(),

                // look up the typed identifier to get its concrete expression and return it
                Expr::TId(t, id) => {
                    let v = *self.expr_tbl.get(id).expect("undefined ref");
                    if let Type::Func(..) | Type::NFunc(..) = t {
                        v
                    } else {
                        LLVMBuildLoad(self.builder, v, [0i8].as_ptr())
                    }
                }

                // taking a reference makes us generate an lexpr
                Expr::Ref(box e) => self.gen_lexpr(alloc_block, e),

                // generate code for dereferencing a pointer to get the value it's pointing to
                Expr::Deref(e) => {
                    LLVMBuildLoad(self.builder, self.gen_expr(alloc_block, e), [0i8].as_ptr())
                }

                // generate a subroutine call. higher order functions used here to map the
                // argument expressions into their LLVM values
                Expr::Call(box Expr::Id(id), args) => {
                    let mut args: Vec<LLVMValueRef> =
                        args.iter().map(|a| self.gen_expr(alloc_block, a)).collect();
                    LLVMBuildCall(
                        self.builder,
                        *self.expr_tbl.get(id).expect("undefined ref"),
                        args.as_mut_ptr(),
                        args.len() as u32,
                        [0i8].as_ptr(),
                    )
                }

                // same as above except we know the type.
                Expr::TCall(_, f, args) | Expr::Call(f, args) => {
                    let mut args: Vec<LLVMValueRef> =
                        args.iter().map(|a| self.gen_expr(alloc_block, a)).collect();
                    LLVMBuildCall(
                        self.builder,
                        self.gen_expr(alloc_block, f),
                        args.as_mut_ptr(),
                        args.len() as u32,
                        [0i8].as_ptr(),
                    )
                }

                // generate code for an expression like `array[1]`
                Expr::ArrSub(arr, idx) => {
                    let arr = self.gen_expr(alloc_block, arr);
                    let idx = self.gen_expr(alloc_block, idx);
                    let mut idx = [idx];
                    let v = LLVMBuildGEP(
                        self.builder,
                        arr,
                        idx.as_mut_ptr(),
                        1,
                        [0i8].as_ptr()
                    );
                    LLVMBuildLoad(
                        self.builder,
                        v,
                        [0i8].as_ptr()
                    )
                },

                // generate a null pointer for a given type
                Expr::TNull(t) => LLVMConstPointerNull(self.gen_type(t)),

                // Nulls and non typed ints should not occur at this point of the compiler
                Expr::Null | Expr::Int(..) => panic!("{:#?}", expr),

                // generate code for expressions such as `my_struct.my_value`
                Expr::Member(box e, i, s) => {
                    let e = self.gen_expr(alloc_block, e);
                    LLVMBuildExtractValue(self.builder, e, *i as u32, [0i8].as_mut_ptr())
                }
                // not yet implemented
                Expr::NMember(..) => todo!(),

                // vector types are GPU only and are not supported by the LLVM backend
                Expr::Vec2(..) => panic!("not supported on CPU"),
                Expr::Vec3(..) => panic!("not supported on CPU"),
                Expr::Vec4(..) => panic!("not supported on CPU"),

                // Generate a shader expression. here we pass the whole tree to the
                // shader code generator and store the SPIR-V code generated into an array
                Expr::Shader(ty, tlds) => {
                    let mut ws = crate::spv::GenSPV::new(*ty);

                    ws.gen(tlds);

                    let ws = ws.pack();
                    self.gen_expr(
                        alloc_block,
                        &Expr::Arr(
                            Type::U32,
                            ws.into_iter().map(|w| Expr::U32(w)).collect::<Vec<_>>(),
                        ),
                    )
                }

                // generate code for a unary not such as `!true`
                Expr::Not(box e) => {
                    LLVMBuildNot(self.builder, self.gen_expr(alloc_block, e), [0i8].as_ptr())
                }
            }
        }
    }

    /// Generate code for each statement in the compiler
    fn gen_stmt(&mut self, func: LLVMValueRef, alloc_block: LLVMBasicBlockRef, stmt: &Stmt) {
        match stmt {
            // generates a variable declaration:
            //  - allocates on the stack
            //  - stores the initial value
            //
            // TODO: should probably do dropping stuff for value returned by insert
            Stmt::VarDecl { id, name, ty, val } => {
                let mut n: Vec<u8> = name.clone().into();
                n.push(0);
                let x = unsafe {
                    let pos = LLVMGetInsertBlock(self.builder);
                    LLVMPositionBuilderAtEnd(self.builder, alloc_block);
                    let x =
                        LLVMBuildAlloca(self.builder, self.gen_type(ty), n.as_ptr() as *const i8);
                    LLVMPositionBuilderAtEnd(self.builder, pos);
                    let v = self.gen_expr(alloc_block, val);
                    LLVMBuildStore(self.builder, v, x);
                    x
                };
                self.expr_tbl.insert(*id, x);
            }
            // generantes an assignmnt.
            Stmt::Ass { loc, val } => unsafe {
                LLVMBuildStore(
                    self.builder,
                    self.gen_expr(alloc_block, val),
                    self.gen_lexpr(alloc_block, loc),
                );
            },

            // generates an If statement for the case we don't have an else
            Stmt::If(e, x, None) => unsafe {
                // append blocks to jump to for then and endif
                let then = LLVMAppendBasicBlock(func, "then\0".as_ptr() as *const i8);
                let endif = LLVMAppendBasicBlock(func, "endif\0".as_ptr() as *const i8);

                // build a conditional branch. if the expresison is true we jump to the then
                // block else we jump to the endif block
                LLVMBuildCondBr(self.builder, self.gen_expr(alloc_block, e), then, endif);

                // generate code for the then block
                LLVMPositionBuilderAtEnd(self.builder, then);
                self.gen_stmt(func, alloc_block, x);

                // uncoditionary break back to the endif block
                LLVMBuildBr(self.builder, endif);

                // position builder at end of the endif block
                LLVMPositionBuilderAtEnd(self.builder, endif);

            },

            // generate an if statement for the case we have an else branch
            Stmt::If(e, x, Some(y)) => unsafe {
                // generate blocks for then else and eldif
                let then = LLVMAppendBasicBlock(func, "then\0".as_ptr() as *const i8);
                let a_then = LLVMAppendBasicBlock(func, "else\0".as_ptr() as *const i8);
                let endif = LLVMAppendBasicBlock(func, "endif\0".as_ptr() as *const i8);

                // build a conditional branch. if the expression is true, jump to the then block
                // else jump to the a_then or else block.
                LLVMBuildCondBr(self.builder, self.gen_expr(alloc_block, e), then, a_then);

                // gnerate code for the then block
                LLVMPositionBuilderAtEnd(self.builder, then);
                self.gen_stmt(func, alloc_block, x);
                // uncoditionaly break to the endif block
                LLVMBuildBr(self.builder, endif);

                // generate code for the a_then or else block
                LLVMPositionBuilderAtEnd(self.builder, a_then);
                self.gen_stmt(func, alloc_block, y);
                // unconditionally break to the endif block
                LLVMBuildBr(self.builder, endif);

                // position builder at end of the endif block
                LLVMPositionBuilderAtEnd(self.builder, endif);
            },

            // build a return statement for when we don't return a value
            Stmt::Ret(None) => unsafe {
                LLVMBuildRetVoid(self.builder);
            },

            // build a return statement for when we do return a value
            Stmt::Ret(Some(e)) => unsafe {
                LLVMBuildRet(self.builder, self.gen_expr(alloc_block, e));
            },

            // build a block of statements
            Stmt::Block(stmts) => {
                for s in stmts {
                    self.gen_stmt(func, alloc_block, s);
                }
            }

            // generate code for a while statement
            Stmt::While(e, s) => unsafe {
                // create basic bolcks for do and done where do is the body of the while
                // and done is the block we exit to.
                let do_loop = LLVMAppendBasicBlock(func, "do\0".as_ptr() as *const i8);
                let done = LLVMAppendBasicBlock(func, "done\0".as_ptr() as *const i8);

                // if the condition is true we break to the do block, else we break to the
                // done block
                LLVMBuildCondBr(self.builder, self.gen_expr(alloc_block, e), do_loop, done);

                // generate code for the while body
                LLVMPositionBuilderAtEnd(self.builder, do_loop);
                self.gen_stmt(func, alloc_block, s);

                // if the condition is true we jump back to the start o fthe do block
                // else we jump to the done block
                LLVMBuildCondBr(self.builder, self.gen_expr(alloc_block, e), do_loop, done);

                // position builder at the end of the done block
                LLVMPositionBuilderAtEnd(self.builder, done);
            },

            // generate code for an expression statement such as `1+1;`
            Stmt::Expr(e) => {
                self.gen_expr(alloc_block, e);
            }

            // generate code for a noop statement such as `;;`
            Stmt::Noop => {}

            // unimplemented statements cause a panic
            s => todo!("{:#?}", s),
        };
    }

    // Generate code for top level declarations such as functions
    // and type declarations
    pub fn gen_tld(&mut self, tld: &TLD) {
        match tld {
            TLD::FuncDecl { id, name, ty, .. } => unsafe {
                let mut n: Vec<u8> = name.clone().into();
                n.push(0);
                self.expr_tbl.insert(
                    *id,
                    LLVMAddFunction(self.module, n.as_ptr() as *const i8, self.gen_type(ty)),
                );
            },

            TLD::Func {
                id, name, stmt, ty, ..
            } => unsafe {
                let f = if let Some(f) = self.expr_tbl.get(id) {
                    *f
                } else {
                    let mut n: Vec<u8> = name.clone().into();
                    n.push(0);
                    let f =
                        LLVMAddFunction(self.module, n.as_ptr() as *const i8, self.gen_type(ty));
                    self.expr_tbl.insert(*id, f);
                    f
                };

                // For variable allocation
                let entry = LLVMAppendBasicBlock(f, "entry\0".as_ptr() as *const i8);
                LLVMPositionBuilderAtEnd(self.builder, entry);

                if let Type::Func(_, v) = ty {
                    v.iter().enumerate().for_each(|(i, (id, t))| {
                        let v = LLVMBuildAlloca(
                            self.builder,
                            self.gen_type(t),
                            "arg\0".as_ptr() as *const i8,
                        );
                        LLVMBuildStore(self.builder, LLVMGetParam(f, i as u32), v);
                        self.expr_tbl.insert(*id, v);
                    });
                } else {
                    panic!()
                }

                let body = LLVMAppendBasicBlock(f, "body\0".as_ptr() as *const i8);
                LLVMPositionBuilderAtEnd(self.builder, body);

                self.gen_stmt(f, entry, stmt);

                LLVMPositionBuilderAtEnd(self.builder, entry);
                LLVMBuildBr(self.builder, body);
            },

            TLD::TypeDecl { id, ty, name, .. } => {
                let ty = if let Type::Struct(tys) = ty {
                    let mut n: Vec<u8> = format!("struct.{}", name).into();
                    n.push(0);
                    unsafe {
                        let ty = LLVMStructCreateNamed(
                            LLVMGetModuleContext(self.module),
                            n.as_ptr() as *const i8,
                        );
                        let mut ltypes: Vec<LLVMTypeRef> =
                            tys.iter().map(|t| self.gen_type(t)).collect();
                        LLVMStructSetBody(ty, ltypes.as_mut_ptr(), ltypes.len() as u32, 0);
                        ty
                    }
                } else {
                    self.gen_type(ty)
                };
                self.type_tbl.insert(*id, ty);
            }

            _ => unreachable!(),
        }
    }

    /// Dump the module to stdout to be piped to the `clang` process to turn the IR
    /// into machine code.
    pub fn dump(&self) {
        unsafe {
            println!(
                "{}",
                std::ffi::CStr::from_ptr(LLVMPrintModuleToString(self.module))
                    .to_str()
                    .unwrap()
            );
        }
    }
}
