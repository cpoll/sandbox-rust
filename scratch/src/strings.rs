fn main() {
    // str is the string slice
    // &str is the borrowed form of that slice
    // String is a growable, mutable, owned, UTF-* string

    // TODO: There's some interesting stuff with Deref, coercion, and &*my_string

    // This is an str
    let s = "hello world";

    // This is a String
    let s = "hello world".to_string();
    let s = String::from("hello world");

    // You can modify strings
    // Note that push_str takes a string slice, and doesn't take ownership of world
    let mut world = String::from("world");
    let mut s = String::from("hello");
    s.push_str(world.as_str()); // Same as world[..], as_str() returns a slice.
    // TODO: We can do &world; what does this imply? Immutably borrow? How do we mutably borrow, can we make an error this way?

    // You can push chars directly
    s.push('!');

    // You can use the + operator
    // Note that the signature of + is add(self, s: &str) -> String
    // Which is why s1's ownership changes
    let s1 = String::from("hello");
    let s2 = String::from("world");
    let s3 = s1 + " " + &s2;
    println!("{s2} {s3}");
    // println!("{s1}"); // Error: Value used after being moved

    // format!
    let s1 = "hello";
    let s3 = format!("{s1} {s2}");

    // Rust strings don't support indexing, because indexing is a memory thing in Rust, and
    // UTF-8 strings have dynamic character lengths.
    // Slicing works, but remember that you're taking bytes, so you might wind up with some unexpected behaviour

    // If you want individual chars, the best way is .chars()
    for c in "Зд".chars() {
        println!("{c}");
    }



}