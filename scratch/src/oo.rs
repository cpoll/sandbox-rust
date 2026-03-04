pub fn main() {



    // Structs and Methods
    pub struct Article {
        pub title: String,
        pub author: String,
        pub content: String,
    }

    impl Article {
        fn get_content_snippet(&self) -> String {
            self.content[0..10].to_string()
        }
    }

    let a = Article {
        title: String::from("101 Uses for Structs"),
        author: String::from("Cristian"),
        content: String::from("Lorem ipsum dolor sit amet"),
    };

    println!("{}", a.get_content_snippet());

    // Traits
    pub trait Summary {
        fn get_author(&self) -> String;
        fn summarize(&self) -> String;

        // Traits can have default implementations
        fn get_author_first_name(&self) -> String {
            self.get_author().split_whitespace().next().unwrap_or("").to_string()
        }
    }

    impl Summary for Article {
        fn summarize(&self) -> String {
            format!("Article {} by {}", self.title, self.author)
        }

        fn get_author(&self) -> String {
            self.author.clone()
        }
    }

    let a = Article {
        title: String::from("101 Uses for Structs"),
        author: String::from("Cristian Poll"),
        content: String::from("Lorem ipsum"),
    };

    println!("{}", a.summarize());
    println!("{}", a.get_author_first_name());

}