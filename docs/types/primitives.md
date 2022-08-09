Primitives are available globally. They are not objects, but are first class values that can be passed to and returned from functions to manipulate them. 

## Numeric Types

### Integer Types

#### Signed

- `int8`: 8 bit integer.
- `int16`: 16 bit integer.
- `int32`: 32 bit integer.
- `int64`: 64 bit integer.
- `bigint`: Chonky integer with changeable size.

#### Unsigned

- `uint8`: 8 bit unsigned integer.
- `uint16`: 16 bit unsigned integer.
- `uint32`: 32 bit unsigned integer.
- `uint64`: 64 bit unsigned integer.

### Fractional Types

- `float`: IEEE 754 floating point number.
- `double`: double precision IEEE 754 floating point number.

## Boolean

- `boolean`: `true` or `false`.

## Char

- `char`: A `utf8` encoded character.

## Null

- `null`: A lack of a value.

## Array

- `type[length]`: An array with items of type `type` and a fixed length of `length`.

## Slice

- `[type]`: A slice of variable length with items of type `type`.
