> **TODO**: streams?????

Modules allow code to be shared and reused across files. M provides a simple, strict module interface to simplify working with modules.

## Exports

## Imports

### Syntax

| import keyword | module name |
| -------------- | ----------- |
| `import`       | `module`    |

### Examples

```m
import module;
```

This imports the file `./module.m`, relative to the dependant

## Example

`./module.m`
```m
import std/io;

static STDOUT = io::stdout();

export function greet(hour: UInt8): Nil -> {
  let greeting = if (hour > 17) {
    "Good Evening!"
  } else {
    "Good Day!"
  };
  STDOUT.write_line(greeting);
}
```

`./index.m`
```m
import module;

export function main() -> Nil {
  module::greet(16);
  module::greet(16);
}
```

> Note: To import namespaces from the standard library, use `import Namepace from "std";`
