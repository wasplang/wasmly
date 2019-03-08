var fs = require('fs');
let wasmly = require("../../index");
let {flatten,str,vec,bytevec,int,uint,I32,FUNC,DESC_FUNCTION,END,I32_CONST,SECTION_TYPE,
  SECTION_FUNCTION,SECTION_EXPORT,SECTION_CODE,MAGIC_NUMBER,VERSION_1,DESC_MEMORY,
  SECTION_MEMORY,LIMIT_MIN_MAX,SECTION_GLOBAL,MUTABLE,NOP,BLOCK,GLOBAL_GET,
  LOCAL_SET,LOCAL_GET,I32_STORE,I32_ADD,GLOBAL_SET,DESC_GLOBAL} = wasmly;

// malloc(length:i32) -> i32 { ... }
let malloc_function_signature = [FUNC,vec([I32]),vec([I32])] // function signature returns 42
let malloc_function_code = bytevec([
  vec([
    [1, I32] // current_heap:i32
  ]),
  // current_heap = global.heap
  GLOBAL_GET, 0,
  LOCAL_SET,  1,
  // memorycurrent_heap = length
  GLOBAL_GET, 0,
  LOCAL_GET,  0,
  I32_STORE,  0, 0,
  // global.heap = current_heap + 1 + length
  LOCAL_GET,  1,
  I32_CONST,  1,
  I32_ADD,
  LOCAL_GET,  0,
  I32_ADD,
  GLOBAL_SET, 0,
  // return current_heap + 1
  LOCAL_GET,  1,
  I32_CONST,  5,
  I32_ADD,
  END
])

// create a heap global set to zero
let heap_global = [I32,MUTABLE,I32_CONST, int(0),END]

//lets make memory at least 2 pages and at most 10 pages long
let memory = [LIMIT_MIN_MAX,uint(2),uint(10)]

// put it all together as a module
let app = [
  MAGIC_NUMBER,
  VERSION_1,
  [SECTION_TYPE,bytevec(vec([malloc_function_signature]))],
  [SECTION_FUNCTION,bytevec(vec([0]))],
  [SECTION_MEMORY,bytevec(vec([memory]))],
  [SECTION_GLOBAL,bytevec(vec([heap_global]))],
  [SECTION_EXPORT,bytevec(vec([
     [str("malloc"),DESC_FUNCTION,0],
     [str("memory"),DESC_MEMORY,0],
     [str("heap"),DESC_GLOBAL,0]
   ]))],
  [SECTION_CODE,bytevec(vec([malloc_function_code]))]
]

// print it out for debugging
console.log(JSON.stringify(app));

// write it to test.wasm
fs.writeFileSync('out.wasm',Buffer.from(flatten(app)))
