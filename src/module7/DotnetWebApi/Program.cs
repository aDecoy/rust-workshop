using DotnetWebApi;
using DotnetWebApi.Adapters;
using DotnetWebApi.Core;
using Microsoft.AspNetCore.Mvc;

var builder = WebApplication.CreateBuilder(args);

builder.Services.AddSingleton<IDataAccess, InMemoryDataAccess>();

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
    
    internal static async Task<IResult> RegisterUser([FromServices] IDataAccess dataAccess, RegisterUserRequest registerUser)
    {
        var user = await dataAccess.WithEmailAddress(registerUser.EmailAddress);
        
        if (user != null)
        {
            return Results.BadRequest();
        }

        var newUser = new User(registerUser.EmailAddress, registerUser.Name, registerUser.Password);
        users.Add(newUser);
        return Results.Created($"/users/{newUser.EmailAddress}", newUser);
    }
    
    internal static async Task<IResult> GetUserDetails([FromServices] IDataAccess dataAccess, string emailAddress)
    {
        var user = await dataAccess.WithEmailAddress(emailAddress);
        if (user == null)
        {
            return Results.NotFound();
        }
        return Results.Ok(user);
    }

    internal static async Task<IResult> Login([FromServices] IDataAccess dataAccess, LoginRequest loginRequest)
    {
        var user = await dataAccess.WithEmailAddress(loginRequest.EmailAddress);
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