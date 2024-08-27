# Rust

Rust is statically typed language, which means that it must know the type of all variable at compile time.

## Rust common comands

### Manage installation

- Install
```
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```

- Check versions
```
compiler
rustc --version

Installer
rustup --version

project manager
cargo --version
```

- Update rust and helpers

```
rustup update
```

- Get rust doc

```
rust doc
```

- Compile a rust code
```
rustc <file_name>
```

- Format rust file

NOTE: This will coding as well

```
rustfmt <file_name>
```

### Cargo

- Create new cargo

```
cargo new <project_name>
```

NOTE: Run command inside project

- Run cargo

Automatically build the code and run

```
cargo run
```

- Check code error before generating binary

```
cargo check
```

- Build cargo

```
cargo build
```

- Generate project document

```
cargo doc
```


### Common terms to remember

- trait - 

- Shadowing
    - let us use the variable name rather than forcing us to create two unique variable, such as guess and guess_string.
    - the second variable will overshadow the first, taking any use of the variable name to itself until either it itself is shadowed

INFORMATION: You cannot shadow a mutable let variable because a mutable variable will hold the data type information, but a variable which is not mutable can use shadowing because the second variable will use whatever it could from the first and then store it as a new one which means a different data type can use assigned to same variable but in case of mutable variable the second one will hold the same data type, it like we can assign the new value but of the same type.

- Ownership -

- panicking - when values is greater then the value an integer type can hold, panicking occures. LOOK integers section it will explain panicking more in details.

### Keywords

- match - like a switch statement, but only works with enum return by the methods
- loop - run the piece of code, untill told otherwise
- break - break out of the code
- continue - continue to run the code
- struct
- fn - function
- let - immutable variable will be created, variable can be converted into mutable by using "mut" keyword.
- const
    - Constants aren't just immutable by default they're always immutable.
    - constants may be set only to a constant expression, not the result of a value that could only be computed at runtime.

```
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
```

- mut - muttable
- 

### Data types

#### Scaler Types

Scaler represent a single value;

- integers
    - use i in type
    - An integers is a number without a fractional component.
    - i is signed integer (can be both positive and negative)
    - u is unsigned integer (can be only positive)

    8-bit       i8      u8
    16-bit      i16     u16
    32-bit      i32     u32
    64-bit      i64     u64
    128-bit     i128    u128
    arch        isize   usize

    - signed number are stored using two's signed compliment.
    - isize and usize type depends on the architecture of the computer your program running on

INFORMATION: two's signed compliment - It is the most common method of representing signed(positive, negative and zero)

    - with --release flag, Rust does not includes checks for integer overflow that cause panics. Instead overflow occurs, Rust performs two's complement wrapping. In short, values greater than the value the type can holds "wrap around" to the minumum values the type can hold. In case of u8, the value 256 will become 0, 257 will become 1 and so on. The program won't panic, but the variable value will not be what we expect.

- floating-point
    - use f in type
    - Rust has two primitive types for floating-point numbers, which are number with decimal points
    - Rust floating point types are f32 f64, which are 32 and 64 bits in size respectively.
    - All floating-points types are signed
    - f32 are single-precision  float, and f64 has double precision.

- Numeric operation
```
fn main() {
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;
}
```

- Booleans
    - use bool in type
    - Boolean are one byte in size.

```
fn main() {
    let t = true;
    let t: bool = false; // with explicite type annotation
}
```
- Character
    - we specify char literals with single quotes, as opposed to string literals which use double.
    - Rust char type is four bytes in size and represent a Unicode Scalar Value. Which means it can represent a lot more than just ASCII. Accented letters; Chinese, Japanese, and Korean characters; emoji and zero width spaces are all valid char value in Rust.

INFORMATION: Unicode Scalar Value - Are the 21-bit codes that are the basic unit of Unicode. Each scaler value is represented by a unicode.

It ranges from U+0000 to U+D7FF and U+E000 to U+10FFFF


- Compund Types

Compound types can group multiple values into one type. Rust has two primitive compound types:

    - tuples
        - A tuple is general way of grouping together a number of values with a varity of types into one componend type. Tuples has fixed length once declared they cannot grow or shrink in size.

```
fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // using destructuring
    let (x, y, z) = tup;

    // .follwed by index
    let five_hundred = x.0;

    println!("The value of y is {y}");
}
```
        - tuple without value is called unit.

    - arrays
        - unlike tuple, every element of an array must have the same type. Array in Rust have fixed length.

        - An array isn't as flexible as the vector type, though. A vector is a similar collection type provided by the standard library that allowed to grow or shrink in size.

```
fn main() {
    let array = [1, 2, 3, 4, 5];

    let array: [i32, 5] = [1, 2, 3, 4, 5];

    let initialized = [3; 5];

    let first = array[0];
    let second = array[1];
}
```

### Functions

Rust code uses snake case as the conventional style for function and variable names, in which all letter are lowercase and underscore seperate words.

```
fn main() {
    println!("Hello, World!");

    another_function();
}

fn another_function() {
    println!("Another function");
}

fn print_label_measure(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}
```

    - Statements - are instructions that perform some action and do not return a value;

    - Expression - evaluate to a resultant value. Let's look at some examples.

NOTE: Rust is an expression based language.

    In rust statement does not return value, unlike c and ruby where assignment return the value of the assignment. And assignment in rust in rust is a statement.

    Expression can be a part of statement, like let x = 6; where 6 is a expression which evaluate to the value 6.

    calling a function and macro is an expression.

```
fn main() {
    let y = {
        let x = 5;
        x + 1
    };

    println!("The value of y is: {y}");
}
```

### Function with a return values

```

fn five() -> i32 {
    5
}

fn main() {
    let x = five();

    println!("The value of x is: {x}");
}

fn add_one(x: i32) -> i32 {
    x + 1
}

```

### Control Flow

- if


```
fn main() {
    let nuber = 3;

    if number < 5 {
        println!("Condition was true");
    } else {
        println!("Condition was false");
    }
}
```

It is also worth noting that the condition in this code must be a bool. If the condition isn't a bool, we'll get an error.

- else if

```
fn main() {
    let number = 6;

    if number % 4 == 0 {
        println!("Number is divisible by 4");
    } else if number % 3 == 0 {
        println!("Number is divisible by 3");
    } else if number % 2 = 0 {
        println!("Number is divisible by 2");
    } else {
        println!("Number is not divisible by 4, 3, or 2");
    }
}
```

- using `if` in a let `statement`

```
fn main() {
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");
}
```

NOTE: type need to same for `if` and `else` block

- Repeating code with `loop`

```
fn main() {
    loop {
        println!("Again!");
    }
}
```

- returning values from Loops

```
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    }

    println!("The result is {result}");
}
```

- loop label to disambiguate between multiple loops

```
fn main() {
    let mut count = 0;

    'counting_up': loop {
        println!("count = {count}");

        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'continue_up';
            }
            remaining -= 1;
        }
        count += 1;
    }

    println!("End count = {count}");
}
```

- conditional loop with `while`

```
fn main() {
    let mut number = 3;

    while number !=0 {
        println!("{number}");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}
```

- looping through a collection with `for`

```
fn main() {
    let a = [1, 2, 3, 4, 5, 6, 7, 8];

    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);
    }

    index += 1;

    for element in a {
        println!("the value is: {element}");
    }
}
```

```
fn main() {
    for number in (1..4).rev() {
        println!("{number}!");
    }

    println!("LIFTOFF!!!");
}
```

## Understanding Ownership

It enable rust to make memory safety guarantees without needing a garbage collector.

    - borrowing
    - slices