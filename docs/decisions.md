- Do we have `%{...}%` or `%{...}` to interpolate values into string templates.
- External packages!!!!
- Module namespace conflicts (`as` required in conflicts?)
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
    - Yes
- Do we scrap semicolons?
    - Yes

# If, while and match

Do we have required parenthesis and/or curly braces, like this?

```m
match money {
    Money.Lots => println("yay!"),
    Money.AsLittleAsProles => println("I will not exist for long!")
}

function apple(pear: Pear) -> Int do 1 + 1

if condition do expression else expression;


while 2 + 2 == 4 do {
    println("Re-education in progress...")
    return 
}

```

What about `if let`?

```m
let x = if let Some(x) = nullable_expression {
    
} else {

};

let x = if Some(x) = (nullable_expression) {
    
} else {

};
```