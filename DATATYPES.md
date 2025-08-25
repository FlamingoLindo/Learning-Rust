# Data Types

## Integer Types

| Length | Signed | Min Value | Max Value | Unsigned | Min Value | Max Value |
|--------|--------|-----------|-----------|----------|-----------|-----------|
|8-bit|i8|-128|127|u8|0|255|
|16-bit|i16|-32,768|32,767|u16|0|65,535|
|32-bit|i32|-2,147,483,648|2,147,483,647|u32|0|4,294,967,295|
|64-bit|i64|-9,223,372,036,854,775,808|9,223,372,036,854,775,807|u64|0|18,446,744,073,709,551,615|
|128-bit|i128|-170,141,183,460,469,231,731,687,303,715,884,105,728|170,141,183,460,469,231,731,687,303,715,884,105,727|u128|0|340,282,366,920,938,463,463,374,607,431,768,211,455|
|arch-dependent|isize|depends on target|depends on target|usize|0|depends on target|

## Floating-Point Types

| Type | Description | Min Value | Max Value | Precision |
|------|-------------|-----------|-----------|-----------|
| f32 | 32-bit floating point | -3.40282347e+38 | 3.40282347e+38 | ~6-7 decimal digits |
| f64 | 64-bit floating point (default) | -1.7976931348623157e+308 | 1.7976931348623157e+308 | ~15-17 decimal digits |

## Boolean Type

| Type | Values |
|------|--------|
| bool | true, false |

## Character Type

| Type | Description |
|------|-------------|
| char | Unicode scalar value (4 bytes) |

## String Types

| Type | Description |
|------|-------------|
| &str | String slice (borrowed) |
| String | Owned, growable string |

## Compound Types

### Tuple

- Fixed-size collection of values with different types
- Example: `(i32, f64, u8)`

### Array

- Fixed-size collection of values with the same type
- Example: `[i32; 5]` (array of 5 i32 values)

### Slice

- Dynamically-sized view into a contiguous sequence
- Example: `&[i32]` (slice of i32 values)

## Pointer Types

| Type | Description |
|------|-------------|
| &T | Immutable reference |
| &mut T | Mutable reference |
| *const T | Raw immutable pointer |
| *mut T | Raw mutable pointer |
| Box\<T\> | Owned pointer (heap allocation) |

## Function Types

| Type | Description |
|------|-------------|
| fn | Function pointer |
| Fn, FnMut, FnOnce | Closure traits |

## Unit Type

| Type | Description |
|------|-------------|
| () | Unit type (empty tuple) |

## Never Type

| Type | Description |
|------|-------------|
| ! | Never type (diverging functions) |

## Option and Result Types

| Type | Description |
|------|-------------|
| Option\<T\> | Some(T) or None |
| Result\<T, E\> | Ok(T) or Err(E) |
