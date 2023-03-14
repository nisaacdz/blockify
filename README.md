# BLOCKIFY


A Rust blockchain library that provides the building blocks for creating a full-fledged blockchain application or platform, allowing you to focus on the higher-level features of your project without worrying about the low-level details of block validation, data serialization, and cryptographic operations.





# FEATURES


## IO - Data storage layer

This layer is responsible for storing data related to the blockchain, including blocks, transactions, and smart contracts. It includes components such as a database and file storage.


## SEC - Security Layer

This layer is responsible for ensuring the security of the blockchain network. It includes components such as cryptography, key management, and authentication.


## TRANS - Record Layer

This layer is responsible for handling transactions on the blockchain, including validating transactions, creating new blocks, and updating the state of the ledger. It includes components such as transaction validation, block creation, and state management.


## VER - Verification Layer

This layer is responsible for verifying blocks and broadcasts as well as various
logics for mining and validating data.









# Contributing

Contributions to this project are welcome. If you find a bug or want to suggest an improvement, please create an issue or submit a pull request.


# License

This project is licensed under the MIT License.




# DEPENDENCIES


- bincode
- chrono
- ed25519-dalek
- hex
- rand
- ring
- serde
- serde_json
- sha2
- tokio
