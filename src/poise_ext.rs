use std::error::Error as StdError;

pub struct Data {}

pub type Error = Box<dyn StdError + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;
