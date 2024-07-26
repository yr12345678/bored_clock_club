use scrypto::prelude::*;

#[blueprint]
mod bored_clock_club {
    struct BoredClockClub {
    }

    impl BoredClockClub {
        pub fn instantiate() -> Global<BoredClockClub> {
            Self {
            }
            .instantiate()
            .prepare_to_globalize(OwnerRole::None)
            .globalize()
        }

        pub fn get_time(&mut self) -> (Instant, Instant) {
            (
                Clock::current_time_rounded_to_minutes(),
                Clock::current_time_rounded_to_seconds()
            )
        }
    }
}
