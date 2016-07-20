#![no_std]

pub mod tables;
pub mod getters;
pub struct Codepoint(pub char);

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
