Modules allow code to be shared and reused across files. M provides a simple, strict module interface to simplify working with modules.

## Exports

Exports are 

Packages can only directly export [namespaces](./namespace).

`package.m`
```
export namespace Module1 {
  ...
}

export namepace Module2 {
  ...
}

export Module3 from "./sub/package1.m";

export Module4, Module5 from "./sub/package2.m";
```
## Imports

Imports use the following syntax, which imports the `Namespace1` and the `Namespace2` namespaces from `./package.m`.

```
import Namespace1, Namespace2 from "./package.m";
```

> Note: To import namespaces from the standard library, use `import Namepace from "std";`
