namespace DotnetWebApi.Core;

public record LoginRequest {
    public string EmailAddress { get; set; } = "";
    public string Password {get;set;} = "";
}