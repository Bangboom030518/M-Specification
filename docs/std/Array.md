> TODO: What about slices and tuples?

> TODO: `UnsignedInt`???

The library containing all the basic utility functions for arrays.

## Reverse

```
export const reverse: pure <T>(array: T[]) -> T[];
```

Returns the array `array`, in reverse order.

## Length

```
export const length: pure <T>(array: T[]) -> Integer::Unsigned;
```

Returns the length of the array `array`.
