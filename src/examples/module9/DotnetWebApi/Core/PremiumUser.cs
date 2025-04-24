namespace DotnetWebApi.Core;

public class PremiumUser : User {
    public PremiumUser(string emailAddress, string name, string password): base(emailAddress, name, password) {

    }

    public override void SayHello() {
        Console.WriteLine("Hello! I'm " + Name + ", I'm a premium user");
    }

    private bool isPremium = true;
}