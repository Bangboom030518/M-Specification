- Do we have `${...}` or `{...}` to interpolate values into string templates.
- External packages!!!!
- Module namespace conflicts (`as` required in conflicts?)
- Do we have `true`/`false` or a `Bool` enum.
- Do we have a return label system, like [Kotlin](https://kotlinlang.org/docs/returns.html#return-to-labels)
  - Yes
- Do we use `let` for properties?
  - If so, lets add Trait-defined properties:
  Initialised With
  ```
  let a = MyStruct {
    prop: "_",
    MyTrait {
      prop1: "_",
    }
  }
  ```
  
- Do we scrap `::` and just have `.`?
