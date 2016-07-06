use rand::chacha::ChaChaRng;
use rand::os::OsRng;
use rand::Rng;
use rand::SeedableRng;

use core::array::FixedSizeArray;

use macaroons::token::Token;
use macaroons::caveat::{Caveat};
use macaroons::verifier::Verifier;

use std::io::Write;
use std::prelude;
use memmap::{Mmap, Protection};

struct Key {
    key: Mmap
}

impl Key {
    fn new() -> Key {
        let mut key_map = Mmap::anonymous(512, Protection::ReadWrite).unwrap();
        let osrng = OsRng::new().unwrap();
        let key_nonce: u32 = osrng.next_u32();
        let chacha_rng = SeedableRng::from_seed(key_nonce);
        let new_key = [0; 512];
        let new_key = Rng::fill_bytes(chacha_rng, &mut new_key);
        unsafe { key_map.as_mut_slice() }.write(new_key.as_slice()).unwrap();
        let key = Key {
            key: key_map
        };
        key
    }
}

pub struct Macaroon {
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

//  fn gen_service_token()
//  This fancy little subversion of the system is for testing of the greater
//  system until we can get add_caveat() to take a vec.  Some things to think
//  about add_caveat() and vector is that order absolutely does matter for 
//  macaroons.

    fn gen_service_token(mint: Macaroon_Mint, key: Key) -> Token {
        let service_caveat = Caveat::first_party(b"service = TestService".to_vec());
        let ip_caveat = Caveat::first_party(b"ip = 67.205.61.180".to_vec());
        let user_caveat = Caveat::first_party(b"user = test_user".to_vec());
        Token::new(key.key.as_slice(), mint.nonce.to_vec(), None)
              .add_caveat(service_caveat)
              .add_caveat(ip_caveat)
              .add_caveat(user_caveat)
    }

//  build_verifier()
//  add_matcher() allows you to check the caveats, currently this
//  matches bytes but it /could/ theoretically match anything.
//  We might also be able to use nom to build the verifier.
//  github.com/geal/nom

    fn build_verifier() -> Verifier {
        Verifier::new()
                 .add_matcher(|c| c == "service = TestService")
                 .add_matcher(|c| c == "ip = 67.205.61.180")
                 .add_matcher(|c| c == "user = test_user")
    }
    
//  fn verify()
//  Uses the "Verifier" type from self.build_verifier()
//  verify() takes a "Key" type, and the "Token" type.
//  It is used to check if the Token is valid.
//  bool = true, valid: bool = false, invalid.

    fn verify(key: Key, token: Token) -> bool {
        let verifier = Macaroon_Auth::build_verifier();
        let verified = verifier.verify(key.key.as_slice(), &token);
        verified
    }

}

struct Macaroon_Mint {
    nonce: [u8; 512],
    caveats: Vec<Caveat>
}

impl Macaroon_Mint {

    pub fn new() -> Macaroon_Mint {
        let chacha_rng = Macaroon_Mint::nonce_rng();
        let mut identifier_nonce: [u8; 512] = [0; 512];
        chacha_rng.fill_bytes(&mut identifier_nonce.as_slice());
        Macaroon_Mint {
            nonce: identifier_nonce,
            caveats: Macaroon_Mint::service_caveats()
        }
    }

//  nonce_rng() 
//  Generates a ChaChaRng Stream for identifier tokens
//  This is only used for identifier tokens and should
//  not be used to create keys.  You have been warned.

    pub fn nonce_rng<ChaChaRng: SeedableRng<u32>>() -> ChaChaRng {
        let osrng = OsRng::new().unwrap();
        let nonce: u32 = osrng.next_u32();
        let chacha_rng = SeedableRng::from_seed(nonce);
        chacha_rng
    }

//  service_caveats()
//  Generates the default service caveats, these caveats
//  are used to define a the location and minimum credentials
//  necessary for the service, you can also think of this as
//  your "identity token" or "service identity token".
//  this function is currently worthless until add_caveat()
//  can take a Vec. TODO: Submit PR to allow add_caveat to take a vector

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
