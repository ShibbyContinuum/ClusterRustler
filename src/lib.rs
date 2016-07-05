extern crate persistent;
extern crate rand;
extern crate memmap;
extern crate rust-crypto;
extern crate serde;
extern crate serde_json;
extern crate serde_macros;
extern crate bodyparser;
extern crate iron;
extern crate macaroons;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}

pub mod auth;
