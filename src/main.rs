use self;

fn main() {
    let key = Key::new();
    let api = Api::new();
    let mint = Macaroon_Mint::new();
    let auth = Macaroon_Auth::new();
    let service_token = auth.gen_service_token(&mint, &key);
    let verifier = auth.build_verifier();
    let is_verified = verifier.verify(&key, &service_token);
    println!("{}", is_verified);
}
