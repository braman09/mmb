use std::collections::HashMap;
use std::sync::Arc;

use parking_lot::{Mutex, RwLock};
use serde::{Deserialize, Serialize};

use super::exchanges::common::{Amount, Price, TradePlaceAccount};

// FIXME Probably it has to be pub(crate)
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct TradePlaceAccountStatistic {
    opened_orders_amount: usize,
    canceled_orders_amount: usize,
    partially_filled_orders_amount: usize,
    fully_filled_orders_amount: usize,
    summary_filled_amount: Amount,
    summary_commission: Price,
}

impl TradePlaceAccountStatistic {
    fn first_order_opened() -> Self {
        let mut trade_place_account_statistic = Self::default();
        trade_place_account_statistic.opened_orders_amount = 1;
        trade_place_account_statistic
    }

    fn first_order_cancelled() -> Self {
        let mut trade_place_account_statistic = Self::default();
        trade_place_account_statistic.canceled_orders_amount = 1;
        trade_place_account_statistic
    }

    fn first_partially_filled_order() -> Self {
        let mut trade_place_account_statistic = Self::default();
        trade_place_account_statistic.partially_filled_orders_amount = 1;
        trade_place_account_statistic
    }

    fn first_completelly_filled_order() -> Self {
        let mut trade_place_account_statistic = Self::default();
        trade_place_account_statistic.fully_filled_orders_amount = 1;
        trade_place_account_statistic
    }

    fn first_filled_amount(first_filled_amount: Amount) -> Self {
        let mut trade_place_account_statistic = Self::default();
        trade_place_account_statistic.summary_filled_amount = first_filled_amount;
        trade_place_account_statistic
    }

    fn first_commission(commission: Price) -> Self {
        let mut trade_place_account_statistic = Self::default();
        trade_place_account_statistic.summary_commission = commission;
        trade_place_account_statistic
    }

    fn order_created(&mut self) {
        self.opened_orders_amount += 1;
    }

    fn order_canceled(&mut self) {
        self.canceled_orders_amount += 1;
    }

    fn order_partially_filled(&mut self) {
        self.partially_filled_orders_amount += 1;
    }

    fn order_completely_filled(&mut self) {
        self.partially_filled_orders_amount = self.partially_filled_orders_amount.saturating_sub(1);
        self.fully_filled_orders_amount += 1;
    }

    fn add_summary_filled_amount(&mut self, filled_amount: Amount) {
        self.summary_filled_amount += filled_amount;
    }

    fn add_summary_commission(&mut self, commission: Price) {
        self.summary_commission += commission;
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DispositionExecutorStatistic {
    skipped_events_amount: usize,
}

impl DispositionExecutorStatistic {
    fn new(skipped_events_amount: usize) -> Self {
        Self {
            skipped_events_amount,
        }
    }
}

// FIXME in what meaning should it be Service? Should it be able to call graceful shutdown?
#[derive(Debug, Serialize, Deserialize)]
pub struct StatisticService {
    trade_place_data: RwLock<HashMap<TradePlaceAccount, TradePlaceAccountStatistic>>,
    disposition_executor_data: Mutex<DispositionExecutorStatistic>,
}

impl StatisticService {
    // FIXME Probably it has to be pub(crate)
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            trade_place_data: Default::default(),
            disposition_executor_data: Default::default(),
        })
    }

    pub(crate) fn order_created(self: Arc<Self>, trade_place_account: TradePlaceAccount) {
        self.trade_place_data
            .write()
            .entry(trade_place_account)
            .or_insert(TradePlaceAccountStatistic::first_order_opened())
            .order_created();
    }

    pub(crate) fn order_canceled(self: Arc<Self>, trade_place_account: TradePlaceAccount) {
        self.trade_place_data
            .write()
            .entry(trade_place_account)
            .or_insert(TradePlaceAccountStatistic::first_order_cancelled())
            .order_canceled();
    }

    pub(crate) fn order_partially_filled(self: Arc<Self>, trade_place_account: TradePlaceAccount) {
        self.trade_place_data
            .write()
            .entry(trade_place_account)
            .or_insert(TradePlaceAccountStatistic::first_partially_filled_order())
            .order_partially_filled();
    }

    pub(crate) fn order_completely_filled(self: Arc<Self>, trade_place_account: TradePlaceAccount) {
        self.trade_place_data
            .write()
            .entry(trade_place_account)
            .or_insert(TradePlaceAccountStatistic::first_completelly_filled_order())
            .order_completely_filled();
    }

    pub(crate) fn add_summary_amount(
        self: Arc<Self>,
        trade_place_account: TradePlaceAccount,
        filled_amount: Amount,
    ) {
        self.trade_place_data
            .write()
            .entry(trade_place_account)
            .or_insert(TradePlaceAccountStatistic::first_filled_amount(
                filled_amount,
            ))
            .add_summary_filled_amount(filled_amount);
    }

    pub(crate) fn add_summary_commission(
        self: Arc<Self>,
        trade_place_account: TradePlaceAccount,
        commission: Price,
    ) {
        self.trade_place_data
            .write()
            .entry(trade_place_account)
            .or_insert(TradePlaceAccountStatistic::first_commission(commission))
            .add_summary_commission(commission);
    }

    pub(crate) fn event_missed(self: Arc<Self>) {
        (*self.disposition_executor_data.lock()).skipped_events_amount += 1;
    }
}
