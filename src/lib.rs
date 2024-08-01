#![allow(dead_code)]
#![allow(unused_imports)]

pub const API_VERSION: u8 = 10;

mod apiclient;
mod gateway;

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
}
