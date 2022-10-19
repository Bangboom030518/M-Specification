> **TODO**: streams?????

Modules allow code to be shared and reused across files. M provides a simple, strict module interface to simplify working with modules.

## Exports

Exporting a declaration allows it to be accessible when the module is imported. It is achieved through the `export` keyword

| Export Keyword | Declaration                           |
| -------------- | ------------------------------------- |
| `export`       | `const MY_CONSTANT = "Hello World!"`  |
| `export`       | `function my_function(): Nil -> {..}` |

> **NOTE**: Declarations can only be exported at the top level, meaning runtime values using `let` or `var` cannot be exported.

## Imports

### Syntax

| import keyword | module name      |
| -------------- | ---------------- |
| `import`       | `module`         |
| `import`       | `package/module` |

### Package Imports

It is sometimes necessary to import a module from an package in M. When doing this, simply prefix the module name with with the package name followed by a forward slash (`/`).

#### Syntax

| import keyword | package name | forward slash | module name |
| -------------- | ------------ | ------------- | ----------- |
| `import`       | `my_package` | `/`           | `my_module` |

> **NOTE**: To import from the standard library, use the `std` namespace, e.g `import std/io;`.

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
