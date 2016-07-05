use rand::chacha::ChaChaRng;
use rand::os::OsRng;

use macaroons::token::Token;
use macaroons::caveat::{Caveat, Predicate};
use macaroons::verifier::Verifier;

use std::io::Write;
use memmap::{Mmap, Protection};

struct Key {
    key: Mmap
}

impl Key {
    fn new() -> Key {
        let mut key_map = Mmap::anonymous(512, Protection::ReadWrite).unwrap();
        let osrng = Osrng::new();
        let key_nonce: [u8; 512] = [0; 512];
        osrng.fill_bytes(&mut key_nonce);
        let chacha_rng = ChaChaRng::new_unseeded();
        chacha_rng.reseed(& mut key_nonce);
        unsafe { key_map.as_mut_slice() }.write(chacha_rng).unwrap();
        let key = Key {
            key: key_map
        };
        key
    }
}
   
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

struct Macaroon {
    macaroon_auth: Macaroon_Auth,
    macaroon_mint: Macaroon_Mint
}

impl Macaroon {
    pub fn new() -> Macaroon {
        Macaroon {
            macaroon_auth: Macaroon_Auth::new(),
            macaroon_mint: Macaroon_Mint::new()
        }
    }
}
struct Macaroon_Auth {
    trusted: bool,
    untrusted_token: Option<Token>,
    new_token: Option<Token>
}

impl Macaroon_Auth {
    pub fn new() -> Macaroon_Auth {
        Macaroon_Auth {
            trusted: false,
            untrusted_token: None,
            new_token: None
        }
    }
    
    fn gen_service_token(mint: Macaroon_Mint, key: Key) -> Token {
        let service_caveat = Caveat::first_party(b"service = TestService".to_vec());
        let ip_caveat = Caveat::first_party(b"ip = 67.205.61.180".to_vec());
        let user_caveat = Caveat::first_party(b"user = test_user".to_vec());
        Token::new(key.key.to_vec(), self.nonce.to_vec(), None)
              .add_caveat(service_caveat)
              .add_caveat(ip_caveat)
              .add_caveat(user_caveat)
    }

    fn build_verifier() -> Verifier {
        Verifier::new()
                 .add_matcher(|c| c == "service = TestService")
                 .add_matcher(|c| c == "ip = 67.205.61.180")
                 .add-matcher(|c| c == "user = test_user")
        }
    }

    fn verify(key: Key, token: Token) -> bool {
        let verifier = build_verifier();
        let verified = verifier.verify(key, &token);
        verified
    }

}

struct Macaroon_Mint {
    nonce: [u8; 512],
    caveats: Vec<Caveat>
}

impl Macaroon_Mint {

    static chacha_rng: ChaChaRng = Macaroon_Mint::nonce_rng();

    pub fn new(chacha_rng: ChaChaRng, caveats: Vec<Caveat>) -> Macaroon_Mint {
        let mut identifier_nonce: [u8, 512] = [0; 512];
        chacha_rng.fill_bytes(&mut identifier_nonce);
        Macaroon_Mint {
            nonce: identifier_nonce,
            caveats: Macaroon_Mint::service_caveats()
        }
    }

    pub fn nonce_rng() -> ChaChaRng {
        let osrng = OsRng::new();
        let nonce: [u8; 512] = [0; 512];
        osrng.fill_bytes(&mut nonce);
        let chacha_rng = ChaChaRng::new_unseeded();
        chacha_rng.reseed(nonce);
        chacha_rng
    }

    pub fn service_caveats() -> Vec<Caveat> {
        let mut caveats: Vec<Caveat> = Vec::new();
        let service_caveat = Caveat::first_party(b"service = TestService".to_vec());
        let ip_caveat = Caveat::first_party(b"ip = 67.205.61.180".to_vec());
        let user_caveat = Caveat::first_party(b"user = test_user".to_vec());
        caveats.push(service_caveat);
        caveats.push(ip_caveat);
        caveats.push(user_caveat);
        caveats
    }
}
