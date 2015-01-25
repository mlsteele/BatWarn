# BatWarn
Battery warning for people without Desktop Environments.

BatWarn monitors laptop battery level and displays a warning bar at the top of the screen 
when the battery dips below 20% and an urgent red bar when it goes below 8%.
It is meant for use with i3wm, a window manager whose default setup does not come
with this sort of amenity pre-configured.

BatWarn polls `acpi` every 5 minutes and checks whether the battery is being charged
and its current level.

# Usage

To use BatWarn, clone the repository and run `make` to compile.

Then `setsid make run` to spawn an instance of BatWarn which will begin polling.

If you think there might be an instance of BatWarn already running,
you can run `make pkill` to kill it.

# Dependencies
BatWarn uses `i3-nagbar` to display a popup bar when the battery is low.
It shouldn't be too hard to modify it to use some other graphic, but easiest would be to have i3wm installed.

You will need `rustc` to compile the source.
Rust is a quickly evolving language, so drop me a line if something breaks.
Last compiled with `rustc 1.0.0-nightly`
