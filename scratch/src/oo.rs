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

    // Traits (are just interfaces)
    pub trait Summary {
        fn get_author(&self) -> String;
        fn summarize(&self) -> String;

        // Traits can have default implementations
        fn get_author_first_name(&self) -> String {
            self.get_author().split_whitespace().next().unwrap_or("").to_string()
        }
    }

    impl Summary for Article {
        fn get_author(&self) -> String {
            self.author.clone()
        }

        fn summarize(&self) -> String {
            format!("Article {} by {}", self.title, self.author)
        }
    }

    let a = Article {
        title: String::from("101 Uses for Structs"),
        author: String::from("Cristian Poll"),
        content: String::from("Lorem ipsum"),
    };

    println!("{}", a.summarize());
    println!("{}", a.get_author_first_name());


    // You can use traits as parameters, i.e. specify functions that take a trait as a type
    pub fn display_summary(article: &impl Summary) {
        println!("Summary: {}", article.summarize());
    }
    display_summary(&a);

    // Note: &impl is just syntactic sugar for:
    pub fn display_summary_2<T: Summary>(article: &T) {
        println!("Summary: {}", article.summarize());
    }

    // You can use multiple trait bounds:
    pub trait Document {}
    pub trait Paper {}
    fn foo(item: &(impl Summary + Document)) {}

    // This can get a bit messy, so you can use a where clause. The following are equivalent.
    // But, hmm, isn't this worse in a way?
    fn bar(item1: &(impl Summary + Document), item2: &(impl Summary + Paper)) {}
    fn bar_2<T, U>(item1: &T, item2: &U)
    where
        T: Summary + Document,
        U: Summary + Paper,
    {
        println!("hi");
    }

}

#[cfg(test)]
mod test {
    #[test]
    fn hi() {
        assert_eq!(1, 1);
    }
}