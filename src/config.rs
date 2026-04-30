use std::env;
use std::fmt;
use std::time::Duration;

pub const DEFAULT_SIZE: usize = 24;
pub const DEFAULT_DELAY_MS: u64 = 60;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Algorithm {
    Bubble,
    Selection,
    Insertion,
    Quick,
}

impl Algorithm {
    pub fn all() -> &'static [Algorithm] {
        &[
            Algorithm::Bubble,
            Algorithm::Selection,
            Algorithm::Insertion,
            Algorithm::Quick,
        ]
    }

    pub fn parse(value: &str) -> Option<Self> {
        match value.trim().to_lowercase().as_str() {
            "bubble" | "bubblesort" | "1" => Some(Self::Bubble),
            "selection" | "selectionsort" | "2" => Some(Self::Selection),
            "insertion" | "insertionsort" | "3" => Some(Self::Insertion),
            "quick" | "quicksort" | "4" => Some(Self::Quick),
            _ => None,
        }
    }
}

impl fmt::Display for Algorithm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Algorithm::Bubble => "Bubble Sort",
            Algorithm::Selection => "Selection Sort",
            Algorithm::Insertion => "Insertion Sort",
            Algorithm::Quick => "Quick Sort",
        };
        write!(f, "{name}")
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DataMode {
    Random,
    Reversed,
    NearlySorted,
}

impl DataMode {
    pub fn all() -> &'static [DataMode] {
        &[DataMode::Random, DataMode::Reversed, DataMode::NearlySorted]
    }

    pub fn parse(value: &str) -> Option<Self> {
        match value.trim().to_lowercase().as_str() {
            "random" | "zufall" | "r" => Some(Self::Random),
            "reversed" | "reverse" | "absteigend" | "a" => Some(Self::Reversed),
            "nearly" | "nearly-sorted" | "fast" | "f" => Some(Self::NearlySorted),
            _ => None,
        }
    }
}

impl fmt::Display for DataMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            DataMode::Random => "zufaellig",
            DataMode::Reversed => "absteigend",
            DataMode::NearlySorted => "fast sortiert",
        };
        write!(f, "{name}")
    }
}

#[derive(Clone, Debug)]
pub struct Config {
    pub algorithm: Algorithm,
    pub data_mode: DataMode,
    pub size: usize,
    pub delay: Duration,
}

pub fn read_config() -> Result<Config, String> {
    let raw_args: Vec<String> = env::args().skip(1).collect();
    read_config_from(raw_args)
}

fn read_config_from(raw_args: Vec<String>) -> Result<Config, String> {
    if matches!(raw_args.first().map(String::as_str), Some("--help" | "-h")) {
        print_usage();
        std::process::exit(0);
    }

    let mut args = raw_args.into_iter();
    let algorithm = match args.next() {
        Some(value) => {
            Algorithm::parse(&value).ok_or_else(|| format!("Unbekannter Algorithmus: {value}"))?
        }
        None => Algorithm::Bubble,
    };

    let mut data_mode = DataMode::Random;
    let mut size = DEFAULT_SIZE;
    let mut delay_ms = DEFAULT_DELAY_MS;

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--mode" | "-m" => {
                let value = args
                    .next()
                    .ok_or_else(|| "Nach --mode fehlt ein Wert.".to_string())?;
                data_mode =
                    DataMode::parse(&value).ok_or_else(|| format!("Unbekannter Modus: {value}"))?;
            }
            "--size" | "-s" => {
                let value = args
                    .next()
                    .ok_or_else(|| "Nach --size fehlt eine Zahl.".to_string())?;
                size = parse_range(&value, 5, 60, "--size")?;
            }
            "--delay" | "-d" => {
                let value = args
                    .next()
                    .ok_or_else(|| "Nach --delay fehlt eine Zahl.".to_string())?;
                delay_ms = parse_range(&value, 0, 2_000, "--delay")? as u64;
            }
            other => return Err(format!("Unbekanntes Argument: {other}")),
        }
    }

    Ok(Config {
        algorithm,
        data_mode,
        size,
        delay: Duration::from_millis(delay_ms),
    })
}

fn parse_range(value: &str, min: usize, max: usize, name: &str) -> Result<usize, String> {
    let parsed = value
        .parse::<usize>()
        .map_err(|_| format!("{name} muss eine Zahl sein."))?;

    if (min..=max).contains(&parsed) {
        Ok(parsed)
    } else {
        Err(format!("{name} muss zwischen {min} und {max} liegen."))
    }
}

pub fn print_usage() {
    println!(
        "Sortieralgorithmen in der Konsole\n\n\
         Nutzung:\n  cargo run -- <algorithmus> [optionen]\n\n\
         Algorithmen:\n  bubble | selection | insertion | quick\n\n\
         Optionen:\n  -s, --size <5..60>       Anzahl der Werte (Standard: {DEFAULT_SIZE})\n  \
         -d, --delay <0..2000>    Pause pro Schritt in ms (Standard: {DEFAULT_DELAY_MS})\n  \
         -m, --mode <modus>       random | reversed | nearly (Standard: random)\n\n\
         Beispiele:\n  cargo run -- bubble -s 20 -d 80\n  cargo run -- quick --mode reversed --delay 30"
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_algorithms() {
        assert!(matches!(
            Algorithm::parse("bubble"),
            Some(Algorithm::Bubble)
        ));
        assert!(matches!(Algorithm::parse("4"), Some(Algorithm::Quick)));
        assert!(Algorithm::parse("merge").is_none());
    }

    #[test]
    fn parses_data_modes() {
        assert!(matches!(DataMode::parse("random"), Some(DataMode::Random)));
        assert!(matches!(
            DataMode::parse("absteigend"),
            Some(DataMode::Reversed)
        ));
        assert!(matches!(
            DataMode::parse("nearly-sorted"),
            Some(DataMode::NearlySorted)
        ));
        assert!(DataMode::parse("sorted").is_none());
    }

    #[test]
    fn parses_cli_options() {
        let config = read_config_from(vec![
            "quick".to_string(),
            "--size".to_string(),
            "12".to_string(),
            "--delay".to_string(),
            "25".to_string(),
            "--mode".to_string(),
            "reversed".to_string(),
        ])
        .expect("config should parse");

        assert!(matches!(config.algorithm, Algorithm::Quick));
        assert!(matches!(config.data_mode, DataMode::Reversed));
        assert_eq!(config.size, 12);
        assert_eq!(config.delay, Duration::from_millis(25));
    }

    #[test]
    fn rejects_cli_values_outside_allowed_ranges() {
        assert!(
            read_config_from(vec![
                "bubble".to_string(),
                "--size".to_string(),
                "4".to_string()
            ])
            .is_err()
        );
        assert!(
            read_config_from(vec![
                "bubble".to_string(),
                "--delay".to_string(),
                "2001".to_string()
            ])
            .is_err()
        );
    }
}
