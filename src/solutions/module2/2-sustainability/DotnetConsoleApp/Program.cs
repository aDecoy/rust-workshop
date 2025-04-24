// .NET MEMORY USAGE DEMONSTRATION
//
// This example shows how .NET manages memory using the garbage collector.
// Run this alongside the Rust version to compare memory usage and efficiency.
//
// Key differences from Rust:
// 1. .NET uses garbage collection instead of deterministic memory management
// 2. Memory can grow significantly before collection occurs
// 3. GC pauses can impact performance during collection cycles

using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.Threading.Tasks;

// Track initial state
var initialMemory = GC.GetTotalMemory(false) / 1024 / 1024;
var process = Process.GetCurrentProcess();
var initialWorkingSet = process.WorkingSet64 / 1024 / 1024;

Console.WriteLine("------- .NET MEMORY SUSTAINABILITY DEMO -------");
Console.WriteLine("This demonstrates .NET's garbage collection behavior");
Console.WriteLine("Compare with the Rust version to see differences in memory efficiency");
Console.WriteLine();
Console.WriteLine($"Initial managed memory: {initialMemory} MB");
Console.WriteLine($"Initial working set: {initialWorkingSet} MB");
Console.WriteLine($"Initial Gen 0 collections: {GC.CollectionCount(0)}");
Console.WriteLine($"Initial Gen 1 collections: {GC.CollectionCount(1)}");
Console.WriteLine($"Initial Gen 2 collections: {GC.CollectionCount(2)}");
Console.WriteLine();
Console.WriteLine("Starting memory allocation test on all CPU cores...");
Console.WriteLine("Press any key to exit");

// Store some long-lived data to force GC pressure
var longLivedObjects = new List<byte[]>();

// Run multiple tasks to create memory pressure (one per CPU core)
var tasks = new List<Task>();
for (int i = 0; i < Environment.ProcessorCount; i++)
{
    tasks.Add(Task.Run(() => AllocateMemory(i)));
}

// Track statistics over time
var startTime = DateTime.Now;
var lastGen2Count = 0;

// Every second, print memory statistics
while (!Console.KeyAvailable)
{
    await Task.Delay(1000);
    
    // Calculate memory metrics
    var managedMemory = GC.GetTotalMemory(false) / 1024 / 1024;
    var workingSet = Process.GetCurrentProcess().WorkingSet64 / 1024 / 1024;
    var gen0Collections = GC.CollectionCount(0);
    var gen1Collections = GC.CollectionCount(1);
    var gen2Collections = GC.CollectionCount(2);
    var uptime = (DateTime.Now - startTime).TotalSeconds;
    
    // Calculate collection rates
    var gen0Rate = gen0Collections / uptime;
    var gen2Rate = gen2Collections / uptime;
    
    // Check if a Gen2 collection just happened
    var gen2Message = "";
    if (gen2Collections > lastGen2Count)
    {
        gen2Message = " <-- Full GC just occurred!";
        lastGen2Count = gen2Collections;
    }
    
    Console.WriteLine("\n-------------------------------------");
    Console.WriteLine($"Uptime: {uptime:F1} seconds");
    Console.WriteLine("\nMemory Usage:");
    Console.WriteLine($"Managed Memory: {managedMemory} MB");
    Console.WriteLine($"Working Set: {workingSet} MB");
    Console.WriteLine($"Memory Growth: {workingSet - initialWorkingSet} MB since start");
    
    Console.WriteLine("\nGarbage Collections:");
    Console.WriteLine($"Gen 0 (young objects): {gen0Collections} ({gen0Rate:F1}/second)");
    Console.WriteLine($"Gen 1 (middle-aged): {gen1Collections}");
    Console.WriteLine($"Gen 2 (old objects): {gen2Collections} ({gen2Rate:F2}/second){gen2Message}");
    
    // Every 10 seconds, create long-lived objects that survive collections
    if (uptime % 10 < 1)
    {
        var largeObject = new byte[50 * 1024 * 1024]; // 50 MB
        Array.Fill<byte>(largeObject, 1);
        longLivedObjects.Add(largeObject);
        Console.WriteLine("\n*** Added 50 MB long-lived object to increase memory pressure ***");
    }
}

// Cleanup
Console.WriteLine("\nExiting program and cleaning up...");

// Method to continuously allocate memory
static void AllocateMemory(int taskId)
{
    var random = new Random(taskId);
    int counter = 0;
    
    while (true)
    {
        counter++;
        
        // Create large array (this will likely go to the Large Object Heap in .NET)
        var size = random.Next(1024 * 1024, 5 * 1024 * 1024);
        var largeArray = new byte[size];
        
        // Create many small string objects
        var objects = new List<object>(1000);
        for (int i = 0; i < 1000; i++)
        {
            objects.Add(new string('x', random.Next(100, 1000)));
        }
        
        // Every so often, create memory pressure to force collections
        if (counter % 20 == 0)
        {
            var tempLargeArrays = new List<byte[]>();
            for (int i = 0; i < 5; i++)
            {
                tempLargeArrays.Add(new byte[10 * 1024 * 1024]); // 10 MB each
            }
        }
        
        // Allow other tasks to run
        Task.Delay(50).Wait();
    }
}
