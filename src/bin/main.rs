fn main() {
    use blockify::{sec, trans::record::Record};
    use serde::{Deserialize, Serialize};
    use record_derive::Record;

    #[derive(Clone, Serialize, Deserialize, Record)]
    struct Vote {
        session: i32,
        choice: i32,
    }

    let keypair = sec::generate_ed25519_key_pair();
    let my_record = Vote {
        session: 0,
        choice: 2,
    };

    let signature = my_record.sign(&keypair).unwrap();
    
    assert!(my_record.verify(signature, keypair.into_public_key()).is_ok())
}
