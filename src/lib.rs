use anyhow::{anyhow, Context, Result};
use embedded_hal::can::{ExtendedId, Id};
use log::info;
use std::time::Instant;

const REG01: &[u8] = &[0x1, 0x0, 0x1, 0x0, 0x0, 0x0, 0x0, 0x0];
const REG02: &[u8] = &[0x2, 0x0, 0x1, 0x0, 0x0, 0x0, 0x0, 0x0];
const REG05: &[u8] = &[0x5, 0x0, 0x1, 0x0, 0x0, 0x0, 0x0, 0x0];

/*
[04, 10, b8, 0b, c8, 00, 5e, 01] [4100, 3000, 200, 350] = Pack limits - Max 410V Min 300V Charg&e 20A Discharge 35A
[14, 0f, 00, 00, 5b, 00, 12, 02] [3860, 0, 91, 530] = Pack Now - 386V  0A  SoC:91%  5.2999997kWh
[e1, 00, d1, 00, 29, 00, 28, 00] [225, 209, 41, 40] = Pack limts - Max 22.5º Min 20.9º Max 4.1V Min 4V
[e1, 00, 01, 00, 01, 00, 00, 00] [225, 1, 1, 0] = BMS status - Int temp 22.5ºC Unknown true Contactor true
[01, 00, ba, 0f, 00, 00, ae, 0f] [1, 4026, 0, 4014] = Pack status - 1? 4026mV 0? 4014mV

Need to transfer fixed values off to Slave BMS and recieve them via json UART comms

*/
#[derive(Debug, Default, Clone, Copy)]
pub enum SolaxStatus {
    #[default]
    NoInverter,
    Handshake,
    InverterReady,
}

#[derive(Debug, Default, Clone, Copy)]
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
    pub time: [u8; 6], // Broadcast date: 20{}/{}/{} {:02}:{:02}:{:02} or [YY,MM,DD,hh,mm,ss]
}

impl SolaxBms {
    // Returns Some(vec of can frames) or Ok(None) for no tx frames needed
    pub fn set_valid(&mut self) {
        self.valid = true
    }
    pub fn is_valid(&mut self) -> bool {
        self.valid
    }
    pub fn parser<T: embedded_hal::can::Frame + std::clone::Clone + std::marker::Copy>(
        &mut self,
        can_frame: T,
    ) -> Result<Option<Vec<T>>> {
        if can_frame.id() != Id::Extended(ExtendedId::new(0x1871).unwrap()) {
            return Err(anyhow!(
                "{:02x?} is not a valid Solax can Id",
                can_frame.id()
            ));
        };

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

        if let Some(time) = self.last_rx {
            if time.elapsed().as_secs() >= 3 {
                self.announce = None;
            }
        };

        Ok(match can_frame.data() {
            REG01 | REG02 => Some(self.reg01()?),
            REG05 => Some(self.reg05()?),
            [0x3, 0x6, _, _, _, _, _, _] => {
                self.reg03(can_frame.data());
                None
            }
            _ => None,
        })
    }

    fn reg01<T: embedded_hal::can::Frame>(&mut self) -> Result<Vec<T>> {
        if self.announce.is_none() {
            self.announce = Some(Instant::now());
            info!("Gateway announce sent");
            return Ok(vec![self.announce()?]);
        };
        match self.is_valid() {
            true => {
                if self.counter > 3 {
                    self.status = SolaxStatus::InverterReady;
                    self.tx_data_frames()
                } else {
                    self.status = SolaxStatus::Handshake;
                    self.handshake()
                }
            }
            false => Err(anyhow!("valid flag is false")),
        }
    }
    fn reg03(&mut self, data: &[u8]) {
        self.time = data[2..=7].try_into().unwrap_or_default();
        println!(
            "Broadcast date: 20{}/{}/{} {:02}:{:02}:{:02}",
            data[2], data[3], data[4], data[5], data[6], data[7]
        );
    }
    fn reg05<T: embedded_hal::can::Frame + std::clone::Clone + std::marker::Copy>(
        &self,
    ) -> Result<Vec<T>> {
        reg05_data()
    }
    // pub fn new(&mut self) -> SolaxBms {
    //     self.pack_voltage_max = bmsdata.pack_voltage_max;
    //     self.slave_voltage_max = bmsdata.slave_voltage_max; // + (bmsdata.pack_volts as f32 * 0.1) as u16; // +3v based on know good fox
    //     self.slave_voltage_min = bmsdata.slave_voltage_min;
    //     self.charge_max = bmsdata.charge_max;
    //     self.discharge_max = bmsdata.discharge_max;
    //     self.voltage = bmsdata.pack_volts;
    //     self.current = bmsdata.current as i16; // 100mA = 1
    //     self.capacity = bmsdata.soc;
    //     self.kwh = bmsdata.kwh_remaining * 10;
    //     self.cell_temp_min = min as i16 * 10;
    //     self.cell_temp_max = max as i16 * 10;
    //     // self.int_temp = (bmsdata.temps.iter().sum::<i8>() / bmsdata.temps.len() as i8) as i16;
    //     self.int_temp = bmsdata.temps[6] as i16 * 10;
    //     self.cell_voltage_min = (bmsdata.min_volts as f32 * 0.01) as u16;
    //     self.cell_voltage_max = 1 + (bmsdata.max_volts as f32 * 0.01) as u16;
    //     self.wh_total = 10000;
    //     self.contactor = false; //bmsdata.read_contactor(),
    //     self.v_max = bmsdata.max_volts;
    //     self.v_min = bmsdata.min_volts;
    // }
    fn announce<T: embedded_hal::can::Frame>(&self) -> Result<T> {
        T::new(Id::Extended(ExtendedId::new(0x100A001).unwrap()), &[0u8; 0]).context("1873")
    }
    fn handshake<T: embedded_hal::can::Frame>(&mut self) -> Result<Vec<T>> {
        self.id = 0x00;
        self.byte1 = 0x0d;
        self.byte2 = 0x01;

        info!("SENDING TO INV -> {:#?}", self);
        if self.counter == 1 {
            (self.byte1, self.byte2) = (0xf7, 0x16);
        } else if self.counter == 2 {
            // (self.contactor, self.id, self.byte1, self.byte2) = (true, 0x53, 0x1d, 0x20);
            (self.contactor, self.id, self.byte1, self.byte2) = (true, 0x53, 0x1d, 0x20);
        } else if self.counter == 3 {
            (self.byte1, self.byte2) = (0x0d, 0x01);
        } /*else if self.counter == 4 {
                                                    (self.byte1, self.byte2) = (0x1d, 0x10);
          }*/

        // taking into account incrementer
        let mut x1872_payload: [u8; 8] = [0; 8];
        [x1872_payload[0], x1872_payload[1]] = self.slave_voltage_max.to_le_bytes();
        [x1872_payload[2], x1872_payload[3]] = self.slave_voltage_min.to_le_bytes();

        let mut x1878_payload: [u8; 8] = [0; 8];
        [x1878_payload[0], x1878_payload[1]] = self.pack_voltage_max.to_le_bytes();

        self.counter += 1;
        self.last_success = Some(Instant::now());
        Ok(vec![
            T::new(
                Id::Extended(ExtendedId::new(0x1877).unwrap()),
                &self.x1877(),
            )
            .context("1877")?,
            T::new(
                Id::Extended(ExtendedId::new(0x1872).unwrap()),
                &x1872_payload,
            )
            .context("1872")?,
            T::new(Id::Extended(ExtendedId::new(0x1873).unwrap()), &[0u8; 8]).context("1873")?,
            T::new(Id::Extended(ExtendedId::new(0x1874).unwrap()), &[0u8; 8]).context("1874")?,
            T::new(
                Id::Extended(ExtendedId::new(0x1875).unwrap()),
                &[0, 0, 2, 0, 0, 0, 0, 0],
            )
            .context("1875")?,
            T::new(Id::Extended(ExtendedId::new(0x1876).unwrap()), &[0u8; 8]).context("1876")?,
            T::new(
                Id::Extended(ExtendedId::new(0x1878).unwrap()),
                &x1878_payload,
            )
            .context("1878")?,
            // TXFrame::new(0x1879, [0u8; 8]),
        ])
    }
    fn tx_data_frames<T: embedded_hal::can::Frame>(&mut self) -> Result<Vec<T>> {
        if !(1..=100u16).contains(&self.capacity) {
            return Err(anyhow!("Data - fault condition - soc = {}", self.capacity));
        }
        if !self.is_valid() {
            return Err(anyhow!("Data valid flag is false"));
        }

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

        self.last_success = Some(Instant::now());

        Ok(vec![
            T::new(
                Id::Extended(ExtendedId::new(0x1877).unwrap()),
                &self.x1877(),
            )
            .context("x1877")?,
            T::new(
                Id::Extended(ExtendedId::new(0x1872).unwrap()),
                &self.x1872(),
            )
            .context("x1872")?,
            T::new(
                Id::Extended(ExtendedId::new(0x1873).unwrap()),
                &self.x1873(),
            )
            .context("x1873")?,
            T::new(
                Id::Extended(ExtendedId::new(0x1874).unwrap()),
                &self.x1874(),
            )
            .context("x1874")?,
            T::new(
                Id::Extended(ExtendedId::new(0x1875).unwrap()),
                &self.x1875(),
            )
            .context("x1875")?,
            T::new(
                Id::Extended(ExtendedId::new(0x1876).unwrap()),
                &self.x1876(),
            )
            .context("x1876")?,
            T::new(
                Id::Extended(ExtendedId::new(0x1878).unwrap()),
                &self.x1878(),
            )
            .context("1878")?,
            // self.x1879(),
        ])
    }

    fn x1872(self) -> [u8; 8] {
        // - BMS_Limits
        let mut tx_payload: [u8; 8] = [0; 8];
        [tx_payload[0], tx_payload[1]] = self.slave_voltage_max.to_le_bytes();
        [tx_payload[2], tx_payload[3]] = self.slave_voltage_min.to_le_bytes();
        [tx_payload[4], tx_payload[5]] = self.charge_max.to_le_bytes();
        [tx_payload[6], tx_payload[7]] = self.discharge_max.to_le_bytes();

        // verify with
        self.x1872_decode(&tx_payload);
        tx_payload
    }

    fn x1872_decode(self, bytes: &[u8]) {
        let ints = as_u16le(bytes);
        println!(
            "{:02x?} {:?} = Pack limits - Max {}V Min {}V Charge {}A Discharge {}A",
            // self.ID,
            bytes,
            ints,
            ints[0] as f32 * 0.1,
            ints[1] as f32 * 0.1,
            ints[2] as f32 * 0.1,
            ints[3] as f32 * 0.1,
        );
    }

    fn x1873(self) -> [u8; 8] {
        //BMS_PackData
        // let x1873 = vec![0x2A, 0x09, 0x00, 0x00, 0x0E, 0x00, 0xDC, 0x00];
        let mut tx_payload: [u8; 8] = [0; 8];
        [tx_payload[0], tx_payload[1]] = self.voltage.to_le_bytes();
        [tx_payload[2], tx_payload[3]] = self.current.to_le_bytes();
        [tx_payload[4], tx_payload[5]] = self.capacity.to_le_bytes();
        [tx_payload[6], tx_payload[7]] = self.kwh.to_le_bytes();
        self.x1873_decode(&tx_payload);
        tx_payload
    }

    fn x1873_decode(self, bytes: &[u8]) {
        let ints = as_u16le(bytes);
        println!(
            "{:02x?} {:?} = Pack Now - {}V  {}A  SoC:{}%  {}kWh",
            // self.ID,
            bytes,
            ints,
            ints[0] as f32 * 0.1,
            (ints[1] as i16) as f32 / 10.0,
            ints[2] as u16,
            ints[3] as f32 * 0.01,
        );
    }

    fn x1874(self) -> [u8; 8] //Cell data
    {
        let mut tx_payload: [u8; 8] = [0; 8];

        [tx_payload[0], tx_payload[1]] = self.cell_temp_max.to_le_bytes();
        [tx_payload[2], tx_payload[3]] = self.cell_temp_min.to_le_bytes();
        [tx_payload[4], tx_payload[5]] = self.cell_voltage_max.to_le_bytes();
        [tx_payload[6], tx_payload[7]] = self.cell_voltage_min.to_le_bytes();
        self.x1874_decode(&tx_payload);
        tx_payload
    }

    fn x1874_decode(self, bytes: &[u8]) //Cell data
    {
        let ints = as_u16le(bytes);
        println!(
            "{:02x?} {:?} = Pack limts - Max {}º Min {}º Max {}V Min {}V ",
            // self.ID,
            bytes,
            ints,
            ints[0] as f32 * 0.1,
            ints[1] as f32 * 0.1,
            ints[2] as f32 * 0.1,
            ints[3] as f32 * 0.1,
        )
    }

    fn x1875(self) -> [u8; 8] {
        //BMS_PackData

        let mut tx_payload: [u8; 8] = [0; 8];
        [tx_payload[0], tx_payload[1]] = self.int_temp.to_le_bytes();
        // [tx_payload[2], tx_payload[3]] = [0x4, 0x0];

        tx_payload[2] = 0x1; // quanity of batteries https://secondlifestorage.com/index.php?threads/three-phase-hv-hybrid-inverter-solax-x3-hybrid-8-0-solax-triple-power-t58-hv-battery.10747/post-84312
        tx_payload[4] = 1; //self.contactor as u8;

        self.x1875_decode(&tx_payload);
        tx_payload
    }

    fn x1875_decode(self, bytes: &[u8]) // BMS status
    {
        let ints = as_u16le(bytes);
        println!(
            "{:02x?} {:?} = BMS status - Int temp {}ºC Unknown {} Contactor {}",
            // self.ID,
            bytes,
            ints,
            ints[0] as f32 * 0.1,
            bytes[3] != 1,
            bytes[5] != 1,
        );
    }

    fn x1876(self) -> [u8; 8] // BMS_PackStats
    {
        let mut tx_payload: [u8; 8] = [0; 8];
        tx_payload[0] = 1;
        [tx_payload[2], tx_payload[3]] = self.v_max.to_le_bytes();
        [tx_payload[6], tx_payload[7]] = self.v_min.to_le_bytes();
        self.x1876_decode(&tx_payload);
        tx_payload
    }
    fn x1876_decode(self, bytes: &[u8]) // BMS status
    {
        // let x1875 = vec![0x53, 0x01, 0x04, 0x00, 0x00, 0x00, 0x01, 0x00];
        let ints = as_u16le(bytes);
        println!(
            "{:02x?} {:?} = Pack status - {}? {}mV {}? {}mV",
            // self.ID,
            bytes,
            ints,
            ints[0] as f32 * 1.0,
            ints[1] as f32 * 1.0,
            ints[2] as f32 * 1.0,
            ints[3] as f32 * 1.0,
        );
    }

    fn x1877(self) -> [u8; 8] {
        //Fixed data
        let mut tx_payload: [u8; 8] = [0x0, 0x0, 0x0, 0x0, 0x00, 0x0, 0x0, 0x0];
        tx_payload[4] = self.id;
        tx_payload[6] = self.byte1;
        tx_payload[7] = self.byte2;

        tx_payload
    }

    fn x1878(self) -> [u8; 8] {
        //Fixed data
        let mut tx_payload: [u8; 8] = [0x0, 0x0, 0x0, 0x0, 0x79, 0x43, 0x0, 0x2];
        [tx_payload[0], tx_payload[1]] = self.pack_voltage_max.to_le_bytes();
        [tx_payload[4], tx_payload[5], tx_payload[6], tx_payload[7]] = self.wh_total.to_le_bytes();
        tx_payload
    }

    /*
    6145 x1801 BMS_Response - not in logs
    let x1871 = vec![0x01, 0x00 , 0x01 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00];
    let Packet_Type = u16::from_le_bytes([x1871[0],x1871[1]]);
    let Data_0 = u16::from_le_bytes([x1871[2],x1871[3]]);
    let Data_1 = u16::from_le_bytes([x1871[4],x1871[5]]);
    let Data_2 = u16::from_le_bytes([x1871[6],x1871[7]]);
    println!("Packet_Type {Packet_Type} Data_0 {Data_0} Data_1 {Data_1} Data_2 {Data_2} ");
    */
}

fn reg05_data<T: embedded_hal::can::Frame + std::clone::Clone + std::marker::Copy>(
) -> Result<Vec<T>> {
    //simplify this

    let zeros1 = T::new(
        Id::Extended(ExtendedId::new(0x1881).unwrap()),
        &[0x1, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0],
    )
    .context("reg05")?;
    let zeros2 = T::new(
        Id::Extended(ExtendedId::new(0x1882).unwrap()),
        &[0x1, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0],
    )
    .context("reg05")?;
    Ok([
        T::new(
            Id::Extended(ExtendedId::new(0x1881).unwrap()),
            &[0x0, 0x54, 0x83, 0x66, 0x77, 0x83, 0x70, 0x65],
        )
        .context("reg05")?,
        T::new(
            Id::Extended(ExtendedId::new(0x1882).unwrap()),
            &[0x0, 0x50, 0x51, 0x65, 0x66, 0x48, 0x53, 0x50],
        )
        .context("reg05")?,
        zeros1,
        zeros2,
        zeros1,
        zeros2,
        zeros1,
        zeros2,
        zeros1,
        zeros2,
        zeros1,
        zeros2,
        zeros1,
        zeros2,
        zeros1,
        zeros2,
        zeros1,
        zeros2,
    ]
    .to_vec())
}

fn as_u16le(bytes: &[u8]) -> [u16; 4] {
    [
        u16::from_le_bytes([bytes[0], bytes[1]]),
        u16::from_le_bytes([bytes[2], bytes[3]]),
        u16::from_le_bytes([bytes[4], bytes[5]]),
        u16::from_le_bytes([bytes[6], bytes[7]]),
    ]
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
