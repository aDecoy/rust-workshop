using DotnetWebApi.Core;

namespace DotnetWebApi.UnitTest;

public class UserTests
{
    [Fact]
    public void WhenUserUpgraded_ShouldUpdateToPremiumUserAndCopyValues()
    {
        var user = new User("test@test.com", "James", "James!23");
        
        var premiumUser = user.UpgradeToPremium();
        
        Assert.True(premiumUser.GetType() == typeof(PremiumUser));
        Assert.Equal("James", premiumUser.Name);
        Assert.Equal("test@test.com", premiumUser.EmailAddress);
        Assert.Equal("James!23", premiumUser.Password);
    }
}