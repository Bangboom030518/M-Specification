The library used for interacting with the file system.

## Read To Bytes

> TODO: do we provide encoding method?

> TODO: method overloading doesn't exist. How do we represent this?

```
export const read_to_bytes: (path: char[]) -> uint8[];
```

## Read To String

```
export const read_to_string: (path: char[], encoding: Encoding::Method) -> char[];
```


