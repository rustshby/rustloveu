pub mod news;


#[cfg(test)]
mod tests {
    use super::*;

    fn news_display() -> String{
        let n = news::News{
            id:String::from("1"),
            desc:String::from("google"),
            url:String::from("google.com"),
        };
        format!("{}", n)
    }

    #[test]
    fn test_news_display(){
        let n = news_display();
        println!("{}",n);
    }
}
