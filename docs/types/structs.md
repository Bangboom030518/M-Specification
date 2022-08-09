A struct is a type that defines the structure of an object, with key-value pairs.

## Initialisation

Structs are initialised using the `type` and `struct` keywords.

For example, the following creates a struct called `MyStruct`, with the property `property1` as a `utf8` array and `property2` as an `int16`.

```
type MyStruct struct {
  property1: utf8[];
  property2: uint8;
}
```

These can also be given defaults using the `=` symbol.

For example, the following initialises `property1` to `"Hello World"` and `property2` to `42` if no other value is specified on construction.

```
type MyStruct struct {
  property1: utf8[] = "Hello World";
  property2: uint8 = 42;
}
```
