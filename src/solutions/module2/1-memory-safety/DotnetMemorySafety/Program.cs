// This example demonstrates how .NET allows data races at runtime
// Unlike Rust, C# doesn't prevent concurrent mutable access at compile time

var user = new User(){
    Name = "James",
    UpdateCount = 0
};

Console.WriteLine($"Starting with Name: {user.Name}, UpdateCount: {user.UpdateCount}");

// Start two concurrent tasks that both modify the same object
// This creates a race condition - the final values are unpredictable
var task1 = Task.Run(() => user.UpdateName("John"));
var task2 = Task.Run(() => user.UpdateName("Doe"));

// Wait for both tasks to complete
await Task.WhenAll(task1, task2);

// The final values are unpredictable due to the race condition
// Different runs may produce different results
Console.WriteLine($"Final Name: {user.Name}, UpdateCount: {user.UpdateCount}");
Console.WriteLine("NOTE: The UpdateCount should be 2 if both updates occurred properly");
Console.WriteLine("      If you see a different value, you've witnessed a race condition!");

class User {
    private static Random random = new Random();
    public string Name { get; set; }
    
    // This counter should be incremented exactly once per update
    // In a race condition, increments may be lost
    public int UpdateCount { get; set; }

    public async Task UpdateName(string newName) {
        // Simulate some processing time
        await Task.Delay(50);
        
        // DANGER: These operations are not atomic!
        // Another thread may be executing this code simultaneously
        var currentCount = UpdateCount;
        
        // Introduce artificial delay to make race condition more likely
        await Task.Delay(100);
        
        Name = newName;
        UpdateCount = currentCount + 1; // This may overwrite another thread's increment
        
        Console.WriteLine($"Updated to {newName}, count is now {UpdateCount}");
    }
}