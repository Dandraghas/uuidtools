use crate::utils;

use anyhow::{Result, anyhow};
use uuid::{Uuid, Context, Timestamp};

pub fn random() -> Result<Uuid> {
    let uuid = uuid::Uuid::new_v4();
    Ok(uuid)
}

pub fn md5(namespace: &Uuid, name: &str) -> Result<String> {
    let uuid = Uuid::new_v3(namespace, name.as_bytes()).to_string();
    Ok(uuid)
}

pub fn sha1(namespace: &Uuid, name: &str) -> Result<String> {
    let uuid = Uuid::new_v5(namespace, name.as_bytes()).to_string();
    Ok(uuid)
}

pub fn time(seconds: &u64, nanos: &u32, seed: Option<u16>) -> Result<Uuid> {
    let seed = match seed {
        Some(seed) => seed,
        None => utils::random_seed()?,
    };

    let context = Context::new(seed);
    let ts = Timestamp::from_unix(context, *seconds, *nanos);

    let uuid = Uuid::new_v1(ts, &[1, 2, 3, 4, 5, 6]);
    Ok(uuid)
}

pub fn data(data: &str) -> Result<Uuid, anyhow::Error> {
    if data.len() > 16 {
        return Err(anyhow!("Data cannot be longer than 16 bytes"));
    }

    let buf: [u8; 16] = {
        let mut array = [0u8; 16];
        array[..data.len()].copy_from_slice(data.as_bytes());
        array
    };

    let uuid = Uuid::new_v8(buf);
    Ok(uuid)
}