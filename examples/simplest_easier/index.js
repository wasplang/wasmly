var fs = require('fs');
let wasmly = require("../../index");
let {makeSimple,flatten,str,vec,bytevec,int,uint,I32,I32_CONST,END} = wasmly;

const app = makeSimple([],[I32],[
  vec([]),        // no local variables
  I32_CONST, 42, // return 42
  END
])

// write it to test.wasm
fs.writeFileSync('out.wasm',Buffer.from(app))
