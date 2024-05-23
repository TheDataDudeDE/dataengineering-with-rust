//Create external tests for the lib functions
use trust::add;
use trust::divide;
use trust::multiply;
use trust::subtract;

#[test]
fn test_add() {
    assert_eq!(add(1, 2), 3);
}

#[test]
fn test_subtract() {
    assert_eq!(subtract(2, 1), 1);
}

#[test]
fn test_multiply() {
    assert_eq!(multiply(2, 3), 6);
}

#[test]
fn test_divide() {
    assert_eq!(divide(6, 3), 2);
}
