#[cfg(test)]
mod time_tests {
    // use super::*;
    use std::time::{Duration, SystemTime, SystemTimeError};

    // #[test]
    // fn system_time() {
    //     let now = SystemTime::now();
    //     // we sleep for 2 seconds
    //     // let duration = Duration::new(1, 0);
    //     // await sleep(duration);
    //     let milli = match now.elapsed() {
    //         Ok(elapsed) => {
    //             // it prints '2'
    //             return elapsed.as_millis();
    //         }
    //         Err(e) => {
    //             // an error occurred!
    //             panic!("Error: {e:?}");
    //         }
    //     };
    //     assert_eq!(0, milli);
    //     // assert_eq!(0, duration);
    // }

    #[test]
    fn system_time_duration_since_unix_epoch() {
        let now:SystemTime = SystemTime::now();
        let epoch:SystemTime = SystemTime::UNIX_EPOCH;
        let duration:Result<Duration, SystemTimeError> = now.duration_since(epoch);
        let sec = match duration {
            // since 1970
            Ok(n) => n.as_secs(),
            Err(_) => panic!("SystemTime before UNIX EPOCH!"),
        };
        let years = sec / 60 / 60 / 24 / 365;
        assert!(52 < years);
    }

    #[test]
    fn system_time_duration() {
        let duration = Duration::new(5, 730023852);
        assert_eq!(duration.as_millis(), 5730);
    }
}
