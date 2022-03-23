## Instructions


Write a simple parser that will parse and run Deadfish.
Deadfish has four commands, each one character's long:

- i increments the value (initially 0)
- d decrements the value
- s squares the value
- o outputs the value into the return array

### Notes

Invalid characters should be ignored.

### Examples

```rust
parse("iiisdoso") => [ 8, 64 ]
```
