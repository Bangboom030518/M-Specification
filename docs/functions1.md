## Syntax

Functions are declared with their parameters in brackets

| [Type Parameters] | Open Bracket | [Parameters]           | Close Braket | Arrow | Return Type | Body      |
| ----------------- | ------------ | ---------------------- | ------------ | ----- | ----------- | --------- |
| `<T, R>`          | `(`          | `a: String, b: uint16` | `)`          | `->`  | `void`      | `{ ... }` |

### Parameters

Parmameters within

| Name | Colon | Type     |
| ---- | ----- | -------- |
| `a`  | `: `  | `String` |

> **Note**: Unlike some languages, where the type of parameters can be inferred, type annotations are required on functional parameters.

> **Note**: Functions don't implicitly return; they require a `return` statement.

## The Void Type

Functions that return nothing can have a return type of `void`. They will then implicitly return after the body has finished executing, but can return early with an empty `return` statement.

> TODO: can you have single expressions after ifs without curly braces.

```
const function = (param: uint8) -> void {
  if (param > 20) return;
  // else do something
}
```

## The Pure Keyword

There are times when a function just needs to compute a given argument and returns the result, without producing any side-effects, such as editing global variables or writing to `stdout`. In these cases, the `pure` keyword is recommended.

```
const function = (arg_1: [type], arg_2: [type]) -> [return type] {
  if ([guard clause]) return;
  // do something
}
```

> **Note**: Pure keywords cannot have a return type of `void`.
