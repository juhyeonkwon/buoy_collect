//test
#[cfg(test)]
mod tests {
    use chrono;
    use chrono::prelude::*;

    #[test]
    fn get_time() {
        let now: DateTime<Local> = Local::now();
        let now_str = now.to_string();

        let ab = format!("{}_{}", &now_str[0..10], &now_str[11..13]);

        println!("{:?}", ab);
    }


}
