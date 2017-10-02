#[macro_use]
extern crate error_chain;
extern crate oqs;
extern crate oqs_kex_rpc;
extern crate sha2;
extern crate base64;

use oqs::kex::OqsKexAlg;
use oqs_kex_rpc::client::OqsKexClient;
use sha2::{Sha256, Digest};

error_chain! {
    foreign_links {
        KeyExchangeFailed(::oqs_kex_rpc::client::Error);
    }
}

fn main() {
    let exit_code = real_main();
    std::process::exit(exit_code);
}

fn real_main() -> i32 {
    let server = String::from("10.99.0.1:1984");
    let algs = vec![OqsKexAlg::RlweNewhope, OqsKexAlg::CodeMcbits, OqsKexAlg::SidhCln16];

    match wg_psk_kex(&server, &algs) {
        Ok(psk) => {
            println!("{}", psk);
            return 0;
        }
        Err(error) => {
            println!("Error: {}", error);
            return 1;
        }
   }
}

fn wg_psk_kex(server: &str, algorithms: &Vec<OqsKexAlg>) -> Result<String> {
    let mut client = OqsKexClient::new(server)?;
    let keys = client.kex(&algorithms)?;

    let mut hasher = Sha256::default();
    for key in &keys {
        hasher.input(key.data());
    }

    let digest = hasher.result().to_vec();
    Ok(base64::encode(&digest))
}