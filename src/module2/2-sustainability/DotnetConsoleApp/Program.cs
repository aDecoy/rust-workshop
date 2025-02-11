using System;
using System.Collections.Generic;
using System.Threading.Tasks;

Console.WriteLine("Starting GC stress test...");
Console.WriteLine($"Gen 0 collections: {GC.CollectionCount(0)}");
Console.WriteLine($"Gen 1 collections: {GC.CollectionCount(1)}");
Console.WriteLine($"Gen 2 collections: {GC.CollectionCount(2)}");

// Run multiple tasks to create pressure
var tasks = new List<Task>();
for (int i = 0; i < Environment.ProcessorCount; i++)
{
    tasks.Add(Task.Run(AllocateMemory));
}

// Every second, print GC statistics
while (!Console.KeyAvailable)
{
    await Task.Delay(1000);
    Console.WriteLine("\nGC Collections:");
    Console.WriteLine($"Gen 0: {GC.CollectionCount(0)}");
    Console.WriteLine($"Gen 1: {GC.CollectionCount(1)}");
    Console.WriteLine($"Gen 2: {GC.CollectionCount(2)}");
    Console.WriteLine($"Total Memory: {GC.GetTotalMemory(false) / 1024 / 1024} MB");
}

static void AllocateMemory()
{
    var random = new Random();
    while (true)
    {
        // Create large byte arrays
        var largeArray = new byte[random.Next(1024 * 1024, 5 * 1024 * 1024)];

        // Create many small objects
        var objects = new List<object>();
        for (int i = 0; i < 1000; i++)
        {
            objects.Add(new string('x', random.Next(100, 1000)));
        }

        // Force some Gen 2 objects to be created
        if (random.Next(100) < 5)
        {
            GC.Collect(2, GCCollectionMode.Forced);
        }
    }
}
