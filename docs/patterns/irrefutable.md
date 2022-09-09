> **TODO**: Update `struct` syntax as displayed here.

## Destructuring

Struct destructuring:

## Examples

### Code

```m
import Stdout from "std:io";

struct MyStruct {
    a: uint8;
    b: uint8;
}

let instance = MyStruct {
    a: 1,
    b: 1,
};

let MyStruct { a, b } = instance;

Stdout::write_line("a: ${a}")
```

### Output

```console

```