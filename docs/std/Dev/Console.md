> TODO: is `IO` called `Console`?

The library used for temporary logs during development. Unlike in the `IO` module, these can be used within pure functions but their use anywhere within the project will raise a warning during development and prevent building for production.

> TODO: figure out how to make IO events pure for this, we should not be inconsistent

## Log

> TODO: better type

```
export const log: <T>(item: T) -> void; 
```
