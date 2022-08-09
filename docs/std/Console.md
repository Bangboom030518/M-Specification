## Write Line
```
export const write_line: ((text: char[]) -> void);
```

Appends a line to `stdout` containing the text `text`.

## Read Line
```
export const read_line: (() -> char[]);
```

Reads a line from `stdin` and returns the result.

## Clear
```
export const clear: (() -> void);
```

Clears `stdout`.
