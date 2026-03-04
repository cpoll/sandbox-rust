use std::collections::HashMap;

pub fn main() {
    let mut h = HashMap::new();

    // What you'd expect from a hashmap.
    // Note that when you insert a value, the hashmap becomes the owner (Copy excepting)
    // Default function is SipHash (DoS-resistant), but you can set others as needed

    h.insert("abc", 111);
    h.insert("def", 222);

    // get returns an Option
    println!("{}", h.get("abc").unwrap());

    // TODO: Get vs Entry. Entry returns the Entry, Get returns a ref to the value

    // Modifying values
    h.entry("def").and_modify(|n| *n += 5);

    // When you take a value out of the hashmap, you take ownership of the entire hashmap\
    //
    // let a = h.entry("abc").or_insert(5);
    // h.entry("abc").and_modify(|n| *n += 10); // error: cannot borrow h
    // println!("{a}");
    //
    // Note that h.remove() would be fine there, as it'll give up ownership of the entry.


    println!("{h:?}");
}