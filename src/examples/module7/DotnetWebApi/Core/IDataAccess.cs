namespace DotnetWebApi.Core;

public interface IDataAccess
{
    Task<User> WithEmailAddress(string emailAddress);
    Task Store(User user);
}