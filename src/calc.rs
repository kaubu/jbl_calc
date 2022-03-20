// Calculate the percentage of JBL headphones (specifically, my pair)

// 100% - 16 hours/960 minutes
// 1% - 9 minutes 36 seconds (9.6 minutes)

// In 5 minutes it produces 1 hour/60 minutes of music charge
// So in 5 minutes it produces 6.25% of the charge
// So in 1 minute it produces 1.25% of the charge
// So it takes 16 of those 5 minutes to charge
// Thus it takes 80 minutes, or 1 hour and 20 minutes to charge

// Calculate time from percentage
// time_from_percentage = (100 - PERCENTAGE) * 9.6 = amount of minutes

// Input percentage:
// Calculate time it has left: time_from_percentage(PERCENTAGE)

// Input how long it has been charge for (in minutes):
// Calculate percentage: 1.25 * MINUTES = PERCENTAGE
// Calculate time it has left: time_from_percentage(PERCENTAGE)

use std::fmt;
use tramp::{tramp, Rec};

#[derive(Debug, Clone)]
pub struct HMSTime {
    hours: f64,
    minutes: f64,
    seconds: f64,
}

impl HMSTime {
    fn new(hours: f64, minutes: f64, seconds: f64) -> HMSTime {
        HMSTime {
            hours,
            minutes,
            seconds
        }
    }
    
    // Rounding down functions
    fn hours_to_minutes(hours: f64) -> f64 {
        hours * 60.0
    }

    fn minutes_to_seconds(minutes: f64) -> f64 {
        minutes * 60.0
    }

    // Rounding up functions
    fn seconds_to_minutes(&self) -> (f64, f64) {
        // (minutes, seconds)
        let minutes = self.seconds / 60.0;
        (minutes, self.seconds - (minutes * 60.0))
    }

    fn minutes_to_hours(&self) -> (f64, f64) {
        // (hours, minutes)
        let hours = self.minutes / 60.0;
        (hours, self.minutes - (hours * 60.0))
    }

    
}

impl fmt::Display for HMSTime {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let hours = match self.hours {
            x if x == 1.0 => "1 hour".to_owned(),
            _ => format!("{} hours", round_float_to_dp(self.hours, 2)),
        };

        let minutes = match self.minutes {
            x if x == 1.0 => "1 minute".to_owned(),
            _ => format!("{} minutes", round_float_to_dp(self.minutes, 2)),
        };

        let seconds = match self.seconds {
            x if x == 1.0 => "1 second".to_owned(),
            _ => format!("{} seconds", round_float_to_dp(self.seconds, 2)),
        };

        write!(f, "{}, {}, and {}", hours, minutes, seconds)
    }
}

// Rounds a float to a decimal place
// round_float_to_dp(53.12859, 2) = 52.13
fn round_float_to_dp(num: f64, dp: i32) -> f64 {
    let tens = f64::powi(10.0, dp);
    (num * tens).round() / tens
}

fn round_up(time_object: HMSTime) -> HMSTime {
    // println!("Minutes = {}\nSeconds = {}", 
    //     time_object.minutes, time_object.seconds);
    
    fn round_up_rec(time_object: HMSTime) -> Rec<HMSTime> {
        if time_object.minutes.fract() != 0.0 {
            let seconds = HMSTime::minutes_to_seconds(
                time_object.minutes.fract()
            );
            
            let time = HMSTime::new(
                time_object.hours,
                time_object.minutes - time_object.minutes.fract(),
                time_object.seconds + seconds,
            );
            
            rec_call!(round_up_rec(time))
        } else if time_object.hours.fract() != 0.0 {
            let minutes = HMSTime::hours_to_minutes(
                time_object.hours.fract()
            );
    
            let time = HMSTime::new(
                time_object.hours - time_object.hours.fract(),
                time_object.minutes + minutes,
                time_object.seconds,
            );
            
            rec_call!(round_up_rec(time))
        } else if time_object.minutes < 60.0 && time_object.seconds < 60.0 {
            rec_ret!(time_object);
        } else if time_object.minutes < 60.0 && time_object.seconds > 60.0 {
            // (minutes, seconds)
            let ms = time_object.seconds_to_minutes();

            let time = HMSTime {
                hours: time_object.hours,
                minutes: time_object.minutes + ms.0,
                seconds: ms.1,
            };

            rec_call!(round_up_rec(time))
        } else {
            // (hours, minutes)
            let hm = time_object.minutes_to_hours(); 
            // (minutes, seconds)
            let ms = time_object.seconds_to_minutes();
            
            let time = HMSTime {
                hours: hm.0,
                minutes: hm.1 + ms.0,
                seconds: ms.1,
            };
            
            rec_call!(round_up_rec(time))
        }
    }

    tramp(round_up_rec(time_object))
}

pub fn time_from_percentage(percentage: f64) -> HMSTime {
    let minutes = percentage * 9.6;
    let time = HMSTime::new(0.0, minutes, 0.0);
    round_up(time)
}