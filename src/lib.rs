#![allow(non_snake_case)]
#![allow(unused_imports)]
#![feature(rustc_private)]

#![feature(proc_macro_hygiene, decl_macro,custom_attribute,plugin)]

extern crate ethereum_tx_sign;
extern crate json;
extern crate crypto;
extern crate emerald_rs;
extern crate tempdir;
extern crate nanoid;
extern crate hex;
extern crate web3;

#[macro_use] extern crate serde_derive;

use std::io::{stdin,stdout,Write};
use std::process;
use std::fs::File;
use std::fs;
use std::io::Read; 
use std::path::PathBuf;
use std::path::Path;
use std::mem;
use std::{thread, time};
use std::collections::HashMap;
use std::env;
use std::str::from_utf8;

use std::ffi::CStr;
use std::os::raw::c_char;

use self::crypto::digest::Digest;
use self::crypto::sha2::Sha256;

use ethereum_tx_sign::RawTransaction;
use web3::types::{Address, U256, H160, H256, Bytes, H512, BlockNumber};
use web3::futures::Future;
use tempdir::TempDir;
use ethereum_rust_utils::eth_utils;
use emerald_rs::keystore::{os_random, Kdf, KeyFile};
use emerald_rs::PrivateKey;
use emerald_rs::storage::{KeyfileStorage, FsStorage, StorageController};
use base64::{encode, decode};

//Constants
const GWEI_UNIT: &'static str = "1000000000";

#[no_mangle]
pub extern fn generate_keystore(p_secret: *const c_char) -> &'static str {
    let secret = unsafe { CStr::from_ptr(p_secret) };
    if Path::new("keystore").exists() {
        // Existing keystore folder
        let db = FsStorage::new("keystore");
        let acc = db.list_accounts(false);
        let accs = acc.unwrap();

        if accs.len() > 0 {
            // A keystore exists
            return "Keystore already exists."

        }else{
            // Keystore does not exist
            let db = FsStorage::new("keystore");

            // Keystore generation
            db.put(&get_keyfile(secret.to_str().unwrap().to_string())).unwrap();
            return "Wallet created."
        }
        
    } else {
        // Keystore folder does not exist
    	let _r = fs::create_dir_all("keystore");
        let db = FsStorage::new("keystore");

        // Keystore generation
        db.put(&get_keyfile(secret.to_str().unwrap().to_string())).unwrap();
        return "Wallet created."
    }
}

#[no_mangle]
pub extern fn get_address() -> &'static str {
    let db = FsStorage::new("keystore");
    let accs = db.list_accounts(false).unwrap();

    let path = keyfile_path(&accs[0].filename.to_string());

    let mut file = File::open(path).expect("Unable to open the file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read the file");
    let _keyfile = KeyFile::decode(&contents).unwrap();

    //let s = SECRET.to_string();
    //let k = _keyfile.decrypt_key(&s).unwrap();

    let h = KeyFile::decode(&contents).unwrap().address;

    let hex_address = h.to_string();
    let checksummed = eth_utils::to_checksum_address(&hex_address);
    println!("{:?}",checksummed);
    return string_to_static_str(checksummed.to_string())
}

#[no_mangle]
pub extern fn sign_transaction(nonce: i32, gwei_amount: i32, p_gas_price: i32, p_gas: i32, p_chain_id: i32, p_recipient: *const c_char, p_secret: *const c_char) ->  &'static str {
    // Get address and private key
    let recipient = unsafe { CStr::from_ptr(p_recipient) };
    let secret    = unsafe { CStr::from_ptr(p_secret)    };
    let db = FsStorage::new("keystore");
    let accs = db.list_accounts(false).unwrap();

    let path = keyfile_path(&accs[0].filename.to_string());

    let mut file = File::open(path).expect("Unable to open the file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read the file");
    let keyfile = KeyFile::decode(&contents).unwrap();

    let s = secret;
    let k = keyfile.decrypt_key(s.to_str().unwrap()).unwrap();
    let h = KeyFile::decode(&contents).unwrap().address;

    let hex_address = h.to_string();
    let pk = string_to_static_str(k.to_string());

    let checksummed = eth_utils::to_checksum_address(&hex_address);
    let _wallet_address = H160::from(string_to_static_str(checksummed.to_string()));

    // Recipient address
    let r = recipient.to_str().unwrap();
    let recipient_address = H160::from(string_to_static_str(r.to_string()));
    println!("recipient: {:?}",recipient_address);
    // Get nonce
    let non = U256::from(nonce);
    println!("{:?}",non);
    // Defining transaction parameters
    let wei_amount = gwei_amount.to_string()+&GWEI_UNIT.to_string();
    println!("{:?}",wei_amount);
    let val = U256::from_dec_str(&wei_amount).unwrap();
    println!("{:?}",val);
    let gas = U256::from(p_gas);
    println!("{:?}",gas);
    let gas_price = U256::from(p_gas_price);
    println!("{:?}",gas_price);

    // Creating transaction
    let tx = RawTransaction {
                    nonce: non,
                    gas_price: gas_price,
                    gas: gas,
                    to: serde::export::Some(recipient_address),
                    value: val,
                    data: [].to_vec(),
    };

    //Sign transaction
    let private_key = H256::from(pk);
    let CHAIN_ID: u8 = p_chain_id as u8;
    
    let signed_tx = tx.sign(&private_key,&CHAIN_ID);
    println!("{:?}",signed_tx);

    let string_signed_tx = hex::encode(signed_tx);

    println!("{:?}",string_signed_tx);
    return string_to_static_str(string_signed_tx)

}

pub fn get_keyfile(passphrase: String) -> KeyFile {
    let pk = PrivateKey::gen();
    let kdf = Kdf::from((8, 2, 1));
    let mut rng = os_random();
    let s=passphrase;
    
    KeyFile::new_custom(pk, &s, kdf, &mut rng, None, None).unwrap()

}

pub fn temp_dir() -> PathBuf {
    let dir = TempDir::new("emerald").unwrap();

    File::create("keystore").ok();
    dir.into_path()
}

pub fn keyfile_path(name: &str) -> PathBuf {
    let mut path = keystore_path();
    path.push(name);
    println!("{:?}", path);
    path
}

pub fn keystore_path() -> PathBuf {
    let mut buf = PathBuf::from("");
    buf.push("keystore/");
    println!("{:?}", buf);
    buf
}

fn string_to_static_str(s: String) -> &'static str {
    unsafe {
        let ret = mem::transmute(&s as &str);
        mem::forget(s);
        ret
    }
}
