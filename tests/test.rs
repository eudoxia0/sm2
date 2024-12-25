use sm2::Item;
use sm2::Quality;

/// Quality maps to expected natural number values.
#[test]
fn quality_values() {
    assert_eq!(Quality::Blackout as u8, 0);
    assert_eq!(Quality::Incorrect as u8, 1);
    assert_eq!(Quality::IncorrectEasy as u8, 2);
    assert_eq!(Quality::Hard as u8, 3);
    assert_eq!(Quality::Good as u8, 4);
    assert_eq!(Quality::Perfect as u8, 5);
}

/// `Quality::forgot` works as expected.
#[test]
fn test_forgot() {
    assert!(Quality::Blackout.forgot());
    assert!(Quality::Incorrect.forgot());
    assert!(Quality::IncorrectEasy.forgot());
    assert!(!Quality::Hard.forgot());
    assert!(!Quality::Good.forgot());
    assert!(!Quality::Perfect.forgot());
}

/// `Quality::repeat` works as expected.
#[test]
fn test_repeat() {
    assert!(Quality::Blackout.repeat());
    assert!(Quality::Incorrect.repeat());
    assert!(Quality::IncorrectEasy.repeat());
    assert!(Quality::Hard.repeat());
    assert!(!Quality::Good.repeat());
    assert!(!Quality::Perfect.repeat());
}

/// A default `Item` has zero repetitions and the initial EF.
#[test]
fn test_default() {
    let item = Item::default();
    assert_eq!(item.n(), 0);
    assert_eq!(item.ef(), sm2::INITIAL_EF);
}

/// Item constructor and accessors work.
#[test]
fn test_constructor_and_accessors() {
    let item = Item::new(0, 3.0);
    assert_eq!(item.n(), 0);
    assert_eq!(item.ef(), 3.0);
}

/// Test the `Item::interval` method.
#[test]
fn test_interval() {
    let ef = 2.5;
    let cases = [
        (Item::new(0, ef), 0),
        (Item::new(1, ef), 1),
        (Item::new(2, ef), 6),
        (Item::new(3, ef), 15),
        (Item::new(4, ef), 38),
        (Item::new(5, ef), 94),
    ];
    for (item, expected) in cases {
        assert_eq!(item.interval(), expected);
    }
}

/// Forgetting an item sets the repetitions to zero.
#[test]
fn test_forgetting() {
    let qs = [
        Quality::Blackout,
        Quality::Incorrect,
        Quality::IncorrectEasy,
    ];
    for q in qs {
        let item = Item::new(3, 2.5);
        let item = item.review(q);
        assert_eq!(item.n(), 0);
    }
}

/// EF is never below the minimum after a review.
#[test]
fn test_min_ef() {
    let qs = [
        Quality::Blackout,
        Quality::Incorrect,
        Quality::IncorrectEasy,
        Quality::Hard,
        Quality::Good,
        Quality::Perfect,
    ];
    for q1 in qs {
        for q2 in qs {
            for q3 in qs {
                for q4 in qs {
                    for q5 in qs {
                        let mut item = Item::default();
                        let qv = [q1, q2, q3, q4, q5];
                        for q in qv {
                            item = item.review(q);
                            assert!(item.ef() >= 1.3);
                        }
                    }
                }
            }
        }
    }
}

fn close_enough(a: f32, b: f32) -> bool {
    (a - b).abs() < 0.01
}

/// Test how EF evolves.
///
/// These values were calculated manually in Python:
///
/// ```python
/// >>> ef = lambda f, q: f-0.8+0.28*q-0.02*q*q
/// >>> ef(2.5, 3)
/// 2.36
/// >>> ef(2.36, 4)
/// 2.36
/// >>> ef(2.36, 5)
/// 2.46
/// >>> ef(2.46, 5)
/// 2.56
/// ```
#[test]
fn test_ef() {
    let item = Item::default();
    let item = item.review(Quality::Hard);
    assert!(close_enough(item.ef(), 2.36));
    let item = item.review(Quality::Good);
    assert!(close_enough(item.ef(), 2.36));
    let item = item.review(Quality::Perfect);
    assert!(close_enough(item.ef(), 2.46));
    let item = item.review(Quality::Perfect);
    assert!(close_enough(item.ef(), 2.56));
}
