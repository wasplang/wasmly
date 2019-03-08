var fs = require('fs');
let wasmly = require("../../index");
let {flatten,str,vec,bytevec,int,uint,I32,FUNC,DESC_FUNCTION,END,I32_CONST,SECTION_TYPE,
  SECTION_FUNCTION,SECTION_EXPORT,SECTION_CODE,MAGIC_NUMBER,VERSION_1,DESC_MEMORY,
  SECTION_MEMORY,LIMIT_MIN_MAX} = wasmly;

// main() -> i32 { return 42 }
let main_function_signature = [FUNC,vec([]),vec([I32])] // function signature returns 42
let main_function_code = bytevec([
  vec([]),              // no local variables
  [I32_CONST, int(42)], // return 42
  END
])

//lets make memory at least 2 pages and at most 10 pages long
let memory = [LIMIT_MIN_MAX,uint(2),uint(10)]

// put it all together as a module
let app = [
  MAGIC_NUMBER,
  VERSION_1,
  [SECTION_TYPE,bytevec(vec([main_function_signature]))],
  [SECTION_FUNCTION,bytevec(vec([int(0)]))],
  [SECTION_MEMORY,bytevec(vec([memory]))],
  [SECTION_EXPORT,bytevec(vec([
    [str("main"),DESC_FUNCTION,0],
    [str("memory"),DESC_MEMORY,0]
  ]))],
  [SECTION_CODE,bytevec(vec([main_function_code]))]
]

// print it out for debugging
console.log(JSON.stringify(app));

// write it to test.wasm
fs.writeFileSync('out.wasm',Buffer.from(flatten(app)))
