use core::fmt::Debug;
use core::str::FromStr;

use cowboy_proc_macros::Characteristic;
use cowboy_proc_macros::Command;
use cowboy_proc_macros::Service;
use uuid::uuid;
use uuid::Uuid;

use crate::cmd;
use crate::error::CmdError;
use crate::types::FieldWeakening;
use crate::types::HallInterpolation;
use crate::types::Speed;
use crate::types::TorqueGain;
use crate::types::TorqueMode;
pub const DESCRIPTOR_CHARACTERISTIC_WRITE: Uuid = uuid!("6E400002-B5A3-F393-E0A9-E50E24DCCA9E");

#[non_exhaustive]
#[derive(Service, Debug, Clone, Copy)]
pub enum CowboyService {
    #[service("6E400001-B5A3-F393-E0A9-E50E24DCCA9E")]
    Settings(SettingsCharacteristic),

    #[service("C0B0A000-18EB-499D-B266-2F2910744274")]
    Cowboy(CowboyCharacteristic),
}

#[non_exhaustive]
#[derive(Characteristic, Debug, Clone, Copy)]
pub enum SettingsCharacteristic {
    /// Characteristic for modifying the settings of the Cowboy.
    #[characteristic("6E400002-B5A3-F393-E0A9-E50E24DCCA9E")]
    Write(SettingsWriteCmd),

    /// Characteristic for reading the settings of the Cowboy.
    #[characteristic("C0B0A001-18EB-499D-B266-2F2910744274")]
    Read(SettingsReadCmd),
}

#[non_exhaustive]
#[derive(Characteristic, Debug, Clone, Copy)]
pub enum CowboyCharacteristic {
    /// Characteristic for locking/unlocking the Cowboy.
    #[characteristic("C0B0A001-18EB-499D-B266-2F2910744274")]
    Lock(CowboyLockCmd),

    /// Characteristic for reading the dashboard information of the Cowboy.
    #[characteristic("C0B0A001-18EB-499D-B266-2F2910744274")]
    Dashboard(CowboyDashboardCmd),

    /// Characteristic for reading the fitness information of the Cowboy.
    #[characteristic("C0B0A001-18EB-499D-B266-2F2910744274")]
    DataFitnessCollectorRequest(CowboyDfcCmd),

    /// Characteristic for getting the current trip information of the Cowboy.
    #[characteristic("C0B0A001-18EB-499D-B266-2F2910744274")]
    Trip(CowboyTripCmd),
}

#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
pub enum CowboyLockCmd {
    /// Lock the bike.
    ///
    /// `true` for lock, `false` for unlock.
    SetLock(bool),

    /// Read the lock status of the bike.
    ReadLock,
}

#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
pub enum CowboyDashboardCmd {
    Read,
}

#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
pub enum CowboyDfcCmd {
    /// Read the fitness information of the bike starting from the given offset.
    Read(u32),
}

#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
pub enum CowboyTripCmd {
    /// Read the current trip information of the bike.
    Read,
}

#[non_exhaustive]
#[derive(Command, Debug, Clone, Copy)]
pub enum SettingsWriteCmd {
    /// Turn the light on or off.
    ///
    /// `true` for on, `false` for off.
    #[mode(write)]
    SetLight(bool),

    /// Configure if the bike should lock automatically.
    #[mode(write_with_response)]
    SetAutoLock(bool),

    /// Read the auto lock configuration.
    #[mode(write)]
    ReadAutoLock,

    /// Set the maximum assisted speed in km/h.
    #[mode(write_with_response)]
    SetMaxAssistedSpeed(Speed),

    /// Read the maximum assisted speed in km/h.
    #[mode(write)]
    ReadMaxAssistedSpeed,

    /// Set the field weakening for the motor.
    ///
    /// This increase the speed of a motor above its rated speed by
    /// reducing the strength of the magnetic field in the motor,
    /// which allows it to spin faster without producing excessive torque.
    ///
    /// This is required if you want to go faster than 29km/h.
    #[mode(write_with_response)]
    SetFieldWeakening(FieldWeakening),

    /// Read the field weakening configuration of the motor.
    #[mode(write)]
    ReadFieldWeakening,

    /// Set the hall interpolation.
    ///
    /// This adjust how **quickly** the motor is giving boost when you just
    /// start pedalling.
    #[mode(write_with_response)]
    SetHallInterpolation(HallInterpolation),

    /// Read the hall interpolation of the motor.
    #[mode(write)]
    ReadHallInterpolation,

    /// Set the torque gain of the motor in Nm.
    ///
    /// Modify the rotational force or assistance provided by the motor.
    /// This allows the rider tackle challenging terrains or situations with less effort.
    #[mode(write_with_response)]
    SetTorqueGain(TorqueGain),

    /// Read the torque gain of the motor.
    #[mode(write)]
    ReadTorqueGain,

    /// Read the content of the given register.
    #[mode(write)]
    ReadRegister(u16),

    /// Set the motor's torque mode.
    ///
    /// This configures how the motor will behave when you pedal.
    #[mode(write_with_response)]
    SetMotorTorqueMode(TorqueMode),

    /// Read the motor's torque mode.
    #[mode(write)]
    ReadMotorTorqueMode,

    /// Write all the modified settings to flash memory of the bike.
    ///
    /// This will make the settings persistent if the is locked, or the battery
    /// is removed.
    #[mode(write)]
    WriteFlash,

    /// Close the flash.
    #[mode(write)]
    CloseFlash,
}

#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
pub enum SettingsReadCmd {}

impl TryFrom<CowboyLockCmd> for [u8; 11] {
    type Error = CmdError;

    fn try_from(cmd: CowboyLockCmd) -> Result<Self, Self::Error> {
        use CowboyLockCmd::*;

        Ok(match cmd {
            SetLock(v) => cmd!([v as u8, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0]),
            CowboyLockCmd::ReadLock => return Err(CmdError::StrictlyRead),
        })
    }
}

impl TryFrom<CowboyDashboardCmd> for [u8; 11] {
    type Error = CmdError;

    fn try_from(cmd: CowboyDashboardCmd) -> Result<Self, Self::Error> {
        use CowboyDashboardCmd::*;

        match cmd {
            Read => Err(CmdError::StrictlyRead),
        }
    }
}

impl TryFrom<CowboyDfcCmd> for [u8; 11] {
    type Error = CmdError;

    fn try_from(cmd: CowboyDfcCmd) -> Result<Self, Self::Error> {
        use CowboyDfcCmd::*;

        Ok(match cmd {
            Read(offset) => {
                let mut base = [0x0; 11];
                base.copy_from_slice(&offset.to_le_bytes());
                base
            }
        })
    }
}

impl TryFrom<CowboyTripCmd> for [u8; 11] {
    type Error = CmdError;

    fn try_from(cmd: CowboyTripCmd) -> Result<Self, Self::Error> {
        use CowboyTripCmd::*;

        match cmd {
            Read => Err(CmdError::StrictlyRead),
        }
    }
}

impl TryFrom<SettingsReadCmd> for [u8; 11] {
    type Error = CmdError;

    fn try_from(cmd: SettingsReadCmd) -> Result<Self, Self::Error> {
        match cmd {}
    }
}

impl TryFrom<SettingsWriteCmd> for [u8; 11] {
    type Error = CmdError;

    fn try_from(cmd: SettingsWriteCmd) -> Result<Self, Self::Error> {
        use SettingsWriteCmd::*;
        Ok(match cmd {
            SetLight(v) => {
                cmd!([0xA, 0x10, 0x0, 0x1, 0x0, 0x1, 0x2, 0x0, v as u8])
            }
            SetMaxAssistedSpeed(Speed { value: v, .. }) => {
                cmd!([0xA, 0x10, 0x0, 0x4, 0x0, 0x1, 0x2, 0x0, 0x1E], v as u16)
            }
            ReadMaxAssistedSpeed => {
                cmd!([0xA, 0x3, 0x0, 0x4, 0x0, 0x1, 0x0, 0x0, 0x0])
            }
            SetAutoLock(v) => {
                cmd!([0xA, 0x10, 0x0, 0x0, 0x0, 0x1, 0x2, 0x0, 0x0], v as u16)
            }
            ReadAutoLock => cmd!([0xA, 0x3, 0x0, 0x0, 0x0, 0x1, 0x0, 0x0, 0x0]),
            SetFieldWeakening(v) => {
                cmd!(
                    [0x1, 0x10, 0x0, 0x81, 0x0, 0x1, 0x2, 0x0, 0x0],
                    v.weakening().into()
                )
            }
            ReadFieldWeakening => {
                cmd!([0x1, 0x3, 0x0, 0x81, 0x0, 0x1, 0x0, 0x0, 0x0])
            }
            ReadHallInterpolation => {
                cmd!([0x1, 0x3, 0x0, 0x80, 0x0, 0x1, 0x0, 0x0, 0x0])
            }
            SetHallInterpolation(v) => {
                cmd!(
                    [0x1, 0x10, 0x0, 0x80, 0x0, 0x1, 0x2, 0x0, 0x0],
                    v.interpolation().into()
                )
            }
            ReadTorqueGain => {
                cmd!([0x1, 0x3, 0x0, 0xB3, 0x0, 0x1, 0x0, 0x0, 0x0])
            }
            SetTorqueGain(v) => {
                cmd!(
                    [0x1, 0x10, 0x0, 0xB3, 0x0, 0x1, 0x2, 0x0, 0x0],
                    v.gain.into()
                )
            }
            ReadRegister(v) => {
                cmd!([0x1, 0x3, 0x0, 0x0, 0x0, 0x1, 0x0, 0x0, 0x0], v)
            }
            ReadMotorTorqueMode => {
                cmd!([0x1, 0x3, 0x0, 0xB, 0x0, 0x1, 0x0, 0x0, 0x0])
            }
            SetMotorTorqueMode(TorqueMode { speed_limit }) => {
                cmd!([
                    0x1,
                    0x10,
                    0x0,
                    0xB,
                    0x0,
                    0x1,
                    0x2,
                    0x0,
                    speed_limit as u8 + 1
                ])
            }
            WriteFlash => cmd!([0x1, 0x10, 0x1, 0xFF, 0x0, 0x1, 0x2, 0x7F, 0xFF]),
            CloseFlash => cmd!([0x1, 0x10, 0x1, 0xFF, 0x0, 0x1, 0x2, 0x0, 0x0]),
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::assert_all;
    use crate::che;
    use crate::chk;

    #[test]
    fn test_cmd_checksum_is_valid() {
        use CowboyLockCmd::*;
        use SettingsWriteCmd::*;

        assert_all!([
            (chk!(SetMaxAssistedSpeed(Default::default())), [21, 46]),
            (chk!(ReadMaxAssistedSpeed), [52, 61]),
            (chk!(SetAutoLock(false)), [213, 96]),
            (chk!(SetAutoLock(true)), [20, 160]),
            (chk!(ReadAutoLock), [53, 185]),
            (chk!(SetFieldWeakening(Default::default())), [184, 65]),
            (chk!(ReadFieldWeakening), [88, 152]),
            (chk!(ReadHallInterpolation), [89, 73]),
            (chk!(SetHallInterpolation(Default::default())), [185, 144]),
            (chk!(ReadTorqueGain), [92, 138]),
            (chk!(SetTorqueGain(Default::default())), [188, 83]),
            (chk!(ReadRegister(0)), [70, 137]),
            (chk!(ReadMotorTorqueMode), [71, 242]),
            (chk!(SetMotorTorqueMode(Default::default())), [102, 235]),
            (chk!(WriteFlash), [194, 239]),
            (chk!(CloseFlash), [162, 159]),
            (chk!(SetLight(true)), [21, 113]),
            (chk!(SetLight(false)), [212, 177]),
            (chk!(SetLock(false)), [10, 240]),
            (chk!(SetLock(true)), [7, 96])
        ]);
    }

    #[test]
    fn test_strictly_readonly() {
        use CmdError::StrictlyRead;
        assert_all!([
            (che!(CowboyLockCmd::ReadLock), StrictlyRead),
            (che!(CowboyDashboardCmd::Read), StrictlyRead),
            (che!(CowboyTripCmd::Read), StrictlyRead),
        ])
    }
}
