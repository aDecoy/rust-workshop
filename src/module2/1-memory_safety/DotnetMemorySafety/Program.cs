// See https://aka.ms/new-console-template for more information
Console.WriteLine("Hello, World!");

var user = new User(){
    Name = "James"
};

var task1 = user.UpdateName("John");
var task2 = user.UpdateName("Doe");

await Task.WhenAll(task1, task2);

Console.WriteLine(user.Name);

class User {
    private bool isFirst = true;
    private static Random random = new Random();
    public string Name { get; set; }

    public async Task UpdateName(string newName) {
        await Task.Delay(isFirst ? 5000 : 1000);
        Name = newName;
    }
}