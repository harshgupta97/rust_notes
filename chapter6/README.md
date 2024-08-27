# Enums

```
enum IPAddrKind {
    V4,
    V6
}

fn main() {
    let four = IPAddrKind::V4;
    let siz = IPAddrKind::V6;
}

fn route(ip_type: IPAddrKind) {}

route(IPAddrKind::V4);
route(IPAddrKind::V6);
```


```
enum IPAddrKind {
    V4,
    V6
}

struct IPAddr {
    kind: IPAddrKind,
    address: String
}

let home = IPAddr {
    kind: V4,
    address: String::from("127.0.0.1"),
}

let loopback = IPAddr {
    kind: V6,
    address: String::from("::1"),
}
```

Another way to represent the above, just using enum

```
enum IPAddr {
    V4(String),
    V6(String),
}

let home = IPAddr::V4(String::from("127.0.0.1"))

let loopback = IPAddr::V6(String::from("::1"));
```

### Defining Methods on enums

```
enum Message {
    Quit,
    Move{ x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32), 
}

impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}

let m = Message::Write(String::from("hello"));
m.call();
```

### The `option` Enum and Its Advantages Over Null Values

NOTE: In his 2009 presentation "Null References: The Billion Dollar Mistake," Tony Hoare, the inventor of null.

The problem with null value is that if you try to use a null value as a not-null, you'll get an error of some kind. Because this null or not-null property is passive, it's extremely easy to make this kind of error.

However, the concept that null is trying to express is still a useful one: a null is a value that is currently invalid or absent for some reason.

```
enum option<T> {
    None,
    Some(T)
}
```

The `Option<T>` enum is so useful that it's even included in the prelude; you don't need to bring it into scope explicitly. Its variants are also included in the prelude: you can use `Some` and None directly without the `Option::` prefix. The `Option<T>` enum is still just a regular enum, and `Some(T)` and `None` are still variants of types `Option<T>`.

```
let x: i8 = 9;
let y: option<i8> = Some(5);

let sum = x + y;
```

## The `match` Control Flow Constructor

Rust has an extremely powerful control flow construct called `match` that allow you to compare a value against a series of pattern and thenn execute code based on which pattern matches. Patterns can be made up of leteral values, variable names, wildcards, and many other things.

The power of `match` comes from the expressiveness of the patterns and the fact that the compiler that all possible cases are handled.

```
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```

### Patterns That Bind to Values
Another useful feature of match arms is that they can bind to the parts of the value that match the pattern. This is how we can extract values out of enum vairants.

```
#[derived(Debug)]

enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}
```

If we were to call `value_in_cents(Coin::Quarter(UsState::Alaska))`

### Matching with `Option<T>`



