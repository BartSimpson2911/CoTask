use rand::Rng;

pub fn generate_id() -> String {
    let mut rng = rand::thread_rng();
    let bytes: [u8; 4] = rng.random(); 
    hex::encode(bytes) 
}
