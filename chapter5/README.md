# Defining and Instantiating Structs

Structs asre similar to tuples, both holds multiple related values. Like tuples, the pieces of a struct can be different types. Unlike with tuples, in a struct you'll name each peice of data so it's clear what the value means. Add these names means that structure are more flexible than tuples: you don't have to rely on the order of the data to specify or access the values of an instance.

- Define `struct`

```
struct User {
    active: bool;
    username: String;
    email: String;
    sing_in_count: u64,
}
```

- to use a `struct` after we define it, we create an instance of that struct by specifying concrete values for each fields.

```
fn main() {
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sing_in_count: 1
    }
}
```

- to access specific value from a `struct`.

```
fn main() {

    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sing_in_count: 1
    };

    user1.email = String::from("anotheremail@example.com");
}
```


- return a `struct`

```
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sing_in_count: 1
    }
}
```

- using the field Init Shorthand

```
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sing_in_count: 1
    }
}
```

Creating Instance from other Instance with Struct Update Syntax

```
fn main() {

    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sing_in_count: 1
    };

    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("someone@example.com"),
        sing_in_count: user1.sing_in_count
    }
}

or

fn main() {

    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sing_in_count: 1
    };

    let user2 = User {
        email: String::from("someone@example.com"),
        ..user1
    }
}
```

The syntax `..` specifies that the remaining fields not explicitly set should have the same value as the fields in the given instance.

- tuples structs

```
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}
```

- unit-like structs

Unit-like structs can be useful  when you need to implement a trait on some type but don't have any data that you want to store in the type itself.

```
struct AlwaysEqual;

fn main() {
    let subject = AlwaysEqual
}
```

