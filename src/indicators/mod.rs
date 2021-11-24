#![allow(missing_docs)]

use crate::core::{ValueType, OHLCV};

pub mod example;

#[derive(Clone, Copy, Debug)]
struct HLC {
	high: ValueType,
	low: ValueType,
	close: ValueType,
}

impl HLC {
	fn from<T: OHLCV>(src: &T) -> Self {
		Self {
			high: src.high(),
			low: src.low(),
			close: src.close(),
		}
	}
}

impl OHLCV for HLC {
	fn open(&self) -> ValueType {
		ValueType::NAN
	}

	#[inline]
	fn high(&self) -> ValueType {
		self.high
	}

	#[inline]
	fn low(&self) -> ValueType {
		self.low
	}

	#[inline]
	fn close(&self) -> ValueType {
		self.close
	}

	fn volume(&self) -> ValueType {
		ValueType::NAN
	}
}

mod aroon;
pub use aroon::Aroon;

mod average_directional_index;
pub use average_directional_index::AverageDirectionalIndex;

mod awesome_oscillator;
pub use awesome_oscillator::AwesomeOscillator;

mod bollinger_bands;
pub use bollinger_bands::BollingerBands;

mod chaikin_money_flow;
pub use chaikin_money_flow::ChaikinMoneyFlow;

mod chaikin_oscillator;
pub use chaikin_oscillator::ChaikinOscillator;

mod chande_kroll_stop;
pub use chande_kroll_stop::ChandeKrollStop;

mod chande_momentum_oscillator;
pub use chande_momentum_oscillator::ChandeMomentumOscillator;

mod commodity_channel_index;
pub use commodity_channel_index::CommodityChannelIndex;

mod coppock_curve;
pub use coppock_curve::CoppockCurve;

mod detrended_price_oscillator;
pub use detrended_price_oscillator::DetrendedPriceOscillator;

mod donchian_channel;
pub use donchian_channel::DonchianChannel;

mod ease_of_movement;
pub use ease_of_movement::EaseOfMovement;

mod elders_force_index;
pub use elders_force_index::EldersForceIndex;

mod envelopes;
pub use envelopes::Envelopes;

mod fisher_transform;
pub use fisher_transform::FisherTransform;

mod hull_moving_average;
pub use hull_moving_average::HullMovingAverage;

mod ichimoku_cloud;
pub use ichimoku_cloud::IchimokuCloud;

mod kaufman;
pub use kaufman::{Kaufman, KAMA};

mod keltner_channel;
pub use keltner_channel::KeltnerChannel;

mod klinger_volume_oscillator;
pub use klinger_volume_oscillator::KlingerVolumeOscillator;

mod know_sure_thing;
pub use know_sure_thing::KnowSureThing;

mod macd;
pub use macd::{MovingAverageConvergenceDivergence, MACD, MACDInstance};

mod momentum_index;
pub use momentum_index::MomentumIndex;

mod money_flow_index;
pub use money_flow_index::MoneyFlowIndex;

mod parabolic_sar;
pub use parabolic_sar::{ParabolicSAR, ParabolicStopAndReverse};

mod pivot_reversal_strategy;
pub use pivot_reversal_strategy::PivotReversalStrategy;

mod price_channel_strategy;
pub use price_channel_strategy::PriceChannelStrategy;

mod relative_strength_index;
pub use relative_strength_index::{RelativeStrengthIndex, RSI, RelativeStrengthIndexInstance};

mod relative_vigor_index;
pub use relative_vigor_index::RelativeVigorIndex;

mod smi_ergodic_indicator;
pub use smi_ergodic_indicator::SMIErgodicIndicator;

mod stochastic_oscillator;
pub use stochastic_oscillator::StochasticOscillator;

mod trix;
pub use trix::Trix;

mod trend_strength_index;
pub use trend_strength_index::TrendStrengthIndex;

mod true_strength_index;
pub use true_strength_index::TrueStrengthIndex;

mod woodies_cci;
pub use woodies_cci::WoodiesCCI;
