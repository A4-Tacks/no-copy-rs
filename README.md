A simple packer, but no implement copy

This crate originates from the behavior of non move closures when capturing Copy types

# Examples

```rust,compile_fail
let value;
{
    let n = 2;
    value = || n+1;
}
assert_eq!(value(), 3);
```

Imagine that it will compile? Actually, it won't

For non move closures, if owned uses a copy types value, it will actually capture the reference

Use `NoCopy` to make `n` no longer a copy types, compile passed:

```rust
let value;
{
    let n = no_copy::NoCopy(2);
    value = || n+1;
}
assert_eq!(value(), 3);
```
