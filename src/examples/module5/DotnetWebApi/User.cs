namespace DotnetWebApi;

public record RegisterUserRequest {
    public string EmailAddress { get; set; } = "";
    public string Name { get; set; } = "";
    public string Password {get;set;} = "";
}

public record LoginRequest {
    public string EmailAddress { get; set; } = "";
    public string Password {get;set;} = "";
}


internal class User {
    public string EmailAddress { get; private set; }
    public string Name { get; private set; }
    private string password;

    public User(string emailAddress, string name, string password) {
        EmailAddress = emailAddress;
        Name = name;
        this.password = password;
    }

    public void UpdateName(string newName) {
        Name = newName;
    }

    public virtual void SayHello() {
        Console.WriteLine("Hello! I'm " + Name + ", I'm a standard user");
    }

    public bool ValidatePassword(string password) {
        return this.password == password;
    }

    public PremiumUser UpgradeToPremium() {
        return new PremiumUser(this.EmailAddress, this.Name, this.password);
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