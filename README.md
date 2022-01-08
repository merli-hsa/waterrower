# WaterRower Command Line Tool

This command line tool implements basic functionalities to record workout
sessions on [WaterRower](https://www.waterrower.com/) rowing machines that
have an S4 performance monitor installed.

Workout data is saved in the simple Comma-Separated Values (CSV) format and
is therefore easy to process further.

The tool is written in Rust and has been tested on an Ubuntu Linux system.

## Example Usage

When connecting the S4 performance monitor to a USB port of your PC, a new
serial device is recognized, e.g. ``/dev/ttyACM0``.

Afterwards, the tool can be run as follows:

```sh
waterrower record -s /dev/ttyACM0
```

In this case, the directory ``./workouts`` will be created in which every
workout is stored according to date and time of workout start.

## License

This tool is licensed under either of

 * [Apache License, Version 2.0](http://www.apache.org/licenses/LICENSE-2.0)
 * [MIT license](http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
