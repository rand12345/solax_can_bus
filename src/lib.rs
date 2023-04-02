#![allow(clippy::enum_variant_names)]
#![allow(clippy::uninlined_format_args)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![feature(error_in_core)]
#![no_std]
use crate::messages::*;
use bms_standard::Bms;
#[cfg(feature = "defmt")]
use defmt::{error, info, warn};
use embedded_hal::can::{ExtendedId, Id};
use heapless::Vec as hVec;
#[cfg(not(feature = "defmt"))]
use log::{error, info, warn};

#[cfg(feature = "serde_support")]
use serde::{Deserialize, Serialize};
mod messages;

const REG01: &[u8] = &[0x1, 0x0, 0x1, 0x0, 0x0, 0x0, 0x0, 0x0];
const REG05: &[u8] = &[0x5, 0x0, 0x1, 0x0, 0x0, 0x0, 0x0, 0x0];

#[cfg(feature = "serde_support")]
#[derive(Deserialize, Serialize, Debug, Default)]
pub enum SolaxStatus {
    #[default]
    NoInverter,
    Handshake,
    InverterReady,
}
#[cfg(not(feature = "serde_support"))]
#[derive(Debug, Default)]
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
    UnwantedFrame,
}
impl core::error::Error for SolaxError {}
impl core::fmt::Display for SolaxError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match *self {
            SolaxError::InvalidData => write!(f, "Invalid data"),
            SolaxError::BadId(id) => write!(f, "Invalid ID: {id:02x?}"),
            SolaxError::InvalidFrameEncode(id) => write!(f, "Invalid Frame encode ID: {id:02x?}"),
            SolaxError::TimeStamp(time) => write!(f, "Recieved timestamp: {time:02x?}"),
            SolaxError::InvalidTimeData => write!(f, "Invalid timedata decode"),
            SolaxError::UnwantedFrame => write!(f, "Unwanted can bus frame"),
        }
    }
}

#[cfg(feature = "serde_support")]
#[derive(Deserialize, Serialize, Debug, Default)]
pub struct SolaxBms {
    pub status: SolaxStatus,
    contactor: bool,
    id: u8,
    byte1: u8,
    byte2: u8,
    counter: u8,
    valid: bool,
    announce: bool,
    pub time: [u8; 6],
}

#[cfg(not(feature = "serde_support"))]
#[derive(Debug, Default)]
pub struct SolaxBms {
    pub status: SolaxStatus,
    contactor: bool,
    id: u8,
    byte1: u8,
    byte2: u8,
    counter: u8,
    valid: bool,
    announce: bool,
    pub time: [u8; 6],
}

impl SolaxBms {
    pub fn parser<T: embedded_hal::can::Frame + core::clone::Clone>(
        &mut self,
        can_frame: T,
        bms: &Bms,
        contactor: bool,
    ) -> Result<heapless::Vec<T, 20>, SolaxError> {
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
        self.contactor = contactor;
        if matches!(can_frame.data(), REG01) {
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
        }

        let mut frames: hVec<T, 20> = hVec::new();
        let results = || -> Result<hVec<T, 20>, SolaxError> {
            if matches!(can_frame.data(), REG01) {
                if !self.announce {
                    warn!("Unannounced, sending announcement");

                    frames
                        .push(
                            T::new(Id::Extended(ExtendedId::new(0x100A001).unwrap()), &[0u8; 0])
                                .unwrap(),
                        )
                        .map_err(|_| SolaxError::InvalidData)?;
                    self.announce = true;
                    Ok(frames)
                    // ))
                } else {
                    frames.extend(self.reg01(bms)?);
                    Ok(frames)
                }
            } else if matches!(can_frame.data(), REG05) {
                frames.extend(self.reg05()?);
                Ok(frames)
            } else {
                Err(SolaxError::UnwantedFrame)
            }
        };
        results()
    }

    fn reg01<T: embedded_hal::can::Frame + core::clone::Clone>(
        &mut self,
        bms: &Bms,
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
                    // self.slave_voltage_max,
                    *bms.get_pack_voltage_limits().max(),
                    *bms.get_pack_voltage_limits().min(),
                    bms.charge_max,
                    bms.discharge_max,
                )
                .map_err(|_e| SolaxError::InvalidFrameEncode(BmsLimits::MESSAGE_ID))?
                .raw(),
            )
            .unwrap(),
            // 0x1873 ====================
            T::new(
                canid(BmsPackData::MESSAGE_ID)?,
                BmsPackData::new(
                    bms.pack_volts,
                    bms.current,
                    bms.soc as u16,
                    bms.kwh_remaining,
                )
                .map_err(|_e| SolaxError::InvalidFrameEncode(BmsPackData::MESSAGE_ID))?
                .raw(),
            )
            .unwrap(),
            // 0x1874 ====================
            T::new(
                canid(BmsCellData::MESSAGE_ID)?,
                BmsCellData::new(
                    *bms.cell_range_mv.min() as f32 / 1000.0,
                    *bms.cell_range_mv.max() as f32 / 1000.0,
                    *bms.temps.min(),
                    *bms.temps.max(),
                )
                .map_err(|_e| SolaxError::InvalidFrameEncode(BmsCellData::MESSAGE_ID))?
                .raw(),
            )
            .unwrap(),
            // 0x1875 ====================
            T::new(
                canid(BmsStatus::MESSAGE_ID)?,
                BmsStatus::new(true, self.contactor, bms.temp)
                    .map_err(|_e| SolaxError::InvalidFrameEncode(BmsStatus::MESSAGE_ID))?
                    .raw(),
            )
            .unwrap(),
            // 0x1876 ====================
            T::new(
                canid(BmsPackTemps::MESSAGE_ID)?,
                BmsPackTemps::new(true, *bms.cell_range_mv.max(), *bms.cell_range_mv.min())
                    .map_err(|_e| SolaxError::InvalidFrameEncode(BmsPackTemps::MESSAGE_ID))?
                    .raw(),
            )
            .unwrap(),
            // 0x1878 ====================
            T::new(
                canid(BmsPackStats::MESSAGE_ID)?,
                BmsPackStats::new(bms.pack_volts, 10000)
                    .map_err(|_e| SolaxError::InvalidFrameEncode(BmsPackStats::MESSAGE_ID))?
                    .raw(),
            )
            .unwrap(),
        ];
        // self.last_success = Some(Instant::now());
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
        self.valid
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
