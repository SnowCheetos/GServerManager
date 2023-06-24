use sysinfo::{System, SystemExt, ProcessorExt};
use std::thread;
use std::time::Duration;
use console::Term;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use ctrlc::set_handler;

pub fn monitor_system_info() {
    let term = Term::stdout();
    let mut sys = System::new_all();

    let running = Arc::new(AtomicBool::new(true));
    let running_clone = Arc::clone(&running);

    // Set the SIGINT signal handler
    set_handler(move || {
        running_clone.store(false, Ordering::Relaxed);
    })
    .expect("Failed to set SIGINT signal handler");

    while running.load(Ordering::Relaxed) {
        term.clear_screen().expect("Failed to clear the console screen");
        print_cpu_usage(&mut sys);
        print_memory_info(&mut sys);
        println!("\nPress Ctrl+C to exit...");
        std::thread::sleep(Duration::from_secs(1));
    }
}

fn print_memory_info(sys: &mut System) {
    sys.refresh_memory();
    
    let total_memory = sys.total_memory();
    let used_memory = sys.used_memory();
    let memory_usage = used_memory as f64 / total_memory as f64;
    let memory_bar = generate_display_bar(memory_usage, 20);  // 20 units wide bar

    println!("{:<15} {:6.2} % |{}|", "MEMORY USAGE:", memory_usage * 100.0, memory_bar);
}

fn print_cpu_usage(sys: &mut System) {
    sys.refresh_cpu();
    
    let processor_info = sys.processors();

    let mut total_cpu_usage = 0.0;

    for processor in processor_info {
        total_cpu_usage += processor.cpu_usage();
    }

    let average_cpu_usage = total_cpu_usage as f64 / (processor_info.len() as f64 * 100.0);
    let cpu_bar = generate_display_bar(average_cpu_usage, 20);

    println!("{:<15} {:6.2} % |{}|", "CPU USAGE:", average_cpu_usage * 100.0, cpu_bar);
}


// Generate a textual display bar given a value between 0 and 1 and the bar width
fn generate_display_bar(value: f64, bar_width: usize) -> String {
    assert!(0.0 <= value && value <= 1.0);

    let filled = (value * bar_width as f64).round() as usize;
    let unfilled = bar_width - filled;
    format!("{}{}", "=".repeat(filled), " ".repeat(unfilled))
}
