use crate::utils;

use anyhow::Result;
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