> **TODO**: Traits?

## Syntax


Functions are declared with their parameters in brackets.

| Return Type | [Name] | Type Params | Params | Arrow | Return Expression
| --- | --- | --- | --- | --- | --- |
| `int8` | `add` | `<...>` | `(num1: int8, num2: int8)` | `->` | `num1 + num2`

```m
int8 add(num1: int8, num2: int8) -> num1 + num2;
```

> **Note**: Unlike some languages, where the type of parameters can be inferred, type annotations are required on functional parameters.

### Parameters

Parmameters are declared with their name, followed by a colon and type to serve as their type annotation

| Name | Colon | Type | 
| --- | --- | --- |
| `a` | `: ` | `char[]` | 

### Type Parameters

```m
R[] map<T, R>(map_fn: ((val: T, [index: usize]) -> R), iter: T[]);

character
integer
boolean

map([1, 2], (a) -> a ** 2);
```

```m
trait Map {
   R[] map<T, R>(self, map_fn: ((val: T, [index: usize]) -> R))) {

   };
}

impl Map for <T: Sized> {

}

struct T {
    prop1: int16,
    prop2: char[],

    Self new(&self, self.int16, self.char[]) -> {
        
    }

    trait Map {
        fn map()
    }
}
```

```
int16 x = 12;
```