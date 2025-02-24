// See https://aka.ms/new-console-template for more information

using System.Text.Json;
using DotnetSerialization;

var user = new User("john@doe.com", "John Doe", "password");

Console.WriteLine(JsonSerializer.Serialize(user));
Console.WriteLine(JsonSerializer.Serialize(user, CustomSerializationContext.Default.User));

var premiumUser = user.UpgradeToPremium();

Console.WriteLine(JsonSerializer.Serialize(premiumUser));
Console.WriteLine(JsonSerializer.Serialize(premiumUser, CustomSerializationContext.Default.PremiumUser));

internal class User {
    public string EmailAddress { get; }
    public string Name { get; private set; }
    private string _password;

    public User(string emailAddress, string name, string password) {
        EmailAddress = emailAddress;
        Name = name;
        this._password = password;
    }

    public void UpdateName(string newName) {
        Name = newName;
    }

    public virtual void SayHello() {
        Console.WriteLine("Hello! I'm " + Name + ", I'm a standard user");
    }

    public bool ValidatePassword(string password) {
        return this._password == password;
    }

    public PremiumUser UpgradeToPremium() {
        return new PremiumUser(this.EmailAddress, this.Name, this._password);
    }
}

internal class PremiumUser : User {
    public PremiumUser(string emailAddress, string name, string password): base(emailAddress, name, password) {

    }

    public override void SayHello() {
        Console.WriteLine("Hello! I'm " + Name + ", I'm a premium user");
    }

    private bool isPremium = true;
}