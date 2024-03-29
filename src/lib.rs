#![allow(clippy::not_unsafe_ptr_arg_deref)]
use crypto_market_type::MarketType;
use crypto_msg_type::MessageType;

use std::{
    ffi::{CStr, CString},
    os::raw::c_char,
};

/// Extract the symbol from the message.
///
/// * If the message contains multiple symbols, `ALL` is returned;
/// * If the message has no symbol, `NONE` is returned.
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
        if let Ok(symbol) = crypto_msg_parser::extract_symbol(exchange_rust, market_type, msg_rust)
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
            eprintln!("{exchange_rust}, {market_type}, {msg_rust}, error: {err:?}");
            std::ptr::null()
        }
    }
}

/// Extract the timestamp from the message.
///
/// Returns 0 if the message doesn't have a timestamp, -1 if an error happens.
#[no_mangle]
pub extern "C" fn extract_timestamp(
    exchange: *const c_char,
    market_type: MarketType,
    msg: *const c_char,
) -> i64 {
    let exchange_rust = unsafe {
        debug_assert!(!exchange.is_null());
        CStr::from_ptr(exchange).to_str().unwrap()
    };
    let msg_rust = unsafe {
        debug_assert!(!msg.is_null());
        CStr::from_ptr(msg).to_str().unwrap()
    };

    let result = std::panic::catch_unwind(|| {
        if let Ok(timestamp) =
            crypto_msg_parser::extract_timestamp(exchange_rust, market_type, msg_rust)
        {
            timestamp.unwrap_or_default()
        } else {
            -1_i64
        }
    });
    match result {
        Ok(timestamp) => timestamp,
        Err(err) => {
            eprintln!("{exchange_rust}, {market_type}, {msg_rust}, error: {err:?}");
            0_i64
        }
    }
}

/// Infer the message type from the message.
#[no_mangle]
pub extern "C" fn get_msg_type(exchange: *const c_char, msg: *const c_char) -> MessageType {
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
            eprintln!("{err:?}");
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
            eprintln!("{exchange_rust}, {market_type}, {msg_rust}, error: {err:?}");
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
    received_at: i64,
) -> *const c_char {
    let exchange_rust = unsafe {
        debug_assert!(!exchange.is_null());
        CStr::from_ptr(exchange).to_str().unwrap()
    };
    let msg_rust = unsafe {
        debug_assert!(!msg.is_null());
        CStr::from_ptr(msg).to_str().unwrap()
    };
    let timestamp_rust = if received_at <= 0 {
        None
    } else {
        Some(received_at)
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
            eprintln!("{exchange_rust}, {market_type}, {msg_rust}, error: {err:?}");
            std::ptr::null()
        }
    }
}

/// Parse a level2 topk orderbook message into a Vec<OrderBookMsg> and then convert to a JSON string.
#[no_mangle]
pub extern "C" fn parse_l2_topk(
    exchange: *const c_char,
    market_type: MarketType,
    msg: *const c_char,
    received_at: i64,
) -> *const c_char {
    let exchange_rust = unsafe {
        debug_assert!(!exchange.is_null());
        CStr::from_ptr(exchange).to_str().unwrap()
    };
    let msg_rust = unsafe {
        debug_assert!(!msg.is_null());
        CStr::from_ptr(msg).to_str().unwrap()
    };
    let timestamp_rust = if received_at <= 0 {
        None
    } else {
        Some(received_at)
    };

    let result = std::panic::catch_unwind(|| {
        if let Ok(orderbooks) =
            crypto_msg_parser::parse_l2_topk(exchange_rust, market_type, msg_rust, timestamp_rust)
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
            eprintln!("{exchange_rust}, {market_type}, {msg_rust}, error: {err:?}");
            std::ptr::null()
        }
    }
}

/// Parse a BBO(best bid&offer) message into a Vec<BboMsg> and then convert to a JSON string.
#[no_mangle]
pub extern "C" fn parse_bbo(
    exchange: *const c_char,
    market_type: MarketType,
    msg: *const c_char,
    received_at: i64,
) -> *const c_char {
    let exchange_rust = unsafe {
        debug_assert!(!exchange.is_null());
        CStr::from_ptr(exchange).to_str().unwrap()
    };
    let msg_rust = unsafe {
        debug_assert!(!msg.is_null());
        CStr::from_ptr(msg).to_str().unwrap()
    };
    let timestamp_rust = if received_at <= 0 {
        None
    } else {
        Some(received_at)
    };

    let result = std::panic::catch_unwind(|| {
        if let Ok(msgs) =
            crypto_msg_parser::parse_bbo(exchange_rust, market_type, msg_rust, timestamp_rust)
        {
            let text = serde_json::to_string(&msgs).unwrap();
            let raw = CString::new(text).unwrap();
            raw.into_raw() as *const c_char
        } else {
            std::ptr::null()
        }
    });
    match result {
        Ok(ptr) => ptr,
        Err(err) => {
            eprintln!("{exchange_rust}, {market_type}, {msg_rust}, error: {err:?}");
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
    received_at: i64,
) -> *const c_char {
    let exchange_rust = unsafe {
        debug_assert!(!exchange.is_null());
        CStr::from_ptr(exchange).to_str().unwrap()
    };
    let msg_rust = unsafe {
        debug_assert!(!msg.is_null());
        CStr::from_ptr(msg).to_str().unwrap()
    };
    let timestamp_rust = if received_at <= 0 {
        None
    } else {
        Some(received_at)
    };

    let result = std::panic::catch_unwind(|| {
        if let Ok(rates) = crypto_msg_parser::parse_funding_rate(
            exchange_rust,
            market_type,
            msg_rust,
            timestamp_rust,
        ) {
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
            eprintln!("{exchange_rust}, {market_type}, {msg_rust}, error: {err:?}");
            std::ptr::null()
        }
    }
}

/// Parse a raw candlestick message into a Vec<CandlestickMsg> and then convert to a JSON string.
#[no_mangle]
pub extern "C" fn parse_candlestick(
    exchange: *const c_char,
    market_type: MarketType,
    msg: *const c_char,
    received_at: i64,
) -> *const c_char {
    let exchange_rust = unsafe {
        debug_assert!(!exchange.is_null());
        CStr::from_ptr(exchange).to_str().unwrap()
    };
    let msg_rust = unsafe {
        debug_assert!(!msg.is_null());
        CStr::from_ptr(msg).to_str().unwrap()
    };
    let timestamp_rust = if received_at <= 0 {
        None
    } else {
        Some(received_at)
    };

    let result = std::panic::catch_unwind(|| {
        if let Ok(msgs) = crypto_msg_parser::parse_candlestick(
            exchange_rust,
            market_type,
            msg_rust,
            timestamp_rust,
        ) {
            let text = serde_json::to_string(&msgs).unwrap();
            let raw = CString::new(text).unwrap();
            raw.into_raw() as *const c_char
        } else {
            std::ptr::null()
        }
    });
    match result {
        Ok(ptr) => ptr,
        Err(err) => {
            eprintln!("{exchange_rust}, {market_type}, {msg_rust}, error: {err:?}");
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

    use super::{
        deallocate_string, extract_timestamp, get_msg_type, parse_bbo, parse_candlestick,
        parse_funding_rate, parse_l2, parse_trade,
    };
    use float_cmp::approx_eq;
    use std::ffi::{CStr, CString};

    #[test]
    fn test_parse_trade() {
        let exchange = CString::new("binance").unwrap();
        let raw_msg = CString::new(r#"{"stream":"btcusd_perp@aggTrade","data":{"e":"aggTrade","E":1616201883458,"a":41045788,"s":"BTCUSD_PERP","p":"58570.1","q":"58","f":91864326,"l":91864327,"T":1616201883304,"m":true}}"#).unwrap();

        let (json_ptr, json_str) = {
            let json_ptr =
                parse_trade(exchange.as_ptr(), MarketType::InverseSwap, raw_msg.as_ptr());
            let json_c_str = unsafe {
                debug_assert!(!json_ptr.is_null());
                CStr::from_ptr(json_ptr)
            };

            (json_ptr, json_c_str.to_str().unwrap())
        };

        let trades = serde_json::from_str::<Vec<crypto_message::TradeMsg>>(json_str).unwrap();
        assert_eq!(trades.len(), 1);
        let trade = &trades[0];

        assert_eq!(trade.exchange, "binance");
        assert_eq!(trade.market_type, MarketType::InverseSwap);
        assert_eq!(trade.msg_type, MessageType::Trade);
        assert_eq!(
            MessageType::Trade,
            get_msg_type(exchange.as_ptr(), raw_msg.as_ptr())
        );
        assert_eq!(trade.price, 58570.1);
        assert!(approx_eq!(
            f64,
            trade.quantity_base,
            5800.0 / 58570.1,
            epsilon = 0.00000000000000002
        ));
        assert_eq!(trade.quantity_quote, 5800.0);
        assert_eq!(trade.quantity_contract, Some(58.0));
        assert_eq!(trade.side, crypto_message::TradeSide::Sell);

        deallocate_string(json_ptr);
    }

    #[test]
    fn test_parse_l2() {
        let exchange = CString::new("binance").unwrap();
        let raw_msg = CString::new(r#"{"stream":"btcusd_perp@depth@100ms","data":{"e":"depthUpdate","E":1622370862564,"T":1622370862553,"s":"BTCUSD_PERP","ps":"BTCUSD","U":127559587191,"u":127559588177,"pu":127559587113,"b":[["35365.9","1400"],["35425.8","561"]],"a":[["35817.8","7885"],["35818.7","307"]]}}"#).unwrap();
        let (json_ptr, json_str) = {
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
            serde_json::from_str::<Vec<crypto_message::OrderBookMsg>>(json_str).unwrap();
        assert_eq!(orderbooks.len(), 1);
        let orderbook = &orderbooks[0];

        assert_eq!(orderbook.exchange, "binance");
        assert_eq!(orderbook.market_type, MarketType::InverseSwap);
        assert_eq!(orderbook.msg_type, MessageType::L2Event);
        assert_eq!(
            MessageType::L2Event,
            get_msg_type(exchange.as_ptr(), raw_msg.as_ptr())
        );
        assert_eq!(orderbook.asks.len(), 2);
        assert_eq!(orderbook.bids.len(), 2);
        assert!(!orderbook.snapshot);
        assert_eq!(orderbook.timestamp, 1622370862564);

        assert_eq!(orderbook.bids[0].price, 35365.9);
        assert_eq!(orderbook.bids[0].quantity_contract, Some(1400.0));
        assert_eq!(orderbook.asks[0].price, 35817.8);
        assert_eq!(orderbook.asks[0].quantity_contract, Some(7885.0));

        deallocate_string(json_ptr);
    }

    #[test]
    fn test_parse_bbo() {
        let exchange = CString::new("binance").unwrap();
        let raw_msg = CString::new(r#"{"stream":"ethusdt@bookTicker","data":{"e":"bookTicker","u":1553413152520,"s":"ETHUSDT","b":"1778.54","B":"15.164","a":"1778.55","A":"7.289","T":1653817855284,"E":1653817855289}}"#).unwrap();
        let (json_ptr, json_str) = {
            let json_ptr = parse_bbo(
                exchange.as_ptr(),
                MarketType::LinearSwap,
                raw_msg.as_ptr(),
                0,
            );
            let json_c_str = unsafe {
                debug_assert!(!json_ptr.is_null());
                CStr::from_ptr(json_ptr)
            };

            (json_ptr, json_c_str.to_str().unwrap())
        };

        let arr = serde_json::from_str::<Vec<crypto_message::BboMsg>>(json_str).unwrap();
        assert_eq!(arr.len(), 1);
        let bbo_msg = &arr[0];

        assert_eq!(MessageType::BBO, bbo_msg.msg_type);
        assert_eq!("ETHUSDT", bbo_msg.symbol);
        assert_eq!(1653817855289, bbo_msg.timestamp);
        assert_eq!(Some(1553413152520), bbo_msg.id);

        assert_eq!(1778.55, bbo_msg.ask_price);
        assert_eq!(7.289, bbo_msg.ask_quantity_base);
        assert_eq!(1778.55 * 7.289, bbo_msg.ask_quantity_quote);
        assert_eq!(Some(7.289), bbo_msg.ask_quantity_contract);

        assert_eq!(1778.54, bbo_msg.bid_price);
        assert_eq!(15.164, bbo_msg.bid_quantity_base);
        assert_eq!(1778.54 * 15.164, bbo_msg.bid_quantity_quote);
        assert_eq!(Some(15.164), bbo_msg.bid_quantity_contract);

        deallocate_string(json_ptr);
    }

    #[test]
    fn test_parse_funding_rate() {
        let exchange = CString::new("binance").unwrap();
        let raw_msg = CString::new(r#"{"stream":"btcusd_perp@markPrice","data":{"e":"markPriceUpdate","E":1617309477000,"s":"BTCUSD_PERP","p":"59012.56007222","P":"58896.00503145","r":"0.00073689","T":1617321600000}}"#).unwrap();

        let (json_ptr, json_str) = {
            let json_ptr = parse_funding_rate(
                exchange.as_ptr(),
                MarketType::InverseSwap,
                raw_msg.as_ptr(),
                -1,
            );
            let json_c_str = unsafe {
                debug_assert!(!json_ptr.is_null());
                CStr::from_ptr(json_ptr)
            };

            (json_ptr, json_c_str.to_str().unwrap())
        };

        let rates = serde_json::from_str::<Vec<crypto_message::FundingRateMsg>>(json_str).unwrap();
        assert_eq!(rates.len(), 1);
        let rate = &rates[0];

        assert_eq!(rate.exchange, "binance");
        assert_eq!(rate.market_type, MarketType::InverseSwap);
        assert_eq!(rate.msg_type, MessageType::FundingRate);
        assert_eq!(
            MessageType::FundingRate,
            get_msg_type(exchange.as_ptr(), raw_msg.as_ptr())
        );
        assert_eq!(rate.pair, "BTC/USD".to_string());
        assert_eq!(rate.funding_rate, 0.00073689);
        assert_eq!(rate.funding_time, 1617321600000);

        deallocate_string(json_ptr);
    }

    #[test]
    fn test_extract_timestamp() {
        let exchange = CString::new("binance").unwrap();
        let raw_msg = CString::new(r#"{"stream":"btcusd_perp@depth@100ms","data":{"e":"depthUpdate","E":1622370862564,"T":1622370862553,"s":"BTCUSD_PERP","ps":"BTCUSD","U":127559587191,"u":127559588177,"pu":127559587113,"b":[["35365.9","1400"],["35425.8","561"]],"a":[["35817.8","7885"],["35818.7","307"]]}}"#).unwrap();
        let timestamp =
            extract_timestamp(exchange.as_ptr(), MarketType::InverseSwap, raw_msg.as_ptr());
        assert_eq!(1622370862564, timestamp);
    }

    #[test]
    fn test_parse_candlestick() {
        let exchange = CString::new("binance").unwrap();
        let raw_msg = CString::new(r#"{"stream":"btcusdt@kline_1M","data":{"e":"kline","E":1653819041520,"s":"BTCUSDT","k":{"t":1651363200000,"T":1654041599999,"s":"BTCUSDT","i":"1M","f":2172726276,"L":2301806561,"o":"37614.40","c":"29075.50","h":"40071.70","l":"26631.00","v":"13431981.671","n":129025447,"x":false,"q":"423075730671.12853","V":"6700065.176","Q":"211000435586.65000","B":"0"}}}"#).unwrap();
        let (json_ptr, json_str) = {
            let json_ptr = parse_candlestick(
                exchange.as_ptr(),
                MarketType::LinearSwap,
                raw_msg.as_ptr(),
                -1,
            );
            let json_c_str = unsafe {
                debug_assert!(!json_ptr.is_null());
                CStr::from_ptr(json_ptr)
            };

            (json_ptr, json_c_str.to_str().unwrap())
        };

        let arr = serde_json::from_str::<Vec<crypto_message::CandlestickMsg>>(json_str).unwrap();
        assert_eq!(arr.len(), 1);
        let candlestick_msg = &arr[0];

        assert_eq!("BTCUSDT", candlestick_msg.symbol);
        assert_eq!(1653819041520, candlestick_msg.timestamp);
        assert_eq!("1M", candlestick_msg.period);
        assert_eq!(1651363200, candlestick_msg.begin_time);

        assert_eq!(37614.40, candlestick_msg.open);
        assert_eq!(40071.70, candlestick_msg.high);
        assert_eq!(26631.0, candlestick_msg.low);
        assert_eq!(29075.5, candlestick_msg.close);
        assert_eq!(13431981.671, candlestick_msg.volume);
        assert_eq!(Some(423075730671.12853), candlestick_msg.quote_volume);

        deallocate_string(json_ptr);
    }
}
