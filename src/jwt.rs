use jsonwebtoken::{Header, EncodingKey, Algorithm, errors::Error};
use serde::{Serialize};

pub fn encode_message_to_jwt_rsa<T: Serialize>(message: T, private_key: &Vec<u8>) -> Result<String, Error>{
    let key = EncodingKey::from_rsa_pem(private_key)?;
    let result = jsonwebtoken::encode(&Header::new(Algorithm::RS256), &message, &key)?;

    return Ok(result);
}
