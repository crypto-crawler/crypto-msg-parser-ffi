/* Licensed under Apache-2.0 */

#ifndef CRYPTO_MSG_PARSER_FFI_H_
#define CRYPTO_MSG_PARSER_FFI_H_

/* Generated with cbindgen:0.20.0 */

/* Warning, this file is autogenerated by cbindgen. Don't modify this manually.
 */

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

#include "crypto_market_type.h"
#include "crypto_msg_type.h"

#ifdef __cplusplus
extern "C" {
#endif  // __cplusplus

/**
 * Extract the symbol from the message.
 */
const char *extract_symbol(const char *exchange, MarketType market_type,
                           const char *msg);

/**
 * Infer the message type from the message.
 */
MessageType get_msg_type(const char *exchange, const char *msg);

/**
 * Parse a raw trade message into a Vec<TradeMsg> and then convert to a JSON
 * string.
 */
const char *parse_trade(const char *exchange, MarketType market_type,
                        const char *msg);

/**
 * Parse a raw level2 orderbook message into a Vec<OrderBookMsg> and then
 * convert to a JSON string.
 */
const char *parse_l2(const char *exchange, MarketType market_type,
                     const char *msg, int64_t timestamp);

/**
 * Parse a raw funding rate message into a Vec<FundingRateMsg> and then convert
 * to a JSON string.
 */
const char *parse_funding_rate(const char *exchange, MarketType market_type,
                               const char *msg);

/**
 * Deallocate a string.
 */
void deallocate_string(const char *pointer);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus

#endif /* CRYPTO_MSG_PARSER_FFI_H_ */
