#![allow(unstable)]
// BatWarn
// Low Battery Warning Utility
// Polls battery status and pops up warning when low.

extern crate collections;
use std::io::timer::sleep;
use std::io::fs::File;
use std::io::IoResult;
use std::time::duration::Duration;
use collections::string::String;
use std::io::process::{Command, ProcessOutput};

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
        acpi_battery_state();
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

fn acpi_battery_state() {
    let mut cmd = Command::new("rustc");
    cmd.arg("--version");
    match cmd.output() {
        Err(_) =>
            println!("failed"),
        Ok(ProcessOutput { status: exit, output: stdout, error: _ }) => {
            match exit.success() {
                false => println!("failed"),
                true => {
                    let stdout = String::from_utf8(stdout).unwrap();
                    println!("acpi out {}", stdout);
                },
            }
        },
    }
}
