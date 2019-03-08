# Wasmly
This is a minimalistic web assembly tool for creating modules from arrays of bytes. This helps you develop much more closer to how web assembly apps exist at a low level.

If you check this project out and just want to get the dependencies to setup:

```
make setup
```

Go into any example directory:

```terminal
make
make serve
```
Then open a browser to http://localhost:9999

Check the console for output!

If you are using this as a npm module

```
npm install wasmly
```

# Very Simple Modules
If all you are doing is writing a single exported "main" function that takes in inputs, returns an output, and exposes "memory". Try this to save boiler plate:

```javascript
var fs = require('fs');
let {makeSimple,int,I32,I32_CONST,END} = require("wasmly");

// main() -> i32
app = makeSimple([],[I32],[   
  vec([]),             // no local variables
  I32_CONST, int(42),  // return 42
  END
])

fs.writeFileSync('out.wasm',Buffer.from(app))
```
# Simplest Module Possible
If you are trying to create the most simplest module from scratch. This has no functions

```javascript
var fs = require('fs');
let {flatten,MAGIC_NUMBER,VERSION_1} = require("wasmly");

app = [
  MAGIC_NUMBER,
  VERSION_1,
]

// this is just a nested array of bytes:
// [[[0, 97, 115, 109]][[1, 0, 0, 0]]]

fs.writeFileSync('out.wasm',Buffer.from(flatten(app)))
```
Not very useful!

# `main()` From Scratch

```javascript
...
// main() -> i32 { return 42 }
main_function_signature = [FUNC,vec([]),vec([I32])] // function signature returns 42
main_function_code = bytevec([
  vec([]),              // no local variables
  [I32_CONST, int(42)], // return 42
  END
])

//lets make memory at least 2 pages and at most 10 pages long
memory = [LIMIT_MIN_MAX,uint(2),uint(10)]

// put it all together as a module
app = [
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
...
```

# Memory Allocator

Let's make a very simple memory allocator.

```javascript
...
// malloc(length:i32) -> i32 { ... }
malloc_function_signature = [FUNC,vec([I32]),vec([I32])] // function signature returns 42
malloc_function_code = bytevec([
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
heap_global = [I32,MUTABLE,I32_CONST, int(0),END]

//lets make memory at least 2 pages and at most 10 pages long
memory = [LIMIT_MIN_MAX,uint(2),uint(10)]

// put it all together as a module
app = [
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
...
```
