const TEST_IN: &str = "\
Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";

struct Coords {
    x: isize,
    y: isize,
}

struct Sensor {
    coords: Coords,
    beacon: Coords,
}
impl Sensor {
    fn from_str(inp: &str) -> Self {
        let (sensor_str, beacon_str) = inp.split_once(":").unwrap();
    }
}

#[cfg(test)]
#[test]
fn gets_sensor() {
    let line_1 = TEST_IN.lines().take(1).unwrap();

    let sensor = Sensor::from_str(line_1);
    assert_eq!(sensor.coords.x, 2);
    assert_eq!(sensor.coords.y, 18);
    // assert_eq!(sensor.beacon.y, -2);
    // assert_eq!(sensor.beacon.x, 15);
}
