extern crate core;

mod constants;

pub mod error;

pub mod futures_usd {
    mod client;
    mod deserializer;
    pub mod listen_key;
    pub mod response;
    pub mod stream;

    pub mod enums {
        pub mod binance;
        pub mod events;
        pub(crate) mod streams;
    }
}
