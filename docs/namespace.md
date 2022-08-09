> TODO: update formatted string syntax

Namespaces help you organise your code by scoping variables to a specific portion of the file.

## Declaration

Namespaces are declared using the `namespace` keyword. Variables can then be initialised within them, but will not be accessible outside of the namespace unless they are preceeded with the `export` keyword.

> Note: Namespaces should be written in `PascalCase`.

```
namespace MyNamespace {
  const name = "Bob";
  export const greeting = () -> {
    return f"Hello, {name}";
  };
}
```

## Accessing Values

Exported variables are accessed using the double colon syntax (`::`).

For example, the following should log "Hello, Bob" to the console.

```
import IO from "std";

IO::write_line(MyNamespace::greeting());
```

Values that are not exported *cannot* be accessed in this way.

For example, this will throw an error.

```
import IO from "std";

IO::write_line(MyNamespace::name);
```
