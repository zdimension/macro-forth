# macro-forth

Forth, in macros.

```rust
fn main() {
    const TWO: i32 = forth!(5 3 -);
    forth!(
        TWO . // 2
    );

    const HUNDRED: i8 = forth!(
        1 2 dup * + dup + // 10
        dup * // 100
        hundred !
        3 dup swap drop drop
        hundred @
    );

    forth!(
        HUNDRED @ dup . // 100
        50 > if "bigger" else "smaller" then . // bigger
    );
}
```
