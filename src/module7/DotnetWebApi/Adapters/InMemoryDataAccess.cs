using DotnetWebApi.Core;

namespace DotnetWebApi.Adapters;

public class InMemoryDataAccess : IDataAccess
{
    private List<User> _users = new List<User>() {
        new User("john@doe.com", "John Doe", "John!23")
    };
    
    public Task<User?> WithEmailAddress(string emailAddress)
    {
        return Task.FromResult(_users.FirstOrDefault(u => u.EmailAddress == emailAddress));
    }

    public Task Store(User user)
    {
        _users.Append(user);

        return Task.CompletedTask;
    }
}