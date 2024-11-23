# Ownership

## Ownership Rules

Keep these simple rules in mind as we work through examples that illustrate them:

- Each value in Rust has an **owner**.
- There can only be one owner at a time.
- When the owner goes out of scope, the value is dropped (its memory is automatically deallocated).

For example, consider the following code snippet:

```rust
let s = String::from("Hello");
```

Here, the variable `s` is the owner of the string `"Hello"`. When `s` goes out of scope, the string `"Hello"` will be dropped, meaning its memory will be freed.

### What Does "One Owner at a Time" Mean?

Now, let's explore what it means when we say "there can only be one owner at a time."

Consider this code snippet (Snippet 1):

```rust
fn main() {
    let s1 = String::from("Hello");
    let s2 = s1;

    println!("{}, {}", s1, s2); // error here
}
```

This code won't compile! If you're new to Rust, this might seem confusing at first, but let's break it down using the ownership rules.

1. First, we create a string `"Hello"` and assign it to `s1`. According to Rule 1, `s1` is the owner of `"Hello"`.
2. Then, we create a new variable `s2` and assign `s1` to it. According to Rule 2, there can only be one owner at a time. So, when we write `s2 = s1`, Rust **moves** the value owned by `s1` to `s2`. After this, `s1` is no longer valid—it has been left in an "undefined state."

When we try to use `s1` in the `println!` statement, Rust throws a compile-time error because `s1` no longer owns any value.

---

Now, compare this with the following code (Snippet 2):

```rust
fn main() {
    let num1 = 42;
    let num2 = num1;

    println!("{}, {}", num1, num2);
}
```

This code compiles without any issues. Why? Because in this case, `num2 = num1` **copies** the value owned by `num1` to `num2`. This means both `num1` and `num2` have their own copies of the value `42`.

The difference lies in the type of the data. Primitive types like integers (`i32`, `u32`, etc.) implement the `Copy` trait, so their values are copied instead of moved. For complex types like `String`, which do not implement the `Copy` trait, ownership is transferred (moved) instead.

# Move Semantics and the `Copy` Trait

You might still be wondering why Snippet 2 compiles while Snippet 1 does not. The formal reason is that `i32` implements the `Copy` trait, whereas `String` does not.

If a data structure implements the `Copy` trait, its value will be **copied** when assigned to another variable or passed as a function parameter. On the other hand, if the `Copy` trait is not implemented, the value will be **moved**, transferring ownership instead.

### What Data Structures Implement the `Copy` Trait?

The following types implement the `Copy` trait:

- **Primitive types**: `char`, `bool`, integers (`i8`, `i16`, ...), unsigned integers (`u8`, `u16`, ...), floating-point numbers (`f32`, `f64`), `usize`, and `isize`.
- **Function pointers** (e.g., `fn() -> i32`).
- **Raw pointers** (e.g., `*const String`, `*mut String`).
- **Immutable references** (e.g., `&String`, `&[u8]`).
- **Arrays, tuples, or structs** where all elements implement the `Copy` trait (e.g., `[u8; 4]`, `(u8, &str, f32)`, or `struct MyStruct { a: u8, b: usize }`).

### What Data Structures Do Not Implement the `Copy` Trait?

These types do not implement the `Copy` trait:

- **Dynamically sized types (DSTs)**: such as `str`, `[u8]`, `Vec<u8>`, or `String`.
- **Mutable references**: (e.g., `&mut String`).
- **Arrays, tuples, or structs** with at least one element that does not implement the `Copy` trait\*\*: (e.g., `[Vec<u8>; 4]`, `(u8, String)`, or `struct MyStruct { a: String, b: usize }`).

### Why Does This Matter?

The distinction between copying and moving is central to Rust's memory safety guarantees. Types that implement the `Copy` trait are small and inexpensive to duplicate, making it safe to copy them automatically. However, for more complex types like `String` or `Vec`, Rust moves ownership to prevent unexpected behavior, ensuring there is always a clear owner responsible for cleanup.

# What If We Don't Want a Move to Happen?

Consider the following example:

```rust
fn main() {
    let nums = vec![2, 7, 8, 9, 13, 21];
    let target = 8;

    if let Some(pos) = find_pos(nums, target) {
        println!("Data {} found at position {}", target, pos);
    }

    // Print the array
    nums.iter().for_each(|x| print!("{}, ", x));
}

fn find_pos(s: Vec<i32>, num: i32) -> Option<usize> {
    for (idx, elem) in s.iter().enumerate() {
        if *elem == num {
            return Some(idx);
        }
    }
    None
}
```

This code won't compile. Why? Let's analyze it using Rust's ownership rules.

The issue lies in the value of `nums` (defined on line 2). When we call the `find_pos` function on line 5, the `nums` value is **moved** into the function parameter `s`. This happens because `Vec` does not implement the `Copy` trait. As a result, the ownership of `nums` is transferred, and we can no longer access it on line 10.

### How Can We Fix This?

#### Solution 1: Clone the Data

The simplest fix is to make a copy of `nums` and **move** the copy into the `find_pos` function. Update line 5 as follows:

```rust
if let Some(pos) = find_pos(nums.clone(), target) {
```

This works, but it has two significant drawbacks:

1. Copying a data structure can be expensive or even impossible for complex or large types.
2. Sometimes, we need to modify the original data structure outside of the function, so making a copy doesn't solve the problem.

#### Solution 2: Use References

If you're familiar with Java or C++, you might recognize the concept of **references**. Rust has a similar concept but with strict rules. Let's see how references can help in this case.

We can modify the `find_pos` function to take a reference to `nums` instead of owning it. Update line 13 as follows:

```rust
fn find_pos(s: &Vec<i32>, num: i32) -> Option<usize> {
```

Now, `s` is a reference to the `Vec<i32>` rather than owning it.

Next, update the function call on line 5:

```rust
if let Some(pos) = find_pos(&nums, target) {
```

Here, we're passing a reference to `nums` using the `&` symbol. With these changes, the code compiles successfully!

---

### Mutable References

Now, let's try modifying the `find_pos` function to push a new element to the end of the `nums` vector. Add this line to the function at line 19:

```rust
s.push(5);
```

The code won't compile, and you'll see an error message like this:

```txt
s.push(5);
  ^^^ `s` is a `&` reference, so the data it refers to cannot be borrowed as mutable
```

The compiler even offers a helpful suggestion:

```txt
help: consider changing this to be a mutable reference
 fn find_pos(s: &mut Vec<i32>) {
                  +++
```

With this information, we can conclude:

1. `&` is a **read-only reference**: you can only read the data it refers to.
2. `&mut` is a **mutable reference**: you can both read and modify the data it refers to.

---

### Key Takeaways About References

1. A reference does not own the data it refers to, which aligns with Rust's second ownership rule: only one owner exists at a time.
2. References allow you to **borrow** data (either immutably with `&` or mutably with `&mut`) without transferring ownership.

By using references, you can avoid unnecessary moves while maintaining Rust's strict safety guarantees.

# Borrowing Rules

Rust is a programming language with a strong emphasis on memory safety. However, if we don't impose additional constraints on the behavior of references, we could face the same memory safety issues as in C++. To prevent this, Rust enforces the following borrowing rules.

---

## Rule 1: The Value Under the Reference Must Live Long Enough (No Dangling References)

Rust does not allow dangling references—references to data that no longer exists. Consider the following example:

```rust
fn main() {
    let reference_to_nothing = dangle();
}

fn dangle<'a>() -> &'a String {
    let s = String::from("hello");
    &s
}
```

This code won't compile. The problem lies in the `dangle` function, which tries to return a reference to the local variable `s`. Once the `dangle` function ends, `s` is dropped, leaving the reference (`&s`) pointing to invalid memory. Here's a breakdown of the issue:

```rust
fn dangle() -> &String { // dangle returns a reference to a String

    let s = String::from("hello"); // s is a new String

    &s // we return a reference to the String, s
} // Here, s goes out of scope, and is dropped. Its memory goes away.
  // Danger!
```

Now, let's examine a slightly more complex example:

```rust
fn main() {
    let mut data: Vec<&i32> = Vec::new();
    push_dangling_ref(&mut data);
}

fn push_dangling_ref(data: &mut Vec<&i32>) {
    let v = 42;
    data.push(&v);
}
```

This code also fails to compile for the same reason. Let's break down the error message:

```txt
fn push_dangling_ref(data: &mut Vec<&i32>) {
                                     - let's call the lifetime of this reference `'1`
    let v = 42;
        - binding `v` declared here
    data.push(&v);
    ----------^^-
    |         |
    |         borrowed value does not live long enough
    argument requires that `v` is borrowed for `'1`
}
- `v` dropped here while still borrowed
```

Here, `v` is dropped at the end of `push_dangling_ref`, but `data.push(&v)` tries to store a reference to it. Rust's compiler prevents this, saving us from subtle bugs that might take hours to debug in C++.

---

## Rule 2: A Value Can Have Multiple Immutable References at the Same Time

Immutable references allow shared, read-only access. For example:

```rust
fn main() {
    let mut s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    println!("{}, {}", r1, r2);
}
```

This code compiles successfully and outputs `hello, hello`. Multiple immutable references to the same value are safe because they only allow reading, not modification.

---

## Rule 3: A Value Can Have Only One Active Mutable Reference at a Time

Mutable references provide exclusive, writable access to a value. Consider this example:

```rust
fn main() {
    let mut s = String::from("hello");
    let r1 = &mut s;
    let r2 = &mut s; // ERROR
    println!("{}, {}", r1, r2);
}
```

This code won't compile because `r1` and `r2` are both active mutable references to `s` at the same time.

This rule helps prevent subtle errors. For instance, consider the following example:

```rust
fn main() {
    let mut data = vec![1, 2, 3, 4, 5];

    for item in data.iter_mut() {
        data.push(*item * 2);
    }
}
```

This code attempts to double each element in `data` and append the results back to the same vector. Here's an equivalent Python example:

```python
data = [1, 2, 3, 4, 5]
for item in data:
    data.append(item * 2)
```

The Python code runs infinitely because `data` grows with each iteration. In Rust, the compiler detects this issue at compile time, throwing the following error:

```txt
for item in data.iter_mut() {
            ---------------
            |
            first mutable borrow occurs here
            first borrow later used here
    data.push(*item * 2);
    ^^^^ second mutable borrow occurs here
}
```

Now consider this example:

```rust
fn main() {
    let mut s = String::from("hello");
    let r1 = &mut s;
    r1.push_str(" world");

    let r2 = &mut s;
    r2.push('!');

    println!("{}", r2);
}
```

This code compiles successfully because `r1` becomes inactive after `" world"` is pushed to `s`. Rust ensures only one active mutable reference exists at any given time.

---

## Rule 4: A Value Cannot Have Mutable and Immutable References at the Same Time

Mutable and immutable references cannot coexist. For example:

```rust
fn main() {
    let mut s = String::from("hello");
    let r1 = &s;
    let r2 = &mut s; // ERROR
    println!("{}, {}", r1, r2);
}
```

This code won't compile because `r1` is an immutable reference, while `r2` is a mutable reference. This rule prevents accidental modifications while immutable references are in use.

Here's another example:

```rust
fn main() {
    let mut data = vec![1, 2, 3];
    let data1 = &data[0];
    for i in 0..10000 {
        data.push(i);
    }
    println!("data1 = {}", data1);
}
```

In this program, `data1` is an immutable reference to the first element of `data`. Later, we push many new elements into `data`. If `data` reallocates during the push operations, the reference to its first element (`data1`) could become invalid. Rust's compiler detects this issue at compile time, avoiding potential runtime bugs.

---

## Practical Example: Making Rule 3 Work

To fix the issue in the earlier mutable reference example, you can restructure the loop like this:

```rust
fn main() {
    let mut data = vec![1, 2, 3, 4, 5];

    let len = data.len();

    for i in 0..len {
        data.push(data[i] * 2);
    }
}
```

This ensures only one mutable reference is active at a time, allowing the code to compile successfully.
