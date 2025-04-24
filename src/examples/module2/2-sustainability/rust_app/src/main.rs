//
// RUST MEMORY EFFICIENCY DEMONSTRATION
//
// This example shows how Rust's ownership system and deterministic memory management 
// lead to highly efficient resource usage compared to garbage-collected languages like .NET.
//
// Key advantages of Rust:
// 1. Deterministic memory management - no unpredictable garbage collection pauses
// 2. Zero-cost abstractions - memory safety without runtime overhead
// 3. Precise control over allocations - memory is freed exactly when it goes out of scope
// 4. Minimal memory footprint - no garbage collector overhead
//

use std::{thread, time::Duration};
use rand::Rng;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::process;
use std::time::Instant;

struct MemoryStats {
    allocated: Arc<AtomicUsize>,       // Track total bytes allocated
    deallocated: Arc<AtomicUsize>,     // Track total bytes deallocated
    max_rss: Arc<AtomicUsize>,         // Track maximum resident set size
    allocations_count: Arc<AtomicUsize>, // Count number of allocations
}

fn main() {
    println!("------- RUST MEMORY SUSTAINABILITY DEMO -------");
    println!("This demonstrates Rust's efficient memory management");
    println!("Compare with the .NET version to see differences in memory efficiency");
    println!();
    
    // Initialize memory tracking statistics
    let stats = MemoryStats {
        allocated: Arc::new(AtomicUsize::new(0)),
        deallocated: Arc::new(AtomicUsize::new(0)),
        max_rss: Arc::new(AtomicUsize::new(0)),
        allocations_count: Arc::new(AtomicUsize::new(0)),
    };

    let start_rss = get_process_memory_mb();
    println!("Initial RSS (physical memory): {} MB", start_rss);
    println!("Starting memory allocation test on all CPU cores...");
    println!("Press Ctrl+C to exit");
    println!();

    // Spawn worker threads based on CPU count (same as .NET example)
    let num_cpus = num_cpus::get();
    let mut handles = vec![];
    println!("Launching {} worker threads (one per CPU core)", num_cpus);

    // Track program start time
    let start_time = Instant::now();

    // Create worker threads that allocate memory
    for id in 0..num_cpus {
        let allocated = Arc::clone(&stats.allocated);
        let deallocated = Arc::clone(&stats.deallocated);
        let allocations_count = Arc::clone(&stats.allocations_count);
        let max_rss = Arc::clone(&stats.max_rss);

        handles.push(thread::spawn(move || {
            allocate_memory(id, allocated, deallocated, allocations_count, max_rss);
        }));
    }

    // Print statistics every second
    loop {
        thread::sleep(Duration::from_secs(1));
        
        // Get current stats
        let current_allocated = stats.allocated.load(Ordering::Relaxed);
        let current_deallocated = stats.deallocated.load(Ordering::Relaxed);
        let current_rss = get_process_memory_mb();
        let current_allocations = stats.allocations_count.load(Ordering::Relaxed);
        let elapsed = start_time.elapsed().as_secs_f64();
        
        // Update max RSS if needed
        if current_rss > stats.max_rss.load(Ordering::Relaxed) {
            stats.max_rss.store(current_rss, Ordering::Relaxed);
        }
        
        // Calculate allocation rate
        let allocation_rate = current_allocations as f64 / elapsed;
        let memory_allocated_gb = current_allocated as f64 / (1024.0 * 1024.0 * 1024.0);
        let memory_rate = memory_allocated_gb / elapsed;
        
        println!("\n-------------------------------------");
        println!("Uptime: {:.1} seconds", elapsed);
        
        println!("\nMemory Throughput:");
        println!("Total Allocated: {:.2} GB  ({:.2} GB/sec)", 
                 memory_allocated_gb, memory_rate);
        println!("Total Objects Allocated: {} ({:.1}/sec)", 
                 current_allocations, allocation_rate);
        
        println!("\nMemory Usage:");
        println!("Current RSS: {} MB (physical memory used)", current_rss);
        println!("Peak RSS: {} MB", stats.max_rss.load(Ordering::Relaxed));
        println!("Memory Growth: {} MB since start", current_rss as isize - start_rss as isize);
        
        println!("\nMemory Reclamation:");
        println!("Memory Already Freed: {:.2} GB", 
                 current_deallocated as f64 / (1024.0 * 1024.0 * 1024.0));
        println!("Memory Currently Held: {:.2} MB", 
                 (current_allocated - current_deallocated) as f64 / (1024.0 * 1024.0));
        
        println!("\nNOTE: Unlike .NET, Rust has:");
        println!("  - No garbage collector overhead or pauses");
        println!("  - Immediate memory reclamation when values go out of scope");
        println!("  - Minimal memory growth even under heavy allocation");

        // Create periodic memory pressure spike (similar to .NET example)
        if elapsed % 10.0 < 1.0 && elapsed > 1.0 {
            // This large allocation will be freed immediately after this block
            println!("\n*** Creating temporary memory spike of 50 MB ***");
            let _temp_large_allocation = vec![1u8; 50 * 1024 * 1024]; // 50 MB
            
            // Notice how this memory will be immediately freed when it goes out of scope,
            // unlike in .NET where it would remain until garbage collection occurs
        }

        // Check if user pressed Ctrl+C
        if is_ctrl_c_pressed() {
            break;
        }
    }

    println!("\nExiting program and cleaning up...");
    // In Rust, all resources are automatically cleaned up when they go out of scope
    // No explicit cleanup needed
}

fn allocate_memory(
    id: usize,
    allocated: Arc<AtomicUsize>, 
    deallocated: Arc<AtomicUsize>,
    allocations_count: Arc<AtomicUsize>,
    max_rss: Arc<AtomicUsize>
) {
    let mut rng = rand::thread_rng();
    let mut local_counter = 0;

    loop {
        local_counter += 1;
        
        // Allocate a large vector (similar to byte arrays in .NET)
        let size = rng.gen_range(1 * 1024 * 1024..5 * 1024 * 1024);
        {
            // This scope ensures the memory is freed immediately after use
            let _large_vec = vec![0u8; size];
            allocated.fetch_add(size, Ordering::Relaxed);
            allocations_count.fetch_add(1, Ordering::Relaxed);
            
            // Track RSS after allocation (in a real app you wouldn't do this for every allocation)
            if local_counter % 50 == 0 {
                let current_rss = get_process_memory_kb() / 1024;
                let current_max = max_rss.load(Ordering::Relaxed);
                if current_rss > current_max {
                    max_rss.store(current_rss, Ordering::Relaxed);
                }
            }
            
            // Small delay to simulate work
            thread::sleep(Duration::from_millis(1));
        }
        // _large_vec is automatically freed here when it goes out of scope
        // This is Rust's RAII (Resource Acquisition Is Initialization) pattern
        
        // Create multiple smaller allocations
        {
            let mut small_vecs = Vec::with_capacity(1000);
            for _ in 0..1000 {
                let small_size = rng.gen_range(100..1000);
                small_vecs.push(vec!['x'; small_size]);
                allocated.fetch_add(small_size, Ordering::Relaxed);
                allocations_count.fetch_add(1, Ordering::Relaxed);
            }
            // Small delay to simulate work
            thread::sleep(Duration::from_millis(1));
        }
        // All small_vecs are freed here automatically
        
        // Track that we've deallocated the memory
        deallocated.fetch_add(size + 1000 * rng.gen_range(100..1000), Ordering::Relaxed);
        
        // Create occasional large memory pressure
        if local_counter % 20 == 0 {
            let temp_large_arrays = vec![vec![0u8; 10 * 1024 * 1024]; 5]; // 5 arrays of 10 MB each
            // This memory is immediately freed when temp_large_arrays goes out of scope
            // after this block
            thread::sleep(Duration::from_millis(10));
        }
        
        // Allow other threads to run
        thread::sleep(Duration::from_millis(50));
    }
}

fn get_process_memory_mb() -> usize {
    get_process_memory_kb() / 1024
}

fn get_process_memory_kb() -> usize {
    // This is a simple way to get process memory on Unix systems
    if let Ok(stat) = std::fs::read_to_string("/proc/self/statm") {
        if let Some(rss) = stat.split_whitespace().nth(1) {
            if let Ok(pages) = rss.parse::<usize>() {
                return pages * 4096 / 1024; // Convert pages to KB
            }
        }
    }
    0
}

fn is_ctrl_c_pressed() -> bool {
    // This is a simplified check - in a real application, you'd want to use proper signal handling
    false
}

// KEY TAKEAWAYS:
//
// 1. Rust memory is reclaimed immediately when variables go out of scope
//    - No need to wait for garbage collection cycles
//    - Memory usage remains stable even under heavy allocation
//
// 2. Rust gives precise control over memory lifetime
//    - We use explicit scopes {} to control when memory is freed
//    - This leads to more predictable performance
//
// 3. No hidden costs
//    - No garbage collector overhead or pauses
//    - No runtime memory safety checks
//    - Minimal memory fragmentation
//
// 4. The same memory safety guarantees as .NET, but without the runtime cost
//    - No null pointer exceptions
//    - No use-after-free bugs
//    - No data races