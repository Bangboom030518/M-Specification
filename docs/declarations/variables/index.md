Variables are references to values, which tend to be stored in memory and can sometimes be mutated and reasigned.

Variables in `M` behave similarly to those in rust, though there are some differences.

## Syntax

Variables are declared with a variable keyword, followed by their name, followed by an optional type annotation and a value.

| keyword | pattern       | assignment |
| ------- | ------------- | ---------- |
| `let`   | `my_variable` | `= 12u16`  |

### Const

Variables declared with the `const` keyword are constants, meaning their value can *never* be changed, and must be a compile-time constant.

Constants are not stored in memory, but instead have their value interpolated wherever they are used. They are useful for describing constants such as magic numbers with no cost to performance.

#### Example

##### Code

```m
import Stdout from "std:io";

const MILES_TO_KILOMETERS = 1.60934;

float miles_to_km(miles: float) -> miles * MILES_TO_KILOMETERS;

void main() -> {
    Stdout::write_line(f"2 miles ≈ {miles_to_km(2)}km");
    Stdout::write_line(f"3 miles ≈ {miles_to_km(3)}km");
    Stdout::write_line(f"4 miles ≈ {miles_to_km(4)}km");
}
```

##### Output

```console
2 miles ≈ 3.21869km
3 miles ≈ 4.82803km
4 miles ≈ 6.43738km
```

### Let

`let` is like rust `let`

### Var

`var` is like rust `let mut`

### Static

A `static` variable is lazily evaluated once and stored until it's no longer needed. They should be declared with `UPPER_SNAKE_CASE`, and can only be declared in the global scope. They cannot be exported.
