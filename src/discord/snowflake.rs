use alloc::string::{String, ToString};
use core::num::{NonZeroU64, ParseIntError};

use serde::de::Error;
use serde::{Deserialize, Deserializer, Serialize};
use time::OffsetDateTime;

const DISCORD_EPOCH: u64 = 1420070400000;

#[derive(Serialize, Debug)]
pub struct Snowflake(pub NonZeroU64);

impl Snowflake {
    fn get_timestamp(self) -> u64 {
        (self.0.get() >> 22) + DISCORD_EPOCH
    }

    #[allow(dead_code)]
    fn get_internal_worker_id(self) -> u8 {
        ((self.0.get() & 0x3E0000) >> 17) as u8
    }

    #[allow(dead_code)]
    fn get_internal_process_id(self) -> u8 {
        ((self.0.get() & 0x1F000) >> 12) as u8
    }

    #[allow(dead_code)]
    fn get_increment(self) -> u16 {
        (self.0.get() & 0xFFF) as u16
    }
}

impl<'de> Deserialize<'de> for Snowflake {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw_value = String::deserialize(deserializer)?;
        let value: u64 = raw_value
            .parse()
            .map_err(|error: ParseIntError| Error::custom(error.to_string().as_str()))?;
        Ok(Snowflake(NonZeroU64::new(value).unwrap()))
    }
}

impl From<OffsetDateTime> for Snowflake {
    fn from(value: OffsetDateTime) -> Self {
        Snowflake(NonZeroU64::new((value.second() as u64 - DISCORD_EPOCH) << 22).unwrap())
    }
}

impl From<Snowflake> for OffsetDateTime {
    fn from(value: Snowflake) -> Self {
        OffsetDateTime::from_unix_timestamp(value.get_timestamp() as i64).unwrap()
    }
}
