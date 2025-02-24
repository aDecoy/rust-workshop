using DotnetWebApi.Core;
using Microsoft.EntityFrameworkCore;

namespace DotnetWebApi.Adapters;

public class PostgresDataAccess(UsersContext context) : IDataAccess
{
    public async Task<User?> WithEmailAddress(string emailAddress)
    {
        var user = await context.Users.FirstOrDefaultAsync(user =>
            user.EmailAddress.ToLower( )== emailAddress.ToLower());

        return user;
    }

    public async Task Store(User user)
    {
        await context.Users.AddAsync(user);
        await context.SaveChangesAsync();
    }
}