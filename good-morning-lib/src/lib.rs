pub mod calendar;
mod draw;
mod error;
pub mod weather;
pub mod datetime;

pub use draw::DrawData;
pub use error::BadMorning;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
