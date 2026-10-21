

Run `cargo run` and look at the output.

Look at the first error and try to understand what it say


```rustdoc
error[E0382]: use of moved value: `user`
  --> module2/rust_app/src/main.rs:8:5
   |
 2 |     let user = User {
   |         ---- move occurs because `user` has type `User`, which does not implement the `Copy` trait
...
 6 |     say_hello(user);
   |               ---- value moved here
 7 |
 8 |     user.update_name("John");
   |     ^^^^ value used here after move
   |
note: consider changing this parameter type in function `say_hello` to borrow instead if owning the value isn't necessary
  --> module2/rust_app/src/main.rs:11:20
   |
11 | fn say_hello(user: User) {
   |    ---------       ^^^^ this parameter takes ownership of the value
   |    |
   |    in this function
note: if `User` implemented `Clone`, you could clone the value
  --> module2/rust_app/src/main.rs:15:1
   |
 6 |     say_hello(user);
   |               ---- you could clone this value
...
15 | struct User {
   | ^^^^^^^^^^^ consider implementing `Clone` for this type
```

"use of moved value: `user`" translates to it complaining that a variable "user" was used twice.

Is it illegal to use a variable twice in rust ?! No, but also yes. It is allowed, but rust has a lot of guard rails, and here you see one of the most fundamental ones

To understand why you sort of need to read some documentation: https://doc.rust-lang.org/stable/book/ch04-00-understanding-ownership.html  . BUT FIRST, LETS LOOK AT THE REST OF THE COMPILE OUTPUT!

There is two "note:" sections, and these "notes" often straight up gives you a solution. For this task there are two possible solution to the same thing. 

### Note 1
```
note: consider changing this parameter type in function `say_hello` to borrow instead if owning the value isn't necessary
```
Just like an operating system often does not like two programs having write access to the same file at the same time, rust does not like it when many people have "write" to the same variable at the same time. But "read" is more flexible. As many as you want can read a variable at the same time.
A borrow is simply this. It is a type of "read only"/"borrow"/"refference" way to access the variable. It is not owned, but just being lent away.
(https://doc.rust-lang.org/stable/book/ch04-02-references-and-borrowing.html)

So a possible solution is to see if the first use of the variable really need to use the real value, or if a simple refference is enough. The &User would be a borrow/refference

### Note 2
```
note: if `User` implemented `Clone`, you could clone the value
```
Just do a deepcopy! Simple and easy!

But the message say nothing about how to implement this Clone thing. It is sort of so basic that it is not explained... 

Almost every class / struct in Rust can be deepcloned, but it is not on by default. There exist an anontation that will create a clone function for you.

Just slapp on a `#[derive(Clone)]` above the struct, and you can now call user.clone()

This might feel like a sheat, or shortcut. And it kind of is. But is fully allowed to use it a lot when new to rust. Start to use more borrows when you feel a bit more confident. 

# Next error

error[E0594]: cannot assign to `self.name`, as `self` is not declared as mutable

The solution to the mutable issue is hidden in the compile warning "help:"
