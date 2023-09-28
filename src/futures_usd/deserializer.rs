use log::info;
use serde::ser::Error;
use serde_json::Value;

use crate::futures_usd::enums::events::Event;
use crate::futures_usd::enums::events::Event::*;
use crate::futures_usd::enums::events::EventType::*;
use crate::futures_usd::response::{
    AssetIndexUpdate, AssetIndexUpdates, BookTicker, BookTickers, EventTypeWrapper,
    MarkPriceUpdate, MarkPriceUpdates, MiniTicker, MiniTickers, SubscribeResponse, Ticker, Tickers,
};

/// Deserialize a JSON response into an Event.
///
/// This function takes a JSON response as a String and attempts to deserialize it into an Event.
///
/// # Arguments
///
/// * `json_response` - A JSON response as a String.
///
/// # Returns
///
/// * A Result containing the deserialized Event or a serde_json::Error if deserialization fails.
///
pub fn deserialize(json_response: String) -> Result<Event, serde_json::Error> {
    // Try to deserialize into EventTypeWrapper
    if let Some(result) = try_deserialize_event_type_wrapper(&json_response) {
        return result;
    }
    // Try to deserialize into SubscribeResponse
    if let Some(result) = try_deserialize_subscribe_response(&json_response) {
        return result;
    }
    // Try to deserialize an anonymous array into EventTypeWrapper
    if let Some(result) = try_deserialize_anonymous_array(&json_response) {
        return result;
    }
    // Don't know what to do with response
    Err(serde_json::Error::custom(format!(
        "No deserializer for {:?}",
        json_response
    )))
}

/// Try to deserialize a JSON response into an Event based on EventTypeWrapper.
fn try_deserialize_event_type_wrapper(
    json_response: &str,
) -> Option<Result<Event, serde_json::Error>> {
    if let Ok(event_type_wrapper) = serde_json::from_str::<EventTypeWrapper>(json_response) {
        // Match the event_type field inside the EventTypeWrapper
        return Some(match event_type_wrapper.event_type {
            /* MARKET DATA */
            BookTickerEventType => Ok(BookTickerEvent(
                serde_json::from_str(json_response).unwrap(),
            )),
            AggTradeEventType => Ok(AggTradeEvent(serde_json::from_str(json_response).unwrap())),
            MarkPriceUpdateEventType => Ok(MarkPriceUpdateEvent(
                serde_json::from_str(json_response).unwrap(),
            )),
            KlineEventType => Ok(KlineEvent(serde_json::from_str(json_response).unwrap())),
            ContinuousKlineEventType => Ok(ContinuousKlineEvent(
                serde_json::from_str(json_response).unwrap(),
            )),
            MiniTickerEventType => Ok(MiniTickerEvent(
                serde_json::from_str(json_response).unwrap(),
            )),
            TickerEventType => Ok(TickerEvent(serde_json::from_str(json_response).unwrap())),
            ForceOrderEventType => Ok(ForceOrderEvent(
                serde_json::from_str(json_response).unwrap(),
            )),
            BookDepthEventType => Ok(BookDepthEvent(serde_json::from_str(json_response).unwrap())),
            CompositeIndexEventType => Ok(CompositeIndexEvent(
                serde_json::from_str(json_response).unwrap(),
            )),
            ContractInfoEventType => Ok(ContractInfoEvent(
                serde_json::from_str(json_response).unwrap(),
            )),
            AssetIndexUpdateEventType => Ok(AssetIndexUpdateEvent(
                serde_json::from_str(json_response).unwrap(),
            )),
            /* USER DATA */
            AccountUpdateEventType => Ok(AccountUpdateEvent(
                serde_json::from_str(json_response).unwrap(),
            )),
            OrderTradeUpdateEventType => Ok(OrderTradeUpdateEvent(
                serde_json::from_str(json_response).unwrap(),
            )),
            MarginCallEventType => Ok(MarginCallEvent(
                serde_json::from_str(json_response).unwrap(),
            )),
            AccountConfigUpdateEventType => Ok(AccountConfigUpdateEvent(
                serde_json::from_str(json_response).unwrap(),
            )),
            StrategyUpdateEventType => Ok(StrategyUpdateEvent(
                serde_json::from_str(json_response).unwrap(),
            )),
            GridUpdateEventType => Ok(GridUpdateEvent(
                serde_json::from_str(json_response).unwrap(),
            )),
            ConditionalOrderTriggerRejectEventType => Ok(ConditionalOrderTriggerRejectEvent(
                serde_json::from_str(json_response).unwrap(),
            )),
        });
    }
    None
}

/// Try to deserialize a JSON response into a SubscribeResponse.
fn try_deserialize_subscribe_response(
    json_response: &String,
) -> Option<Result<Event, serde_json::Error>> {
    // Try to deserialize the JSON response into a SubscribeResponse
    if let Ok(subscribe_response) = serde_json::from_str::<SubscribeResponse>(&json_response) {
        // Check if the SubscribeResponse contains an 'id' field
        if let Some(id) = subscribe_response.id {
            // Log a message indicating a futures_usd subscription request was received with the 'id'
            info!("futures_usd subscription request received ({})", id);
        }
        // Return a Some variant containing the deserialized SubscribeResponseE event
        return Some(Ok(SubscribeResponseEvent));
    }
    None
}

/// Try to deserialize anonymous array and convert it into an Event.
fn try_deserialize_anonymous_array(
    json_response: &String,
) -> Option<Result<Event, serde_json::Error>> {
    // Deserialize the JSON response into a serde_json::Value and handle errors
    let value: Value = serde_json::from_str(json_response).map_err(Some).ok()?;
    // Check if the deserialized value is an array
    if let Some(arr) = value.as_array() {
        // Check if the array contains at least one item
        if let Some(first_item) = arr.get(0) {
            // Try to deserialize the first item into an EventTypeWrapper
            if let Ok(event_type_wrapper) =
                serde_json::from_value::<EventTypeWrapper>(first_item.clone())
            {
                match event_type_wrapper.event_type {
                    MarkPriceUpdateEventType => {
                        let market_price_updates: Vec<MarkPriceUpdate> = arr
                            .iter()
                            .map(|item| serde_json::from_value(item.clone()).unwrap())
                            .collect();
                        return Some(Ok(MarkPriceUpdatesEvent(MarkPriceUpdates {
                            data: market_price_updates,
                        })));
                    }
                    MiniTickerEventType => {
                        let mini_tickers: Vec<MiniTicker> = arr
                            .iter()
                            .map(|item| serde_json::from_value(item.clone()).unwrap())
                            .collect();
                        return Some(Ok(MiniTickersEvent(MiniTickers { data: mini_tickers })));
                    }
                    TickerEventType => {
                        let tickers: Vec<Ticker> = arr
                            .iter()
                            .map(|item| serde_json::from_value(item.clone()).unwrap())
                            .collect();
                        return Some(Ok(TickersEvent(Tickers { data: tickers })));
                    }
                    BookTickerEventType => {
                        let book_tickers: Vec<BookTicker> = arr
                            .iter()
                            .map(|item| serde_json::from_value(item.clone()).unwrap())
                            .collect();
                        return Some(Ok(BookTickersEvent(BookTickers { data: book_tickers })));
                    }
                    AssetIndexUpdateEventType => {
                        let asset_index_updates: Vec<AssetIndexUpdate> = arr
                            .iter()
                            .map(|item| serde_json::from_value(item.clone()).unwrap())
                            .collect();
                        return Some(Ok(AssetIndexUpdatesEvent(AssetIndexUpdates {
                            data: asset_index_updates,
                        })));
                    }
                    _ => {}
                }
            }
        }
    }
    None
}
