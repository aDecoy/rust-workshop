namespace DotnetWebApi.Core;

public record RegisterUserRequest {
    public string EmailAddress { get; set; } = "";
    public string Name { get; set; } = "";
    public string Password {get;set;} = "";
}