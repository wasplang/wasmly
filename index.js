'use strict';

Object.defineProperty(exports, "__esModule", {
  value: true
});
exports.str = str;
exports.raw = raw;
exports.vec = vec;
exports.bytevec = bytevec;
exports.uint = uint;
exports.int = int;
exports.makeSimple = makeSimple
var I32 = exports.I32 = [127];
var I64 = exports.I64 = [126];
var F32 = exports.F32 = [125];
var F64 = exports.F64 = [124];
var ANYFUNC = exports.ANYFUNC = [112];
var FUNC = exports.FUNC = [96];
var EMPTY = exports.EMPTY = [64];
var MAGIC_NUMBER = exports.MAGIC_NUMBER = [0, 97, 115, 109];
var VERSION_1 = exports.VERSION_1 = [1, 0, 0, 0];
var SECTION_CUSTOM = exports.SECTION_CUSTOM = [0];
var SECTION_TYPE = exports.SECTION_TYPE = [1];
var SECTION_IMPORT = exports.SECTION_IMPORT = [2];
var SECTION_FUNCTION = exports.SECTION_FUNCTION = [3];
var SECTION_TABLE = exports.SECTION_TABLE = [4];
var SECTION_MEMORY = exports.SECTION_MEMORY = [5];
var SECTION_GLOBAL = exports.SECTION_GLOBAL = [6];
var SECTION_EXPORT = exports.SECTION_EXPORT = [7];
var SECTION_START = exports.SECTION_START = [8];
var SECTION_ELEMENT = exports.SECTION_ELEMENT = [9];
var SECTION_CODE = exports.SECTION_CODE = [10];
var SECTION_DATA = exports.SECTION_DATA = [11];
var UNREACHABLE = exports.UNREACHABLE = [0];
var NOP = exports.NOP = [1];
var BLOCK = exports.BLOCK = [2];
var LOOP = exports.LOOP = [3];
var IF = exports.IF = [4];
var ELSE = exports.ELSE = [5];
var END = exports.END = [11];
var BR = exports.BR = [12];
var BR_IF = exports.BR_IF = [13];
var BR_TABLE = exports.BR_TABLE = [14];
var RETURN = exports.RETURN = [15];
var CALL = exports.CALL = [16];
var CALL_INDIRECT = exports.CALL_INDIRECT = [17];
var DROP = exports.DROP = [26];
var SELECT = exports.SELECT = [27];
var LOCAL_GET = exports.LOCAL_GET = [32];
var LOCAL_SET = exports.LOCAL_SET = [33];
var LOCAL_TEE = exports.LOCAL_TEE = [34];
var GLOBAL_GET = exports.GLOBAL_GET = [35];
var GLOBAL_SET = exports.GLOBAL_SET = [36];
var I32_LOAD = exports.I32_LOAD = [40];
var I64_LOAD = exports.I64_LOAD = [41];
var F32_LOAD = exports.F32_LOAD = [42];
var F64_LOAD = exports.F64_LOAD = [43];
var I32_LOAD8_S = exports.I32_LOAD8_S = [44];
var I32_LOAD8_U = exports.I32_LOAD8_U = [45];
var I32_LOAD16_S = exports.I32_LOAD16_S = [46];
var I32_LOAD16_U = exports.I32_LOAD16_U = [47];
var I64_LOAD8_S = exports.I64_LOAD8_S = [48];
var I64_LOAD8_U = exports.I64_LOAD8_U = [49];
var I64_LOAD16_S = exports.I64_LOAD16_S = [50];
var I64_LOAD16_U = exports.I64_LOAD16_U = [51];
var I64_LOAD32_S = exports.I64_LOAD32_S = [52];
var I64_LOAD32_U = exports.I64_LOAD32_U = [53];
var I32_STORE = exports.I32_STORE = [54];
var I64_STORE = exports.I64_STORE = [55];
var F32_STORE = exports.F32_STORE = [56];
var F64_STORE = exports.F64_STORE = [57];
var I32_STORE8 = exports.I32_STORE8 = [58];
var I32_STORE16 = exports.I32_STORE16 = [59];
var I64_STORE8 = exports.I64_STORE8 = [60];
var I64_STORE16 = exports.I64_STORE16 = [61];
var I64_STORE32 = exports.I64_STORE32 = [62];
var CURRENT_MEMORY = exports.CURRENT_MEMORY = [63];
var GROW_MEMORY = exports.GROW_MEMORY = [64];
var I32_CONST = exports.I32_CONST = [65];
var I64_CONST = exports.I64_CONST = [66];
var F32_CONST = exports.F32_CONST = [67];
var F64_CONST = exports.F64_CONST = [68];
var I32_EQZ = exports.I32_EQZ = [69];
var I32_EQ = exports.I32_EQ = [70];
var I32_NE = exports.I32_NE = [71];
var I32_LT_S = exports.I32_LT_S = [72];
var I32_LT_U = exports.I32_LT_U = [73];
var I32_GT_S = exports.I32_GT_S = [74];
var I32_GT_U = exports.I32_GT_U = [75];
var I32_LE_S = exports.I32_LE_S = [76];
var I32_LE_U = exports.I32_LE_U = [77];
var I32_GE_S = exports.I32_GE_S = [78];
var I32_GE_U = exports.I32_GE_U = [79];
var I64_EQZ = exports.I64_EQZ = [80];
var I64_EQ = exports.I64_EQ = [81];
var I64_NE = exports.I64_NE = [82];
var I64_LT_S = exports.I64_LT_S = [83];
var I64_LT_U = exports.I64_LT_U = [84];
var I64_GT_S = exports.I64_GT_S = [85];
var I64_GT_U = exports.I64_GT_U = [86];
var I64_LE_S = exports.I64_LE_S = [87];
var I64_LE_U = exports.I64_LE_U = [88];
var I64_GE_S = exports.I64_GE_S = [89];
var I64_GE_U = exports.I64_GE_U = [90];
var F32_EQ = exports.F32_EQ = [91];
var F32_NE = exports.F32_NE = [92];
var F32_LT = exports.F32_LT = [93];
var F32_GT = exports.F32_GT = [94];
var F32_LE = exports.F32_LE = [95];
var F32_GE = exports.F32_GE = [96];
var F64_EQ = exports.F64_EQ = [97];
var F64_NE = exports.F64_NE = [98];
var F64_LT = exports.F64_LT = [99];
var F64_GT = exports.F64_GT = [100];
var F64_LE = exports.F64_LE = [101];
var F64_GE = exports.F64_GE = [102];
var I32_CLZ = exports.I32_CLZ = [103];
var I32_CTZ = exports.I32_CTZ = [104];
var I32_POPCNT = exports.I32_POPCNT = [105];
var I32_ADD = exports.I32_ADD = [106];
var I32_SUB = exports.I32_SUB = [107];
var I32_MUL = exports.I32_MUL = [108];
var I32_DIV_S = exports.I32_DIV_S = [109];
var I32_DIV_U = exports.I32_DIV_U = [110];
var I32_REM_S = exports.I32_REM_S = [111];
var I32_REM_U = exports.I32_REM_U = [112];
var I32_AND = exports.I32_AND = [113];
var I32_OR = exports.I32_OR = [114];
var I32_XOR = exports.I32_XOR = [115];
var I32_SHL = exports.I32_SHL = [116];
var I32_SHR_S = exports.I32_SHR_S = [117];
var I32_SHR_U = exports.I32_SHR_U = [118];
var I32_ROTL = exports.I32_ROTL = [119];
var I32_ROTR = exports.I32_ROTR = [120];
var I64_CLZ = exports.I64_CLZ = [121];
var I64_CTZ = exports.I64_CTZ = [122];
var I64_POPCNT = exports.I64_POPCNT = [123];
var I64_ADD = exports.I64_ADD = [124];
var I64_SUB = exports.I64_SUB = [125];
var I64_MUL = exports.I64_MUL = [126];
var I64_DIV_S = exports.I64_DIV_S = [127];
var I64_DIV_U = exports.I64_DIV_U = [128];
var I64_REM_S = exports.I64_REM_S = [129];
var I64_REM_U = exports.I64_REM_U = [130];
var I64_AND = exports.I64_AND = [131];
var I64_OR = exports.I64_OR = [132];
var I64_XOR = exports.I64_XOR = [133];
var I64_SHL = exports.I64_SHL = [134];
var I64_SHR_S = exports.I64_SHR_S = [135];
var I64_SHR_U = exports.I64_SHR_U = [136];
var I64_ROTL = exports.I64_ROTL = [137];
var I64_ROTR = exports.I64_ROTR = [138];
var F32_ABS = exports.F32_ABS = [139];
var F32_NEG = exports.F32_NEG = [140];
var F32_CEIL = exports.F32_CEIL = [141];
var F32_FLOOR = exports.F32_FLOOR = [142];
var F32_TRUNC = exports.F32_TRUNC = [143];
var F32_NEAREST = exports.F32_NEAREST = [144];
var F32_SQRT = exports.F32_SQRT = [145];
var F32_ADD = exports.F32_ADD = [146];
var F32_SUB = exports.F32_SUB = [147];
var F32_MUL = exports.F32_MUL = [148];
var F32_DIV = exports.F32_DIV = [149];
var F32_MIN = exports.F32_MIN = [150];
var F32_MAX = exports.F32_MAX = [151];
var F32_COPYSIGN = exports.F32_COPYSIGN = [152];
var F64_ABS = exports.F64_ABS = [153];
var F64_NEG = exports.F64_NEG = [154];
var F64_CEIL = exports.F64_CEIL = [155];
var F64_FLOOR = exports.F64_FLOOR = [156];
var F64_TRUNC = exports.F64_TRUNC = [157];
var F64_NEAREST = exports.F64_NEAREST = [158];
var F64_SQRT = exports.F64_SQRT = [159];
var F64_ADD = exports.F64_ADD = [160];
var F64_SUB = exports.F64_SUB = [161];
var F64_MUL = exports.F64_MUL = [162];
var F64_DIV = exports.F64_DIV = [163];
var F64_MIN = exports.F64_MIN = [164];
var F64_MAX = exports.F64_MAX = [165];
var F64_COPYSIGN = exports.F64_COPYSIGN = [166];
var I32_WRAP_F64 = exports.I32_WRAP_F64 = [167];
var I32_TRUNC_S_F32 = exports.I32_TRUNC_S_F32 = [168];
var I32_TRUNC_U_F32 = exports.I32_TRUNC_U_F32 = [169];
var I32_TRUNC_S_F64 = exports.I32_TRUNC_S_F64 = [170];
var I32_TRUNC_U_F64 = exports.I32_TRUNC_U_F64 = [171];
var I64_EXTEND_S_I32 = exports.I64_EXTEND_S_I32 = [172];
var I64_EXTEND_U_I32 = exports.I64_EXTEND_U_I32 = [173];
var I64_TRUNC_S_F32 = exports.I64_TRUNC_S_F32 = [174];
var I64_TRUNC_U_F32 = exports.I64_TRUNC_U_F32 = [175];
var I64_TRUNC_S_F64 = exports.I64_TRUNC_S_F64 = [176];
var I64_TRUNC_U_F64 = exports.I64_TRUNC_U_F64 = [177];
var F32_CONVERT_S_I32 = exports.F32_CONVERT_S_I32 = [178];
var F32_CONVERT_U_I32 = exports.F32_CONVERT_U_I32 = [179];
var F32_CONVERT_S_I64 = exports.F32_CONVERT_S_I64 = [180];
var F32_CONVERT_U_I64 = exports.F32_CONVERT_U_I64 = [181];
var F32_DEMOTE_F64 = exports.F32_DEMOTE_F64 = [182];
var F64_CONVERT_S_I32 = exports.F64_CONVERT_S_I32 = [183];
var F64_CONVERT_U_I32 = exports.F64_CONVERT_U_I32 = [184];
var F64_CONVERT_S_I64 = exports.F64_CONVERT_S_I64 = [185];
var F64_CONVERT_U_I64 = exports.F64_CONVERT_U_I64 = [186];
var F64_PROMOTE_F32 = exports.F64_PROMOTE_F32 = [187];
var I32_REINTERPRET_F32 = exports.I32_REINTERPRET_F32 = [188];
var I64_REINTERPRET_F64 = exports.I64_REINTERPRET_F64 = [189];
var F32_REINTERPRET_I32 = exports.F32_REINTERPRET_I32 = [190];
var F64_REINTERPRET_I64 = exports.F64_REINTERPRET_I64 = [191];
var DESC_FUNCTION = exports.DESC_FUNCTION = [0];
var DESC_TABLE = exports.DESC_TABLE = [1];
var DESC_MEMORY = exports.DESC_MEMORY = [2];
var DESC_GLOBAL = exports.DESC_GLOBAL = [3];
var LIMIT_MIN = exports.LIMIT_MIN = [0];
var LIMIT_MIN_MAX = exports.LIMIT_MIN_MAX = [1];
var IMMUTABLE = exports.IMMUTABLE = [0]
var MUTABLE = exports.MUTABLE = [1]

// HELPERS

// this function flattens out a nested array of bytes
var flatten = exports.flatten  = require('flattenbytes');

// turns a string into byte array
function str(s) {
  var bytes = [];
  for (var i in s) {
    bytes.push(s.charCodeAt(i));
  }
  return [bytes.length, bytes];
}

function raw(s) {
  var bytes = [];
  for (var i in s) {
    bytes.push(s.charCodeAt(i));
  }
  return bytes;
}

// helper to create a common data structure in bytes [count(items), item[0], item[1] ...]
function vec(vals) {
  return [uint(vals.length), vals];
}

// helper to create a common vector of bytes [count(bytes), byte[0], byte[1] ... ]
function bytevec(vals) {
  return vec(flatten(vals));
}

let {varint7,varint32,varint64,varuint1,varuint7,varuint32,varuint64} = require('wasm-leb128');

// create a compressed uint
function uint(v,b) {
  if(b===1){
    return Array.from(varuint1(v));
  } else if(b===7){
    return Array.from(varuint7(v));
  } else if(b===32){
    return Array.from(varuint32(v));
  } else if(b===64){
    return Array.from(varuint64(v));
  }
  return Array.from(varuint32(v));
}

// create a compressed int
function int(v,b) {
  if(b===7){
    return Array.from(varint7(v));
  } else if(b===32){
    return Array.from(varint32(v));
  } else if(b===64){
    return Array.from(varint64(v));
  }
  return Array.from(varint32(v));
}

function makeSimple(inputs,outputs,code){
  let main_function_signature = [FUNC,vec(inputs),vec([outputs])];
  let main_function_code = bytevec(code);

  let memory = [LIMIT_MIN_MAX,uint(2),uint(10)];

  return flatten([
    MAGIC_NUMBER,
    VERSION_1,
    [SECTION_TYPE,bytevec(vec([main_function_signature]))],
    [SECTION_FUNCTION,bytevec(vec([0]))],
    [SECTION_MEMORY,bytevec(vec([memory]))],
    [SECTION_EXPORT,bytevec(vec([
      [str("main"),DESC_FUNCTION,0],
      [str("memory"),DESC_MEMORY,0]
    ]))],
    [SECTION_CODE,bytevec(vec([main_function_code]))]
  ]);
}
