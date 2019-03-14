#![allow(non_camel_case_types)]

use std::fs::File;
use std::io::prelude::*;

pub trait ToBytes {
    fn to_bytes(&self) -> Vec<u8>;
}

#[derive(Debug, Clone)]
pub enum WebAssembly {
    MAGIC_NUMBER,
    VERSION_1,
    I32,
    I64,
    F32,
    F64,
    ANYFUNC,
    FUNC,
    EMPTY,
    SECTION_CUSTOM,
    SECTION_TYPE,
    SECTION_IMPORT,
    SECTION_FUNCTION,
    SECTION_TABLE,
    SECTION_MEMORY,
    SECTION_GLOBAL,
    SECTION_EXPORT,
    SECTION_START,
    SECTION_ELEMENT,
    SECTION_CODE,
    SECTION_DATA,
    UNREACHABLE,
    NOP,
    BLOCK,
    LOOP,
    IF,
    ELSE,
    END,
    BR,
    BR_IF,
    BR_TABLE,
    RETURN,
    CALL,
    CALL_INDIRECT,
    DROP,
    SELECT,
    LOCAL_GET,
    LOCAL_SET,
    LOCAL_TEE,
    GLOBAL_GET,
    GLOBAL_SET,
    I32_LOAD,
    I64_LOAD,
    F32_LOAD,
    F64_LOAD,
    I32_LOAD8_S,
    I32_LOAD8_U,
    I32_LOAD16_S,
    I32_LOAD16_U,
    I64_LOAD8_S,
    I64_LOAD8_U,
    I64_LOAD16_S,
    I64_LOAD16_U,
    I64_LOAD32_S,
    I64_LOAD32_U,
    I32_STORE,
    I64_STORE,
    F32_STORE,
    F64_STORE,
    I32_STORE8,
    I32_STORE16,
    I64_STORE8,
    I64_STORE16,
    I64_STORE32,
    CURRENT_MEMORY,
    GROW_MEMORY,
    I32_CONST,
    I64_CONST,
    F32_CONST,
    F64_CONST,
    I32_EQZ,
    I32_EQ,
    I32_NE,
    I32_LT_S,
    I32_LT_U,
    I32_GT_S,
    I32_GT_U,
    I32_LE_S,
    I32_LE_U,
    I32_GE_S,
    I32_GE_U,
    I64_EQZ,
    I64_EQ,
    I64_NE,
    I64_LT_S,
    I64_LT_U,
    I64_GT_S,
    I64_GT_U,
    I64_LE_S,
    I64_LE_U,
    I64_GE_S,
    I64_GE_U,
    F32_EQ,
    F32_NE,
    F32_LT,
    F32_GT,
    F32_LE,
    F32_GE,
    F64_EQ,
    F64_NE,
    F64_LT,
    F64_GT,
    F64_LE,
    F64_GE,
    I32_CLZ,
    I32_CTZ,
    I32_POPCNT,
    I32_ADD,
    I32_SUB,
    I32_MUL,
    I32_DIV_S,
    I32_DIV_U,
    I32_REM_S,
    I32_REM_U,
    I32_AND,
    I32_OR,
    I32_XOR,
    I32_SHL,
    I32_SHR_S,
    I32_SHR_U,
    I32_ROTL,
    I32_ROTR,
    I64_CLZ,
    I64_CTZ,
    I64_POPCNT,
    I64_ADD,
    I64_SUB,
    I64_MUL,
    I64_DIV_S,
    I64_DIV_U,
    I64_REM_S,
    I64_REM_U,
    I64_AND,
    I64_OR,
    I64_XOR,
    I64_SHL,
    I64_SHR_S,
    I64_SHR_U,
    I64_ROTL,
    I64_ROTR,
    F32_ABS,
    F32_NEG,
    F32_CEIL,
    F32_FLOOR,
    F32_TRUNC,
    F32_NEAREST,
    F32_SQRT,
    F32_ADD,
    F32_SUB,
    F32_MUL,
    F32_DIV,
    F32_MIN,
    F32_MAX,
    F32_COPYSIGN,
    F64_ABS,
    F64_NEG,
    F64_CEIL,
    F64_FLOOR,
    F64_TRUNC,
    F64_NEAREST,
    F64_SQRT,
    F64_ADD,
    F64_SUB,
    F64_MUL,
    F64_DIV,
    F64_MIN,
    F64_MAX,
    F64_COPYSIGN,
    I32_WRAP_F64,
    I32_TRUNC_S_F32,
    I32_TRUNC_U_F32,
    I32_TRUNC_S_F64,
    I32_TRUNC_U_F64,
    I64_EXTEND_S_I32,
    I64_EXTEND_U_I32,
    I64_TRUNC_S_F32,
    I64_TRUNC_U_F32,
    I64_TRUNC_S_F64,
    I64_TRUNC_U_F64,
    F32_CONVERT_S_I32,
    F32_CONVERT_U_I32,
    F32_CONVERT_S_I64,
    F32_CONVERT_U_I64,
    F32_DEMOTE_F64,
    F64_CONVERT_S_I32,
    F64_CONVERT_U_I32,
    F64_CONVERT_S_I64,
    F64_CONVERT_U_I64,
    F64_PROMOTE_F32,
    I32_REINTERPRET_F32,
    I64_REINTERPRET_F64,
    F32_REINTERPRET_I32,
    F64_REINTERPRET_I64,
    DESC_FUNCTION,
    DESC_TABLE,
    DESC_MEMORY,
    DESC_GLOBAL,
    LIMIT_MIN,
    LIMIT_MIN_MAX,
    IMMUTABLE,
    MUTABLE,
    RAW(Vec<u8>),
    EMPTY_VEC,
}

impl From<&str> for WebAssembly {
    fn from(s: &str) -> Self {
        str(s)
    }
}

impl From<&DataType> for WebAssembly {
    fn from(t: &DataType) -> Self {
        match *t {
            DataType::F32 => WebAssembly::F32,
            DataType::F64 => WebAssembly::F64,
            DataType::I32 => WebAssembly::I32,
            DataType::I64 => WebAssembly::I64,
        }
    }
}

impl From<i32> for WebAssembly {
    fn from(i: i32) -> Self {
        int(i)
    }
}

impl From<u32> for WebAssembly {
    fn from(i: u32) -> Self {
        uint(i)
    }
}

impl From<Vec<WebAssembly>> for WebAssembly {
    fn from(l: Vec<WebAssembly>) -> Self {
        let bytes: Vec<u8> = l
            .iter()
            .map(|x| x.to_bytes())
            .flat_map(|s| s.into_iter())
            .collect();
        WebAssembly::RAW(bytes)
    }
}

impl From<&Vec<WebAssembly>> for WebAssembly {
    fn from(l: &Vec<WebAssembly>) -> Self {
        let bytes: Vec<u8> = l
            .iter()
            .map(|x| x.to_bytes())
            .flat_map(|s| s.into_iter())
            .collect();
        WebAssembly::RAW(bytes)
    }
}

impl ToBytes for WebAssembly {
    fn to_bytes(&self) -> Vec<u8> {
        match self {
            WebAssembly::MAGIC_NUMBER => vec![0, 97, 115, 109],
            WebAssembly::VERSION_1 => vec![1, 0, 0, 0],
            WebAssembly::I32 => vec![127],
            WebAssembly::I64 => vec![126],
            WebAssembly::F32 => vec![125],
            WebAssembly::F64 => vec![124],
            WebAssembly::ANYFUNC => vec![112],
            WebAssembly::FUNC => vec![96],
            WebAssembly::EMPTY => vec![64],
            WebAssembly::SECTION_CUSTOM => vec![0],
            WebAssembly::SECTION_TYPE => vec![1],
            WebAssembly::SECTION_IMPORT => vec![2],
            WebAssembly::SECTION_FUNCTION => vec![3],
            WebAssembly::SECTION_TABLE => vec![4],
            WebAssembly::SECTION_MEMORY => vec![5],
            WebAssembly::SECTION_GLOBAL => vec![6],
            WebAssembly::SECTION_EXPORT => vec![7],
            WebAssembly::SECTION_START => vec![8],
            WebAssembly::SECTION_ELEMENT => vec![9],
            WebAssembly::SECTION_CODE => vec![10],
            WebAssembly::SECTION_DATA => vec![11],
            WebAssembly::UNREACHABLE => vec![0],
            WebAssembly::NOP => vec![1],
            WebAssembly::BLOCK => vec![2],
            WebAssembly::LOOP => vec![3],
            WebAssembly::IF => vec![4],
            WebAssembly::ELSE => vec![5],
            WebAssembly::END => vec![11],
            WebAssembly::BR => vec![12],
            WebAssembly::BR_IF => vec![13],
            WebAssembly::BR_TABLE => vec![14],
            WebAssembly::RETURN => vec![15],
            WebAssembly::CALL => vec![16],
            WebAssembly::CALL_INDIRECT => vec![17],
            WebAssembly::DROP => vec![26],
            WebAssembly::SELECT => vec![27],
            WebAssembly::LOCAL_GET => vec![32],
            WebAssembly::LOCAL_SET => vec![33],
            WebAssembly::LOCAL_TEE => vec![34],
            WebAssembly::GLOBAL_GET => vec![35],
            WebAssembly::GLOBAL_SET => vec![36],
            WebAssembly::I32_LOAD => vec![40],
            WebAssembly::I64_LOAD => vec![41],
            WebAssembly::F32_LOAD => vec![42],
            WebAssembly::F64_LOAD => vec![43],
            WebAssembly::I32_LOAD8_S => vec![44],
            WebAssembly::I32_LOAD8_U => vec![45],
            WebAssembly::I32_LOAD16_S => vec![46],
            WebAssembly::I32_LOAD16_U => vec![47],
            WebAssembly::I64_LOAD8_S => vec![48],
            WebAssembly::I64_LOAD8_U => vec![49],
            WebAssembly::I64_LOAD16_S => vec![50],
            WebAssembly::I64_LOAD16_U => vec![51],
            WebAssembly::I64_LOAD32_S => vec![52],
            WebAssembly::I64_LOAD32_U => vec![53],
            WebAssembly::I32_STORE => vec![54],
            WebAssembly::I64_STORE => vec![55],
            WebAssembly::F32_STORE => vec![56],
            WebAssembly::F64_STORE => vec![57],
            WebAssembly::I32_STORE8 => vec![58],
            WebAssembly::I32_STORE16 => vec![59],
            WebAssembly::I64_STORE8 => vec![60],
            WebAssembly::I64_STORE16 => vec![61],
            WebAssembly::I64_STORE32 => vec![62],
            WebAssembly::CURRENT_MEMORY => vec![63],
            WebAssembly::GROW_MEMORY => vec![64],
            WebAssembly::I32_CONST => vec![65],
            WebAssembly::I64_CONST => vec![66],
            WebAssembly::F32_CONST => vec![67],
            WebAssembly::F64_CONST => vec![68],
            WebAssembly::I32_EQZ => vec![69],
            WebAssembly::I32_EQ => vec![70],
            WebAssembly::I32_NE => vec![71],
            WebAssembly::I32_LT_S => vec![72],
            WebAssembly::I32_LT_U => vec![73],
            WebAssembly::I32_GT_S => vec![74],
            WebAssembly::I32_GT_U => vec![75],
            WebAssembly::I32_LE_S => vec![76],
            WebAssembly::I32_LE_U => vec![77],
            WebAssembly::I32_GE_S => vec![78],
            WebAssembly::I32_GE_U => vec![79],
            WebAssembly::I64_EQZ => vec![80],
            WebAssembly::I64_EQ => vec![81],
            WebAssembly::I64_NE => vec![82],
            WebAssembly::I64_LT_S => vec![83],
            WebAssembly::I64_LT_U => vec![84],
            WebAssembly::I64_GT_S => vec![85],
            WebAssembly::I64_GT_U => vec![86],
            WebAssembly::I64_LE_S => vec![87],
            WebAssembly::I64_LE_U => vec![88],
            WebAssembly::I64_GE_S => vec![89],
            WebAssembly::I64_GE_U => vec![90],
            WebAssembly::F32_EQ => vec![91],
            WebAssembly::F32_NE => vec![92],
            WebAssembly::F32_LT => vec![93],
            WebAssembly::F32_GT => vec![94],
            WebAssembly::F32_LE => vec![95],
            WebAssembly::F32_GE => vec![96],
            WebAssembly::F64_EQ => vec![97],
            WebAssembly::F64_NE => vec![98],
            WebAssembly::F64_LT => vec![99],
            WebAssembly::F64_GT => vec![100],
            WebAssembly::F64_LE => vec![101],
            WebAssembly::F64_GE => vec![102],
            WebAssembly::I32_CLZ => vec![103],
            WebAssembly::I32_CTZ => vec![104],
            WebAssembly::I32_POPCNT => vec![105],
            WebAssembly::I32_ADD => vec![106],
            WebAssembly::I32_SUB => vec![107],
            WebAssembly::I32_MUL => vec![108],
            WebAssembly::I32_DIV_S => vec![109],
            WebAssembly::I32_DIV_U => vec![110],
            WebAssembly::I32_REM_S => vec![111],
            WebAssembly::I32_REM_U => vec![112],
            WebAssembly::I32_AND => vec![113],
            WebAssembly::I32_OR => vec![114],
            WebAssembly::I32_XOR => vec![115],
            WebAssembly::I32_SHL => vec![116],
            WebAssembly::I32_SHR_S => vec![117],
            WebAssembly::I32_SHR_U => vec![118],
            WebAssembly::I32_ROTL => vec![119],
            WebAssembly::I32_ROTR => vec![120],
            WebAssembly::I64_CLZ => vec![121],
            WebAssembly::I64_CTZ => vec![122],
            WebAssembly::I64_POPCNT => vec![123],
            WebAssembly::I64_ADD => vec![124],
            WebAssembly::I64_SUB => vec![125],
            WebAssembly::I64_MUL => vec![126],
            WebAssembly::I64_DIV_S => vec![127],
            WebAssembly::I64_DIV_U => vec![128],
            WebAssembly::I64_REM_S => vec![129],
            WebAssembly::I64_REM_U => vec![130],
            WebAssembly::I64_AND => vec![131],
            WebAssembly::I64_OR => vec![132],
            WebAssembly::I64_XOR => vec![133],
            WebAssembly::I64_SHL => vec![134],
            WebAssembly::I64_SHR_S => vec![135],
            WebAssembly::I64_SHR_U => vec![136],
            WebAssembly::I64_ROTL => vec![137],
            WebAssembly::I64_ROTR => vec![138],
            WebAssembly::F32_ABS => vec![139],
            WebAssembly::F32_NEG => vec![140],
            WebAssembly::F32_CEIL => vec![141],
            WebAssembly::F32_FLOOR => vec![142],
            WebAssembly::F32_TRUNC => vec![143],
            WebAssembly::F32_NEAREST => vec![144],
            WebAssembly::F32_SQRT => vec![145],
            WebAssembly::F32_ADD => vec![146],
            WebAssembly::F32_SUB => vec![147],
            WebAssembly::F32_MUL => vec![148],
            WebAssembly::F32_DIV => vec![149],
            WebAssembly::F32_MIN => vec![150],
            WebAssembly::F32_MAX => vec![151],
            WebAssembly::F32_COPYSIGN => vec![152],
            WebAssembly::F64_ABS => vec![153],
            WebAssembly::F64_NEG => vec![154],
            WebAssembly::F64_CEIL => vec![155],
            WebAssembly::F64_FLOOR => vec![156],
            WebAssembly::F64_TRUNC => vec![157],
            WebAssembly::F64_NEAREST => vec![158],
            WebAssembly::F64_SQRT => vec![159],
            WebAssembly::F64_ADD => vec![160],
            WebAssembly::F64_SUB => vec![161],
            WebAssembly::F64_MUL => vec![162],
            WebAssembly::F64_DIV => vec![163],
            WebAssembly::F64_MIN => vec![164],
            WebAssembly::F64_MAX => vec![165],
            WebAssembly::F64_COPYSIGN => vec![166],
            WebAssembly::I32_WRAP_F64 => vec![167],
            WebAssembly::I32_TRUNC_S_F32 => vec![168],
            WebAssembly::I32_TRUNC_U_F32 => vec![169],
            WebAssembly::I32_TRUNC_S_F64 => vec![170],
            WebAssembly::I32_TRUNC_U_F64 => vec![171],
            WebAssembly::I64_EXTEND_S_I32 => vec![172],
            WebAssembly::I64_EXTEND_U_I32 => vec![173],
            WebAssembly::I64_TRUNC_S_F32 => vec![174],
            WebAssembly::I64_TRUNC_U_F32 => vec![175],
            WebAssembly::I64_TRUNC_S_F64 => vec![176],
            WebAssembly::I64_TRUNC_U_F64 => vec![177],
            WebAssembly::F32_CONVERT_S_I32 => vec![178],
            WebAssembly::F32_CONVERT_U_I32 => vec![179],
            WebAssembly::F32_CONVERT_S_I64 => vec![180],
            WebAssembly::F32_CONVERT_U_I64 => vec![181],
            WebAssembly::F32_DEMOTE_F64 => vec![182],
            WebAssembly::F64_CONVERT_S_I32 => vec![183],
            WebAssembly::F64_CONVERT_U_I32 => vec![184],
            WebAssembly::F64_CONVERT_S_I64 => vec![185],
            WebAssembly::F64_CONVERT_U_I64 => vec![186],
            WebAssembly::F64_PROMOTE_F32 => vec![187],
            WebAssembly::I32_REINTERPRET_F32 => vec![188],
            WebAssembly::I64_REINTERPRET_F64 => vec![189],
            WebAssembly::F32_REINTERPRET_I32 => vec![190],
            WebAssembly::F64_REINTERPRET_I64 => vec![191],
            WebAssembly::DESC_FUNCTION => vec![0],
            WebAssembly::DESC_TABLE => vec![1],
            WebAssembly::DESC_MEMORY => vec![2],
            WebAssembly::DESC_GLOBAL => vec![3],
            WebAssembly::LIMIT_MIN => vec![0],
            WebAssembly::LIMIT_MIN_MAX => vec![1],
            WebAssembly::IMMUTABLE => vec![0],
            WebAssembly::MUTABLE => vec![1],
            WebAssembly::RAW(v) => v.to_vec(),
            WebAssembly::EMPTY_VEC => vec![0],
        }
    }
}

pub fn uint(v: u32) -> WebAssembly {
    let mut value = v;
    let mut bytes: Vec<u8> = vec![];
    while {
        let mut byte = (value & 0x7F) as u8;
        value = value >> 0x07;
        if value != 0 {
            byte = byte | 0x80;
        }
        bytes.push(byte);
        value != 0
    } {}
    WebAssembly::RAW(bytes)
}

pub fn int(v: i32) -> WebAssembly {
    let mut value = v;
    let mut bytes: Vec<u8> = vec![];
    let size = (value as f32).abs().log2().ceil() as i32;
    let negative = value < 0;
    let mut more = true;
    while more {
        let mut byte = (value & 127) as u8;
        value = value >> 7;

        if negative {
            value = value | (-(1 << (size - 7)));
        }

        if (value == 0 && ((byte & 0x40) == 0)) || (value == -1 && ((byte & 0x40) == 0x40)) {
            more = false;
        } else {
            byte = byte | 128;
        }

        bytes.push(byte);
    }
    WebAssembly::RAW(bytes)
}

pub fn byte_count(wasm: Vec<WebAssembly>) -> WebAssembly {
    let mut bytes = flatten(&wasm);
    let mut len_bytes = uint(bytes.len() as u32).to_bytes();
    len_bytes.append(&mut bytes);
    WebAssembly::RAW(len_bytes)
}

impl ToBytes for Vec<WebAssembly> {
    fn to_bytes(&self) -> Vec<u8> {
        self.iter().fold(vec![], |mut all, val| {
            let mut mv = val.to_bytes();
            all.append(&mut mv);
            all
        })
    }
}

pub fn count<T>(wasm: Vec<T>) -> WebAssembly
where
    T: ToBytes,
{
    let mut bytes: Vec<u8> = wasm.iter().fold(vec![], |mut all, val| {
        let mut mv = val.to_bytes();
        all.append(&mut mv);
        all
    });
    let mut len_bytes = uint(wasm.len() as u32).to_bytes();
    len_bytes.append(&mut bytes);
    WebAssembly::RAW(len_bytes)
}

pub fn str(a: &str) -> WebAssembly {
    let mut bytes = String::from(a).into_bytes();
    let mut l = uint(a.len() as u32).to_bytes();
    l.append(&mut bytes);
    WebAssembly::RAW(l)
}

pub fn flatten(wasm: &Vec<WebAssembly>) -> Vec<u8> {
    wasm.iter()
        .map(|x| x.to_bytes())
        .collect::<Vec<Vec<u8>>>()
        .iter()
        .fold(vec![], |mut bytes, val| {
            let mut mv = val.clone();
            bytes.append(&mut mv);
            bytes
        })
}

#[derive(Debug, Clone,Eq,PartialEq)]
pub enum DataType {
    I32,
    I64,
    F32,
    F64,
}

#[derive(Debug)]
pub struct Function {
    name: Option<String>,
    inputs: Option<Vec<DataType>>,
    output: Option<DataType>,
    locals: Vec<DataType>,
    instructions: Vec<WebAssembly>,
}

#[derive(Debug)]
pub struct Table {
    min: u32,
    max: u32,
}

impl Table {
    pub fn new(min:u32,max:u32) -> Table{
        Table {
            min:min,
            max:max,
        }
    }
}

impl Function {
    pub fn new() -> Self {
        return Function {
            name: None,
            inputs: None,
            output: None,
            locals: vec![],
            instructions: vec![],
        };
    }

    pub fn with_name(&mut self, n: &str) {
        self.name = Some(String::from(n));
    }

    pub fn with_inputs(&mut self, n: Vec<DataType>) {
        self.inputs = Some(n);
    }

    pub fn with_output(&mut self, n: DataType) {
        self.output = Some(n);
    }

    pub fn with_local(&mut self, n: DataType) {
        self.locals.push(n);
    }

    pub fn with_instructions(&mut self, n: Vec<WebAssembly>) {
        self.instructions.extend_from_slice(&n);
    }
}

#[derive(Debug)]
pub struct Data {
    memory: u32,
    offset: i32,
    data: Vec<u8>,
}

impl Data {
    pub fn new(offset: i32, data: Vec<u8>) -> Self {
        return Data {
            memory: 0,
            offset: offset,
            data: data,
        };
    }
}

#[derive(Debug)]
pub enum Import {
    ImportFunction(ImportFunction),
    ImportTable(ImportTable),
    ImportMemory(ImportMemory)
}

#[derive(Debug)]
pub struct ImportTable {
    name: String,
    idx: u32,
}

impl ImportTable {
    pub fn new(name: String, idx:u32) -> Self {
        return ImportTable {
            name: name,
            idx:idx
        };
    }
}

#[derive(Debug)]
pub struct ImportMemory {
    name: String,
    idx: u32,
}

impl ImportMemory {
    pub fn new(name: String, idx:u32) -> Self {
        return ImportMemory {
            name: name,
            idx:idx
        };
    }
}

#[derive(Debug)]
pub struct ImportFunction {
    name: String,
    inputs: Vec<DataType>,
    output: Option<DataType>,
}

impl ImportFunction {
    pub fn new(name: String, inputs: Vec<DataType>, output: Option<DataType>) -> Self {
        return ImportFunction {
            name: name,
            inputs: inputs,
            output: output,
        };
    }
}

#[derive(Debug,Eq,PartialEq)]
pub struct FunctionType {
    inputs: Vec<DataType>,
    output: Option<DataType>,
}

impl FunctionType {
    pub fn new(inputs: Vec<DataType>, output: Option<DataType>) -> Self {
        return FunctionType {
            inputs: inputs,
            output: output,
        };
    }
}

#[derive(Debug)]
pub struct Global {
    value: i32,
    editable: bool,
}

impl Global {
    pub fn new(value: i32, editable: bool) -> Self {
        return Global {
            value: value,
            editable: editable,
        };
    }
}

#[derive(Debug)]
pub struct App {
    types: Vec<FunctionType>,
    imports: Vec<Import>,
    functions: Vec<Function>,
    tables: Vec<Table>,
    data: Vec<Data>,
    globals: Vec<Global>,
}

impl App {
    pub fn new(imports: Vec<Import>) -> Self {
        return App {
            types: vec![],
            imports: imports,
            functions: vec![],
            data: vec![],
            globals: vec![],
            tables: vec![]
        };
    }

    pub fn add_type(&mut self, f: FunctionType) -> u32 {
        for i in 0..self.types.len(){
            let t = &self.types[i];
            if t.output == f.output && t.inputs == f.inputs {
                return i as u32;
            }
        }
        self.types.push(f);
        (self.types.len() - 1) as u32
    }

    pub fn add_function(&mut self, f: Function) -> u32 {
        self.functions.push(f);
        (self.imports.len() + self.functions.len() - 1) as u32
    }

    pub fn add_data(&mut self, g: Data) -> u32 {
        self.data.push(g);
        (self.data.len() - 1) as u32
    }

    pub fn add_global(&mut self, g: Global) {
        self.globals.push(g)
    }

    pub fn add_table(&mut self, g: Table) {
        self.tables.push(g)
    }

    pub fn write_to_file(&self, file_name: &str) -> Result<(), std::io::Error> {
        File::create(file_name)?.write(&self.to_bytes())?;
        Ok(())
    }

    pub fn get_function_index(&self, func: &Function) -> i32 {
        self.functions
            .iter()
            .position(|r| r as *const _ == func as *const _)
            .unwrap() as i32
    }
}

fn bytevec(l: &Vec<WebAssembly>) -> WebAssembly {
    let content_bytes: Vec<u8> = l
        .iter()
        .map(|x| x.to_bytes())
        .flat_map(|s| s.into_iter())
        .collect();
    let mut bytes = uint(content_bytes.len() as u32).to_bytes();
    bytes.extend_from_slice(&content_bytes);
    WebAssembly::RAW(bytes)
}

pub fn vec(c: &Vec<WebAssembly>) -> WebAssembly {
    let content_bytes: Vec<u8> = c
        .iter()
        .map(|x| x.to_bytes())
        .flat_map(|s| s.into_iter())
        .collect();
    let mut bytes =uint(c.len() as u32).to_bytes();
    bytes.extend_from_slice(&content_bytes);
    WebAssembly::RAW(bytes)
}

fn section(c: Vec<WebAssembly>) -> WebAssembly {
    bytevec(&vec![vec(&c)])
}

impl ToBytes for App {
    fn to_bytes(&self) -> Vec<u8> {
        let mut signatures = vec![];
        let mut tables = vec![];
        for i in 0..self.tables.len() {
            let table = &self.tables[i];
            tables.push(vec![WebAssembly::ANYFUNC,WebAssembly::LIMIT_MIN_MAX,uint(table.min).into(),uint(table.max).into()].into())
        }


        let mut functions = vec![];
        let mut code = vec![];
        let mut exports = vec![vec!["memory".into(), WebAssembly::DESC_MEMORY, 0.into()].into()];

        for i in 0..self.types.len() {
            let function_type = &self.types[i];
            let function_inputs = (&function_type.inputs)
                .into_iter()
                .map(|x| x.into())
                .collect::<Vec<WebAssembly>>();
            let mut function_outputs = vec![];
            if let Some(x) = &function_type.output {
                function_outputs = vec![x.into()];
            }
            let sig = vec![
                WebAssembly::FUNC,
                vec(&function_inputs),
                vec(&function_outputs),
            ]
            .into();
            signatures.push(sig);
        }

        for i in 0..self.functions.len() {
            let function = &self.functions[i];

            let mut sig_idx = None;
            for i in 0..self.types.len(){
                let t = &self.types[i];
                if t.output == function.output && t.inputs == function.inputs.clone().unwrap_or(vec![]) {
                    sig_idx = Some(i as u32);
                }
            }
            if sig_idx.is_none() {
                let mut function_inputs = vec![];
                if let Some(x) = &function.inputs {
                    function_inputs = x
                        .into_iter()
                        .map(|x| x.into())
                        .collect::<Vec<WebAssembly>>()
                }
                let mut function_outputs = vec![];
                if let Some(x) = &function.output {
                    function_outputs = vec![x.into()];
                }
                let sig = vec![
                    WebAssembly::FUNC,
                    vec(&function_inputs),
                    vec(&function_outputs),
                ]
                .into();
                signatures.push(sig);
                sig_idx = Some((signatures.len() - 1) as u32)
            }
            functions.push(sig_idx.unwrap().into());
            if let Some(x) = &function.name {
                exports.push(
                    vec![
                        x.as_str().into(),
                        WebAssembly::DESC_FUNCTION,
                        ((self.imports.len() + functions.len() - 1) as u32).into(),
                    ]
                    .into(),
                )
            }
            let mut locs: Vec<WebAssembly> = vec![];
            for j in 0..function.locals.len() {
                locs.push(vec![1.into(), (&function.locals[j]).into()].into())
            }
            let mut c = vec![vec(&locs).into()];
            c.extend_from_slice(&function.instructions);
            code.push(bytevec(&c));
        }

        let mut data = vec![];
        for i in 0..self.data.len() {
            let d = &self.data[i];
            data.push(
                vec![
                    d.memory.into(),
                    WebAssembly::I32_CONST,
                    d.offset.into(),
                    WebAssembly::END,
                    bytevec(&vec![WebAssembly::RAW(d.data.clone())]),
                ]
                .into(),
            )
        }

        let mut imports = vec![];
        for i in 0..self.imports.len() {
            if let Import::ImportFunction(import) = &self.imports[i] {
                let function_inputs = import
                    .inputs
                    .iter()
                    .map(|x| x.into())
                    .collect::<Vec<WebAssembly>>();
                let mut function_outputs = vec![];
                if let Some(x) = &import.output {
                    function_outputs = vec![x.into()];
                }
                let sig = vec![
                    WebAssembly::FUNC,
                    vec(&function_inputs),
                    vec(&function_outputs),
                ]
                .into();
                signatures.push(sig);
                imports.push(
                    vec![
                        str("env"),
                        str(&import.name),
                        WebAssembly::DESC_FUNCTION,
                        ((signatures.len() - 1) as u32).into(),
                    ]
                    .into(),
                )
            } else if let Import::ImportTable(import) = &self.imports[i] {
                imports.push(
                    vec![
                        str("env"),
                        str(&import.name),
                        WebAssembly::DESC_TABLE,
                        import.idx.into(),
                    ]
                    .into(),
                )
            } else if let Import::ImportMemory(import) = &self.imports[i] {
                imports.push(
                    vec![
                        str("env"),
                        str(&import.name),
                        WebAssembly::DESC_MEMORY,
                        import.idx.into(),
                    ]
                    .into(),
                )
            }
        }

        let mut globals = vec![];
        for i in 0..self.globals.len() {
            let global = &self.globals[i];
            let mut global_type = WebAssembly::IMMUTABLE;
            if global.editable {
                global_type = WebAssembly::MUTABLE
            }
            globals.push(
                vec![
                    WebAssembly::I32,
                    global_type,
                    WebAssembly::I32_CONST,
                    global.value.into(),
                    WebAssembly::END,
                ]
                .into(),
            )
        }

        flatten(&vec![
            WebAssembly::MAGIC_NUMBER,
            WebAssembly::VERSION_1,
            WebAssembly::SECTION_TYPE,
            section(signatures),
            WebAssembly::SECTION_IMPORT,
            section(imports),
            WebAssembly::SECTION_FUNCTION,
            section(functions),
            WebAssembly::SECTION_TABLE,
            section(tables),
            WebAssembly::SECTION_MEMORY,
            section(
                vec![vec![WebAssembly::LIMIT_MIN_MAX, 2u32.into(), 10u32.into()].into()].into(),
            ),
            WebAssembly::SECTION_GLOBAL,
            section(globals),
            WebAssembly::SECTION_EXPORT,
            section(exports),
            WebAssembly::SECTION_CODE,
            section(code),
            WebAssembly::SECTION_DATA,
            section(data),
        ])
    }
}

fn cmd(c: Vec<WebAssembly>) -> WebAssembly {
    return WebAssembly::RAW(flatten(&c));
}

pub fn i32_const(i: i32) -> WebAssembly {
    return cmd(vec![WebAssembly::I32_CONST, int(i)]);
}
