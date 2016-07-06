use auth::macaroon::Macaroon;

struct Api {
    auth: Macaroon
}

impl Api {
    pub fn new() -> Api {
        Api {
            auth: Macaroon::new()
        }
    }
}
