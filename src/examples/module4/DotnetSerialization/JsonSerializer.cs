using System.Text.Json.Serialization;

namespace DotnetSerialization;

[JsonSourceGenerationOptions(WriteIndented = true)]
[JsonSerializable(typeof(User))]
[JsonSerializable(typeof(PremiumUser))]
internal partial class CustomSerializationContext : JsonSerializerContext
{
}