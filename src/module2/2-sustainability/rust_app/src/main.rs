use std::{thread, time::Duration};
use rand::Rng;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::process;

struct MemoryStats {
    allocated: Arc<AtomicUsize>,
    deallocated: Arc<AtomicUsize>,
}

fn main() {
    println!("Starting memory stress test...");
    
    let stats = MemoryStats {
        allocated: Arc::new(AtomicUsize::new(0)),
        deallocated: Arc::new(AtomicUsize::new(0)),
    };

    // Spawn threads based on CPU count
    let num_cpus = num_cpus::get();
    let mut handles = vec![];

    for _ in 0..num_cpus {
        let allocated = Arc::clone(&stats.allocated);
        let deallocated = Arc::clone(&stats.deallocated);

        handles.push(thread::spawn(move || {
            allocate_memory(allocated, deallocated);
        }));
    }

    // Print statistics every second
    loop {
        thread::sleep(Duration::from_secs(1));
        println!("\nMemory Statistics:");
        println!("Total Allocated: {} MB (This is the cumulative sum of all allocations)", stats.allocated.load(Ordering::Relaxed) / (1024 * 1024));
        println!("Total Deallocated: {} MB  (This is the cumulative sum of all freed memory)", stats.deallocated.load(Ordering::Relaxed) / (1024 * 1024));
        println!("Current RSS: {} MB (Actual physical memory currently consumed)", get_process_memory_mb());

        // Check if user pressed Ctrl+C
        if is_ctrl_c_pressed() {
            break;
        }
    }
}

fn allocate_memory(allocated: Arc<AtomicUsize>, deallocated: Arc<AtomicUsize>) {
    let mut rng = rand::thread_rng();

    loop {
        // Allocate large vectors (similar to byte arrays in .NET)
        let size = rng.gen_range(1024 * 1024..5 * 1024 * 1024);
        let _large_vec = vec![0u8; size];
        allocated.fetch_add(size, Ordering::Relaxed);

        // Create multiple smaller allocations
        for _ in 0..1000 {
            let small_size = rng.gen_range(100..1000);
            let _small_vec = vec!['x'; small_size];
            allocated.fetch_add(small_size, Ordering::Relaxed);
        }

        // Variables go out of scope here and memory is automatically freed
        deallocated.fetch_add(size + 1000 * rng.gen_range(100..1000), Ordering::Relaxed);
    }
}

fn get_process_memory_mb() -> usize {
    // This is a simple way to get process memory on Unix systems
    if let Ok(stat) = std::fs::read_to_string("/proc/self/statm") {
        if let Some(rss) = stat.split_whitespace().nth(1) {
            if let Ok(pages) = rss.parse::<usize>() {
                return pages * 4096 / 1024 / 1024; // Convert pages to MB
            }
        }
    }
    0
}

fn is_ctrl_c_pressed() -> bool {
    // This is a simplified check - in a real application, you'd want to use proper signal handling
    false
}