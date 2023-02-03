A `no-std` compatible crate which provides wrappers for imposing arbitrary invariants on floating point types.
The [`FloatChecker`] trait can be implemented on a type to create an invariant checker that can then
be used in the [`CheckedFloat`] type to create a wrapper that enforces the invariant for all operations.

## Example

The following is an example of how to use `checked-float` to create a floating point wrapper that forbids NaN.

```rust
# use checked_float::*;
#[derive(Debug)]
struct NanError;

struct NoNanChecker;
impl<T: Float> FloatChecker<T> for NoNanChecker {
    type Error = NanError;
    fn check(value: T) -> Result<(), Self::Error> {
        if value.is_nan() { Err(NanError) } else { Ok(()) }
    }
}

type NoNan64 = CheckedFloat<f64, NoNanChecker>; // our checked float wrapper

let y = NoNan64::new(0.0).unwrap(); // not nan, so we can unwrap
let x = NoNan64::new(2.0).unwrap(); // not nan, so we can unwrap
assert_eq!(x.powf(y).unwrap().get(), 1.0); // not nan, so we can unwrap
assert!(y.div(y).is_err()); // 0/0 is nan, so we get Err
```

## `no-std` support

`checked-float` supports building in `no-std` environments out of the box.
However, for future-proofing, you may like to explicitly opt out of default features in case
a dependency on `std` is ever added.

```toml
[dependencies]
checked-float = { version = "...", default-features = false }
```

## Features


| name | default | description |
| ---- | ------- | ----------- |
| `serde` | off | Enables serialization of [`CheckedFloat`] |
