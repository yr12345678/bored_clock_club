use scrypto_test::prelude::*;
use bored_clock_club::bored_clock_club_test::*;

#[test]
fn returned_times_are_equal() -> Result<(), RuntimeError> {
    let mut env = TestEnvironment::new();
    let package_address = 
        PackageFactory::compile_and_publish(this_package!(), &mut env, CompileProfile::Fast)?;

    let mut bored_clock_club = BoredClockClub::instantiate(package_address, &mut env)?;

    // Advance time so seconds since Unix epoch is not 0
    let twenty_eight_days_later = env.get_current_time().add_days(28).unwrap();
    env.set_current_time(twenty_eight_days_later);

    // Get the current time via Clock
    let (rounded_to_minutes, rounded_to_seconds) = bored_clock_club.get_time(&mut env)?;

    // Times should be equal of course
    assert_eq!(
        rounded_to_minutes,
        rounded_to_seconds,
        "Times were not equal"
    );

    Ok(())
}
