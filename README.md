NOTES:
1. Variables are immutable by default. To make it mutable, put mut. Eg. let mut sampleVariable = String::from("Hello World");
2. To reference a mutable variable, put mut after &. Eg. &mut samepleVariable
3. Whenever referencing a variable, it should have an '&'
4. Variables have "Ownership".
5. Constant variables needs to include "type".
6. Arrays doesn't allow dynamic sizing. Use vector instead.


<img width="1007" height="462" alt="image" src="https://github.com/user-attachments/assets/6fe8c957-e138-426e-bff1-8d13356a29e8" />


Ownership Rules
1. Each value has a variable that's called its owner.
2. There can only be one owner at a time.
3. When the owner goes out of scope, the value will be dropped.


Eg.
{ //s is not valid here. It's not yet declared
  let s = "Hello"; // s is valid from this point forward
  // do stuff with s
} // this scope is now over, and s is no longer valid
