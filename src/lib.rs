#![allow(clippy::enum_variant_names)]
#![allow(clippy::uninlined_format_args)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![feature(error_in_core)]
#![no_std]

use crate::messages::*;
use embassy_time::{Duration, Instant};
use embedded_hal::can::{ExtendedId, Id};
use heapless::Vec as hVec;

#[cfg(not(feature = "defmt"))]
use log::{error, info};

#[cfg(feature = "defmt")]
use defmt::{error, info};

use serde::{Deserialize, Serialize};
mod messages; // to be implemented in later commit

const REG01: &[u8] = &[0x1, 0x0, 0x1, 0x0, 0x0, 0x0, 0x0, 0x0];
// const REG02: &[u8] = &[0x2, 0x0, 0x1, 0x0, 0x0, 0x0, 0x0, 0x0];
const REG05: &[u8] = &[0x5, 0x0, 0x1, 0x0, 0x0, 0x0, 0x0, 0x0];

#[cfg(feature = "serde_support")]
#[derive(Deserialize, Serialize, Debug, Default, Clone, Copy)]
pub enum SolaxStatus {
    #[default]
    NoInverter,
    Handshake,
    InverterReady,
}
#[derive(Debug)]
pub enum SolaxError {
    InvalidData,
    BadId(Option<u32>),
    InvalidFrameEncode(u32),
    TimeStamp([u8; 6]),
    InvalidTimeData,
}
impl core::error::Error for SolaxError {}
impl core::fmt::Display for SolaxError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match *self {
            SolaxError::InvalidData => write!(f, "Invalid data"),
            SolaxError::BadId(id) => write!(f, "Invalid ID: {id:02x?}"),
            SolaxError::InvalidFrameEncode(id) => write!(f, "Invalid Frame encode ID: {id:02x?}"),
            SolaxError::TimeStamp(time) => write!(f, "Recieved timestamp: {time:02?}"),
            SolaxError::InvalidTimeData => write!(f, "Invalid timedata decode"),
        }
    }
}

#[cfg(feature = "serde_support")]
#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct SolaxBms {
    // no conversions in this struct
    pub status: SolaxStatus,
    pub slave_voltage_max: f32, // 1000 = 100.0v
    pub slave_voltage_min: f32, // 800 = 80.0v
    pub charge_max: f32,        // 201 = 20A
    pub discharge_max: f32,     // 350 = 35A
    pub voltage: f32,           // 1130 = 113.0V
    pub current: f32,           // -2 = -0.2A
    pub capacity: u16,          // %
    pub kwh: f32,               // 419 = 41.9 Kwh (* 0.1)
    pub cell_temp_min: f32,     // 18 = 1.8ºC signed
    pub cell_temp_max: f32,     // 21 = 2.1ºC
    pub cell_voltage_min: u16,  // 40 = 4.0V
    pub cell_voltage_max: u16,  // 41 = 4.1V
    pub pack_voltage_max: f32,  // 4100 = 410.0V
    pub wh_total: u32,          // watt hours total in wh
    pub contactor: bool,
    pub int_temp: f32, // 20 = 20ºC
    pub v_max: f32,    // 4501 = 45.01º
    pub v_min: f32,    // 1501 = 15.01º
    pub id: u8,
    pub byte1: u8,
    pub byte2: u8,
    pub counter: u8,
    pub valid: bool,
    #[serde(skip)]
    pub announce: Option<Instant>,
    #[serde(skip)]
    pub last_success: Option<Instant>,
    #[serde(skip)]
    pub last_rx: Option<Instant>,
    #[serde(skip)]
    pub timestamp: Option<Instant>,
    pub time: [u8; 6], // Broadcast date: 20{}/{}/{} {:02}:{:02}:{:02} or [YY,MM,DD,hh,mm,ss]
    #[serde(skip)]
    timeout: Duration,
}

#[cfg(not(feature = "serde_support"))]
#[derive(Debug, Default, Clone)]
pub struct SolaxBms {
    // no conversions out of this struct
    pub status: SolaxStatus,
    pub slave_voltage_max: u16, // 1000 = 100.0v
    pub slave_voltage_min: u16, // 800 = 80.0v
    pub charge_max: u16,        // 201 = 20A
    pub discharge_max: u16,     // 350 = 35A
    pub voltage: u16,           // 1130 = 113.0V
    pub current: i16,           // -2 = -0.2A
    pub capacity: u16,          // %
    pub kwh: u16,               // 419 = 41.9 Kwh (* 0.1)
    pub cell_temp_min: i16,     // 18 = 1.8ºC signed
    pub cell_temp_max: i16,     // 21 = 2.1ºC
    pub cell_voltage_min: u16,  // 40 = 4.0V
    pub cell_voltage_max: u16,  // 41 = 4.1V
    pub pack_voltage_max: u16,  // 4100 = 410.0V
    pub wh_total: u32,          // watt hours total in wh
    pub contactor: bool,
    pub int_temp: i16, // 20 = 20ºC
    pub v_max: u16,    // 4501 = 45.01º
    pub v_min: u16,    // 1501 = 15.01º
    pub id: u8,
    pub byte1: u8,
    pub byte2: u8,
    pub counter: u8,
    pub valid: bool,
    pub announce: Option<Instant>,
    pub last_success: Option<Instant>,
    pub last_rx: Option<Instant>,
    pub timestamp: Option<Instant>,
    pub time: [u8; 6], // Broadcast date: 20{}/{}/{} {:02}:{:02}:{:02} or [YY,MM,DD,hh,mm,ss]
    timeout: Duration,
}

// #[cfg(feature = "std")]
impl SolaxBms {
    pub fn parser<T: embedded_hal::can::Frame + core::clone::Clone>(
        &mut self,
        can_frame: T,
        timeout: Duration,
    ) -> Result<heapless::Vec<T, 20>, SolaxError> {
        self.timeout = timeout;
        if can_frame.id() != Id::Extended(ExtendedId::new(0x1871).unwrap()) {
            let id_decode = |id| -> Option<u32> {
                if let Id::Extended(id_enum) = id {
                    Some(id_enum.as_raw())
                } else {
                    None
                }
            };
            return Err(SolaxError::BadId(id_decode(can_frame.id())));
        };

        if matches!(can_frame.data(), [0x3, 0x6, _, _, _, _, _, _]) {
            self.time = can_frame.data()[2..8]
                .try_into()
                .map_err(|_e| SolaxError::InvalidTimeData)?;
            return Err(SolaxError::TimeStamp(self.time));
        }

        if matches!(can_frame.data(), REG01) {
            if let Some(time) = self.last_rx {
                if time.elapsed().as_secs() >= 3 {
                    // reset annouce timer and force reannoucement of master
                    self.announce = None;
                }
            };
            if !self.is_valid() {
                return Err(SolaxError::InvalidData);
            }

            if self.counter > 3 {
                // Handshake complete, roll the byte ids
                self.status = SolaxStatus::InverterReady;
                self.normal()?;
            } else {
                self.status = SolaxStatus::Handshake;
                self.handshake()?;
            }
            self.status = if let Some(time) = self.last_success {
                if let 0..=2 = time.elapsed().as_secs() {
                    SolaxStatus::InverterReady
                } else {
                    if let Some(time) = self.announce {
                        if time.elapsed().as_secs() >= 3 {
                            self.announce = None
                        }
                    };
                    SolaxStatus::NoInverter
                }
            } else {
                SolaxStatus::NoInverter
            };
        }

        let mut frames: hVec<T, 20> = hVec::new();
        let results = || -> Result<hVec<T, 20>, SolaxError> {
            if matches!(can_frame.data(), REG01) {
                if self.announce.is_none() {
                    self.announce = Some(Instant::now());
                    frames
                        .push(
                            T::new(Id::Extended(ExtendedId::new(0x100A001).unwrap()), &[0u8; 0])
                                .unwrap(),
                        )
                        .map_err(|_| SolaxError::InvalidData)?;
                    Ok(frames)
                    // ))
                } else {
                    frames.extend(self.reg01()?);
                    Ok(frames)
                }
            } else if matches!(can_frame.data(), REG05) {
                frames.extend(self.reg05()?);
                Ok(frames)
            } else {
                Err(SolaxError::InvalidData)
            }
        };

        results()
    }

    fn reg01<T: embedded_hal::can::Frame + core::clone::Clone>(
        &mut self,
    ) -> Result<[T; 7], SolaxError> {
        let canid = |id| {
            if let Some(ext_id) = ExtendedId::new(id) {
                Ok(Id::Extended(ext_id))
            } else {
                Err(SolaxError::InvalidFrameEncode(id))
            }
        };
        let output: [T; 7] = [
            // 0x1877 ====================
            T::new(
                Id::Extended(ExtendedId::new(0x1877).unwrap()),
                &[0x0, 0x0, 0x0, 0x0, self.id, 0x0, self.byte1, self.byte2],
            )
            .unwrap(),
            // 0x1872 ====================
            T::new(
                canid(BmsLimits::MESSAGE_ID)?,
                BmsLimits::new(
                    self.slave_voltage_max,
                    self.slave_voltage_min,
                    self.charge_max,
                    self.discharge_max,
                )
                .map_err(|_e| SolaxError::InvalidFrameEncode(BmsLimits::MESSAGE_ID))?
                .raw(),
            )
            .unwrap(),
            // 0x1873 ====================
            T::new(
                canid(BmsPackData::MESSAGE_ID)?,
                BmsPackData::new(self.voltage, self.current, self.capacity, self.kwh)
                    .map_err(|_e| SolaxError::InvalidFrameEncode(BmsPackData::MESSAGE_ID))?
                    .raw(),
            )
            .unwrap(),
            // 0x1874 ====================
            T::new(
                canid(BmsCellData::MESSAGE_ID)?,
                BmsCellData::new(
                    self.cell_voltage_min.into(),
                    self.cell_voltage_max.into(),
                    self.cell_temp_min,
                    self.cell_temp_max,
                )
                .map_err(|_e| SolaxError::InvalidFrameEncode(BmsCellData::MESSAGE_ID))?
                .raw(),
            )
            .unwrap(),
            // 0x1875 ====================
            T::new(
                canid(BmsStatus::MESSAGE_ID)?,
                BmsStatus::new(true, self.contactor, self.int_temp)
                    .map_err(|_e| SolaxError::InvalidFrameEncode(BmsStatus::MESSAGE_ID))?
                    .raw(),
            )
            .unwrap(),
            // 0x1876 ====================
            T::new(
                canid(BmsPackTemps::MESSAGE_ID)?,
                BmsPackTemps::new(true, self.cell_voltage_max, self.cell_voltage_min)
                    .map_err(|_e| SolaxError::InvalidFrameEncode(BmsPackTemps::MESSAGE_ID))?
                    .raw(),
            )
            .unwrap(),
            // 0x1878 ====================
            T::new(
                canid(BmsPackStats::MESSAGE_ID)?,
                BmsPackStats::new(self.pack_voltage_max, self.wh_total)
                    .map_err(|_e| SolaxError::InvalidFrameEncode(BmsPackStats::MESSAGE_ID))?
                    .raw(),
            )
            .unwrap(),
        ];
        self.last_success = Some(Instant::now());
        Ok(output)
    }

    fn handshake(&mut self) -> Result<(), SolaxError> {
        self.id = 0x00;
        self.byte1 = 0x0d;
        self.byte2 = 0x01;

        info!("Handshake in progress");
        if self.counter == 1 {
            (self.byte1, self.byte2) = (0xf7, 0x16);
        } else if self.counter == 2 {
            // (self.contactor, self.id, self.byte1, self.byte2) = (true, 0x53, 0x1d, 0x20);
            (self.contactor, self.id, self.byte1, self.byte2) = (true, 0x53, 0x1d, 0x20);
        } else if self.counter == 3 {
            (self.id, self.byte1, self.byte2) = (0x53, 0x0d, 0x01);
        }
        // else if self.counter == 4 {
        // (self.byte1, self.byte2) = (0x1d, 0x10);

        self.counter += 1;
        Ok(())
    }
    fn normal(&mut self) -> Result<(), SolaxError> {
        // Rolling alternate bytes - matches known protocol
        match (self.byte1, self.byte2) {
            (0x1d, 0x10) => {
                (self.byte1, self.byte2) = (0x1d, 0x20);
            }
            (0x1d, 0x20) => {
                (self.byte1, self.byte2) = (0x0d, 0x1);
            }
            (0x0d, 0x1) => {
                (self.byte1, self.byte2) = (0x1d, 0x10);
            }
            _ => {}
        }
        Ok(())
    }
    pub fn set_valid(&mut self) {
        self.valid = true
    }
    pub fn is_valid(&self) -> bool {
        self.valid && self.is_fresh()
    }

    pub fn is_fresh(&self) -> bool {
        match self.timestamp {
            Some(time) => {
                if time.elapsed() < self.timeout {
                    info!("Data is {:?} old", time.elapsed(),);
                    true
                } else {
                    error!(
                        "Data is too old {:?}, timeout is {:?}",
                        time.elapsed(),
                        self.timeout
                    );
                    false
                }
            }
            None => false,
        }
    }
    fn reg05<T: embedded_hal::can::Frame + core::clone::Clone>(
        &self,
    ) -> Result<[T; 18], SolaxError> {
        // Future v2 protocol goes here.
        // Cell volts, temps, etc

        let zero = |id| {
            T::new(
                Id::Extended(ExtendedId::new(id).unwrap()),
                &[0x1, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0],
            )
            .unwrap()
        };

        let data = |id, bytes| T::new(Id::Extended(ExtendedId::new(id).unwrap()), bytes).unwrap();

        Ok([
            data(0x1881, &[0x0, 0x54, 0x83, 0x66, 0x77, 0x83, 0x70, 0x65]),
            data(0x1881, &[0x0, 0x50, 0x51, 0x65, 0x66, 0x48, 0x53, 0x50]),
            zero(0x1881),
            zero(0x1882),
            zero(0x1881),
            zero(0x1882),
            zero(0x1881),
            zero(0x1882),
            zero(0x1881),
            zero(0x1882),
            zero(0x1881),
            zero(0x1882),
            zero(0x1881),
            zero(0x1882),
            zero(0x1881),
            zero(0x1882),
            zero(0x1881),
            zero(0x1882),
        ])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
