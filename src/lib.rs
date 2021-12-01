#![allow(clippy::not_unsafe_ptr_arg_deref)]
use crypto_market_type::MarketType;
use crypto_msg_type::MessageType;

use std::{
    ffi::{CStr, CString},
    os::raw::c_char,
};

/// Extract the symbol from the message.
#[no_mangle]
pub extern "C" fn extract_symbol(
    exchange: *const c_char,
    market_type: MarketType,
    msg: *const c_char,
) -> *const c_char {
    let exchange_rust = unsafe {
        debug_assert!(!exchange.is_null());
        CStr::from_ptr(exchange).to_str().unwrap()
    };
    let msg_rust = unsafe {
        debug_assert!(!msg.is_null());
        CStr::from_ptr(msg).to_str().unwrap()
    };

    let result = std::panic::catch_unwind(|| {
        if let Some(symbol) =
            crypto_msg_parser::extract_symbol(exchange_rust, market_type, msg_rust)
        {
            let text = serde_json::to_string(&symbol).unwrap();
            let raw = CString::new(text).unwrap();
            raw.into_raw() as *const c_char
        } else {
            std::ptr::null()
        }
    });
    match result {
        Ok(ptr) => ptr,
        Err(err) => {
            eprintln!("{:?}", err);
            std::ptr::null()
        }
    }
}

/// Infer the message type from the message.
pub fn get_msg_type(exchange: *const c_char, msg: *const c_char) -> MessageType {
    let exchange_rust = unsafe {
        debug_assert!(!exchange.is_null());
        CStr::from_ptr(exchange).to_str().unwrap()
    };
    let msg_rust = unsafe {
        debug_assert!(!msg.is_null());
        CStr::from_ptr(msg).to_str().unwrap()
    };

    let result =
        std::panic::catch_unwind(|| crypto_msg_parser::get_msg_type(exchange_rust, msg_rust));
    match result {
        Ok(msg_type) => msg_type,
        Err(err) => {
            eprintln!("{:?}", err);
            MessageType::Other
        }
    }
}

/// Parse a raw trade message into a Vec<TradeMsg> and then convert to a JSON string.
#[no_mangle]
pub extern "C" fn parse_trade(
    exchange: *const c_char,
    market_type: MarketType,
    msg: *const c_char,
) -> *const c_char {
    let exchange_rust = unsafe {
        debug_assert!(!exchange.is_null());
        CStr::from_ptr(exchange).to_str().unwrap()
    };
    let msg_rust = unsafe {
        debug_assert!(!msg.is_null());
        CStr::from_ptr(msg).to_str().unwrap()
    };

    let result = std::panic::catch_unwind(|| {
        if let Ok(trades) = crypto_msg_parser::parse_trade(exchange_rust, market_type, msg_rust) {
            let text = serde_json::to_string(&trades).unwrap();
            let raw = CString::new(text).unwrap();
            raw.into_raw() as *const c_char
        } else {
            std::ptr::null()
        }
    });
    match result {
        Ok(ptr) => ptr,
        Err(err) => {
            eprintln!("{:?}", err);
            std::ptr::null()
        }
    }
}

/// Parse a raw level2 orderbook message into a Vec<OrderBookMsg> and then convert to a JSON string.
#[no_mangle]
pub extern "C" fn parse_l2(
    exchange: *const c_char,
    market_type: MarketType,
    msg: *const c_char,
    timestamp: i64,
) -> *const c_char {
    let exchange_rust = unsafe {
        debug_assert!(!exchange.is_null());
        CStr::from_ptr(exchange).to_str().unwrap()
    };
    let msg_rust = unsafe {
        debug_assert!(!msg.is_null());
        CStr::from_ptr(msg).to_str().unwrap()
    };

    let timestamp_rust = if timestamp <= 0 {
        None
    } else {
        Some(timestamp)
    };

    let result = std::panic::catch_unwind(|| {
        if let Ok(orderbooks) =
            crypto_msg_parser::parse_l2(exchange_rust, market_type, msg_rust, timestamp_rust)
        {
            let text = serde_json::to_string(&orderbooks).unwrap();
            let raw = CString::new(text).unwrap();
            raw.into_raw() as *const c_char
        } else {
            std::ptr::null()
        }
    });
    match result {
        Ok(ptr) => ptr,
        Err(err) => {
            eprintln!("{:?}", err);
            std::ptr::null()
        }
    }
}

/// Parse a raw funding rate message into a Vec<FundingRateMsg> and then convert to a JSON string.
#[no_mangle]
pub extern "C" fn parse_funding_rate(
    exchange: *const c_char,
    market_type: MarketType,
    msg: *const c_char,
) -> *const c_char {
    let exchange_rust = unsafe {
        debug_assert!(!exchange.is_null());
        CStr::from_ptr(exchange).to_str().unwrap()
    };
    let msg_rust = unsafe {
        debug_assert!(!msg.is_null());
        CStr::from_ptr(msg).to_str().unwrap()
    };

    let result = std::panic::catch_unwind(|| {
        if let Ok(rates) =
            crypto_msg_parser::parse_funding_rate(exchange_rust, market_type, msg_rust)
        {
            let text = serde_json::to_string(&rates).unwrap();
            let raw = CString::new(text).unwrap();
            raw.into_raw() as *const c_char
        } else {
            std::ptr::null()
        }
    });
    match result {
        Ok(ptr) => ptr,
        Err(err) => {
            eprintln!("{:?}", err);
            std::ptr::null()
        }
    }
}

/// Deallocate a string.
#[no_mangle]
pub extern "C" fn deallocate_string(pointer: *const c_char) {
    unsafe {
        if pointer.is_null() {
            return;
        }
        CString::from_raw(pointer as *mut c_char)
    };
}

#[cfg(test)]
mod tests {
    use crypto_market_type::MarketType;
    use crypto_msg_type::MessageType;

    use super::{deallocate_string, parse_funding_rate, parse_l2, parse_trade};
    use float_cmp::approx_eq;
    use std::ffi::{CStr, CString};

    #[test]
    fn test_parse_trade() {
        let (json_ptr, json_str) = {
            let exchange = CString::new("binance").unwrap();
            let raw_msg = CString::new(r#"{"stream":"btcusd_perp@aggTrade","data":{"e":"aggTrade","E":1616201883458,"a":41045788,"s":"BTCUSD_PERP","p":"58570.1","q":"58","f":91864326,"l":91864327,"T":1616201883304,"m":true}}"#).unwrap();

            let json_ptr =
                parse_trade(exchange.as_ptr(), MarketType::InverseSwap, raw_msg.as_ptr());
            let json_c_str = unsafe {
                debug_assert!(!json_ptr.is_null());
                CStr::from_ptr(json_ptr)
            };

            (json_ptr, json_c_str.to_str().unwrap())
        };

        let trades = serde_json::from_str::<Vec<crypto_msg_parser::TradeMsg>>(json_str).unwrap();
        assert_eq!(trades.len(), 1);
        let trade = &trades[0];

        assert_eq!(trade.exchange, "binance");
        assert_eq!(trade.market_type, MarketType::InverseSwap);
        assert_eq!(trade.msg_type, MessageType::Trade);
        assert_eq!(trade.price, 58570.1);
        assert!(approx_eq!(
            f64,
            trade.quantity_base,
            5800.0 / 58570.1,
            epsilon = 0.00000000000000002
        ));
        assert_eq!(trade.quantity_quote, 5800.0);
        assert_eq!(trade.quantity_contract, Some(58.0));
        assert_eq!(trade.side, crypto_msg_parser::TradeSide::Sell);

        deallocate_string(json_ptr);
    }

    #[test]
    fn test_parse_l2() {
        let (json_ptr, json_str) = {
            let exchange = CString::new("binance").unwrap();
            let raw_msg = CString::new(r#"{"stream":"btcusd_perp@depth@100ms","data":{"e":"depthUpdate","E":1622370862564,"T":1622370862553,"s":"BTCUSD_PERP","ps":"BTCUSD","U":127559587191,"u":127559588177,"pu":127559587113,"b":[["35365.9","1400"],["35425.8","561"]],"a":[["35817.8","7885"],["35818.7","307"]]}}"#).unwrap();

            let json_ptr = parse_l2(
                exchange.as_ptr(),
                MarketType::InverseSwap,
                raw_msg.as_ptr(),
                0,
            );
            let json_c_str = unsafe {
                debug_assert!(!json_ptr.is_null());
                CStr::from_ptr(json_ptr)
            };

            (json_ptr, json_c_str.to_str().unwrap())
        };

        let orderbooks =
            serde_json::from_str::<Vec<crypto_msg_parser::OrderBookMsg>>(json_str).unwrap();
        assert_eq!(orderbooks.len(), 1);
        let orderbook = &orderbooks[0];

        assert_eq!(orderbook.exchange, "binance");
        assert_eq!(orderbook.market_type, MarketType::InverseSwap);
        assert_eq!(orderbook.msg_type, MessageType::L2Event);
        assert_eq!(orderbook.asks.len(), 2);
        assert_eq!(orderbook.bids.len(), 2);
        assert!(!orderbook.snapshot);
        assert_eq!(orderbook.timestamp, 1622370862553);

        assert_eq!(orderbook.bids[0].price, 35365.9);
        assert_eq!(orderbook.bids[0].quantity_contract, Some(1400.0));
        assert_eq!(orderbook.asks[0].price, 35817.8);
        assert_eq!(orderbook.asks[0].quantity_contract, Some(7885.0));

        deallocate_string(json_ptr);
    }

    #[test]
    fn test_parse_funding_rate() {
        let (json_ptr, json_str) = {
            let exchange = CString::new("binance").unwrap();
            let raw_msg = CString::new(r#"{"stream":"btcusd_perp@markPrice","data":{"e":"markPriceUpdate","E":1617309477000,"s":"BTCUSD_PERP","p":"59012.56007222","P":"58896.00503145","r":"0.00073689","T":1617321600000}}"#).unwrap();

            let json_ptr =
                parse_funding_rate(exchange.as_ptr(), MarketType::InverseSwap, raw_msg.as_ptr());
            let json_c_str = unsafe {
                debug_assert!(!json_ptr.is_null());
                CStr::from_ptr(json_ptr)
            };

            (json_ptr, json_c_str.to_str().unwrap())
        };

        let rates =
            serde_json::from_str::<Vec<crypto_msg_parser::FundingRateMsg>>(json_str).unwrap();
        assert_eq!(rates.len(), 1);
        let rate = &rates[0];

        assert_eq!(rate.exchange, "binance");
        assert_eq!(rate.market_type, MarketType::InverseSwap);
        assert_eq!(rate.msg_type, MessageType::FundingRate);
        assert_eq!(rate.pair, "BTC/USD".to_string());
        assert_eq!(rate.funding_rate, 0.00073689);
        assert_eq!(rate.funding_time, 1617321600000);

        deallocate_string(json_ptr);
    }
}
