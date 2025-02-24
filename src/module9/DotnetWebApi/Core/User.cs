namespace DotnetWebApi.Core;

public class User {
    public string Password { get; private set; }
    public string EmailAddress { get; private set; }
    public string Name { get; private set; }
    
    internal User(){}

    public User(string emailAddress, string name, string password) {
        EmailAddress = emailAddress;
        Name = name;
        Password = password;
    }

    public void UpdateName(string newName) {
        Name = newName;
    }

    public virtual void SayHello() {
        Console.WriteLine("Hello! I'm " + Name + ", I'm a standard user");
    }

    public bool ValidatePassword(string password) {
        return this.Password == password;
    }

    public PremiumUser UpgradeToPremium() {
        return new PremiumUser(this.EmailAddress, this.Name, this.Password);
    }
}