use jsonwebtoken::{encode, Header, EncodingKey};
use std::time::{SystemTime, UNIX_EPOCH};
use dotenv_codegen::dotenv;
use crate::structs::account::Claims;
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};

fn create_jwt(id: String) -> String {
    let my_claims = Claims {
        sub: id.to_owned(),
        company: "RMS".to_owned(),
        exp: (SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs() + 3600) as usize,
    };
    let tkn = dotenv!("TOKEN");
    let token = encode(
        &Header::default(),
        &my_claims,
        &EncodingKey::from_secret(tkn.as_ref()),
    ).unwrap();

    token
}

fn validate_jwt(token: &str) -> bool {
    let tkn = dotenv!("TOKEN");
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(tkn.as_ref()),
        &Validation::new(Algorithm::HS256),
    );

    match token_data {
        Ok(data) => {
            println!("Doğrulandı: {:?}", data.claims);
            true
        }
        Err(err) => {
            println!("JWT doğrulama hatası: {:?}", err);
            false
        }
    }
}


