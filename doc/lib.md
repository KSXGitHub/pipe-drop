# Purpose

Sometimes it is critical to drop an object as soon as it is no longer used, even before the end of scope.
For example: Dropping a `MutexGuard` to prevent deadlock.

The [`PipeDrop`] trait provides methods that allow you to get a reference (immutable or otherwise) to an object, do something with the reference (inside the callback function), and then drop the object.

# Exhibit A: Drop order

**Given the following code which declare an object that logs its own creation and destruction:**

```rust
#[derive(Debug)]
struct Object {
    id: usize,
}

impl Default for Object {
    fn default() -> Self {
        println!("create 0");
        Object { id: 0 }
    }
}

impl Drop for Object {
    fn drop(&mut self) {
        println!("drop {}", self.id)
    }
}

impl Clone for Object {
    fn clone(&self) -> Self {
        let id = self.id + 1;
        println!("create {}", id);
        Object { id }
    }
}
```

**The following code would only drop `a` and `b` when they are out of scope:**

```rust
# #[derive(Debug)]
# struct Object {
#     id: usize,
# }
#
# impl Default for Object {
#     fn default() -> Self {
#         println!("create 0");
#         Object { id: 0 }
#     }
# }
#
# impl Drop for Object {
#     fn drop(&mut self) {
#         println!("drop {}", self.id)
#     }
# }
#
# impl Clone for Object {
#     fn clone(&self) -> Self {
#         let id = self.id + 1;
#         println!("create {}", id);
#         Object { id }
#     }
# }
let a = Object::default();
let b = a.clone();
let c = b.clone();
println!("-- end of scope --");
```

_Output:_

```text
create 0
create 1
create 2
-- end of scope --
drop 2
drop 1
drop 0
```

In the above output, the numbers (`0`, `1`, `2`) correspond to the variables (`a`, `b`, `c` respectively), and the verbs (`create` and `drop`) corresponds to actions (`Object::default()`/`Object::clone()` and `drop` respectively).

**In order to force `a` and `b` to be dropped before the end of scope, we must explicitly call [`drop`]:**

```rust
# #[derive(Debug)]
# struct Object {
#     id: usize,
# }
#
# impl Default for Object {
#     fn default() -> Self {
#         println!("create 0");
#         Object { id: 0 }
#     }
# }
#
# impl Drop for Object {
#     fn drop(&mut self) {
#         println!("drop {}", self.id)
#     }
# }
#
# impl Clone for Object {
#     fn clone(&self) -> Self {
#         let id = self.id + 1;
#         println!("create {}", id);
#         Object { id }
#     }
# }
let a = Object::default();
let b = a.clone();
drop(a);
let c = b.clone();
drop(b);
println!("-- end of scope --");
```

_Output:_

```text
create 0
create 1
drop 0
create 2
drop 1
-- end of scope --
drop 2
```

As you can see in the output above, both `drop 0` and `drop 1` were called before `-- end of scope --`, which means that `a` and `b` were dropped before the end of scope.

**However, explicitly calling `drop` makes our code ugly, and quite hard to fit in a dot-chain, we can fix this by using [`PipeDrop`]:**

```rust
# #[derive(Debug)]
# struct Object {
#     id: usize,
# }
#
# impl Default for Object {
#     fn default() -> Self {
#         println!("create 0");
#         Object { id: 0 }
#     }
# }
#
# impl Drop for Object {
#     fn drop(&mut self) {
#         println!("drop {}", self.id)
#     }
# }
#
# impl Clone for Object {
#     fn clone(&self) -> Self {
#         let id = self.id + 1;
#         println!("create {}", id);
#         Object { id }
#     }
# }
use pipe_drop::PipeDrop;
let c = Object::default() // a
    .pipe_ref_drop(Object::clone) // b
    .pipe_ref_drop(Object::clone);
println!("-- end of scope --");
```

_Output:_

```text
create 0
create 1
drop 0
create 2
drop 1
-- end of scope --
drop 2
```

The above output is exactly like when we called `drop` explicitly!
