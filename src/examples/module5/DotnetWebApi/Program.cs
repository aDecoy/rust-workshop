using DotnetWebApi;

var builder = WebApplication.CreateBuilder(args);

var app = builder.Build();

var users = new List<User>() {
    new User("john@doe.com", "John Doe", "John!23")
};

Handlers.users = users;

app.MapGet("/users/{emailAddress}", Handlers.GetUserDetails).WithName("GetUserDetails");
app.MapPost("/users", Handlers.RegisterUser).WithName("RegisterUser");
app.MapPost("/login", Handlers.Login).WithName("Login");

app.Run();


static class Handlers
{
    internal static List<User> users;
    
    internal static IResult RegisterUser(RegisterUserRequest registerUser)
    {
        var user = users.FirstOrDefault(u => u.EmailAddress == registerUser.EmailAddress);
        if (user != null)
        {
            return Results.BadRequest();
        }

        var newUser = new User(registerUser.EmailAddress, registerUser.Name, registerUser.Password);
        users.Add(newUser);
        return Results.Created($"/users/{newUser.EmailAddress}", newUser);
    }
    
    internal static IResult GetUserDetails(string emailAddress)
    {
        var user = users.FirstOrDefault(u => u.EmailAddress == emailAddress);
        if (user == null)
        {
            return Results.NotFound();
        }
        return Results.Ok(user);
    }

    internal static IResult Login(LoginRequest loginRequest)
    {
        var user = users.FirstOrDefault(u => u.EmailAddress == loginRequest.EmailAddress);
        if (user == null)
        {
            return Results.NotFound();
        }

        var passwordValid = user.ValidatePassword(loginRequest.Password);

        if (!passwordValid)
        {
            return Results.Unauthorized();
        }

        return Results.Ok();
    }
}