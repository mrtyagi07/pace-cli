use std::env;
use std::process;

/// Formats seconds-per-km as `M:SS /km` (e.g. 330 → "5:30 /km").
fn format_pace(seconds_per_km: u32) -> String {
    let minutes = seconds_per_km / 60;
    let seconds = seconds_per_km % 60;
    format!("{}:{:02} /km", minutes, seconds)
}

/// Formats a duration in seconds as "Xh Ym Zs", dropping leading zero units.
fn format_duration(seconds: u32) -> String {
    let h = seconds / 3600;
    let m = (seconds % 3600) / 60;
    let s = seconds % 60;

    if h > 0 {
        format!("{}h {}m {}s", h, m, s)
    } else if m > 0 {
        format!("{}m {}s", m, s)
    } else {
        format!("{}s", s)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("usage: pace-cli <pace|dur> <seconds>");
        process::exit(1);
    }

    let value: u32 = args[2]
        .parse()
        .map_err(|_| format!("'{}' is not a valid non-negative integer", args[2]))?;

    match args[1].as_str() {
        "pace" => println!("{}", format_pace(value)),
        "dur" => println!("{}", format_duration(value)),
        other => {
            eprintln!("error '{}' is not a valid command", other);
            process::exit(1);
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pace_formats_whole_minutes_per_km() {
        assert_eq!(format_pace(330), "5:30 /km");
    }

    #[test]
    fn pace_pads_single_digit_seconds() {
        assert_eq!(format_pace(305), "5:05 /km");
    }

    #[test]
    fn duration_formats_hours_minutes_seconds() {
        assert_eq!(format_duration(5025), "1h 23m 45s");
    }

    #[test]
    fn duration_drops_the_hour_when_zero() {
        assert_eq!(format_duration(125), "2m 5s");
    }

    #[test]
    fn duration_returns_seconds_only_for_short_durations() {
        assert_eq!(format_duration(30), "30s");
    }
}
