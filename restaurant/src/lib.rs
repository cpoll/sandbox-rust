// Load the file into the module tree. We only need to do this here, all other files in the module will now be aware of front_of_house
mod front_of_house;

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

    // Import
    // This is idiomatic: We import the parent module to make it clear that the function isn't locally defined.
    // Note however that for importing structs or enums, it's idiomatic to bring them in directly.
    use crate::front_of_house::hosting;
    hosting::add_to_waitlist();
}

// You can re-export methods. Now add_to_waitlist can be used as either restaurant::front_of_house::hosting::add_to_waitlist()
// or restaurant::hosting::add_to_waitlist()
pub use crate::front_of_house::hosting;