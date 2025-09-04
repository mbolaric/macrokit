use macrokit::FromReprWithUnknown;

#[derive(Debug, PartialEq, FromReprWithUnknown)]
#[repr(u8)]
pub enum Status {
    Active = 0,
    Inactive = 1,
    // Missing the 'Unknown' variant
}

fn main() {}
