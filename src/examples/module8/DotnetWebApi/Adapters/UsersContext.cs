using DotnetWebApi.Core;
using Microsoft.EntityFrameworkCore;

namespace DotnetWebApi.Adapters;

public class UsersContext(DbContextOptions<UsersContext> options, IConfiguration configuration) : DbContext(options)
{
    public DbSet<User> Users { get; set; }
    
    protected override void OnConfiguring(DbContextOptionsBuilder optionsBuilder)
    {
        var connectionString = configuration.GetConnectionString("UsersContext");
        optionsBuilder.UseNpgsql(connectionString);
    }
    
    protected override void OnModelCreating(ModelBuilder modelBuilder)
    {
        
        modelBuilder.Entity<User>()
            .HasKey(item => item.EmailAddress);

        modelBuilder.Entity<User>()
            .ToTable("UserDetails");
    }
}