use chrono::Local;
use std::{collections::HashMap, io, path::PathBuf, str, thread, time, u32};

const SERIAL_BAUDRATE: u32 = 115_200;
const SERIAL_TIMEOUT: time::Duration = time::Duration::from_millis(10);
const SERIAL_COMMAND_WAIT: time::Duration = time::Duration::from_millis(25);

fn serial_initialize(serial_dev: &str) -> Box<dyn serialport::SerialPort> {
    let port = serialport::new(serial_dev, SERIAL_BAUDRATE)
        .timeout(SERIAL_TIMEOUT)
        .open()
        .expect("!!! Failed to open serial port!");
    port
}

fn serial_send_command(port: &mut Box<dyn serialport::SerialPort>, command: &str, debug: bool) {
    let serial_command = format!("{}\n", command);
    if debug {
        print!("COMMAND: {}", serial_command);
    }
    port.write_all(serial_command.as_bytes())
        .expect("!!! Sending command to serial port failed!");
    thread::sleep(SERIAL_COMMAND_WAIT);
}

fn serial_receive_response(port: &mut Box<dyn serialport::SerialPort>, debug: bool) -> String {
    let mut serial_buf: Vec<u8> = vec![0; 1024];
    let mut serial_response = "";
    match port.read(serial_buf.as_mut_slice()) {
        Ok(t) => serial_response = str::from_utf8(&serial_buf[..t]).unwrap(),
        Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
        Err(e) => eprintln!("!!! Receiving response from serial port failed: {:?}", e),
    };
    if debug && !serial_response.is_empty() {
        println!("--- BEGIN RESPONSE ---");
        print!("{}", serial_response);
        println!("--- END RESPONSE ---");
    }
    String::from(serial_response)
}

const CMD_START: &str = "USB";
const CMD_STOP: &str = "EXIT";
const _CMD_RESET: &str = "RESET";
const CMD_MODEL_INFO: &str = "IV?";
const _CMD_READ_1_BYTE: &str = "IRS";
const _CMD_READ_2_BYTES: &str = "IRD";
const _CMD_READ_3_BYTES: &str = "IRT";

const _RET_OK: &str = "OK";
const RET_ERROR: &str = "ERROR";
const RET_HW_TYPE: &str = "_WR_";
const RET_MODEL_INFO: &str = "IV"; // IV + Model (4/5) + Version High + Version Low
const RET_DATA_1_BYTE: &str = "IDS"; // IDS + Memory Addr + 1st Byte
const RET_DATA_2_BYTES: &str = "IDD"; // IDD + Memory Addr + 2nd Byte + 1st Byte
const RET_DATA_3_BYTES: &str = "IDT"; // IDT + Memory Addr + 3rd Byte + 2nd Byte + 1st Byte

const WR_STROKE_START: &str = "SS";
const _WR_STROKE_END: &str = "SE";
const _WR_PING: &str = "PING";

const REQUESTING_INTERVAL: time::Duration = time::Duration::from_secs(2);

struct WaterRowerValue {
    name: &'static str,
    command: &'static str,
    response: &'static str,
}

const _SCREEN_MODE: WaterRowerValue = WaterRowerValue {
    name: "Screen Mode",
    command: "IRS00D",
    response: "IDS00D",
};

const DISTANCE: WaterRowerValue = WaterRowerValue {
    name: "Distance",
    command: "IRD055",
    response: "IDD055",
};

const _DISPLAY_DISTANCE: WaterRowerValue = WaterRowerValue {
    name: "Distance (Display)",
    command: "IRD057",
    response: "IDD057",
};

const _CLOCK_COUNT_DOWN: WaterRowerValue = WaterRowerValue {
    name: "Clock Count Down",
    command: "IRT05A",
    response: "IDT05A",
};

const _TOTAL_DISTANCE: WaterRowerValue = WaterRowerValue {
    name: "Total Distance",
    command: "IRT080",
    response: "IDT080",
};

const _TANK_VOLUME: WaterRowerValue = WaterRowerValue {
    name: "Tank Volume",
    command: "IRS0A9",
    response: "IDS0A9",
};

const STROKE_COUNT: WaterRowerValue = WaterRowerValue {
    name: "Stroke Count",
    command: "IRD140",
    response: "IDD140",
};

const STROKE_TIME_AVG: WaterRowerValue = WaterRowerValue {
    name: "Stroke Time Average",
    command: "IRS142",
    response: "IDS142",
};

const STROKE_PULL_TIME_AVG: WaterRowerValue = WaterRowerValue {
    name: "Stroke Pull Time Average",
    command: "IRS143",
    response: "IDS143",
};

const _TOTAL_CENTIMETERS_PER_SECOND: WaterRowerValue = WaterRowerValue {
    name: "Total Centimeters per Second",
    command: "IRD148",
    response: "IDD148",
};

const _INSTANT_CENTIMETERS_PER_SECOND: WaterRowerValue = WaterRowerValue {
    name: "Instant Centimeters per Second",
    command: "IRD14A",
    response: "IDD14A",
};

const ZONE_HEART_RATE: WaterRowerValue = WaterRowerValue {
    name: "Heart Rate (Zone)",
    command: "IRS1A0",
    response: "IDS1A0",
};

const _ZONE_CENTIMETERS_PER_SECOND: WaterRowerValue = WaterRowerValue {
    name: "Centimeters per Second (Zone)",
    command: "IRD1A1",
    response: "IDD1A1",
};

const ZONE_SECONDS_PER_500M: WaterRowerValue = WaterRowerValue {
    name: "Seconds per 500 Meters (Zone)",
    command: "IRD1A5",
    response: "IDD1A5",
};

const _ZONE_SECONDS_PER_2KM: WaterRowerValue = WaterRowerValue {
    name: "Seconds per 2 Kilometers (Zone)",
    command: "IRD1A7",
    response: "IDD1A7",
};

const ZONE_STROKE_RATE: WaterRowerValue = WaterRowerValue {
    name: "Stroke Rate (Zone)",
    command: "IRS1A9",
    response: "IDS1A9",
};

const _DISPLAY_SECOND_DECIMALS: WaterRowerValue = WaterRowerValue {
    name: "Second Decimals (Display)",
    command: "IRS1E0",
    response: "IDS1E0",
};

const DISPLAY_SECONDS: WaterRowerValue = WaterRowerValue {
    name: "Seconds (Display)",
    command: "IRS1E1",
    response: "IDS1E1",
};

const DISPLAY_MINUTES: WaterRowerValue = WaterRowerValue {
    name: "Minutes (Display)",
    command: "IRS1E2",
    response: "IDS1E2",
};

const DISPLAY_HOURS: WaterRowerValue = WaterRowerValue {
    name: "Hours (Display)",
    command: "IRS1E3",
    response: "IDS1E3",
};

const WATER_ROWER_VALUES: [WaterRowerValue; 10] = [
    DISPLAY_SECONDS,
    DISPLAY_MINUTES,
    DISPLAY_HOURS,
    DISTANCE,
    STROKE_COUNT,
    STROKE_TIME_AVG,
    STROKE_PULL_TIME_AVG,
    ZONE_HEART_RATE,
    ZONE_SECONDS_PER_500M,
    ZONE_STROKE_RATE,
];

pub enum WorkoutState {
    Init,
    Connected,
    Running,
    Finished,
}

pub struct WorkoutContext {
    pub state: WorkoutState,
    pub port: Box<dyn serialport::SerialPort>,
    pub debug: bool,
}

pub fn workout_context_init(serial_dev: &str, debug: bool) -> self::WorkoutContext {
    let ctx_init = WorkoutContext {
        state: WorkoutState::Init,
        port: serial_initialize(serial_dev),
        debug,
    };
    ctx_init
}

pub struct GlobalWorkoutValues {
    pub date_time_start: String,
    pub date_time_end: String,
    pub model: String,
    pub fw_version: String,
    pub datapoints: u32,
    pub total_time_in_seconds: u32,
    pub total_distance_in_meters: u32,
    pub total_stroke_count: u32,
    pub seconds_per_500m_min: u32,
    pub seconds_per_500m_avg: f32,
    pub seconds_per_500m_max: u32,
    pub strokes_per_minute_min: u32,
    pub strokes_per_minute_avg: f32,
    pub strokes_per_minute_max: u32,
    pub stroke_ratio_min: f32,
    pub stroke_ratio_avg: f32,
    pub stroke_ratio_max: f32,
    pub heart_rate_min: u32,
    pub heart_rate_avg: f32,
    pub heart_rate_max: u32,
}

pub fn global_workout_values_init(ctx: &mut WorkoutContext) -> self::GlobalWorkoutValues {
    let mut gwv_init = GlobalWorkoutValues {
        date_time_start: String::from(""),
        date_time_end: String::from(""),
        model: String::from(""),
        fw_version: String::from(""),
        datapoints: 0,
        total_time_in_seconds: 0,
        total_distance_in_meters: 0,
        total_stroke_count: 0,
        seconds_per_500m_min: 0,
        seconds_per_500m_avg: 0.0,
        seconds_per_500m_max: 0,
        strokes_per_minute_min: 0,
        strokes_per_minute_avg: 0.0,
        strokes_per_minute_max: 0,
        stroke_ratio_min: 0.0,
        stroke_ratio_avg: 0.0,
        stroke_ratio_max: 0.0,
        heart_rate_min: 0,
        heart_rate_avg: 0.0,
        heart_rate_max: 0,
    };

    // Get current date and time
    let dt = Local::now();
    gwv_init.date_time_start = dt.format("%Y-%m-%d %H:%M:%S").to_string();

    // Get WaterRower model and firmware information
    serial_send_command(&mut ctx.port, CMD_MODEL_INFO, ctx.debug);
    let serial_response = serial_receive_response(&mut ctx.port, ctx.debug);
    for line in serial_response.lines() {
        if &line[0..2] == RET_MODEL_INFO {
            gwv_init.model = (&line[2..3]).to_owned();
            gwv_init.fw_version = format!("{}.{}", &line[3..5], &line[5..7]);
        }
    }

    gwv_init
}

pub struct InstantWorkoutValues {
    pub time_in_seconds: u32,
    pub distance_in_meters: u32,
    pub seconds_per_500m: u32,
    pub stroke_count: u32,
    pub strokes_per_minute: u32,
    pub stroke_ratio: f32,
    pub heart_rate: u32,
}

pub fn instant_workout_values_init() -> self::InstantWorkoutValues {
    InstantWorkoutValues {
        time_in_seconds: 0,
        distance_in_meters: 0,
        seconds_per_500m: 0,
        stroke_count: 0,
        strokes_per_minute: 0,
        stroke_ratio: 0.0,
        heart_rate: 0,
    }
}

pub fn start(ctx: &mut WorkoutContext) {
    serial_send_command(&mut ctx.port, CMD_START, ctx.debug);
    let serial_response = serial_receive_response(&mut ctx.port, ctx.debug);
    for line in serial_response.lines() {
        if let RET_HW_TYPE = line {
            ctx.state = WorkoutState::Connected
        }
    }
}

pub fn stop(ctx: &mut WorkoutContext) {
    serial_send_command(&mut ctx.port, CMD_STOP, ctx.debug);
}

pub fn wait_for_first_stroke(ctx: &mut WorkoutContext) {
    loop {
        let serial_response = serial_receive_response(&mut ctx.port, ctx.debug);
        for line in serial_response.lines() {
            if let WR_STROKE_START = line {
                ctx.state = WorkoutState::Running;
            }
        }
        if let WorkoutState::Running = ctx.state {
            break;
        }
    }
}

pub fn workout_values_update(
    ctx: &mut WorkoutContext,
    iwv: &mut InstantWorkoutValues,
    gwv: &mut GlobalWorkoutValues,
) {
    let now = time::Instant::now();
    let mut raw_values: HashMap<&str, String> = HashMap::new();

    // Send command for every WaterRower value to obtain
    for value in WATER_ROWER_VALUES.iter() {
        serial_send_command(&mut ctx.port, value.command, ctx.debug);
    }

    // Receive response(s) in a loop
    while now.elapsed() < REQUESTING_INTERVAL {
        let serial_response = serial_receive_response(&mut ctx.port, ctx.debug);
        for line in serial_response.lines() {
            match line {
                RET_ERROR => println!("!!! Error during WaterRower communication ..."),
                _ => {
                    if line.len() > 6
                        && (&line[0..3] == RET_DATA_1_BYTE
                            || &line[0..3] == RET_DATA_2_BYTES
                            || &line[0..3] == RET_DATA_3_BYTES)
                    {
                        for value in WATER_ROWER_VALUES.iter() {
                            if &line[..value.response.len()] == value.response {
                                raw_values
                                    .insert(value.name, (&line[value.response.len()..]).to_owned());
                            }
                        }
                    }
                }
            }
        }
    }

    instant_workout_values_update(&raw_values, iwv);

    // Check if workout was ended on WaterRower device
    if iwv.time_in_seconds > 0 && iwv.time_in_seconds == gwv.total_time_in_seconds {
        ctx.state = WorkoutState::Finished;
        return;
    }

    global_workout_values_update(&iwv, gwv);
}

#[rustfmt::skip]
fn instant_workout_values_update(raw_values: &HashMap<&str, String>, iwv: &mut InstantWorkoutValues) {
    iwv.time_in_seconds =
        u32::from_str_radix(raw_values.get(DISPLAY_SECONDS.name).unwrap(), 10).unwrap()
        + 60 * u32::from_str_radix(raw_values.get(DISPLAY_MINUTES.name).unwrap(), 10).unwrap()
        + 3600 * u32::from_str_radix(raw_values.get(DISPLAY_HOURS.name).unwrap(), 10).unwrap();
    iwv.distance_in_meters =
        u32::from_str_radix(raw_values.get(DISTANCE.name).unwrap(), 16).unwrap();
    iwv.seconds_per_500m =
        u32::from_str_radix(&(raw_values.get(ZONE_SECONDS_PER_500M.name).unwrap())[0..2], 16).unwrap()
        + 256 * u32::from_str_radix(&(raw_values.get(ZONE_SECONDS_PER_500M.name).unwrap())[2..4], 16).unwrap();
    iwv.stroke_count =
        u32::from_str_radix(raw_values.get(STROKE_COUNT.name).unwrap(), 16).unwrap();
    iwv.strokes_per_minute =
        u32::from_str_radix(raw_values.get(ZONE_STROKE_RATE.name).unwrap(), 16).unwrap();
    
    // Somewhat vague note for stroke ratio calculation from WaterRower docs:
    //   Stroke_pull is first subtracted from stroke_average
    //   then a modifier of 1.25 multiplied by the result to generate the ratio value for display.
    let stroke_time_avg: f32 =
        u32::from_str_radix(raw_values.get(STROKE_TIME_AVG.name).unwrap(), 16).unwrap() as f32;
    let pull_time_avg: f32 =
        u32::from_str_radix(raw_values.get(STROKE_PULL_TIME_AVG.name).unwrap(), 16).unwrap() as f32;
    if pull_time_avg > 0.0 {
        iwv.stroke_ratio = (stroke_time_avg - pull_time_avg) / (pull_time_avg * 1.25);
    }
    else {
        iwv.stroke_ratio = 0.0;
    }
    
    iwv.heart_rate =
        u32::from_str_radix(raw_values.get(ZONE_HEART_RATE.name).unwrap(), 16).unwrap();
}

fn global_workout_values_update(iwv: &InstantWorkoutValues, gwv: &mut GlobalWorkoutValues) {
    gwv.datapoints += 1;
    gwv.total_time_in_seconds = iwv.time_in_seconds;
    gwv.total_distance_in_meters = iwv.distance_in_meters;
    gwv.total_stroke_count = iwv.stroke_count;
}

pub fn global_workout_values_finalize(
    datapoints: &[InstantWorkoutValues],
    gwv: &mut GlobalWorkoutValues,
) {
    // Get current date and time
    let dt = Local::now();
    gwv.date_time_end = dt.format("%Y-%m-%d %H:%M:%S").to_string();

    // Get valid values out of all datapoints
    let mut seconds_per_500m_valid_values: Vec<u32> = Vec::new();
    let mut strokes_per_minute_valid_values: Vec<u32> = Vec::new();
    let mut stroke_ratio_valid_values: Vec<f32> = Vec::new();
    let mut heart_rate_valid_values: Vec<u32> = Vec::new();
    for values in datapoints.iter() {
        if values.seconds_per_500m > 0 {
            seconds_per_500m_valid_values.push(values.seconds_per_500m);
        }
        if values.strokes_per_minute > 0 {
            strokes_per_minute_valid_values.push(values.strokes_per_minute);
        }
        if values.stroke_ratio > 0.0 {
            stroke_ratio_valid_values.push(values.stroke_ratio);
        }
        if values.heart_rate > 0 {
            heart_rate_valid_values.push(values.heart_rate);
        }
    }

    // Calculate min, max, average
    if !seconds_per_500m_valid_values.is_empty() {
        gwv.seconds_per_500m_min = *seconds_per_500m_valid_values.iter().min().unwrap(); //.clone();
        gwv.seconds_per_500m_max = *seconds_per_500m_valid_values.iter().max().unwrap(); //.clone();
        gwv.seconds_per_500m_avg = seconds_per_500m_valid_values.iter().sum::<u32>() as f32
            / seconds_per_500m_valid_values.iter().len() as f32;
    }
    if !strokes_per_minute_valid_values.is_empty() {
        gwv.strokes_per_minute_min = *strokes_per_minute_valid_values.iter().min().unwrap();
        //.clone();
        gwv.strokes_per_minute_max = *strokes_per_minute_valid_values.iter().max().unwrap();
        //.clone();
        gwv.strokes_per_minute_avg = strokes_per_minute_valid_values.iter().sum::<u32>() as f32
            / strokes_per_minute_valid_values.iter().len() as f32;
    }
    if !stroke_ratio_valid_values.is_empty() {
        gwv.stroke_ratio_min = stroke_ratio_valid_values[0];
        gwv.stroke_ratio_max = stroke_ratio_valid_values[0];
        for value in stroke_ratio_valid_values.iter().skip(1) {
            gwv.stroke_ratio_min = gwv.stroke_ratio_min.min(*value); //.clone());
            gwv.stroke_ratio_max = gwv.stroke_ratio_max.max(*value); //.clone());
        }
        gwv.stroke_ratio_avg = stroke_ratio_valid_values.iter().sum::<f32>()
            / stroke_ratio_valid_values.iter().len() as f32;
    }
    if !heart_rate_valid_values.is_empty() {
        gwv.heart_rate_min = *heart_rate_valid_values.iter().min().unwrap(); //.clone();
        gwv.heart_rate_max = *heart_rate_valid_values.iter().max().unwrap(); //.clone();
        gwv.heart_rate_avg = heart_rate_valid_values.iter().sum::<u32>() as f32
            / heart_rate_valid_values.iter().len() as f32;
    }
}

pub fn write_meta_data_file(
    workout_dir: &PathBuf,
    gwv: &GlobalWorkoutValues,
) -> Result<(), Box<dyn std::error::Error>> {
    let csv_file = PathBuf::from(format!(
        "{}{}",
        &workout_dir.to_str().unwrap(),
        "/meta_data.csv",
    ));
    let mut csv_writer = csv::Writer::from_path(csv_file).unwrap();

    csv_writer.write_record(&["Date and Time of Start", &gwv.date_time_start])?;
    csv_writer.write_record(&["Date and Time of End", &gwv.date_time_end])?;
    csv_writer.write_record(&["WaterRower Model", &gwv.model])?;
    csv_writer.write_record(&["Firmware Version", &gwv.fw_version])?;
    csv_writer.write_record(&["Number of Data Points", &format!("{}", gwv.datapoints)])?;
    csv_writer.write_record(&[
        "Total Time in Seconds",
        &format!("{}", gwv.total_time_in_seconds),
    ])?;
    csv_writer.write_record(&[
        "Total Distance in Meters",
        &format!("{}", gwv.total_distance_in_meters),
    ])?;
    csv_writer.write_record(&["Total Stroke Count", &format!("{}", gwv.total_stroke_count)])?;
    csv_writer.write_record(&[
        "Seconds per 500 Meters (min)",
        &format!("{}", gwv.seconds_per_500m_min),
    ])?;
    csv_writer.write_record(&[
        "Seconds per 500 Meters (avg)",
        &format!("{:.2}", gwv.seconds_per_500m_avg),
    ])?;
    csv_writer.write_record(&[
        "Seconds per 500 Meters (max)",
        &format!("{}", gwv.seconds_per_500m_max),
    ])?;
    csv_writer.write_record(&[
        "Strokes per Minute (min)",
        &format!("{}", gwv.strokes_per_minute_min),
    ])?;
    csv_writer.write_record(&[
        "Strokes per Minute (avg)",
        &format!("{:.2}", gwv.strokes_per_minute_avg),
    ])?;
    csv_writer.write_record(&[
        "Strokes per Minute (max)",
        &format!("{}", gwv.strokes_per_minute_max),
    ])?;
    csv_writer.write_record(&[
        "Stroke Ratio (min)",
        &format!("{:.2}", gwv.stroke_ratio_min),
    ])?;
    csv_writer.write_record(&[
        "Stroke Ratio (avg)",
        &format!("{:.2}", gwv.stroke_ratio_avg),
    ])?;
    csv_writer.write_record(&[
        "Stroke Ratio (max)",
        &format!("{:.2}", gwv.stroke_ratio_max),
    ])?;
    csv_writer.write_record(&["Heart Rate (min)", &format!("{}", gwv.heart_rate_min)])?;
    csv_writer.write_record(&["Heart Rate (avg)", &format!("{:.2}", gwv.heart_rate_avg)])?;
    csv_writer.write_record(&["Heart Rate (max)", &format!("{}", gwv.heart_rate_max)])?;
    csv_writer.flush()?;
    Ok(())
}

pub fn write_workout_data_file(
    workout_dir: &PathBuf,
    datapoints: &[InstantWorkoutValues],
) -> Result<(), Box<dyn std::error::Error>> {
    let csv_file = PathBuf::from(format!(
        "{}{}",
        &workout_dir.to_str().unwrap(),
        "/workout_data.csv",
    ));
    let mut csv_writer = csv::Writer::from_path(csv_file).unwrap();

    let csv_header = [
        "Time in Seconds",
        "Distance in Meters",
        "Seconds per 500 Meters",
        "Stroke Count",
        "Strokes per Minute",
        "Stroke Ratio",
        "Heart Rate",
    ];
    csv_writer.write_record(&csv_header)?;
    for values in datapoints.iter() {
        let csv_row = [
            &format!("{}", values.time_in_seconds),
            &format!("{}", values.distance_in_meters),
            &format!("{}", values.seconds_per_500m),
            &format!("{}", values.stroke_count),
            &format!("{}", values.strokes_per_minute),
            &format!("{:.2}", values.stroke_ratio),
            &format!("{}", values.heart_rate),
        ];
        csv_writer.write_record(&csv_row)?;
    }
    csv_writer.flush()?;
    Ok(())
}
