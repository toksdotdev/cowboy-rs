use crate::cmd;
use crate::types::FieldWeakening;
use crate::types::HallInterpolation;
use crate::types::Speed;
use crate::types::TorqueGain;
use crate::types::TorqueMode;

/// Available Cowboy commands.
#[derive(Debug, Clone, Copy)]
pub enum Cmd {
    /// Set the maximum assisted speed in km/h.
    SetMaxAssistedSpeed(Speed),

    /// Read the maximum assisted speed in km/h.
    ReadMaxAssistedSpeed,

    /// Configure if the bike should lock automatically.
    SetAutoLock(bool),

    /// Read the auto lock configuration.
    ReadAutoLock,

    /// Set the field weakening for the motor.
    ///
    /// This increase the speed of a motor above its rated speed by
    /// reducing the strength of the magnetic field in the motor,
    /// which allows it to spin faster without producing excessive torque.
    ///
    /// This is required if you want to go faster than 29km/h.
    SetFieldWeakening(FieldWeakening),

    /// Read the field weakening configuration of the motor.
    ReadFieldWeakening,

    /// Read the hall interpolation of the motor.
    ReadHallInterpolation,

    /// Set the hall interpolation.
    ///
    /// This adjust how **quickly** the motor is giving boost when you just
    /// start pedalling.
    SetHallInterpolation(HallInterpolation),

    /// Read the torque gain of the motor.
    ReadTorqueGain,

    /// Set the torque gain of the motor in Nm.
    ///
    /// Modify the rotational force or assistance provided by the motor.
    /// This allows the rider tackle challenging terrains or situations with less effort.
    SetTorqueGain(TorqueGain),

    /// Read the content of the given register.
    ReadRegister(u16),

    /// Read the motor's torque mode.
    ReadMotorTorqueMode,

    /// Set the motor's torque mode.
    ///
    /// This configures how the motor will behave when you pedal.
    SetMotorTorqueMode(TorqueMode),

    /// Write all the modified settings to flash memory of the bike.
    ///
    /// This will make the settings persistent if the is locked, or the battery
    /// is removed.
    WriteFlash,

    /// Close the flash.
    CloseFlash,

    /// Lock the bike.
    ///
    /// `true` for lock, `false` for unlock.
    Lock(bool),

    /// Turn the light on or off.
    ///
    /// `true` for on, `false` for off.
    LightOn(bool),
}

/// The write type for a given command.
#[derive(Debug, Clone, Copy)]
pub enum CmdWriteType {
    /// Write without expecting a response.
    WriteOnly,

    /// Write and expect a response.
    WriteWithResponse,
}

impl Cmd {
    /// Get the write type for a given command.
    pub fn get_write_type(&self) -> CmdWriteType {
        match self {
            Cmd::SetMaxAssistedSpeed(_) => CmdWriteType::WriteWithResponse,
            Cmd::ReadMaxAssistedSpeed => CmdWriteType::WriteOnly,
            Cmd::SetAutoLock(_) => CmdWriteType::WriteWithResponse,
            Cmd::ReadAutoLock => CmdWriteType::WriteOnly,
            Cmd::SetFieldWeakening(_) => CmdWriteType::WriteWithResponse,
            Cmd::ReadFieldWeakening => CmdWriteType::WriteOnly,
            Cmd::ReadHallInterpolation => CmdWriteType::WriteOnly,
            Cmd::SetHallInterpolation(_) => CmdWriteType::WriteWithResponse,
            Cmd::ReadTorqueGain => CmdWriteType::WriteOnly,
            Cmd::SetTorqueGain(_) => CmdWriteType::WriteWithResponse,
            Cmd::ReadRegister(_) => CmdWriteType::WriteOnly,
            Cmd::ReadMotorTorqueMode => CmdWriteType::WriteOnly,
            Cmd::SetMotorTorqueMode { .. } => CmdWriteType::WriteWithResponse,
            Cmd::WriteFlash => CmdWriteType::WriteOnly,
            Cmd::CloseFlash => CmdWriteType::WriteOnly,
            Cmd::Lock(_) => CmdWriteType::WriteOnly,
            Cmd::LightOn(_) => CmdWriteType::WriteOnly,
        }
    }
}

impl From<Cmd> for [u8; 11] {
    fn from(cmd: Cmd) -> Self {
        match cmd {
            Cmd::SetMaxAssistedSpeed(Speed { value, .. }) => {
                cmd!(
                    [0xA, 0x10, 0x0, 0x4, 0x0, 0x1, 0x2, 0x0, 0x1E],
                    value as u16
                )
            }
            Cmd::ReadMaxAssistedSpeed => cmd!([0xA, 0x3, 0x0, 0x4, 0x0, 0x1, 0x0, 0x0, 0x0]),
            Cmd::SetAutoLock(v) => {
                cmd!([0xA, 0x10, 0x0, 0x0, 0x0, 0x1, 0x2, 0x0, 0x0], v as u16)
            }
            Cmd::ReadAutoLock => cmd!([0xA, 0x3, 0x0, 0x0, 0x0, 0x1, 0x0, 0x0, 0x0]),
            Cmd::SetFieldWeakening(v) => {
                cmd!(
                    [0x1, 0x10, 0x0, 0x81, 0x0, 0x1, 0x2, 0x0, 0x0],
                    v.weakening().into()
                )
            }
            Cmd::ReadFieldWeakening => cmd!([0x1, 0x3, 0x0, 0x81, 0x0, 0x1, 0x0, 0x0, 0x0]),
            Cmd::ReadHallInterpolation => cmd!([0x1, 0x3, 0x0, 0x80, 0x0, 0x1, 0x0, 0x0, 0x0]),
            Cmd::SetHallInterpolation(v) => {
                cmd!(
                    [0x1, 0x10, 0x0, 0x80, 0x0, 0x1, 0x2, 0x0, 0x0],
                    v.interpolation().into()
                )
            }
            Cmd::ReadTorqueGain => cmd!([0x1, 0x3, 0x0, 0xB3, 0x0, 0x1, 0x0, 0x0, 0x0]),
            Cmd::SetTorqueGain(v) => {
                cmd!(
                    [0x1, 0x10, 0x0, 0xB3, 0x0, 0x1, 0x2, 0x0, 0x0],
                    v.gain.into()
                )
            }
            Cmd::ReadRegister(v) => cmd!([0x1, 0x3, 0x0, 0x0, 0x0, 0x1, 0x0, 0x0, 0x0], v),
            Cmd::ReadMotorTorqueMode => cmd!([0x1, 0x3, 0x0, 0xB, 0x0, 0x1, 0x0, 0x0, 0x0]),
            Cmd::SetMotorTorqueMode(TorqueMode { speed_limit }) => {
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
            Cmd::WriteFlash => cmd!([0x1, 0x10, 0x1, 0xFF, 0x0, 0x1, 0x2, 0x7F, 0xFF]),
            Cmd::CloseFlash => cmd!([0x1, 0x10, 0x1, 0xFF, 0x0, 0x1, 0x2, 0x0, 0x0]),
            Cmd::Lock(v) => cmd!([v as u8, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0]),
            Cmd::LightOn(v) => cmd!([0xA, 0x10, 0x0, 0x1, 0x0, 0x1, 0x2, 0x0, v as u8]),
        }
    }
}

#[cfg(test)]
mod test {
    use super::Cmd::*;
    use crate::command::utils::checksum;

    #[test]
    fn test_checksum_is_valid() {
        let cases = vec![
            (SetMaxAssistedSpeed, [84, 236]),
            (ReadMaxAssistedSpeed, [52, 61]),
            (SetAutoLock, [213, 96]),
            (ReadAutoLock, [53, 185]),
            (SetFieldWeakening, [184, 65]),
            (ReadFieldWeakening, [88, 152]),
            (ReadHallInterpolation, [89, 73]),
            (SetHallInterpolation, [185, 144]),
            (ReadTorqueGain, [92, 138]),
            (SetTorqueGain, [188, 83]),
            (ReadRegister, [70, 137]),
            (ReadMotorTorqueMode, [71, 242]),
            (SetMotorTorqueMode, [102, 235]),
            (SetMotorTorqueWithLimit, [38, 234]),
            (WriteFlash, [194, 239]),
            (CloseFlash, [162, 159]),
            (Lock, [10, 240]),
            (Unlock, [7, 96]),
            (LightOn, [21, 113]),
            (LightOff, [212, 177]),
        ];

        for (cmd, expected) in cases {
            assert_eq!(checksum(&cmd.into()), expected);
        }
    }
}
