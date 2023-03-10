use rand::{thread_rng, Rng};
use serde::Serialize;
use sha2::{
    digest::{
        generic_array::GenericArray,
        typenum::{UInt, UTerm, B0, B1},
    },
    Digest, Sha256,
};

use crate::{
    errs::*,
    refs::{Range, TimeStamp},
    trans::{
        blocks::BlockBuilder,
        record::{Record, SignedRecord},
    },
};
pub mod errs;
pub mod merkle;

// Define a type alias `Hxsh` for a fixed-size array of 32 bytes using the `GenericArray` type from the `typenum` crate.
// The type alias represents the output of a SHA-256 hash function.
type Hxsh = GenericArray<u8, UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>>;

/// Compute the SHA-256 hash of a serializable value.
///
/// This function uses the `Serialize` trait from the `serde` crate to serialize the input data into a binary format.
/// The resulting binary data is then hashed using the SHA-256 hash function from the `sha2` crate.
///
/// # Arguments
///
/// * `data` - A serializable value to be hashed.
///
/// # Examples
///
/// ```
/// use serde::{Serialize, Deserialize};
/// use blockify::gen::hash;
///
/// #[derive(Serialize, Deserialize, PartialEq, Debug)]
/// struct Person {
///     name: String,
///     age: u8,
/// }
///
/// let alice = Person { name: "Alice".into(), age: 42 };
/// let hash = hash(&alice);
/// assert_eq!(hash.len(), 32); // SHA-256 produces a 32-byte hash
/// ```
///
/// # Panics
///
/// This function may panic if serialization fails or if there is a problem with the hash function implementation.
/// However, these cases are rare and should not occur under normal circumstances.
///
/// # Safety
///
/// This function is safe to use as long as the input data is serializable and does not cause a memory safety violation.
///
/// # Returns
///
/// A `Vec<u8>` containing the SHA-256 hash value of the serialized input data.
pub fn hash<T: Sized + Serialize>(data: &T) -> Vec<u8> {
    // Serialize the input data into a binary format using the `bincode` crate.
    let bytes = bincode::serialize(data).unwrap();
    hash_bytes(&bytes)
}

pub fn hash_bytes(bytes: &[u8]) -> Vec<u8> {
    // Create a new instance of the SHA-256 hasher from the `sha2` crate.
    let mut hasher = Sha256::new();
    // Update the hash with the binary data.
    hasher.update(bytes);
    // Finalize the hash computation and store the result in a `Hxsh` value.
    let data: Hxsh = hasher.finalize();
    // Convert the `Hxsh` value to a `Vec<u8>` for easier use.
    data.to_vec()
}

pub fn hash_block<R: Record>(
    block: &BlockBuilder<R>,
    prev_hash: &[u8],
    metadata: (&TimeStamp, &Range, &u64),
) -> Vec<u8> {
    let records = bincode::serialize(block.records()).unwrap();
    let metabytes = bincode::serialize(&metadata).unwrap();
    sha_from_4(prev_hash, &records, block.merkle_root(), &metabytes)
}

/// Generates a random 256-bit (32 byte) hash as a byte vector using a secure random number generator.
///
/// # Example
///
/// ```
/// use blockify::gen;
///
/// let random_hash = gen::random_sha256();
/// println!("Random hash: {:?}", random_hash);
/// ```
///
/// # Returns
///
/// A randomly generated 256-bit hash as a byte vector of `len` 32.

pub fn random_sha256() -> Vec<u8> {
    let mut bytes = vec![0; 32];
    thread_rng().fill(&mut bytes[..]);
    bytes
}

/// Generates a random array of 5 bytes using the thread's local random number generator.
///
/// # Examples
///
/// ```
/// use blockify::gen;
///
/// let bytes = gen::random_bytes5();
///
/// assert_eq!(bytes.len(), 5);
/// ```
pub fn random_bytes5() -> [u8; 5] {
    let mut bytes = [0; 5];
    thread_rng().fill(&mut bytes);
    bytes
}

/// Generates a quick hexadecimal ID string using `random_bytes5` function.
///
/// # Examples
///
/// ```
/// use blockify::gen;
///
/// let id = gen::quick_id();
///
/// assert_eq!(id.len(), 10);
/// ```
pub fn quick_id() -> String {
    hex::encode(random_bytes5())
}

/// Computes the SHA-256 hash of the concatenation of two byte slices `data` and `data1`.
///
/// # Arguments
///
/// * `data` - A byte slice containing the first input data to hash.
/// * `data1` - A byte slice containing the second input data to hash.
///
/// # Examples
///
/// ```
/// use blockify::gen::sha_from_2;
///
/// let data = b"hello";
/// let data1 = b"world";
/// let hash = sha_from_2(data, data1);
/// assert_eq!(hash.len(), 32); // SHA-256 produces a 32-byte hash
/// ```
///
/// # Panics
///
/// This function does not panic under normal circumstances.
///
/// # Safety
///
/// This function is safe to use, as long as the input byte slices are valid and do not cause a memory safety violation.
///
/// # Returns
///
/// A `Vec<u8>` containing the SHA-256 hash value of the input data.

pub fn sha_from_2(data: &[u8], data1: &[u8]) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hasher.update(data1);
    hasher.finalize().to_vec()
}

/// Computes the SHA256 hash of the concatenation of three byte slices.
///
/// # Arguments
///
/// * `data` - A slice of bytes to be hashed.
/// * `data1` - Another slice of bytes to be hashed.
/// * `data2` - Another slice of bytes to be hashed.
///
/// # Examples
///
/// ```
/// use blockify::gen;
///
/// let data = "Hello, ".as_bytes();
/// let data1 = "world".as_bytes();
/// let data2 = "!".as_bytes();
///
/// let hash = gen::sha_from_3(data, data1, data2);
///
/// assert_eq!(
///     hex::encode(hash),
///     "9695d7f33b75d9c7ec61ca5f72d7cb66ba8d137e08af67c2e15f056de51d8a06"
/// );
/// ```
pub fn sha_from_3(data: &[u8], data1: &[u8], data2: &[u8]) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hasher.update(data1);
    hasher.update(data2);
    hasher.finalize().to_vec()
}

pub fn sha_from_4(data: &[u8], data1: &[u8], data2: &[u8], data3: &[u8]) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hasher.update(data1);
    hasher.update(data2);
    hasher.update(data3);
    hasher.finalize().to_vec()
}

pub fn sha_from_5(data: &[u8], data1: &[u8], data2: &[u8], data3: &[u8], data4: &[u8]) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hasher.update(data1);
    hasher.update(data2);
    hasher.update(data3);
    hasher.update(data4);
    hasher.finalize().to_vec()
}

/// Computes the SHA-256 hash of the serialized version of a given object and checks if it matches a given hash value.
///
/// # Arguments
///
/// * obj - a reference to a generic type T that implements both Sized(implicitly) and Serialize.
/// * value - a byte slice representing the hash value to be checked against the computed hash.
///
/// # Returns
///
/// A boolean value indicating whether the computed SHA-256 hash of the serialized object matches the given hash value.
///
/// # Examples
///
/// ```
/// use blockify::gen;
///
/// #[derive(Serialize)]
/// struct Test {
///     field1: u32,
///     field2: String
/// }
///
/// let test = Test {
///     field1: 42,
///     field2: "Hello, world!".to_string()
/// };
///
/// let hash = gen::hash(&test);
/// assert!(gen::validate(&test, &hash));
///
/// let wrong_hash = gen::random_sha256();
///
/// assert!(!gen::validate(&test, &wrong_hash));
///
/// ```
///
/// # Panics
///
/// This function will panic if serialization of the given object fails.

pub fn validate<T: Sized + Serialize>(obj: &T, value: &[u8]) -> bool {
    value == hash(obj)
}

use ed25519_dalek::{Keypair, PublicKey, SecretKey, Signature, Signer, Verifier};

pub struct AuthKeyPair(Vec<u8>, Vec<u8>);

impl AuthKeyPair {
    pub fn public_key(&self) -> &[u8] {
        &self.0
    }

    pub fn private_key(&self) -> &[u8] {
        &self.1
    }
}

/// Generates a new Ed25519 key pair as an AuthKeyPair object.
///
/// # Example
///
/// ```
/// use blockify::gen;
/// let keypair = gen::generate_key_pair();
/// println!("private key: {:?}\npublic key: {:?}", keypair.private_key(), keypair.public_key());
///
/// ```
///
/// # Returns
///
/// An AuthKeyPair object containing the generated public and private keys as byte vectors.
///
/// # Panics
///
/// This function will panic if the OS's random number generator fails.

pub fn generate_key_pair() -> AuthKeyPair {
    // Generate a new key pair
    let mut c = rand::rngs::OsRng;
    let keypair = Keypair::generate(&mut c);

    // Serialize the private and public keys as byte vectors
    let private_key = keypair.secret.to_bytes().to_vec();
    let public_key = keypair.public.to_bytes().to_vec();

    // Returns the public and private keys as byte vectors
    AuthKeyPair(public_key, private_key)
}

/// Signs a message with a given private key and returns the signature as a byte vector.
///
/// # Arguments
///
/// * `msg` - A byte vector representing the message to be signed.
/// * `key` - A byte vector representing the private key to be used for signing.
///
/// # Returns
///
/// Returns `Ok(Vec<u8>)` if the signature was successfully generated, otherwise returns a `GenErrs` error
/// indicating the reason for the failure.
///
/// # Examples
///
/// ```
/// use blockify::{gen, GenErrs};
///
/// let keypair = gen::generate_key_pair();
/// let message = b"test message";
/// let signature = gen::sign(&message[..], keypair.private_key()).unwrap();
///
/// assert!(gen::verify_signature(&message[..], &signature[..], keypair.public_key()).unwrap());
/// ```

pub fn sign_msg(msg: &[u8], key: &[u8]) -> Result<Vec<u8>, Failure> {
    // Parse the private key
    match SecretKey::from_bytes(key) {
        Ok(secret) => {
            // Create a signature for the message
            let keypair = Keypair {
                public: PublicKey::from(&secret),
                secret,
            };
            let signature = keypair.sign(msg);
            Ok(signature.as_ref().to_vec())
        }
        Err(_) => Err(Failure::SigningFailure(
            errs::SigningFailures::InvalidPrivateKey,
        )),
    }
}
///
/// Verify the Ed25519 signature of a message with a given public key.
///
/// # Arguments
///
/// * `msg`: A byte array representing the message to be verified.
/// * `signature`: A byte array containing the Ed25519 signature of the message.
/// * `signer`: A byte array representing the public key to be used for verification.
///
/// # Returns
///
/// A `Result<bool, GenErrs>` where `GenErrs` is an enum that represents the possible errors that can occur during the cryptographic operation. If the signature is valid for the message and the provided public key, the function returns `Ok(true)`. Otherwise, the function returns `Ok(false)`.
///
/// If there is an error during the signature verification process, such as an invalid signature or public key, the function returns an appropriate error code from the `GenErrs` enum. Specifically, if the signature is invalid, the function returns `GenErrs::InvalidSignature`. If the public key is invalid, the function returns `GenErrs::InvalidPublicKey`.
///
/// # Examples
///
/// ```
/// use blockify::gen;
///
/// // Generate a key pair
/// let keypair = gen::generate_key_pair();
///
/// // Create a message to sign
/// let msg = b"Hello, world!";
///
/// // Sign the message
/// let signature = gen::sign(msg, keypair.private_key()).unwrap();
///
/// // Verify the signature with the public key
/// let result = gen::verify_signature(msg, &signature, keypair.public_key());
///
/// assert_eq!(result.unwrap(), true);
/// ```
///
/// # Security
///
/// This function uses the Ed25519 digital signature algorithm, which is a fast and secure signature algorithm that provides strong security guarantees. The algorithm is resistant to a wide range of attacks, including side-channel attacks, and has been extensively reviewed by the cryptographic community.

pub fn verify_signature_ed25519(
    msg: &[u8],
    signature: &[u8],
    signer: &[u8],
) -> Result<bool, GenErrs> {
    let dalek = Signature::from_bytes(signature).map_err(|_| GenErrs::InvalidSignature)?;
    match PublicKey::from_bytes(signer) {
        Ok(key) => match key.verify(msg, &dalek) {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        },
        Err(_) => Err(GenErrs::InvalidPublicKey),
    }
}

use ring::signature::{self, VerificationAlgorithm};

use self::errs::Failure;

pub fn verify_signature(
    msg: &[u8],
    signature: &[u8],
    signer: &[u8],
    algo: &'static dyn VerificationAlgorithm,
) -> Result<(), ring::error::Unspecified> {
    let public_key = signature::UnparsedPublicKey::new(algo, signer);

    public_key.verify(msg, signature)
}

pub fn sign<R: Record>(
    record: &R,
    public_key: &[u8],
    private_key: &[u8],
    algorithm: &'static dyn ring::signature::VerificationAlgorithm,
) -> Result<SignedRecord<R>, errs::Failure> {
    let msg = bincode::serialize(record).unwrap();
    let signature = sign_msg(&msg, private_key)?;
    let hash = hash_bytes(&msg);
    Ok(SignedRecord::new(
        record.clone(),
        signature,
        algorithm,
        public_key,
        hash,
    ))
}
