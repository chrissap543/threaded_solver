use threaded_solver::solver;
use threaded_solver::solver::test_board;

#[test]
fn returns_invalid_for_1_0_4() {
    assert!(!solver::valid(&test_board(), 1, 0, 4))
}

#[test]
fn returns_invalid_for_1_0_1() {
    assert!(!solver::valid(&test_board(), 1, 0, 1))
}

#[test]
fn returns_invalid_for_1_1_4() {
    assert!(!solver::valid(&test_board(), 1, 1, 4))
}

#[test]
fn returns_invalid_for_1_2_4() {
    assert!(!solver::valid(&test_board(), 1, 2, 4))
}

#[test]
fn returns_invalid_for_0_3_4() {
    assert!(!solver::valid(&test_board(), 0, 3, 4))
}

#[test]
fn returns_invalid_for_0_3_3() {
    assert!(!solver::valid(&test_board(), 0, 3, 3))
}

#[test]
fn returns_invalid_for_6_6_3() {
    assert!(!solver::valid(&test_board(), 6, 6, 3))
}

#[test]
fn returns_invalid_for_8_6_1() {
    assert!(!solver::valid(&test_board(), 8, 6, 1))
}

#[test]
fn returns_valid_for_0_3_2() {
    assert!(solver::valid(&test_board(), 0, 3, 2))
}
