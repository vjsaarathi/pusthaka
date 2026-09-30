use reader_os::devices::VirtualDevice;
use reader_os::graphics::text::FixedPoint4;

#[test]
fn virtual_device_satisfies_minimum_contract() {
    let device = VirtualDevice::new(800, 480);
    assert!(device.descriptor.satisfies_minimum());
}

#[test]
fn fixed_point_rounds_like_12_4_contract() {
    assert_eq!(FixedPoint4::from_raw(24).to_pixel(), 2);
    assert_eq!(FixedPoint4::from_raw(40).to_pixel(), 3);
}
