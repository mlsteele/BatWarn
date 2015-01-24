#![allow(unstable)]
// BatWarn
// Low Battery Warning Utility
// Polls battery status and pops up warning when low.

use std::io::timer::sleep;
use std::io::fs::File;
use std::io::IoResult;
use std::time::duration::Duration;
use std::str::from_utf8;

static PERCENT_DANGER:     i32 = 20;
static PERCENT_CRITICAL:   i32 = 8;

fn main() {
    // let poll_delay: Duration = Duration::minutes(5);
    let poll_delay: Duration = Duration::seconds(1);

    loop {
        println!("loop body");

        // Check battery status
        read_battery_state().map(|s| {
            println!("{}", s);
        });
        let percent = 72;
        let charging = false;

        // Kill existing warnings

        if charging {
            // Do nothing
        } else if percent <= PERCENT_CRITICAL {
            // Battery critically low.
        } else if percent <= PERCENT_DANGER {
            // Battery low.
        } else {
            // Battery discharging normally.
        }

        sleep(poll_delay);
    }
}

fn read_battery_state() -> IoResult<String> {
    let path = Path::new("/proc/acpi/battery/BAT0/state");
    match File::open(&path) {
        Err(err) => Err(err),
        Ok(mut file) => file.read_to_string(),
    }
}
