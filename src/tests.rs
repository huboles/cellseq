use super::*;

#[test]
fn area_tests() -> Result<()> {
    let origin = Point::new(5, 5);
    let max = Point::new(35, 25);
    let area = Area::new(origin, max);

    assert_eq!(area.height(), 20);
    assert_eq!(area.width(), 30);

    assert!(area.contains(Point::new(20, 20)));
    assert!(area.contains(Point::new(5, 10)));
    assert!(area.contains(Point::new(24, 25)));
    assert!(area.contains(Point::new(35, 25)));
    assert!(area.contains(Point::new(5, 5)));

    assert!(!area.contains(Point::new(0, 10)));
    assert!(!area.contains(Point::new(15, 0)));
    assert!(!area.contains(Point::new(55, 24)));
    assert!(!area.contains(Point::new(15, 60)));

    assert_eq!(area, Area::from((Point::new(5, 5), Point::new(35, 25))));
    assert_eq!(area, Area::from((5, 5, 35, 25)));
    assert_ne!(area, Area::from((5, 55, 35, 25)));
    Ok(())
}

#[test]
fn point_tests() -> Result<()> {
    let p1 = Point::new(10, 10);
    let p2 = Point::new(15, 30);

    assert_eq!(p2 - p1, Point::new(5, 20));
    assert_eq!(p1 - p2, Point::new(0, 0));

    assert_eq!(Point::new(5, 5), Point::from((5, 5)));
    Ok(())
}
