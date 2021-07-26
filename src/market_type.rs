/// Market type.
#[repr(C)]
#[derive(Copy, Clone)]
pub enum MarketType {
    Spot,
    LinearFuture,
    InverseFuture,
    LinearSwap,
    InverseSwap,

    AmericanOption,
    EuropeanOption,

    QuantoFuture,
    QuantoSwap,

    Move,
    #[allow(clippy::upper_case_acronyms)]
    BVOL,
}

impl MarketType {
    // Converts C MarketType to Rust MarketType
    pub fn to_rust(self) -> crypto_msg_parser::MarketType {
        match self {
            MarketType::Spot => crypto_msg_parser::MarketType::Spot,
            MarketType::LinearFuture => crypto_msg_parser::MarketType::LinearFuture,
            MarketType::InverseFuture => crypto_msg_parser::MarketType::InverseFuture,
            MarketType::LinearSwap => crypto_msg_parser::MarketType::LinearSwap,
            MarketType::InverseSwap => crypto_msg_parser::MarketType::InverseSwap,
            MarketType::AmericanOption => crypto_msg_parser::MarketType::AmericanOption,
            MarketType::EuropeanOption => crypto_msg_parser::MarketType::EuropeanOption,
            MarketType::QuantoFuture => crypto_msg_parser::MarketType::QuantoFuture,
            MarketType::QuantoSwap => crypto_msg_parser::MarketType::QuantoSwap,
            MarketType::Move => crypto_msg_parser::MarketType::Move,
            MarketType::BVOL => crypto_msg_parser::MarketType::BVOL,
        }
    }

    // Converts Rust MarketType to C MarketType
    pub fn from_rust(market_type: crypto_msg_parser::MarketType) -> Self {
        match market_type {
            crypto_msg_parser::MarketType::Spot => MarketType::Spot,
            crypto_msg_parser::MarketType::LinearFuture => MarketType::LinearFuture,
            crypto_msg_parser::MarketType::InverseFuture => MarketType::InverseFuture,
            crypto_msg_parser::MarketType::LinearSwap => MarketType::LinearSwap,
            crypto_msg_parser::MarketType::InverseSwap => MarketType::InverseSwap,
            crypto_msg_parser::MarketType::AmericanOption => MarketType::AmericanOption,
            crypto_msg_parser::MarketType::EuropeanOption => MarketType::EuropeanOption,
            crypto_msg_parser::MarketType::QuantoFuture => MarketType::QuantoFuture,
            crypto_msg_parser::MarketType::QuantoSwap => MarketType::QuantoSwap,
            crypto_msg_parser::MarketType::Move => MarketType::Move,
            crypto_msg_parser::MarketType::BVOL => MarketType::BVOL,
        }
    }
}
