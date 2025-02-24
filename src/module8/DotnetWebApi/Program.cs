using DotnetWebApi;
using DotnetWebApi.Adapters;
using DotnetWebApi.Core;
using Microsoft.AspNetCore.Mvc;
using Microsoft.EntityFrameworkCore;

var builder = WebApplication.CreateBuilder(args);

builder.Services.AddDbContext<UsersContext>(opt =>
{
    opt.UseNpgsql(builder.Configuration.GetConnectionString("UsersContext"));
});
builder.Services.AddScoped<IDataAccess, PostgresDataAccess>();

var app = builder.Build();

app.MapGet("/users/{emailAddress}", Handlers.GetUserDetails).WithName("GetUserDetails");
app.MapPost("/users", Handlers.RegisterUser).WithName("RegisterUser");
app.MapPost("/login", Handlers.Login).WithName("Login");

using (var scoped = app.Services.CreateScope())
{
    var context = scoped.ServiceProvider.GetRequiredService<UsersContext>();
    await context.Database.MigrateAsync();
}

app.Run();

static class Handlers
{
    internal static async Task<IResult> RegisterUser([FromServices] IDataAccess dataAccess, RegisterUserRequest registerUser)
    {
        var user = await dataAccess.WithEmailAddress(registerUser.EmailAddress);
        
        if (user != null)
        {
            return Results.BadRequest();
        }

        var newUser = new User(registerUser.EmailAddress, registerUser.Name, registerUser.Password);
        await dataAccess.Store(newUser);
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