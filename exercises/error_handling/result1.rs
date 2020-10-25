// result1.rs
// Make this test pass! Execute `rustlings hint result1` for hints :)

// I AM DONE

#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
        match value {
            i64::MIN..=-1   => { Err(CreationError::Negative) }
            0               => { Err(CreationError::Zero) }
            _               => { Ok(PositiveNonzeroInteger(value as u64)) }
        }

    }
}

#[test]
fn test_creation() {
    assert!(PositiveNonzeroInteger::new(10).is_ok());
    assert_eq!(
        Err(CreationError::Negative),
        PositiveNonzeroInteger::new(-10)
    );
    assert_eq!(
        Err(CreationError::Negative),
        PositiveNonzeroInteger::new(-1)
    );
    assert_eq!(Err(CreationError::Zero), PositiveNonzeroInteger::new(0));
}
