/// Describes the location of a token within the source file. Use for
/// error output.
#[derive(Debug, Clone, Copy)]
pub struct TokenLoc {
    /// Line in the file. 1 is the first line; 0 means EOF
    pub line: usize,
    /// Column within the line. 1 is the first column; 0 means EOF
    pub col: usize,
    /// Length of the token
    pub len: usize,
}

impl TokenLoc {
    /// Creates a TokenLoc describing the end of the file.
    pub fn eof() -> Self {
        Self {
            line: 0,
            col: 0,
            len: 0,
        }
    }

    /// Tests if a given TokenLoc describes the end of the file.
    #[inline]
    pub fn is_eof(&self) -> bool {
        self.line == 0
    }
}

/// Tokens hold information about each "word" of a source file. The lexer takes
/// text from an input file and splits it up into these Tokens.
///
/// Tokens are implemented as a Rust enum, which presents a way of expressing
/// inherentance; each `Token` enum variant inherits from the `Token` enum.
///
/// Rust's enums provide advantages over conventional object inherentance
/// such as advanced pattern matching capabilities useful for compilers. Below
/// illustrates this.
///
/// ```
/// let x: Token;
///
/// match x {
///     Word(s) => println!("I'm the word '{}'", s),
///     Int(i) => println!("I'm the number '{}'", s),
///     _ => println!("I'm something else")
/// }
///
/// ```
///
/// This kind of expression makes checking what tokens come next in the parser
/// very simple and concise.
#[derive(Debug, Clone)]
pub enum Token {
    /// Represents a word like token such as an identifier `x` or a key word
    /// `var` or `fn`. Keywords in SGL are "soft" meaning keywords can be used
    /// as identifiers.
    Word(String),
    /// Represents punctuation type tokens such as `{` and `:`
    Punct(String),
    /// Represents a string literal such as `"test"`
    Str(String),
    /// Represents a real literal such as `0.1`
    _Float(f64),
    /// Represents a character literal such as `'x'`
    Char(char),
    /// Represents an integer literal such as `100`
    Int(u64),
    /// Represents an 8 bit unsigned integer literal
    U8(u8),
    /// Represents an 16 bit unsigned integer literal
    U16(u16),
    /// Represents an 32 bit unsigned integer literal
    U32(u32),
    /// Represents an 64 bit unsigned integer literal
    U64(u64),
    /// Represents an 8 bit signed integer literal
    I8(i8),
    /// Represents an 16 bit signed integer literal
    I16(i16),
    /// Represents an 32 bit signed integer literal
    I32(i32),
    /// Represents an 64 bit signed integer literal
    I64(i64),
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Word(s) | Token::Punct(s) | Token::Str(s) => {
                write!(f, "'{}'", s)
            }
            Token::_Float(float) => write!(f, "'{}'", float),
            Token::Int(i) => write!(f, "'{}'", i),
            Token::Char(c) => write!(f, "'{}'", c),
            Token::U8(i) => write!(f, "'{}'", i),
            Token::U16(i) => write!(f, "'{}'", i),
            Token::U32(i) => write!(f, "'{}'", i),
            Token::U64(i) => write!(f, "'{}'", i),
            Token::I8(i) => write!(f, "'{}'", i),
            Token::I16(i) => write!(f, "'{}'", i),
            Token::I32(i) => write!(f, "'{}'", i),
            Token::I64(i) => write!(f, "'{}'", i),
        }
    }
}

/// Represents a Type node in the syntax tree. Each enum variant represents a
/// different type of type in the language. For example `Type::I8` represents
/// `i8`, an 8 bit signed integer and `Type::Ptr(box Type::U32)` represents a
/// `*u32`,  a pointer to a 32 bit unsigned integer in memory.
#[derive(Debug, Clone)]
pub enum Type {
    Bool,
    I8,
    U8,
    I16,
    U16,
    I32,
    U32,
    I64,
    U64,
    _F16,
    F32,
    _F64,
    Vec2,
    Vec3,
    Vec4,
    Ptr(Box<Type>),
    Func(Box<Type>, Vec<(usize, Type)>),
    NFunc(Box<Type>, Vec<(String, Type)>),
    Struct(Vec<Type>),
    NStruct(Vec<(String, Type)>),
    Id(usize),
    NId(TokenLoc, String),
    TId(usize, Box<Type>),
    Arr(Box<Type>),
    Void,
}

/// Represents the different types of shaders processed by the GPU. A vertex
/// shader handles performing mathematical operations on vertex data such as
/// transforming and projecting vertices. It controls the position of each
/// polygon on the screen. A fragment shader handles processing for each
/// individual pixel on the screen. IT's used for lighting of 3d objects.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShaderType {
    Vert,
    Frag,
}

/// Represents an expression node in the abstract syntax tree. an expression
/// is an operation that evaluates to a value For example:
/// `Expr::Add(box Expr::U8(1), box Expr::U8(2))` represents `1 + 2`, adding
/// the two numbers one and two together.
#[derive(Debug, Clone)]
pub enum Expr {
    Bool(bool),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    Int(u64),
    F32(f32),
    Vec2(Box<Expr>, Box<Expr>),
    Vec3(Box<Expr>, Box<Expr>, Box<Expr>),
    Vec4(Box<Expr>, Box<Expr>, Box<Expr>, Box<Expr>),
    Struct(Vec<Expr>), // TODO: why two different enum variants
    TStruct(Type, Vec<Expr>),
    Arr(Type, Vec<Expr>),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Mod(Box<Expr>, Box<Expr>),
    LT(Box<Expr>, Box<Expr>),
    GT(Box<Expr>, Box<Expr>),
    NId(TokenLoc, String),
    Id(usize),
    TId(Type, usize),
    Ref(Box<Expr>),   // &
    Deref(Box<Expr>), // *
    Call(Box<Expr>, Vec<Expr>),
    ArrSub(Box<Expr>, Box<Expr>), // a[b]
    TCall(Type, Box<Expr>, Vec<Expr>),
    Null,
    TNull(Type),
    Member(Box<Expr>, usize, String),
    NMember(TokenLoc, Box<Expr>, String),
    Shader(ShaderType, Vec<TLD>),
    Not(Box<Expr>),
}

impl Expr {
    pub fn ty<'a>(&'a self) -> Type {
        match self {
            Expr::Bool(bool) => Type::Bool,
            Expr::U8(u8) => Type::U8,
            Expr::U16(u16) => Type::U16,
            Expr::U32(u32) => Type::U32,
            Expr::U64(u64) => Type::U64,
            Expr::I8(i8) => Type::I8,
            Expr::I16(i16) => Type::I16,
            Expr::I32(i32) => Type::I32,
            Expr::I64(i64) => Type::I64,
            Expr::Struct(es) => Type::Struct(es.iter().map(|e| e.ty().clone()).collect()),
            Expr::TStruct(t, _) => t.clone(),
            Expr::Arr(t, ..) => Type::Arr(box t.clone()),
            Expr::Add(..) => todo!(),
            Expr::Sub(..) => todo!(),
            Expr::Mul(..) => todo!(),
            Expr::Mod(..) => todo!(),
            Expr::Div(..) => todo!(),
            Expr::Id(..) => panic!(),
            Expr::TId(t, ..) => t.clone(),
            Expr::Ref(e) => Type::Ptr(box e.ty()),
            Expr::Deref(e) => {
                if let Type::Ptr(box t) = e.ty() {
                    t
                } else {
                    panic!();
                }
            }
            Expr::NId(..) => panic!(),
            Expr::Call(..) => panic!(),
            Expr::ArrSub(..) => panic!(),
            Expr::Null => panic!(),
            Expr::Int(..) => panic!(),
            Expr::TNull(t) => t.clone(),
            Expr::TCall(t, ..) => t.clone(),
            Expr::Member(..) => todo!(),
            Expr::NMember(..) => todo!(),
            Expr::Vec2(..) => Type::Vec2,
            Expr::Vec3(..) => Type::Vec3,
            Expr::Vec4(..) => Type::Vec4,
            Expr::F32(..) => Type::F32,
            Expr::Shader(..) => Type::Arr(box Type::U32),
            Expr::Not(..) => Type::Bool,
            Expr::GT(..) => Type::Bool,
            Expr::LT(..) => Type::Bool,
        }
    }
}

/// Represents a statement node in the syntax tree. A statement is
/// an operation that performs an action as oposed to yeilding a value like
/// expressions do; statements are parent to expressions in the tree.
/// For example, `Expr::Ass(Expr::NId("x"), Expr::I32(1))` represents
/// `x = 1;`, assigning 1 to the varable x.
#[derive(Debug, Clone)]
pub enum Stmt {
    /// Represents a single expression as a statement such as `1;`, `test();`
    /// or `2 + 2;`
    Expr(Expr),
    /// Represents an assignment such as `x = 1;`
    Ass { loc: Expr, val: Expr },
    /// Represents a variable declaration such as `var x: u32 = 10;`
    /// (with internal compiler identifier metadata)
    VarDecl {
        id: usize,
        name: String,
        ty: Type,
        val: Expr,
    },
    /// Represents a variable declaration such as `var x: u32 = 10;`
    NVarDecl { name: String, ty: Type, val: Expr },
    /// Represents a block of statements such as `{ test(); ret 1+1; }`
    Block(Vec<Stmt>),
    /// Represents an if statement such as `if (true) { test(); } else { ret 1+1; }`
    If(Expr, Box<Stmt>, Option<Box<Stmt>>),
    /// Represents a while loop such as `while (true) { puts("hello".ptr); }`
    While(Expr, Box<Stmt>),
    /// Represents a return from a function, yeilding a value if not void.
    /// for example `ret 10;` or `ret;` for void functions.
    Ret(Option<Expr>),
    /// Represents the null statement that does nothing: `;`
    Noop,
}

/// Represents a top level declaration in the syntax tree. A top level
/// declaration defines the layout of source files. It's where type aliases
/// are defined, function declarations and definitions (implementations) are
/// described  and where global constants are defined. For example
/// `TLD::NTypeDecl("test_type", Type::U32)`, represents `ty test_type: u32`,
/// creating an alias to the `u32` type used through the `test_type` identifier.
#[derive(Debug, Clone)]
pub enum TLD {
    /// Represents a declaration of a function to link against (without an
    /// acompanying implementation) such as `fn puts(str: *u8): u32`
    NFuncDecl {
        loc: TokenLoc,
        name: String,
        ty: Type,
    },
    /// Represents a function with an implementation such as
    /// `fn main(): u32 { ret 100u32; }`
    NFunc {
        loc: TokenLoc,
        name: String,
        ty: Type,
        stmt: Stmt,
    },
    /// Represents a type declaration such as `ty type: u32`
    NTypeDecl {
        loc: TokenLoc,
        name: String,
        ty: Type,
    },

    /// Represents a declaration of a function to link against (without an
    /// acompanying implementation) such as `fn puts(str: *u8): u32`
    /// (with internal compiler identifier metadata)
    FuncDecl {
        loc: TokenLoc,
        id: usize,
        name: String,
        ty: Type,
    },
    /// Represents a function with an implementation such as
    /// `fn main(): u32 { ret 100u32; }`
    /// (with internal compiler identifier metadata)
    Func {
        loc: TokenLoc,
        id: usize,
        name: String,
        ty: Type,
        stmt: Stmt,
    },
    /// Represents a type declaration such as `ty type: u32`
    /// (with internal compiler identifier metadata)
    TypeDecl {
        loc: TokenLoc,
        id: usize,
        name: String,
        ty: Type,
    },

    /* Top level declarations specific to shaders */
    In(TokenLoc, usize, usize, String, Type),
    NIn(TokenLoc, usize, String, Type),
    Out(TokenLoc, usize, usize, String, Type),
    NOut(TokenLoc, usize, String, Type),
    Uniform(TokenLoc, usize, usize, String, Type),
    NUniform(TokenLoc, usize, String, Type),

    /* builtin messages sent by the shader generator to control the rest of the
     * compiler
     */
    NBuiltinVertPos,
    BuiltinVertPos(usize),
    NBuiltinVertPointSize,
    BuiltinVertPointSize(usize),
    NBuiltinVertClipDistance,
    BuiltinVertClipDistance(usize),
    NBuiltinVertCullDistance,
    BuiltinVertCullDistance(usize),
}

/// Represents a pass over the syntax tree, such as resolving identifiers and
/// checking types.
pub trait Pass {
    fn res(handler: &crate::err::ErrHandler, tlds: Vec<TLD>) -> Vec<TLD>;
}
