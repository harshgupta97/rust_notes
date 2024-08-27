# Understanding ownership

In rust the memory is manage through a system of ownership with a set of rules that the compiler checks.

```
The stack and the Heap

Both the stack and heap are part of memory available to your code to use at runtime, but they are structured in different ways. 

The stack store values in the order it gets them and remove value in the opposite order. This is refered to as last in first out. All data stored on the stack must have a known, fixed size. Data with unknown size at compile time or a size that might change must be stored on the heap instead.

Data with an unknown size at compile time or a size that might change must be stored on the heap instead.

The heap is less organized: when you put data on the heap, you request a certain amount of space. The memory allocator finds an emtpty big enough, mark it as being in use, adn returns a pointer, which is the address of that location. This process is called allocating on the heap and is sometimes abbreviated as just allocating(pushing value onto stack is not considered allocating). Because the pointer to the heap is a known, fixed size, you can store the pointer on the stack, but when you want the actual data, you must follow the pointer.

 Think of being seated at a resturant. when you enter, you state the number of people in your group, and the host finds an empty table that fits everyone and leads you there. If someone in your group comes late, they can ask where you've been seated to find you.

 Pushing to the stack is faster that allocating on the heap because the allocator never has to search for place to store new data; that location is always at the top of the stack. Comparatively, allocating space on the heap requires more work because the allocator must first a big enough space to hold the data and then perform bookkeeping to prepare for the next allocation.

Accessing data in the heap is slower than accessing data on the stack because you have to follow a pointer to get there. Contemporary processor are faster if they jump around less in memory. Continuing the analogy, consider a server at resturant taking order from many table. Taking an order from table A, then an order from table B, then one from A again, and then one from B again would be a much slower process. By the same token, a processor can do its job better if it work on data that's close to other data (as it is on the stack) rather then farther away(as it can be on the heap).

When your code calls a function, the values passed into the function (including, potentially, pointer to data on the heap) and the function's local variable get pushed onto stack. When the function is over, those values get popped off the stack.

Keeping track of what part of code are using what data on the heap, minimizing the amount of duplicate data on the heap, and cleaning up unused data on the heap so you don't run out of space are all the problem that ownership addresses. Once you understand ownership, you won't need to think about the stack and the heap very often, but knowing that the main purpose of ownership is to manage heap data can help explain why it works the way it does.
```

### Ownership Rules

- Each value in Rust has an owner.
- There can only be one owner at a time.
- When the owner goes out of scope, the value will be dropped.

### Variable Scope

A scope is the range within a program for which an item is valid.

```
let s = "hello";
```

```
{                           // s is not valid here, it's not yet declared
    let s = "hello"         // s is valid from this point forward

// do stuff with s
}                           // this scope is now over, and s is no longer valid
```

- When s comes into scope, it is valid.
It remains valid until it goes out scope.

### The `String` Type

String literals are convenient, but they aren't suitable for every situation in which we may want to use text. One reson is that they're immutable. Another is that not every String value can be known when we write our code: for example, ehat if wen want to take user input and store it? For these situations, Rust has a second string type, String. This type manages data allocated on the heap as such is able to store an amount of text that is unknown to us at compile time. You can create a `String` from string literal using the `from` function, like so:

```
let s = String::from("hello");
```

The double colon `::` operator allow us to namespace this particular `from` function under the String type rather than using some sort of name like `string_from`.

```
let mut s = String::from("hello");

s.push_str(", world!");

println!("{}", s); // this will print `hello, world!`
```

Why can `String` be mutated but literals cannot? The difference is in how two types deals with memory.

### Memory and Allocation


### Variables and Data Interaction with Move

### Variables and Data Interaction with Clone

### Stack-Only Data: Copy

### Ownership and Functions

### Return Values and Scope

## Reference and Borrowing

```
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
```

These ampersands represent references, and they allow you to refer to some value without taking ownership of it.

When function have references as parameters instead of the actual values, we won't need to return the values in order to give back ownership, because we never had ownership.

We call the action of creating a reference `borrowing`.

### Mutable references

```
fn main() {
    let mut s = String::from("hello");

    change(&mut s);
}

fn change(string: &mut String) -> usize {
    string.push_str(", world");
}
```

Mutable references have one big restriction: if you have a mutable reference to a value, you can have no other references to that value.

```
let mut s = String::from("hello");

let r1 = &mut s;

let r2 = &mut s;

println!("{}, {}", r1, r2);
```

Above code will give error.

The restriction preventing multiple references to the same data at the same time allows for mutation but in very controlled fashion.

The benefit of having this restriction is that Rust can prevent data races at compile time. A data race is similar to a race condition and happens when these three behaviors occure:

- Two or more pointers access the same data at the same time.
- At least one of the pointer is being used to write to the data.
- There's no mechanism being used to synchronize access to that data.

Data races cause undefined behaviour and can be difficult to diagnose and fix when you're trying to track them down at runtime; Rust prevents this problem by refusing to compile code with data races!

As always, we can use curly brackets to create a new scope, allowing for multiple mutalbe references, just not simultaneous onces:

```
// correct way
let mut s = String::from("hello");

{
    let r1 = &mut s;
}

let r2 = &mut s;
```


```
// wrong
let mut s = String::from("hello");

let r1 = &s;
let r2 = &s;
let r3 = &mut s;

println!("{}, {}, and {}", r1, r2, r3);

// correct
let mut s = String::from("hello");

let r1 = &s;
let r2 = &s;
println!("{}, {}", r1, r2);

let r3 = &mut s;

println!("{}"r3);
```

### Dangling Refernces

In languages with pointers, it's easy to erroneously create a dangling pointer - a pointer that references a location in memory that may have been given to someone else- by freeing some memory while preserving a pointer to that memory. In Rust, by contrast, the compiler guarantees that references will never be dangling references: if you have a reference to some data, the compile will ensure that the data will not go out of scope before the reference to the data does.

```
fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String {
    let s = String::from("hello");

    &s
}
```

### The Rules of References

- At any given time, you can have either one mutable reference or any number of immutable references.

- References must always be valid.


## The Slice Type

Slice let you reference a contiguous sequence of elements in a collection rather than the whole collection. A slice is a kind of reference, so it does not have ownership.

Write a function that takes a string of words seperated by spaces and returns the first word it finds in that string.

```
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}
```

With the help of above code we a way to find out the index of the end of the first word in the string, but there's a problem. We are returning a usize on its own, but it's only a meaningful number in the context of the &String. In other words, because it's a seperate value from the &String, there's no guarantee that it will still be valid in the future.

Consider below code:

```
fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s);

    s.clear();

    // word still has the value 5 here, but there's no more string we could meaningfully use the value 5 with

```

### String Slices

A string slices is a reference to part of a String, and it looks like this:

```
let s = String::from("hello world");

let hello = &s[0..5];
let world = &s[6..11];
```

We created a slices using a range within brackets by specifying `[starting_index..ending_index]`, 

```
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
```

We now have a straightforward API that's much harder to mess up because the compiler will ensure the reference into the String remain valid. Remember the bug in the previous program when we got the index to the end of the first word but then cleared the string so our index is invalid? That code was logically incorrect but didn't show any immediated errors. The problems would show up later if we kept trying to use the first word index with an emptied string. Slices make this bug impossible and let us know we have a problem with our code much sooner.Using the slice version of the code it will throw a compile-time error

```
fn main() {
    let mut s = String::from("Hello world");

    let word = first_word(&s);

    s.clear();

    println!("The first word is: {}", word);
}
```

Rust disallow the mutable reference in clear and the immutable reference in word from existing at the same time, and compilation fails.

### String Literals as Slices

```
let s = "Hello, world!";
```

string literal are immutable, &str is an immutable reference.


### String Slices as Parameters

```
fn first_word(s: &str) -> &str {}
```

By replacing the method parameter with the above one, it will allow us to use the same function on both &String values and &str values.



```
fn main() {

    // String
    let s = String::from("Hello World");

    let word = first_word(&s[0..6]);
    let word = first_word(&s[..]);

    let word = first_word(&s);

    // string literal
    let sl = "Hello World";

    let word = first_word(&sl[0..6]);
    let word = first_word(&sl[..]);

    let word = first_word(&sl);

}
```