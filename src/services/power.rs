use crate::hal::power::PowerController;

pub struct PowerService<P: PowerController> { pub controller: P, pub auto_sleep_enabled: bool }

impl<P: PowerController> PowerService<P> {
    /// TODO: enforce the configured inactivity timeout and the activity-level prevent-sleep contract.
    pub fn tick(&mut self) -> crate::Result<()> { Ok(()) }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockPower;
    impl crate::hal::power::PowerController for MockPower {
        type Error = ();
        fn hold_power_rails(&mut self) -> Result<(), Self::Error> { Ok(()) }
        fn enter_sleep(&mut self) -> Result<(), Self::Error> { Ok(()) }
        fn reset(&mut self) -> Result<(), Self::Error> { Ok(()) }
        fn prevent_sleep(&mut self, _prevent: bool) -> Result<(), Self::Error> { Ok(()) }
    }

    #[test]
    fn test_power_tick() {
        let mut fsm = PowerService { controller: MockPower, auto_sleep_enabled: true };
        assert!(fsm.tick().is_ok());
    }
}
