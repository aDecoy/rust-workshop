// See https://aka.ms/new-console-template for more information
var stringExample = "Hello";
var integerExample = 10;
var floatExample = 10.5f;
var arrayExample = new string[1] {"Hello"};
var listExample = new List<string>();
var boolExample = true;

var user = new User("James");
user.SayHello();

user.UpdateName("John");

user.SayHello();

var premiumUser = user.UpgradeToPremium();

user.SayHello();
premiumUser.SayHello();

class User {
    public string Name { get; set; }

    public User(string name) {
        Name = name;
    }

    public void UpdateName(string newName) {
        Name = newName;
    }

    public virtual void SayHello() {
        Console.WriteLine("Hello! I'm " + Name + ", I'm a standard user");
    }

    public PremiumUser UpgradeToPremium() {
        return new PremiumUser(this.Name);
    }
}

class PremiumUser : User {
    public PremiumUser(string name): base(name) {

    }

    public override void SayHello() {
        Console.WriteLine("Hello! I'm " + Name + ", I'm a premium user");
    }

    private bool isPremium = true;
}