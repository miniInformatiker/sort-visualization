use sort_visualization::{config, tui};

fn main() {
    match config::read_config() {
        Ok(config) => {
            if let Err(error) = tui::run(config) {
                eprintln!("TUI konnte nicht gestartet werden: {error}");
                std::process::exit(1);
            }
        }
        Err(message) => {
            eprintln!("{message}");
            config::print_usage();
            std::process::exit(1);
        }
    }
}
