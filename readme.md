# slectern

> A **String Reader**

Read different data types, destructure patterns from strings.  


## Usage

```rust
use slectern::{Reader, ReaderError};

let mut r = Reader::new("  keyword 814 true \"sick noooo\"");

r.skip_whitespace();
let literal = r.read_string_until_end()?;
let num = r.read_num::<i64>()?;

r.skip_whitespace();
let bool_1 = r.read_bool()?;

r.skip_whitespace();
let phrase = r.read_quoted_string()?;

Ok::<(), ReaderError>(())
```