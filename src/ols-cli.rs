mod effects;
mod hid;
mod state;

use effects::{EffectRunner, AVAILABLE_EFFECTS};
use hid::{HidController, LedColor};
use state::{load_state, save_state};
use std::env;
use std::process;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

struct Options {
    effect: String,
    brightness: Option<u8>,
    speed: f64,
    power_on: bool,
}

fn main() {
    env_logger::init();

    match run() {
        Ok(()) => {}
        Err(error) => {
            eprintln!("ols-cli: {error}");
            process::exit(1);
        }
    }
}

fn run() -> Result<(), String> {
    let options = match parse_args()? {
        Some(options) => options,
        None => return Ok(()),
    };

    let mut saved = load_state();
    let hid = Arc::new(HidController::new());

    if let Some(sections) = saved.sections {
        hid.set_total_leds(sections.iter().sum());
        *hid.sections.lock().unwrap() = sections;
    }

    if saved.r > 0 || saved.g > 0 || saved.b > 0 {
        hid.set_global_color(LedColor {
            r: saved.r,
            g: saved.g,
            b: saved.b,
        });
    }

    let brightness = options.brightness.or(saved.brightness);
    hid.open_device()?;

    if options.power_on {
        let color = hid.get_global_color();
        hid.power_on(1, color.r, color.g, color.b, brightness)?;
        saved.section = 1;
        saved.r = color.r;
        saved.g = color.g;
        saved.b = color.b;
    } else if let Some(value) = brightness {
        hid.set_brightness(value)?;
    }

    if let Some(value) = options.brightness {
        saved.brightness = Some(value);
    }
    if options.power_on || options.brightness.is_some() {
        save_state(&saved);
    }

    let effects = EffectRunner::new();
    effects.start(
        &options.effect,
        hid.clone(),
        hid.get_total_leds(),
        options.speed,
    )?;

    while effects.is_running() {
        thread::sleep(Duration::from_secs(1));
    }

    Ok(())
}

fn parse_args() -> Result<Option<Options>, String> {
    let mut args = env::args().skip(1);
    let mut effect = None;
    let mut brightness = None;
    let mut speed = 5.0;
    let mut power_on = false;

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "-h" | "--help" => {
                print_usage();
                return Ok(None);
            }
            "--effect" => {
                effect = Some(
                    args.next()
                        .ok_or_else(|| "--effect requires a name".to_string())?,
                );
            }
            "--brightness" => {
                let value = args
                    .next()
                    .ok_or_else(|| "--brightness requires a value from 0 to 255".to_string())?;
                brightness = Some(
                    value
                        .parse::<u8>()
                        .map_err(|_| "--brightness must be between 0 and 255".to_string())?,
                );
            }
            "--speed" => {
                let value = args
                    .next()
                    .ok_or_else(|| "--speed requires a value from 1 to 10".to_string())?;
                speed = value
                    .parse::<f64>()
                    .map_err(|_| "--speed must be a number from 1 to 10".to_string())?;
            }
            "-power_on" | "--power_on" | "--power-on" => power_on = true,
            _ => return Err(format!("unknown argument: {arg}\nTry --help for usage.")),
        }
    }

    let effect = effect.ok_or_else(|| {
        format!(
            "--effect is required. Available effects: {}",
            AVAILABLE_EFFECTS.join(", ")
        )
    })?;

    if !AVAILABLE_EFFECTS.contains(&effect.as_str()) && effect != "static" {
        return Err(format!(
            "unknown effect '{effect}'. Available effects: {}",
            AVAILABLE_EFFECTS.join(", ")
        ));
    }
    if !speed.is_finite() || !(1.0..=10.0).contains(&speed) {
        return Err("--speed must be between 1 and 10".to_string());
    }

    Ok(Some(Options {
        effect,
        brightness,
        speed,
        power_on,
    }))
}

fn print_usage() {
    println!(
        "Usage: ols-cli --effect <name> [--brightness <0-255>] [--speed <1-10>] [-power_on]\n\
         Available effects: {}",
        AVAILABLE_EFFECTS.join(", ")
    );
}
