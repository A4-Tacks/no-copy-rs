A simple packer, but no implement copy

# Examples

```rust,compile_fail
use no_copy::NoCopy;

let mut n = NoCopy(3);
let _moved = n;
let _use_after_moved = n;
```
