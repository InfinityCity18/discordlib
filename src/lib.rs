pub const API_VERSION: &str = "v10";

mod apiclient;
mod gatewayclient;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[tokio::test]
    async fn vis_test() {
        let t = crate::apiclient::ApiClient::new("test").await;
        if let Err(e) = t {
            println!("{}", e);
        }
    }
}
