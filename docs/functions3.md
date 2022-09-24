> **TODO**: Traits?

## Syntax

Functions are declared with their parameters in brackets.

| [Pure Keyword] | Function Keyword | [Name] | Type Params | Params                     | [Return Type] | Arrow | Return Expression |
| -------------- | ---------------- | ------ | ----------- | -------------------------- | ------------- | ----- | ----------------- |
| (`pure`)       | `fn`             | `add`  | (`<T>`)     | `(num1: Int8, num2: Int8)` | `: Int8`      | `->`  | `num1 + num2`     |

```m
fn add(num1: Int8, num2: Int8): Int16 -> num1 + num2;
```

> **Note**: Unlike some languages, where the type of parameters can be inferred, type annotations are required on functional parameters.

### Parameters

Parmameters are declared with their name, followed by a colon and type to serve as their type annotation

| Name | Colon | Type     |
| ---- | ----- | -------- |
| `a`  | `: `  | `String` |

### Type Parameters
