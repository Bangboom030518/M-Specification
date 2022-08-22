The library used for interacting with the file system.

> **TODO**: Write the following:
> - Write File
> - Append File
> - Remove File
> - Read Directory
> - Get File Info
>   - Do we use a path struct?

## Read To Bytes

```
export const read_to_bytes: (path: char[]) -> uint8[];
```

## Read To String

```
export const read_to_string: (path: char[], encoding: Encoding::Method) -> char[];
```

