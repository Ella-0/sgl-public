use crate::util::*;
use std::collections::HashMap;

/// A helper enum that identifies different SPIR-V shader capabilities.
/// TODO: Would need to be expanded to support more than fragment and vertex
/// shaders.
#[repr(u32)]
pub enum Capability {
    Shader = 1,
}

/// Specifices the SPIR-V addressing mode to use. In most cases, logical is apropriate.
/// Other addressing modes are typically used in compute situations not graphics.
#[repr(u32)]
pub enum AddressingModel {
    Logical = 0,
}

/// Specifies the SPIR-V memory model to use
#[repr(u32)]
pub enum MemoryModel {
    Simple = 0,
}

#[repr(u32)]
pub enum ExecutionMode {
    OriginUpperLeft = 7
}

/// Specifies flags for SPIR-V functions
#[repr(u32)]
pub enum FunctionControl {
    None = 0,
}

/// Specifies SPIR-V builtin variables
#[repr(u32)]
pub enum Builtin {
    Position = 0,
    PointSize = 1,
    ClipDistance = 3,
    CullDistance = 4,
}

/// Specifies extra metadata for SPIR-V variables. `Builtin` for example
/// is for builtin variables such as `PointSize`
#[repr(u32)]
pub enum Decoration {
    Builtin = 11,
    Location = 30,
}

/// Specifies what an attribute is from. `Input` from the previous shader stage
/// `Uniform` extra information from the CPU, or `Output` to send to the next shader
/// stage.
#[repr(u32)]
pub enum StorageClass {
    Input = 1,
    Uniform = 2,
    Output = 3,
}

/// SPIR-V is an SSA IR. Each register is identified with a single 32 bit integer.
type SPVId = u32;

/// A helper object for buliding SPIR-V binary IR instructions with human
/// readable function names.
#[derive(Default)]
struct SPVBuilder {
    magic_words: [u32; 3],
    max_var_idx: u32,
    head_words: Vec<u32>,
    debug_words: Vec<u32>,
    decorate_words: Vec<u32>,
    data_words: Vec<u32>,
    text_words: Vec<u32>,

    void_ty: SPVId,
    float_ty: SPVId
}

/// Execution Model used by SPIR-V's OpEntryPoint meta instruction
#[repr(u32)]
pub enum ExecutionModel {
    Vertex = 0,
    Fragment = 4
}

impl SPVBuilder {
    /// Constructs a new SPIR-V builder object, filling words initially with the SPIR-V
    /// magic number.
    pub fn new() -> Self {
        let mut ret = Self {
            magic_words: [
                0x0723_0203,
                0x0001_0000,
                0x0000_0000,
            ],
            max_var_idx: 1,
            ..Self::default()
        };
        ret.init_void_ty();
        ret.init_float_ty();
        ret
    }

    fn init_void_ty(&mut self) {
        self.void_ty = self.alloc_id();
        self.data_words.push(0x0002_0013);
        self.data_words.push(self.void_ty);
    }

    fn init_float_ty(&mut self) {
        self.float_ty = self.alloc_id();
        self.data_words.push(0x0003_0016);
        self.data_words.push(self.float_ty);
        self.data_words.push(32);
    }

    /// Destructs the builder yeilding the SPIR-V code.
    pub fn pack(self) -> Vec<u32> {
        let mut out = Vec::new();
        out.extend_from_slice(&self.magic_words);
        out.push(self.max_var_idx);

        // From the SPIR-V spec:
        //     0 (Reserved for instruction schema, if needed.)

        out.push(0);
        out.extend_from_slice(&self.head_words);
        out.extend_from_slice(&self.debug_words);
        out.extend_from_slice(&self.decorate_words);
        out.extend_from_slice(&self.data_words);
        out.extend_from_slice(&self.text_words);
        out
    }

    pub fn op_execution_mode(&mut self, entry: SPVId, mode: ExecutionMode) {
        self.head_words.push(0x0003_0010);
        self.head_words.push(entry);
        self.head_words.push(mode as u32);
    }

    /// In the SPIR-V header, a value is listed as the max register identifier value used.
    /// We increment this throughout building the SPIR-V code; this function performs that
    /// action.
    fn inc_id_bounds(&mut self) {
        self.max_var_idx += 1;
    }

    /// Allocates a new register by incrementing the identifier bounds.
    fn alloc_id(&mut self) -> u32 {
        let ret = self.max_var_idx;
        self.inc_id_bounds();
        ret
    }

    /// Adds a name to a SPIR-V register to help with debugging.
    fn op_name(&mut self, name: &str) -> SPVId {
        let id = self.alloc_id();
        // return id;
        let mut bytes = name.bytes();
        let count = bytes.len() / 4 + 1;

        self.debug_words.push(0x0002_0005u32 + ((count as u32) << 16));
        self.debug_words.push(id);

        for _ in 0..count {
            // SPIR-V strings, like environments, are null terminated. hence `unwrap_or(0)`
            self.debug_words.push(u32::from_le_bytes([
                bytes.next().unwrap_or(0),
                bytes.next().unwrap_or(0),
                bytes.next().unwrap_or(0),
                bytes.next().unwrap_or(0),
            ]));
        }
        id
    }

    fn op_entry_point(&mut self, exec_model: ExecutionModel, entry: SPVId, name: &str, interface: &[SPVId]) {
        let mut bytes = name.bytes();
        let count = bytes.len() / 4 + 1;

        self.head_words.push(0x0003_000f + ((count as u32 + interface.len() as u32) << 16));
        self.head_words.push(exec_model as u32);
        self.head_words.push(entry);

        for _ in 0..count {
            self.head_words.push(u32::from_le_bytes([
                bytes.next().unwrap_or(0),
                bytes.next().unwrap_or(0),
                bytes.next().unwrap_or(0),
                bytes.next().unwrap_or(0),
            ]));
        }

        self.head_words.extend_from_slice(interface);
    }

    /// Decorates a SPIR-V identifier with metadata such as if it's a SPIR-V builtin.
    fn op_decorate(&mut self, target: SPVId, dec: Decoration, args: &[u32]) {
        self.decorate_words.push(0x0003_0047 + ((args.len() as u32) << 16));
        self.decorate_words.push(target);
        self.decorate_words.push(dec as u32);
        self.decorate_words.extend_from_slice(args);
    }

    /// Creates a new SPIR-V variable
    fn op_variable(
        &mut self,
        ty: SPVId,
        id: Option<SPVId>,
        storage_class: StorageClass,
        init: Option<SPVId>,
    ) -> SPVId {
        let id = id.unwrap_or_else(|| self.alloc_id());
        self.data_words
            .push(0x0004_003b + if init.is_some() { 1 } else { 0 });
        self.data_words.push(ty);
        self.data_words.push(id);
        self.data_words.push(storage_class as u32);
        if let Some(i) = init {
            self.data_words.push(i);
        }
        id
    }

    /// Creates a no op instruction.
    fn op_nop(&mut self) {
        self.text_words.push(0x0001_0000);
    }

    fn op_capability(&mut self, cap: Capability) {
        self.head_words.push(0x0002_0011);
        self.head_words.push(cap as u32);
    }

    fn op_memory_model(&mut self, addr: AddressingModel, mem: MemoryModel) {
        self.head_words.push(0x0003_000e);
        self.head_words.push(addr as u32);
        self.head_words.push(mem as u32);
    }

    fn op_function(
        &mut self,
        id: Option<SPVId>,
        ty: SPVId,
        func_cntrl: FunctionControl,
        fn_ty: SPVId,
    ) -> SPVId {
        let id = id.unwrap_or_else(|| self.alloc_id());
        self.text_words.push(0x0005_0036);
        self.text_words.push(ty);
        self.text_words.push(id);
        self.text_words.push(func_cntrl as u32);
        self.text_words.push(fn_ty);
        id
    }

    fn op_function_end(&mut self) {
        self.text_words.push(0x0001_0038);
    }

    fn op_type_void(&mut self) -> SPVId {
        self.void_ty
    }

    fn op_type_bool(&mut self) -> SPVId {
        let id = self.alloc_id();
        self.data_words.push(0x0002_0014);
        id
    }

    fn op_type_int(&mut self, size: u8, signed: bool) -> SPVId {
        let id = self.alloc_id();
        self.data_words.push(0x0004_0015);
        self.data_words.push(id);
        self.data_words.push(size as u32);
        self.data_words.push(signed as u32);
        id
    }

    fn op_type_float(&mut self, size: u8) -> SPVId {
        assert_eq!(size, 32);
        self.float_ty
    }

    fn op_type_vector(&mut self, ty: SPVId, dim: u32) -> SPVId {
        let id = self.alloc_id();
        self.data_words.push(0x0004_0017);
        self.data_words.push(id);
        self.data_words.push(ty);
        self.data_words.push(dim);
        id
    }

    fn op_type_pointer(&mut self, storage_class: StorageClass, ty: SPVId) -> SPVId {
        let id = self.alloc_id();
        self.data_words.push(0x0004_0020);
        self.data_words.push(id);
        self.data_words.push(storage_class as u32);
        self.data_words.push(ty);
        id
    }

    fn op_type_function(&mut self, ret_type: SPVId, param_types: &[SPVId]) -> SPVId {
        let id = self.alloc_id();
        self.data_words
            .push(0x0003_0021 + ((param_types.len() as u32) << 16));
        self.data_words.push(id);
        self.data_words.push(ret_type);
        param_types.iter().for_each(|p| self.data_words.push(*p));
        id
    }

    fn op_constant(&mut self, type_id: SPVId, val: &[u32]) -> SPVId {
        let id = self.alloc_id();
        self.data_words.push(0x0003_002B + ((val.len() as u32) << 16));
        self.data_words.push(type_id);
        self.data_words.push(id);
        self.data_words.extend_from_slice(val);
        id
    }

    fn op_constant_u8(&mut self, v: u8) -> SPVId {
        // TODO: move to initial creation of SPVBuilder
        let ty = self.op_type_int(8, false);
        self.op_constant(ty, &[v as u32])
    }

    fn op_constant_u16(&mut self, v: u16) -> SPVId {
        // TODO: move to initial creation of SPVBuilder
        let ty = self.op_type_int(16, false);
        self.op_constant(ty, &[v as u32])
    }

    fn op_constant_u32(&mut self, v: u32) -> SPVId {
        // TODO: move to initial creation of SPVBuilder
        let ty = self.op_type_int(32, false);
        self.op_constant(ty, &[v])
    }

    fn op_constant_u64(&mut self, v: u64) -> SPVId {
        // TODO: move to initial creation of SPVBuilder
        let ty = self.op_type_int(64, false);
        self.op_constant(ty, &[(v & (1 << 32 - 1)) as u32, (v >> 32) as u32])
    }

    fn op_constant_f32(&mut self, v: f32) -> SPVId {
        // TODO: move to initial creation of SPVBuilder
        let ty = self.op_type_float(32);
        self.op_constant(ty, &[v.to_bits()])
    }

    fn op_load(&mut self, ptr: SPVId, ty: SPVId) -> SPVId {
        let id = self.alloc_id();
        self.text_words.push(0x0004_003d);
        self.text_words.push(ty);
        self.text_words.push(id);
        self.text_words.push(ptr);
        id
    }

    fn op_store(&mut self, ptr: SPVId, val: SPVId) {
        self.text_words.push(0x0003_003e);
        self.text_words.push(ptr);
        self.text_words.push(val);
    }

    fn op_return(&mut self) {
        self.text_words.push(0x0001_00fd);
    }

    fn op_composite_construct(&mut self, ty: SPVId, parts: &[SPVId]) -> SPVId {
        let id = self.alloc_id();
        self.text_words.push(0x0003_0050 + ((parts.len() as u32) << 16));
        self.text_words.push(ty);
        self.text_words.push(id);
        self.text_words.extend_from_slice(parts);
        id
    }

    fn op_composite_extract(&mut self, ty: SPVId, composite: SPVId, is: &[u32]) -> SPVId {
        let id = self.alloc_id();
        self.text_words.push(0x0004_0051 + ((is.len() as u32) << 16));
        self.text_words.push(ty);
        self.text_words.push(id);
        self.text_words.push(composite);
        self.text_words.extend_from_slice(is);
        id
    }

    fn op_return_value(&mut self, v: SPVId) {
        self.text_words.push(0x0002_00fe);
        self.text_words.push(v);
    }

    fn op_label(&mut self, id: Option<SPVId>) -> SPVId {
        let id = id.unwrap_or_else(|| self.alloc_id());
        self.text_words.push(0x0002_00f8);
        self.text_words.push(id);
        id
    }
}

/// SPIR-V code generater for generating shader code for use by vulkan.
pub struct GenSPV {
    builder: SPVBuilder,
    ty: ShaderType,
    expr_tbl: HashMap<usize, SPVId>,

    // Type cache: allows us to only allocate some types once.
    // TODO: create a hashmap from `sgl::util::Type` to `SPVId`?
    // TODO: maybe cache calls of any `OpType`?
    ty_f16: Option<SPVId>,
    ty_f32: Option<SPVId>,
    ty_f64: Option<SPVId>,
    ty_vec2: Option<SPVId>,
    ty_vec3: Option<SPVId>,
    ty_vec4: Option<SPVId>,
}


impl GenSPV {
    pub fn new(ty: ShaderType) -> Self {
        let mut builder = SPVBuilder::new();
        builder.op_capability(Capability::Shader);
        builder.op_memory_model(AddressingModel::Logical, MemoryModel::Simple);

        Self {
            builder,
            ty,
            expr_tbl: HashMap::new(),
            ty_f16: None,
            ty_f32: None,
            ty_f64: None,
            ty_vec2: None,
            ty_vec3: None,
            ty_vec4: None,
        }
    }

    fn gen_type(&mut self, ty: &Type) -> SPVId {
        match ty {
            Type::Bool => self.builder.op_type_bool(),
            Type::Void => self.builder.op_type_void(),
            Type::Func(rt, pts) => {
                let rt = self.gen_type(rt);
                let pts = pts
                    .iter()
                    // Functional programming technique
                    .map(|(_, t)| self.gen_type(t))
                    .collect::<Vec<u32>>();
                self.builder.op_type_function(rt, &pts)
            }
            Type::U8 => self.builder.op_type_int(8, false),
            Type::U16 => self.builder.op_type_int(16, false),
            Type::U32 => self.builder.op_type_int(32, false),
            Type::U64 => self.builder.op_type_int(64, false),

            Type::I8 => self.builder.op_type_int(8, true),
            Type::I16 => self.builder.op_type_int(16, true),
            Type::I32 => self.builder.op_type_int(32, true),
            Type::I64 => self.builder.op_type_int(64, true),

            Type::_F16 => self.ty_f16.unwrap_or_else(|| {
                self.ty_f32 = Some(self.builder.op_type_float(16));
                self.ty_f32.unwrap()
            }),

            Type::F32 => self.ty_f32.unwrap_or_else(|| {
                self.ty_f32 = Some(self.builder.op_type_float(32));
                self.ty_f32.unwrap()
            }),

            Type::_F64 => self.ty_f64.unwrap_or_else(|| {
                self.ty_f32 = Some(self.builder.op_type_float(64));
                self.ty_f32.unwrap()
            }),

            Type::Vec2 => self.ty_vec2.unwrap_or_else(|| {
                let f_ty = self.gen_type(&Type::F32);
                self.ty_vec2 = Some(self.builder.op_type_vector(f_ty, 2));
                self.ty_vec2.unwrap()
            }),

            Type::Vec3 => self.ty_vec3.unwrap_or_else(|| {
                let f_ty = self.gen_type(&Type::F32);
                self.ty_vec3 = Some(self.builder.op_type_vector(f_ty, 3));
                self.ty_vec3.unwrap()
            }),

            Type::Vec4 => self.ty_vec4.unwrap_or_else(|| {
                let f_ty = self.gen_type(&Type::F32);
                self.ty_vec4 = Some(self.builder.op_type_vector(f_ty, 4));
                self.ty_vec4.unwrap()
            }),

            _ => panic!(),
        }
    }

    fn gen_lexpr(&mut self, expr: &Expr) -> SPVId {
        match expr {
            Expr::TId(_, id) => *self.expr_tbl.get(id).expect("undefined"),
            _ => panic!(),
        }
    }

    fn gen_expr(&mut self, expr: &Expr) -> SPVId {
        match expr {
            Expr::U8(x) => self.builder.op_constant_u8(*x),
            Expr::U16(x) => self.builder.op_constant_u16(*x),
            Expr::U32(x) => self.builder.op_constant_u32(*x),
            Expr::U64(x) => self.builder.op_constant_u64(*x),
            Expr::F32(x) => self.builder.op_constant_f32(*x),
            Expr::TId(ty, id) => {
                let id = *self.expr_tbl.get(id).expect("undefined");
                let ty = self.gen_type(ty);
                self.builder.op_load(id, ty)
            }
            Expr::Vec4(e0, e1, e2, e3) => {
                let e0 = self.gen_expr(e0);
                let e1 = self.gen_expr(e1);
                let e2 = self.gen_expr(e2);
                let e3 = self.gen_expr(e3);
                let ty = self.gen_type(&Type::Vec4);
                self.builder.op_composite_construct(ty, &[e0, e1, e2, e3])
            }
            Expr::Member(e, i, ..) => {
                /* TODO: work out what type to use */
                let ty = self.gen_type(&Type::F32);
                let e = self.gen_expr(e);
                self.builder.op_composite_extract(ty, e, &[*i as u32])
            }
            e => panic!("unimplemented expr {:#?}", e),
        }
    }

    fn gen_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Expr(e) => {
                self.gen_expr(e);
            }
            Stmt::Noop => self.builder.op_nop(),
            Stmt::Ret(Some(e)) => {
                let v = self.gen_expr(e);
                self.builder.op_return_value(v);
            }

            Stmt::Ret(None) => {
                self.builder.op_return();
            }

            Stmt::Block(v) => {
                v.iter().for_each(|v| self.gen_stmt(v));
            }

            Stmt::Ass { loc, val } => {
                let loc = self.gen_lexpr(loc);
                let val = self.gen_expr(val);
                self.builder.op_store(loc, val);
            }

            _ => panic!(),
        }
    }

    pub fn pregen_tld(&mut self, tld: &TLD, interfaces: &mut HashMap::<usize, SPVId>) {
        match tld {
            TLD::In(_, _, id, name, ..) |
            TLD::Out(_, _, id, name, ..) |
            TLD::Uniform(_, _, id, name, ..) => {
                let spv_id = self.builder.op_name(name);
                interfaces.insert(*id, spv_id);
            }
            TLD::BuiltinVertPos(id) => {
                let spv_id = self.builder.op_name("sgl_vert_pos");
                interfaces.insert(*id, spv_id);
            }

            TLD::BuiltinVertPointSize(id) => {
                let spv_id = self.builder.op_name("sgl_vert_point_size");
                interfaces.insert(*id, spv_id);
            }

            _ => {

            }
        }
    }

    pub fn gen_tld(&mut self, tld: &TLD, interfaces: &HashMap::<usize, SPVId>) {
        match tld {
            TLD::Func {
                id, name, ty, stmt, ..
            } => {
                let void_ty = self.builder.op_type_void();
                let ty = self.gen_type(ty);

                let spv_id = self.builder.op_name(name);

                if name == "main" {
                    let values: Vec<_> = interfaces.values().map(|a| *a).collect();
                    self.builder.op_entry_point(match self.ty {
                        ShaderType::Vert => ExecutionModel::Vertex,
                        ShaderType::Frag => ExecutionModel::Fragment
                    }, spv_id, "main", &values);
                    if self.ty == ShaderType::Frag {
                        self.builder.op_execution_mode(spv_id, ExecutionMode::OriginUpperLeft);
                    }
                }

                self.builder
                    .op_function(Some(spv_id), void_ty, FunctionControl::None, ty);

                self.builder.op_label(None);
                self.gen_stmt(stmt);

                self.builder.op_return();
                self.builder.op_function_end();
            }

            TLD::In(_, loc, id, name, ty) => {
                let spv_id = *interfaces.get(id).expect("Internal compiler error");
                self.builder
                    .op_decorate(spv_id, Decoration::Location, &[*loc as u32]);
                let base_ty = self.gen_type(ty);
                let ty = self.builder.op_type_pointer(StorageClass::Input, base_ty);
                let v = self.builder.op_variable(ty, Some(spv_id), StorageClass::Input, None);

                self.expr_tbl.insert(*id, v);
            }

            TLD::Out(_, loc, id, name, ty) => {
                let spv_id = *interfaces.get(id).expect("Internal compiler error");
                self.builder
                    .op_decorate(spv_id, Decoration::Location, &[*loc as u32]);
                let base_ty = self.gen_type(ty);
                let ty = self.builder.op_type_pointer(StorageClass::Output, base_ty);
                let v = self.builder.op_variable(ty, Some(spv_id), StorageClass::Output, None);

                self.expr_tbl.insert(*id, v);
            }

            TLD::Uniform(_, loc, id, name, ty) => {}

            TLD::BuiltinVertPos(id) => {
                let spv_id = *interfaces.get(id).expect("Internal compiler error");
                self.builder
                    .op_decorate(spv_id, Decoration::Builtin, &[Builtin::Position as u32]);
                let ty = self.gen_type(&Type::Vec4);
                let ty = self.builder.op_type_pointer(StorageClass::Output, ty);
                self.builder.op_variable(ty, Some(spv_id), StorageClass::Output, None);
                self.expr_tbl.insert(*id, spv_id);
            }

            TLD::BuiltinVertPointSize(id) => {
                let spv_id = *interfaces.get(id).expect("Internal compiler error");
                self.builder
                    .op_decorate(spv_id, Decoration::Builtin, &[Builtin::PointSize as u32]);
                let ty = self.gen_type(&Type::F32);
                let ty = self.builder.op_type_pointer(StorageClass::Output, ty);
                self.builder.op_variable(ty, Some(spv_id), StorageClass::Output, None);
                self.expr_tbl.insert(*id, spv_id);
            }
            _ => panic!(),
        }
    }

    pub fn gen(&mut self, tlds: &[TLD]) {
        let mut interfaces = HashMap::new();
        for tld in tlds {
            self.pregen_tld(tld, &mut interfaces);
        }
        for tld in tlds {
            self.gen_tld(tld, &interfaces);
        }
    }

    // TODO: restructure SPIR-V generation into several different list of words that
    // are all packed together at the end.


    /// Yeilds the SPIR-V instructions.
    pub fn pack(self) -> Vec<u32> {
        let x = self.builder.pack();
        if matches!(std::env::var("SGL_DEBUG_SPV"), Ok(x) if x == "1") {
            let mut bs = vec![];
            for w in &x {
                let b = w.to_le_bytes();
                bs.push(b[0]);
                bs.push(b[1]);
                bs.push(b[2]);
                bs.push(b[3]);
            }
            std::fs::write(match self.ty {
                ShaderType::Frag => "debug_frag.spv",
                ShaderType::Vert => "debug_vert.spv"
            }, bs).unwrap();
        }
        x
    }
}
