


pub fn main() {
    // Vectors are dynamic arrays that put all values next to each other in memory
    // The consequence of this is that sometimes the entire vector needs to be moved to a different
    // memory location. In practice, this means that pushing to the array mutably borrows the array, and borrowing an element
    // of the array means borrowing the entire array.
    // Does it allocate a contiguous block, and re-allocate if it fills up?
    let _v: Vec<i32> = Vec::new();

    // You can use the vec! macro to quickly construct a vector:
    let _v = vec![1, 2, 3];

    // Operations you'd expect
    let mut v = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    println!("{:?}", v);

    // You can get values by indexing or by get().
    // We're doing an immutable borrow here, but you can do a mutable one,
    // or just the value, since i32 implements the Copy trait.
    let i = &v[1];
    println!("{i}");

    // get() returns an option
    let i: Option<&i32> = v.get(2);
    match i {
        Some(n) => println!("{n}"),
        None => println!("No element at index"),
    }

    // Note that a borrow of an element is a borrow of the entire vector
    // push() also counts as mutable borrowing of the vector
    //
    // let i = &v[0];
    // v.push(4); // cannot borrow `v` as mutable because it is also borrowed as immutable
    // println!("{i}");
    //
    // In this case, we can fix this by using i32's copy: let i = v[0];
    // That's not always an option, so we could also make sure we're doing using i before we do v.push() again.
    //
    // The rust book explains why:
    // The code in Listing 8-6 might look like it should work: Why should a reference to the first element care about changes at the end of the vector?
    // This error is due to the way vectors work: Because vectors put the values next to each other in memory, adding a new element onto the end of the vector
    // might require allocating new memory and copying the old elements to the new space, if there isn’t enough room to put all the elements next to each other
    // where the vector is currently stored. In that case, the reference to the first element would be pointing to deallocated memory.
    // The borrowing rules prevent programs from ending up in that situation.

    // You can loop
    for i in &v {
        println!("{i}");
    }

    // You can make changes to elements in a mutable vector.
    // This'll look a bit weird:
    let mut v = vec![1, 2, 3];
    for i in &mut v {
        // We're using * to deference and get the value in i. See Chapter 15.
        *i += 2;
    }
    println!("{:?}", v);

}

fn string_scratch() {

}