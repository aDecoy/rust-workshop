// See https://aka.ms/new-console-template for more information
var stringExample = "Hello";
var integerExample = 10;
var floatExample = 10.5f;
var arrayExample = new string[1] {"Hello"};
var listExample = new List<string>();
var boolExample = true;

var user = new User("john@doe.com", "John Doe", "John!23");
user.SayHello();

user.UpdateName("John");

user.SayHello();

var premiumUser = user.UpgradeToPremium();

user.SayHello();
premiumUser.SayHello();

switch (user.UserType) {
    case UserType.Standard:
        Console.WriteLine("Standard user");
        break;
}

enum UserType
{
    Standard,
    Premium,
    Freemium
}

internal class User {
    private string _password;

    public User(string emailAddress, string name, string password) {
        EmailAddress = emailAddress;
        Name = name;
        this._password = password;
        UserType = UserType.Standard;
    }
    
    public string EmailAddress { get; }
    public string Name { get; private set; }
    
    public int? Age { get; private set; }
    
    public UserType UserType { get; internal set; }

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

internal class FreemiumUser : User
{
    public FreemiumUser(string emailAddress, string name, string password): base(emailAddress, name, password)
    {
        this.UserType = UserType.Freemium;
    }

    public override void SayHello() {
        Console.WriteLine("Hello! I'm " + Name + ", I'm a free user");
    }
}

internal class PremiumUser : User {
    public PremiumUser(string emailAddress, string name, string password): base(emailAddress, name, password) {
        this.UserType = UserType.Premium;
    }

    public override void SayHello() {
        Console.WriteLine("Hello! I'm " + Name + ", I'm a premium user");
    }

    private bool isPremium = true;
}