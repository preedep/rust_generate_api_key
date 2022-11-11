use rand::Rng;
use base64::encode;

#[derive(Debug)]
struct APIKey {
    api_key: String,
    secret_key: String
}
fn generate_api_key() -> APIKey {

    let random_bytes_api_key: Vec<u8> = (0..32).map(|_| { rand::random::<u8>() }).collect();

    let random_bytes_secret_key : Vec<u8> = (0..128).map(|_| { rand::random::<u8>() }).collect();

    let api_key = APIKey{
        api_key:encode(random_bytes_api_key),
        secret_key:encode(random_bytes_secret_key)
    };

    return api_key;
}
fn main() {
    let api_key = generate_api_key();
    println!("{:?}", api_key);
}