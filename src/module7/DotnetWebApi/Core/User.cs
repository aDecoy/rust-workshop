namespace DotnetWebApi.Core;

public class User {
    private readonly string _password;
    
    public string EmailAddress { get; private set; }
    public string Name { get; private set; }

    public User(string emailAddress, string name, string password) {
        EmailAddress = emailAddress;
        Name = name;
        _password = password;
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