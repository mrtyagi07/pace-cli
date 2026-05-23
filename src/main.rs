use std::env;
use std::process;

fn format_pace(seconds_per_km: u32) -> String {
    let minutes = seconds_per_km / 60;
    let seconds = seconds_per_km % 60;
    format!("{}:{:02} /km", minutes, seconds) // {:02} means "pad this number with zeros to at least 2 digits." So if seconds is 5, it will print as "05". If seconds is 12, it will print as "12". This ensures that the pace always looks like M:SS, even when the seconds are less than 10.
}

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


fn main() -> Result<(), Box<dyn  std::error::Error>> { //
    // Result<(), ...> — this function returns a Result. On success, it produces () (called unit — Rust's "nothing," like void in TS). 
    // On failure, the second slot's type.
    // Box<dyn std::error::Error> — "a boxed dynamic error." Translated: "any type that implements the Error trait, on the heap." 
    // This is the easy-mode error type for main. It accepts any error from any library you call into, which is exactly what you want at the top level 
    // — you don't need to know which kind of failure happened, just print it and exit.

   let args: Vec<String> = env::args().collect(); // env::args() returns an iterator over the command-line arguments. .collect() turns that iterator into a Vec<String>. The first argument (args[0]) is the name of the program, so the actual user input starts from args[1].

   if args.len() < 3 {
        eprintln!("usage: pace-cli <pace|dur> <seconds>");
        process::exit(1); // If the user didn't provide enough arguments, print a usage message to stderr and exit with a non-zero code to indicate an error.
   }

    // let value: u32 = args[2].parse()?;
    //? means: "unwrap this Result. If it's Ok(value), give me the value. If it's Err(e), return early from this whole function with that error."
    // ? is only allowed in a function that itself returns a Result (or Option). That's why main's signature had to change.


    let value: u32 = args[2].parse()
    .map_err(|_| format!("'{}' is not a valid non-negative integer", args[2]))?; // .map_err() is a method on Result that transforms the error case. If parse() returns Ok(value), map_err does nothing and we get Ok(value). If parse() returns Err(e), map_err takes that error e, ignores it, and instead produces a new error with our custom message. The ? then propagates that new error up to the caller of main.

    match args[1].as_str() { // args[1] is the first user argument, which should be either "pace" or "dur". We call .as_str() to get a &str slice for matching.
        "pace" => println!("{}", format_pace(value)),
        "dur" => println!("{}", format_duration(value)),
        other => { // If the first argument is "pace", we call format_pace with the parsed value and print it. If it's "dur", we call format_duration and print it. If it's anything else, we print an error message and exit.
            eprintln!("error '{}' is not a valid command", other); //
            process::exit(1);
        }
        
    }

  Ok(()) // If we got here, everything succeeded, so we return Ok(()). This tells the Rust runtime that the program finished successfully. If any of the ? operators returned an error, we would have exited early with that error instead.
}


#[cfg(test)]
// #[cfg(test)] is an attribute that means "only compile this when running cargo test." 
// When you run cargo build or cargo run, this whole module is invisible. It doesn't bloat the release binary, 
// doesn't slow normal builds, doesn't ship to users at all. Like an if (NODE_ENV === 'test') block, 
// except the compiler actually enforces it instead of leaving dead code in production.
mod tests { // mod tests declares a child module called tests
    use super::*; // use super::* means "bring all the items from the parent module into scope."

    #[test] // #[test] is an attribute that marks this function as a test. When you run cargo test, the test runner looks for all functions with #[test] and runs them.
    fn pace_formats_whole_minutes_per_km() {
        assert_eq!(format_pace(330), "5:30 /km"); // assert_eq! is a macro that checks if the first argument equals the second. If not, it panics and fails the test.
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

// Useful flags
// cargo test pace/* — run only tests whose names start with "pace/"
// cargo test -- --test-threads=1 — run tests in a single thread. By default, Rust runs tests in parallel, which is usually good for speed, but if your tests have side effects (like writing to the same file), you might want to run them sequentially with this flag.
// cargo test -- --nocapture — by default, Rust captures all output from tests and only shows it if the test fails. With --nocapture, you can see the output even for passing tests, which is useful for debugging.