use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    sleep(Duration::from_secs(10)).await;
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    #[should_panic]
    fn it_fails() {
        assert_eq!(1 + 1, 3);
    }

    #[test]
    #[ignore]
    fn expensive_test() {
        // 長い時間がかかるテスト
    }
}
