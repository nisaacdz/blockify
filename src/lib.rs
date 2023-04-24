pub mod io;
pub mod net;
pub mod axs;
pub mod sec;
pub mod trans;
pub mod ver;
pub mod fxns;
pub mod dat;

#[cfg(test)]
mod tests {
    #[test]
    fn derive_test() {
        use serde::{Serialize, Deserialize};
        #[derive(Serialize, Deserialize, Clone)]
        struct Temp;
    }
}