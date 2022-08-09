> TODO: type keyword go brrrr.
> TODO: what about chars?

The library containing all the basic utility functions used on `char[]`.

## To Lower Case
```
export const to_lower_case: pure ((text: char[]) -> char[]);
```

Returns `text`, with all characters in lower case.


## To Upper Case
```
export const to_upper_case: pure ((text: char[]) -> char[]);
```

Returns `text`, with all characters in upper case.

### Example:

> TODO: is it Console instead of IO?

```
import String, DevConsole from "std";

const text = String::to_upper_case("hello world");

```
