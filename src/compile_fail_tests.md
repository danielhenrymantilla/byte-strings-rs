# The following snippets fail to compile

## Inner null is denied at compile-time

```rust ,compile_fail
use ::byte_strings::*;

let _ = c!("Hell\0, World!");
```

## `const_concat!` rejects byte strings

```rust ,compile_fail

use ::byte_strings::*;

let _ = const_::concat!(b"\xff\xff");
let _ = const_::concat!(b"Hi");
```
