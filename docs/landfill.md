```
import Int from "std::integer";
import Length from "std::array";

...

  // var my_int: Int(1, 12) = 1;

  var my_int: uint8 = 1;

  const my_fn = pure <T>(list: T[], index: Int(0, Length(list))) -> {
    return list[index];
  }

  type In

  Min(10)
  
  type NumberUpTo constraint (max = UIntGeneric) -> Range<UIntGeneric>(0, max)

  const my_fn = pure <T>(
    list: T[],
    index: Range<UIntGeneric>(0, Length(list))
  ): T -> {
    return list[index];
  }

...

var func = (list: T[], index) => list[index];
```

```
import Vector from "std";

const x: uint8[] = [1, 12, 43, 2];
const y: Vector<uint8> = Vector::new([1, 12, 43, 2]);
var z: Vector<uint8> = Vector<uint8>! { 1, 12, 43, 2 };

Vector::append(z, 5);

RegExp! {
  <behind>12</behind><number />.<latin1 />+?
}

```
