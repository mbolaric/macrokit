use macrokit::{FromReprAsOption, FromReprWithUnknown};

#[derive(Debug, PartialEq, FromReprAsOption)]
#[repr(u8)]
pub enum Command {
    Read = 1,
    Write = 2,
}

#[derive(Debug, PartialEq, FromReprWithUnknown)]
#[repr(u8)]
pub enum Status {
    Active = 0,
    Inactive = 1,
    Unknown, // This variant is required
}

fn main() {
    // --- FromReprAsOption Example ---
    println!("--- FromReprAsOption Example ---");

    let command_ok = Command::from_repr(1);
    println!("Command from 1: {:?}", command_ok);
    assert_eq!(command_ok, Some(Command::Read));

    let command_none = Command::from_repr(99);
    println!("Command from 99: {:?}", command_none);
    assert_eq!(command_none, None);

    // --- FromReprWithUnknown Example ---
    println!("\n--- FromReprWithUnknown Example ---");

    let status_active: Status = 0u8.into();
    println!("Status from 0: {:?}", status_active);
    assert_eq!(status_active, Status::Active);

    let status_unknown: Status = 255u8.into();
    println!("Status from 255: {:?}", status_unknown);
    assert_eq!(status_unknown, Status::Unknown);

    println!("\nExamples ran successfully!");
}
