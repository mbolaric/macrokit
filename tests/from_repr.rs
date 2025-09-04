#[test]
fn test_from_repr_as_option() {
    use macrokit::FromReprAsOption;

    #[derive(Debug, PartialEq, FromReprAsOption)]
    #[repr(u8)]
    pub enum Command {
        Read = 1,
        Write = 2,
    }

    assert_eq!(Command::from_repr(1), Some(Command::Read));
    assert_eq!(Command::from_repr(2), Some(Command::Write));
    assert_eq!(Command::from_repr(3), None);
}

#[test]
fn test_from_repr_with_unknown() {
    use macrokit::FromReprWithUnknown;

    #[derive(Debug, PartialEq, FromReprWithUnknown)]
    #[repr(u8)]
    pub enum Status {
        Active = 0,
        Inactive = 1,
        Unknown, // This variant is required
    }

    let status_active: Status = 0u8.into();
    assert_eq!(status_active, Status::Active);

    let status_inactive: Status = 1u8.into();
    assert_eq!(status_inactive, Status::Inactive);

    let status_unknown: Status = 99u8.into();
    assert_eq!(status_unknown, Status::Unknown);
}

#[test]
fn ui() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/*.rs");
}
