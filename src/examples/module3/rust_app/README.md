
# Modul 3: Vanlige Data types


# Challenge
Expand the functionality of the User struct a little bit:

1. Add an optional age property
2. Add a new update_age function to update a users age with a new value
3. Add a whats_my_age() function to return the users age

If you're struggling, you can find a solution on GitHub. Try it on your own first, if you're finding it difficult that's good. It means you're learning.

Working with enums: 

4. Change User to be an enum, that is either a "Standard" user or a "Premium" user. (place all user data that used to be inside "User" into a struct called UserDetails)

# Hint og teori
* Se på module3 teori https://rustfor.net/docs/category/data-types
* Se på "the book" https://doc.rust-lang.org/stable/book/ch03-02-data-types.html

* Fra et "Java perspektiv"
  * Nye typer ser rare ut? Se hvordan de er i forhold til Java klasser her : https://chrischiedo.github.io/rust-for-java-devs/language/data-types.html
  * Hvorfor er det ingen "return" i User::new funksjonen? Se svar i [https://chrischiedo.github.io/rust-for-java-devs/language/data-types.html ](https://chrischiedo.github.io/rust-for-java-devs/language/functions.html)
  * Kompileringsfeil som snakker om "lifetime" eller "ownership"? Se [her](https://chrischiedo.github.io/rust-for-java-devs/memory-management/index.html), 
