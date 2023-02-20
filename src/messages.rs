// Generated code!
#![allow(unused_comparisons, unreachable_patterns)]
#![allow(clippy::let_and_return, clippy::eq_op)]
#![allow(clippy::excessive_precision, clippy::manual_range_contains, clippy::absurd_extreme_comparisons)]
#![deny(clippy::integer_arithmetic)]

//! Message definitions from file `"solax.dbc"`
//!
//! - Version: `Version("0.1")`

use core::ops::BitOr;
use bitvec::prelude::*;
#[cfg(feature = "arb")]
use arbitrary::{Arbitrary, Unstructured};

/// All messages
#[derive(Clone)]
#[cfg_attr(feature = "debug", derive(Debug))]
pub enum Messages {
    /// BMS_Limits
    BmsLimits(BmsLimits),
    /// BMS_PackData
    BmsPackData(BmsPackData),
    /// BMS_CellData
    BmsCellData(BmsCellData),
    /// BMS_Status
    BmsStatus(BmsStatus),
    /// BMS_PackTemps
    BmsPackTemps(BmsPackTemps),
    /// BMS_PackStats
    BmsPackStats(BmsPackStats),
}

impl Messages {
    /// Read message from CAN frame
    #[inline(never)]
    pub fn from_can_message(id: u32, payload: &[u8]) -> Result<Self, CanError> {
        use core::convert::TryFrom;
        
        let res = match id {
            6258 => Messages::BmsLimits(BmsLimits::try_from(payload)?),
            6259 => Messages::BmsPackData(BmsPackData::try_from(payload)?),
            6260 => Messages::BmsCellData(BmsCellData::try_from(payload)?),
            6261 => Messages::BmsStatus(BmsStatus::try_from(payload)?),
            6262 => Messages::BmsPackTemps(BmsPackTemps::try_from(payload)?),
            6264 => Messages::BmsPackStats(BmsPackStats::try_from(payload)?),
            n => return Err(CanError::UnknownMessageId(n)),
        };
        Ok(res)
    }
}

/// BMS_Limits
///
/// - ID: 6258 (0x1872)
/// - Size: 8 bytes
/// - Transmitter: Solax_inverter
#[derive(Clone, Copy)]
pub struct BmsLimits {
    raw: [u8; 8],
}

impl BmsLimits {
    pub const MESSAGE_ID: u32 = 6258;
    
    pub const SLAVE_VOLTAGE_MAX_MIN: f32 = 380_f32;
    pub const SLAVE_VOLTAGE_MAX_MAX: f32 = 400_f32;
    pub const SLAVE_VOLTAGE_MIN_MIN: f32 = 290_f32;
    pub const SLAVE_VOLTAGE_MIN_MAX: f32 = 330_f32;
    pub const MAX_CHARGE_RATE_MIN: f32 = 0_f32;
    pub const MAX_CHARGE_RATE_MAX: f32 = 253_f32;
    pub const MAX_DISCHARGE_RATE_MIN: f32 = 0_f32;
    pub const MAX_DISCHARGE_RATE_MAX: f32 = 35_f32;
    
    /// Construct new BMS_Limits from values
    pub fn new(slave_voltage_max: f32, slave_voltage_min: f32, max_charge_rate: f32, max_discharge_rate: f32) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 8] };
        res.set_slave_voltage_max(slave_voltage_max)?;
        res.set_slave_voltage_min(slave_voltage_min)?;
        res.set_max_charge_rate(max_charge_rate)?;
        res.set_max_discharge_rate(max_discharge_rate)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8] {
        &self.raw
    }
    
    /// slave_voltage_max
    ///
    /// - Min: 380
    /// - Max: 400
    /// - Unit: "V"
    /// - Receivers: Solax_inverter
    #[inline(always)]
    pub fn slave_voltage_max(&self) -> f32 {
        self.slave_voltage_max_raw()
    }
    
    /// Get raw value of slave_voltage_max
    ///
    /// - Start bit: 0
    /// - Signal size: 16 bits
    /// - Factor: 0.1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn slave_voltage_max_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[0..16].load_le::<u16>();
        
        let factor = 0.1_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of slave_voltage_max
    #[inline(always)]
    pub fn set_slave_voltage_max(&mut self, value: f32) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 380_f32 || 400_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 6258 });
        }
        let factor = 0.1_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as u16;
        
        self.raw.view_bits_mut::<Lsb0>()[0..16].store_le(value);
        Ok(())
    }
    
    /// slave_voltage_min
    ///
    /// - Min: 290
    /// - Max: 330
    /// - Unit: "V"
    /// - Receivers: Solax_inverter
    #[inline(always)]
    pub fn slave_voltage_min(&self) -> f32 {
        self.slave_voltage_min_raw()
    }
    
    /// Get raw value of slave_voltage_min
    ///
    /// - Start bit: 16
    /// - Signal size: 16 bits
    /// - Factor: 0.1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn slave_voltage_min_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[16..32].load_le::<u16>();
        
        let factor = 0.1_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of slave_voltage_min
    #[inline(always)]
    pub fn set_slave_voltage_min(&mut self, value: f32) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 290_f32 || 330_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 6258 });
        }
        let factor = 0.1_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as u16;
        
        self.raw.view_bits_mut::<Lsb0>()[16..32].store_le(value);
        Ok(())
    }
    
    /// max_charge_rate
    ///
    /// - Min: 0
    /// - Max: 253
    /// - Unit: "A"
    /// - Receivers: Solax_inverter
    #[inline(always)]
    pub fn max_charge_rate(&self) -> f32 {
        self.max_charge_rate_raw()
    }
    
    /// Get raw value of max_charge_rate
    ///
    /// - Start bit: 32
    /// - Signal size: 16 bits
    /// - Factor: 0.1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn max_charge_rate_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[32..48].load_le::<u16>();
        
        let factor = 0.1_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of max_charge_rate
    #[inline(always)]
    pub fn set_max_charge_rate(&mut self, value: f32) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_f32 || 253_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 6258 });
        }
        let factor = 0.1_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as u16;
        
        self.raw.view_bits_mut::<Lsb0>()[32..48].store_le(value);
        Ok(())
    }
    
    /// max_discharge_rate
    ///
    /// - Min: 0
    /// - Max: 35
    /// - Unit: "A"
    /// - Receivers: Solax_inverter
    #[inline(always)]
    pub fn max_discharge_rate(&self) -> f32 {
        self.max_discharge_rate_raw()
    }
    
    /// Get raw value of max_discharge_rate
    ///
    /// - Start bit: 48
    /// - Signal size: 16 bits
    /// - Factor: 0.1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn max_discharge_rate_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[48..64].load_le::<u16>();
        
        let factor = 0.1_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of max_discharge_rate
    #[inline(always)]
    pub fn set_max_discharge_rate(&mut self, value: f32) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_f32 || 35_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 6258 });
        }
        let factor = 0.1_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as u16;
        
        self.raw.view_bits_mut::<Lsb0>()[48..64].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for BmsLimits {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 8 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 8];
        raw.copy_from_slice(&payload[..8]);
        Ok(Self { raw })
    }
}

#[cfg(feature = "debug")]
impl core::fmt::Debug for BmsLimits {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if f.alternate() {
            f.debug_struct("BmsLimits")
                .field("slave_voltage_max", &self.slave_voltage_max())
                .field("slave_voltage_min", &self.slave_voltage_min())
                .field("max_charge_rate", &self.max_charge_rate())
                .field("max_discharge_rate", &self.max_discharge_rate())
            .finish()
        } else {
            f.debug_tuple("BmsLimits").field(&self.raw).finish()
        }
    }
}

#[cfg(feature = "arb")]
impl<'a> Arbitrary<'a> for BmsLimits {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self, arbitrary::Error> {
        let slave_voltage_max = u.float_in_range(380_f32..=400_f32)?;
        let slave_voltage_min = u.float_in_range(290_f32..=330_f32)?;
        let max_charge_rate = u.float_in_range(0_f32..=253_f32)?;
        let max_discharge_rate = u.float_in_range(0_f32..=35_f32)?;
        BmsLimits::new(slave_voltage_max,slave_voltage_min,max_charge_rate,max_discharge_rate).map_err(|_| arbitrary::Error::IncorrectFormat)
    }
}

/// BMS_PackData
///
/// - ID: 6259 (0x1873)
/// - Size: 8 bytes
/// - Transmitter: Solax_inverter
#[derive(Clone, Copy)]
pub struct BmsPackData {
    raw: [u8; 8],
}

impl BmsPackData {
    pub const MESSAGE_ID: u32 = 6259;
    
    pub const MASTER_VOLTAGE_MIN: f32 = 290_f32;
    pub const MASTER_VOLTAGE_MAX: f32 = 400_f32;
    pub const CURRENT_SENSOR_MIN: f32 = -40_f32;
    pub const CURRENT_SENSOR_MAX: f32 = 40_f32;
    pub const SOC_MIN: u16 = 0_u16;
    pub const SOC_MAX: u16 = 100_u16;
    pub const KWH_REMAINING_MIN: f32 = 0_f32;
    pub const KWH_REMAINING_MAX: f32 = 100_f32;
    
    /// Construct new BMS_PackData from values
    pub fn new(master_voltage: f32, current_sensor: f32, soc: u16, kwh_remaining: f32) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 8] };
        res.set_master_voltage(master_voltage)?;
        res.set_current_sensor(current_sensor)?;
        res.set_soc(soc)?;
        res.set_kwh_remaining(kwh_remaining)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8] {
        &self.raw
    }
    
    /// master_voltage
    ///
    /// - Min: 290
    /// - Max: 400
    /// - Unit: "V"
    /// - Receivers: Solax_inverter
    #[inline(always)]
    pub fn master_voltage(&self) -> f32 {
        self.master_voltage_raw()
    }
    
    /// Get raw value of master_voltage
    ///
    /// - Start bit: 0
    /// - Signal size: 16 bits
    /// - Factor: 0.1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn master_voltage_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[0..16].load_le::<u16>();
        
        let factor = 0.1_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of master_voltage
    #[inline(always)]
    pub fn set_master_voltage(&mut self, value: f32) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 290_f32 || 400_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 6259 });
        }
        let factor = 0.1_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as u16;
        
        self.raw.view_bits_mut::<Lsb0>()[0..16].store_le(value);
        Ok(())
    }
    
    /// current_sensor
    ///
    /// - Min: -40
    /// - Max: 40
    /// - Unit: "V"
    /// - Receivers: Solax_inverter
    #[inline(always)]
    pub fn current_sensor(&self) -> f32 {
        self.current_sensor_raw()
    }
    
    /// Get raw value of current_sensor
    ///
    /// - Start bit: 16
    /// - Signal size: 16 bits
    /// - Factor: 0.1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn current_sensor_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[16..32].load_le::<u16>();
        
        let factor = 0.1_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of current_sensor
    #[inline(always)]
    pub fn set_current_sensor(&mut self, value: f32) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < -40_f32 || 40_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 6259 });
        }
        let factor = 0.1_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as u16;
        
        self.raw.view_bits_mut::<Lsb0>()[16..32].store_le(value);
        Ok(())
    }
    
    /// soc
    ///
    /// - Min: 0
    /// - Max: 100
    /// - Unit: "%"
    /// - Receivers: Solax_inverter
    #[inline(always)]
    pub fn soc(&self) -> u16 {
        self.soc_raw()
    }
    
    /// Get raw value of soc
    ///
    /// - Start bit: 32
    /// - Signal size: 16 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn soc_raw(&self) -> u16 {
        let signal = self.raw.view_bits::<Lsb0>()[32..48].load_le::<u16>();
        
        signal
    }
    
    /// Set value of soc
    #[inline(always)]
    pub fn set_soc(&mut self, value: u16) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_u16 || 100_u16 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 6259 });
        }
        self.raw.view_bits_mut::<Lsb0>()[32..48].store_le(value);
        Ok(())
    }
    
    /// kwh_remaining
    ///
    /// - Min: 0
    /// - Max: 100
    /// - Unit: "kWh"
    /// - Receivers: Solax_inverter
    #[inline(always)]
    pub fn kwh_remaining(&self) -> f32 {
        self.kwh_remaining_raw()
    }
    
    /// Get raw value of kwh_remaining
    ///
    /// - Start bit: 48
    /// - Signal size: 16 bits
    /// - Factor: 0.1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn kwh_remaining_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[48..64].load_le::<u16>();
        
        let factor = 0.1_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of kwh_remaining
    #[inline(always)]
    pub fn set_kwh_remaining(&mut self, value: f32) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_f32 || 100_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 6259 });
        }
        let factor = 0.1_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as u16;
        
        self.raw.view_bits_mut::<Lsb0>()[48..64].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for BmsPackData {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 8 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 8];
        raw.copy_from_slice(&payload[..8]);
        Ok(Self { raw })
    }
}

#[cfg(feature = "debug")]
impl core::fmt::Debug for BmsPackData {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if f.alternate() {
            f.debug_struct("BmsPackData")
                .field("master_voltage", &self.master_voltage())
                .field("current_sensor", &self.current_sensor())
                .field("soc", &self.soc())
                .field("kwh_remaining", &self.kwh_remaining())
            .finish()
        } else {
            f.debug_tuple("BmsPackData").field(&self.raw).finish()
        }
    }
}

#[cfg(feature = "arb")]
impl<'a> Arbitrary<'a> for BmsPackData {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self, arbitrary::Error> {
        let master_voltage = u.float_in_range(290_f32..=400_f32)?;
        let current_sensor = u.float_in_range(-40_f32..=40_f32)?;
        let soc = u.int_in_range(0..=100)?;
        let kwh_remaining = u.float_in_range(0_f32..=100_f32)?;
        BmsPackData::new(master_voltage,current_sensor,soc,kwh_remaining).map_err(|_| arbitrary::Error::IncorrectFormat)
    }
}

/// BMS_CellData
///
/// - ID: 6260 (0x1874)
/// - Size: 8 bytes
/// - Transmitter: Solax_inverter
#[derive(Clone, Copy)]
pub struct BmsCellData {
    raw: [u8; 8],
}

impl BmsCellData {
    pub const MESSAGE_ID: u32 = 6260;
    
    pub const CELL_VOLTS_LOW_MIN: f32 = 2900_f32;
    pub const CELL_VOLTS_LOW_MAX: f32 = 4200_f32;
    pub const CELL_VOLTS_HIGH_MIN: f32 = 2900_f32;
    pub const CELL_VOLTS_HIGH_MAX: f32 = 4200_f32;
    pub const CELL_TEMPERATURE_LOW_MIN: f32 = -40_f32;
    pub const CELL_TEMPERATURE_LOW_MAX: f32 = 60_f32;
    pub const CELL_TEMPERATURE_HIGH_MIN: f32 = -40_f32;
    pub const CELL_TEMPERATURE_HIGH_MAX: f32 = 60_f32;
    
    /// Construct new BMS_CellData from values
    pub fn new(cell_volts_low: f32, cell_volts_high: f32, cell_temperature_low: f32, cell_temperature_high: f32) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 8] };
        res.set_cell_volts_low(cell_volts_low)?;
        res.set_cell_volts_high(cell_volts_high)?;
        res.set_cell_temperature_low(cell_temperature_low)?;
        res.set_cell_temperature_high(cell_temperature_high)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8] {
        &self.raw
    }
    
    /// cell_volts_low
    ///
    /// - Min: 2900
    /// - Max: 4200
    /// - Unit: "mV"
    /// - Receivers: Solax_inverter
    #[inline(always)]
    pub fn cell_volts_low(&self) -> f32 {
        self.cell_volts_low_raw()
    }
    
    /// Get raw value of cell_volts_low
    ///
    /// - Start bit: 16
    /// - Signal size: 16 bits
    /// - Factor: 0.1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn cell_volts_low_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[16..32].load_le::<u16>();
        
        let factor = 0.1_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of cell_volts_low
    #[inline(always)]
    pub fn set_cell_volts_low(&mut self, value: f32) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 2900_f32 || 4200_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 6260 });
        }
        let factor = 0.1_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as u16;
        
        self.raw.view_bits_mut::<Lsb0>()[16..32].store_le(value);
        Ok(())
    }
    
    /// cell_volts_high
    ///
    /// - Min: 2900
    /// - Max: 4200
    /// - Unit: "mV"
    /// - Receivers: Solax_inverter
    #[inline(always)]
    pub fn cell_volts_high(&self) -> f32 {
        self.cell_volts_high_raw()
    }
    
    /// Get raw value of cell_volts_high
    ///
    /// - Start bit: 0
    /// - Signal size: 16 bits
    /// - Factor: 0.1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn cell_volts_high_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[0..16].load_le::<u16>();
        
        let factor = 0.1_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of cell_volts_high
    #[inline(always)]
    pub fn set_cell_volts_high(&mut self, value: f32) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 2900_f32 || 4200_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 6260 });
        }
        let factor = 0.1_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as u16;
        
        self.raw.view_bits_mut::<Lsb0>()[0..16].store_le(value);
        Ok(())
    }
    
    /// cell_temperature_low
    ///
    /// - Min: -40
    /// - Max: 60
    /// - Unit: "C"
    /// - Receivers: Solax_inverter
    #[inline(always)]
    pub fn cell_temperature_low(&self) -> f32 {
        self.cell_temperature_low_raw()
    }
    
    /// Get raw value of cell_temperature_low
    ///
    /// - Start bit: 48
    /// - Signal size: 16 bits
    /// - Factor: 100
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn cell_temperature_low_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[48..64].load_le::<u16>();
        
        let factor = 100_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of cell_temperature_low
    #[inline(always)]
    pub fn set_cell_temperature_low(&mut self, value: f32) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < -40_f32 || 60_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 6260 });
        }
        let factor = 100_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as u16;
        
        self.raw.view_bits_mut::<Lsb0>()[48..64].store_le(value);
        Ok(())
    }
    
    /// cell_temperature_high
    ///
    /// - Min: -40
    /// - Max: 60
    /// - Unit: "C"
    /// - Receivers: Solax_inverter
    #[inline(always)]
    pub fn cell_temperature_high(&self) -> f32 {
        self.cell_temperature_high_raw()
    }
    
    /// Get raw value of cell_temperature_high
    ///
    /// - Start bit: 32
    /// - Signal size: 16 bits
    /// - Factor: 100
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn cell_temperature_high_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[32..48].load_le::<u16>();
        
        let factor = 100_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of cell_temperature_high
    #[inline(always)]
    pub fn set_cell_temperature_high(&mut self, value: f32) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < -40_f32 || 60_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 6260 });
        }
        let factor = 100_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as u16;
        
        self.raw.view_bits_mut::<Lsb0>()[32..48].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for BmsCellData {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 8 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 8];
        raw.copy_from_slice(&payload[..8]);
        Ok(Self { raw })
    }
}

#[cfg(feature = "debug")]
impl core::fmt::Debug for BmsCellData {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if f.alternate() {
            f.debug_struct("BmsCellData")
                .field("cell_volts_low", &self.cell_volts_low())
                .field("cell_volts_high", &self.cell_volts_high())
                .field("cell_temperature_low", &self.cell_temperature_low())
                .field("cell_temperature_high", &self.cell_temperature_high())
            .finish()
        } else {
            f.debug_tuple("BmsCellData").field(&self.raw).finish()
        }
    }
}

#[cfg(feature = "arb")]
impl<'a> Arbitrary<'a> for BmsCellData {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self, arbitrary::Error> {
        let cell_volts_low = u.float_in_range(2900_f32..=4200_f32)?;
        let cell_volts_high = u.float_in_range(2900_f32..=4200_f32)?;
        let cell_temperature_low = u.float_in_range(-40_f32..=60_f32)?;
        let cell_temperature_high = u.float_in_range(-40_f32..=60_f32)?;
        BmsCellData::new(cell_volts_low,cell_volts_high,cell_temperature_low,cell_temperature_high).map_err(|_| arbitrary::Error::IncorrectFormat)
    }
}

/// BMS_Status
///
/// - ID: 6261 (0x1875)
/// - Size: 8 bytes
/// - Transmitter: Solax_inverter
#[derive(Clone, Copy)]
pub struct BmsStatus {
    raw: [u8; 8],
}

impl BmsStatus {
    pub const MESSAGE_ID: u32 = 6261;
    
    pub const PACK_TEMPERATURE_MIN: f32 = -40_f32;
    pub const PACK_TEMPERATURE_MAX: f32 = 60_f32;
    
    /// Construct new BMS_Status from values
    pub fn new(bit: bool, contactor: bool, pack_temperature: f32) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 8] };
        res.set_bit(bit)?;
        res.set_contactor(contactor)?;
        res.set_pack_temperature(pack_temperature)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8] {
        &self.raw
    }
    
    /// bit
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: ""
    /// - Receivers: Solax_inverter
    #[inline(always)]
    pub fn bit(&self) -> bool {
        self.bit_raw()
    }
    
    /// Get raw value of bit
    ///
    /// - Start bit: 16
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn bit_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[16..17].load_le::<u8>();
        
        let signal  = i8::from_ne_bytes(signal.to_ne_bytes());
        signal == 1
    }
    
    /// Set value of bit
    #[inline(always)]
    pub fn set_bit(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        let value = u8::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[16..17].store_le(value);
        Ok(())
    }
    
    /// contactor
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: ""
    /// - Receivers: Solax_inverter
    #[inline(always)]
    pub fn contactor(&self) -> bool {
        self.contactor_raw()
    }
    
    /// Get raw value of contactor
    ///
    /// - Start bit: 32
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn contactor_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[32..33].load_le::<u8>();
        
        let signal  = i8::from_ne_bytes(signal.to_ne_bytes());
        signal == 1
    }
    
    /// Set value of contactor
    #[inline(always)]
    pub fn set_contactor(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        let value = u8::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[32..33].store_le(value);
        Ok(())
    }
    
    /// pack_temperature
    ///
    /// - Min: -40
    /// - Max: 60
    /// - Unit: "C"
    /// - Receivers: Solax_inverter
    #[inline(always)]
    pub fn pack_temperature(&self) -> f32 {
        self.pack_temperature_raw()
    }
    
    /// Get raw value of pack_temperature
    ///
    /// - Start bit: 0
    /// - Signal size: 16 bits
    /// - Factor: 0.1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn pack_temperature_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[0..16].load_le::<u16>();
        
        let factor = 0.1_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of pack_temperature
    #[inline(always)]
    pub fn set_pack_temperature(&mut self, value: f32) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < -40_f32 || 60_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 6261 });
        }
        let factor = 0.1_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as u16;
        
        self.raw.view_bits_mut::<Lsb0>()[0..16].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for BmsStatus {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 8 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 8];
        raw.copy_from_slice(&payload[..8]);
        Ok(Self { raw })
    }
}

#[cfg(feature = "debug")]
impl core::fmt::Debug for BmsStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if f.alternate() {
            f.debug_struct("BmsStatus")
                .field("bit", &self.bit())
                .field("contactor", &self.contactor())
                .field("pack_temperature", &self.pack_temperature())
            .finish()
        } else {
            f.debug_tuple("BmsStatus").field(&self.raw).finish()
        }
    }
}

#[cfg(feature = "arb")]
impl<'a> Arbitrary<'a> for BmsStatus {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self, arbitrary::Error> {
        let bit = u.int_in_range(0..=1)? == 1;
        let contactor = u.int_in_range(0..=1)? == 1;
        let pack_temperature = u.float_in_range(-40_f32..=60_f32)?;
        BmsStatus::new(bit,contactor,pack_temperature).map_err(|_| arbitrary::Error::IncorrectFormat)
    }
}

/// BMS_PackTemps
///
/// - ID: 6262 (0x1876)
/// - Size: 8 bytes
/// - Transmitter: Solax_inverter
#[derive(Clone, Copy)]
pub struct BmsPackTemps {
    raw: [u8; 8],
}

impl BmsPackTemps {
    pub const MESSAGE_ID: u32 = 6262;
    
    pub const TEMP1_MIN: f32 = -40_f32;
    pub const TEMP1_MAX: f32 = 60_f32;
    pub const TEMP2_MIN: f32 = -40_f32;
    pub const TEMP2_MAX: f32 = 60_f32;
    
    /// Construct new BMS_PackTemps from values
    pub fn new(bit: bool, temp1: f32, temp2: f32) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 8] };
        res.set_bit(bit)?;
        res.set_temp1(temp1)?;
        res.set_temp2(temp2)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8] {
        &self.raw
    }
    
    /// bit
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: ""
    /// - Receivers: Solax_inverter
    #[inline(always)]
    pub fn bit(&self) -> bool {
        self.bit_raw()
    }
    
    /// Get raw value of bit
    ///
    /// - Start bit: 0
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn bit_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[0..1].load_le::<u8>();
        
        let signal  = i8::from_ne_bytes(signal.to_ne_bytes());
        signal == 1
    }
    
    /// Set value of bit
    #[inline(always)]
    pub fn set_bit(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        let value = u8::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[0..1].store_le(value);
        Ok(())
    }
    
    /// temp1
    ///
    /// - Min: -40
    /// - Max: 60
    /// - Unit: "C"
    /// - Receivers: Solax_inverter
    #[inline(always)]
    pub fn temp1(&self) -> f32 {
        self.temp1_raw()
    }
    
    /// Get raw value of temp1
    ///
    /// - Start bit: 16
    /// - Signal size: 16 bits
    /// - Factor: 0.01
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn temp1_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[16..32].load_le::<u16>();
        
        let factor = 0.01_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of temp1
    #[inline(always)]
    pub fn set_temp1(&mut self, value: f32) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < -40_f32 || 60_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 6262 });
        }
        let factor = 0.01_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as u16;
        
        self.raw.view_bits_mut::<Lsb0>()[16..32].store_le(value);
        Ok(())
    }
    
    /// temp2
    ///
    /// - Min: -40
    /// - Max: 60
    /// - Unit: "C"
    /// - Receivers: Solax_inverter
    #[inline(always)]
    pub fn temp2(&self) -> f32 {
        self.temp2_raw()
    }
    
    /// Get raw value of temp2
    ///
    /// - Start bit: 48
    /// - Signal size: 16 bits
    /// - Factor: 0.01
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn temp2_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[48..64].load_le::<u16>();
        
        let factor = 0.01_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of temp2
    #[inline(always)]
    pub fn set_temp2(&mut self, value: f32) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < -40_f32 || 60_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 6262 });
        }
        let factor = 0.01_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as u16;
        
        self.raw.view_bits_mut::<Lsb0>()[48..64].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for BmsPackTemps {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 8 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 8];
        raw.copy_from_slice(&payload[..8]);
        Ok(Self { raw })
    }
}

#[cfg(feature = "debug")]
impl core::fmt::Debug for BmsPackTemps {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if f.alternate() {
            f.debug_struct("BmsPackTemps")
                .field("bit", &self.bit())
                .field("temp1", &self.temp1())
                .field("temp2", &self.temp2())
            .finish()
        } else {
            f.debug_tuple("BmsPackTemps").field(&self.raw).finish()
        }
    }
}

#[cfg(feature = "arb")]
impl<'a> Arbitrary<'a> for BmsPackTemps {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self, arbitrary::Error> {
        let bit = u.int_in_range(0..=1)? == 1;
        let temp1 = u.float_in_range(-40_f32..=60_f32)?;
        let temp2 = u.float_in_range(-40_f32..=60_f32)?;
        BmsPackTemps::new(bit,temp1,temp2).map_err(|_| arbitrary::Error::IncorrectFormat)
    }
}

/// BMS_PackStats
///
/// - ID: 6264 (0x1878)
/// - Size: 8 bytes
/// - Transmitter: Solax_inverter
#[derive(Clone, Copy)]
pub struct BmsPackStats {
    raw: [u8; 8],
}

impl BmsPackStats {
    pub const MESSAGE_ID: u32 = 6264;
    
    pub const PACK_VOLTAGE_MIN: f32 = 290_f32;
    pub const PACK_VOLTAGE_MAX: f32 = 400_f32;
    pub const TOTAL_WATT_HRS_MIN: u32 = 0_u32;
    pub const TOTAL_WATT_HRS_MAX: u32 = 429497000_u32;
    
    /// Construct new BMS_PackStats from values
    pub fn new(pack_voltage: f32, total_watt_hrs: u32) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 8] };
        res.set_pack_voltage(pack_voltage)?;
        res.set_total_watt_hrs(total_watt_hrs)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8] {
        &self.raw
    }
    
    /// pack_voltage
    ///
    /// - Min: 290
    /// - Max: 400
    /// - Unit: "V"
    /// - Receivers: Solax_inverter
    #[inline(always)]
    pub fn pack_voltage(&self) -> f32 {
        self.pack_voltage_raw()
    }
    
    /// Get raw value of pack_voltage
    ///
    /// - Start bit: 0
    /// - Signal size: 16 bits
    /// - Factor: 0.1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn pack_voltage_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[0..16].load_le::<u16>();
        
        let factor = 0.1_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of pack_voltage
    #[inline(always)]
    pub fn set_pack_voltage(&mut self, value: f32) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 290_f32 || 400_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 6264 });
        }
        let factor = 0.1_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as u16;
        
        self.raw.view_bits_mut::<Lsb0>()[0..16].store_le(value);
        Ok(())
    }
    
    /// total_watt_hrs
    ///
    /// - Min: 0
    /// - Max: 429497000
    /// - Unit: "Wh"
    /// - Receivers: Solax_inverter
    #[inline(always)]
    pub fn total_watt_hrs(&self) -> u32 {
        self.total_watt_hrs_raw()
    }
    
    /// Get raw value of total_watt_hrs
    ///
    /// - Start bit: 32
    /// - Signal size: 32 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn total_watt_hrs_raw(&self) -> u32 {
        let signal = self.raw.view_bits::<Lsb0>()[32..64].load_le::<u32>();
        
        signal
    }
    
    /// Set value of total_watt_hrs
    #[inline(always)]
    pub fn set_total_watt_hrs(&mut self, value: u32) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_u32 || 429497000_u32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 6264 });
        }
        self.raw.view_bits_mut::<Lsb0>()[32..64].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for BmsPackStats {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 8 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 8];
        raw.copy_from_slice(&payload[..8]);
        Ok(Self { raw })
    }
}

#[cfg(feature = "debug")]
impl core::fmt::Debug for BmsPackStats {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if f.alternate() {
            f.debug_struct("BmsPackStats")
                .field("pack_voltage", &self.pack_voltage())
                .field("total_watt_hrs", &self.total_watt_hrs())
            .finish()
        } else {
            f.debug_tuple("BmsPackStats").field(&self.raw).finish()
        }
    }
}

#[cfg(feature = "arb")]
impl<'a> Arbitrary<'a> for BmsPackStats {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self, arbitrary::Error> {
        let pack_voltage = u.float_in_range(290_f32..=400_f32)?;
        let total_watt_hrs = u.int_in_range(0..=429497000)?;
        BmsPackStats::new(pack_voltage,total_watt_hrs).map_err(|_| arbitrary::Error::IncorrectFormat)
    }
}


/// This is just to make testing easier
#[allow(dead_code)]
fn main() {}

#[derive(Clone, Copy, PartialEq)]
#[cfg_attr(any(feature = "debug", feature = "std"), derive(Debug))]
pub enum CanError {
    UnknownMessageId(u32),
    /// Signal parameter is not within the range
    /// defined in the dbc
    ParameterOutOfRange {
        /// dbc message id
        message_id: u32,
    },
    InvalidPayloadSize,
    /// Multiplexor value not defined in the dbc
    InvalidMultiplexor {
        /// dbc message id
        message_id: u32,
        /// Multiplexor value not defined in the dbc
        multiplexor: u16,
    },
}

#[cfg(feature = "std")]
use std::error::Error;
#[cfg(feature = "std")]
use std::fmt;

#[cfg(feature = "std")]
impl fmt::Display for CanError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[cfg(feature = "std")]
impl Error for CanError {}
#[cfg(feature = "arb")]
trait UnstructuredFloatExt {
    fn float_in_range(&mut self, range: core::ops::RangeInclusive<f32>) -> arbitrary::Result<f32>;
}

#[cfg(feature = "arb")]
impl UnstructuredFloatExt for arbitrary::Unstructured<'_> {
    fn float_in_range(&mut self, range: core::ops::RangeInclusive<f32>) -> arbitrary::Result<f32> {
        let min = range.start();
        let max = range.end();
        let steps = u32::MAX;
        let factor = (max - min) / (steps as f32);
        let random_int: u32 = self.int_in_range(0..=steps)?;
        let random = min + factor * (random_int as f32);
        Ok(random)
    }
}

