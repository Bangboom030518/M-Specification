> TODO: Number type!

## Mean

```
export const mean: pure <T extends Number::Generic, R extends Number::Generic>(list: T[]) -> R;
```

Returns the mean of the list `list`.

## Median

```
export const median: pure <T extends Number::Generic, R extends Number::Generic>(list: T[]) -> R;
```

Returns the median of the list `list`.

## Range

```
export const range: pure <T extends Number::Generic>(list: T[]) -> T;
```

Returns the range of the list `list`.

## Interquartile Range

```
export const interquartile_range: pure <T extends Number::Generic>(list: T[]) -> T;
```

Returns the interquartile range of the list `list`.
