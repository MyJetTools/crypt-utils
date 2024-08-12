use openssl::{error::ErrorStack, rsa::Rsa};

pub fn generate_rsa_key_pair() -> Result<(Vec<u8>, Vec<u8>), ErrorStack> {
    let rsa = Rsa::generate(2048)?;
    let private_key: Vec<u8> = rsa.private_key_to_pem()?;
    let public_key: Vec<u8> = rsa.public_key_to_pem()?;

    Ok((private_key, public_key))
}

pub fn get_public_rsa_key_from_private(private_key: Vec<u8>) -> Result<Vec<u8>, ErrorStack> {
    let private = Rsa::private_key_from_pem(&private_key.clone()).unwrap();
    Ok(private.public_key_to_pem()?)
}
