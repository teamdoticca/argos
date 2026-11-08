using System.Text.Json;
using System.Text.Json.Serialization;

namespace Argos;

/// <summary>
/// Typed open options serialized to the JSON accepted by <see cref="ArgosWorkspace.Open"/>.
/// Workspace mode remains the default when <see cref="Mode"/> is null.
/// </summary>
public sealed class OpenOptions
{
    /// <summary>"workspace" (default) or "filesystem".</summary>
    [JsonPropertyName("mode")]
    [JsonIgnore(Condition = JsonIgnoreCondition.WhenWritingNull)]
    public string? Mode { get; set; }

    /// <summary>"follow", "nofollow", or "followConfined".</summary>
    [JsonPropertyName("symlinkMode")]
    [JsonIgnore(Condition = JsonIgnoreCondition.WhenWritingNull)]
    public string? SymlinkMode { get; set; }

    [JsonPropertyName("include")]
    [JsonIgnore(Condition = JsonIgnoreCondition.WhenWritingNull)]
    public IReadOnlyList<string>? Include { get; set; }

    [JsonPropertyName("exclude")]
    [JsonIgnore(Condition = JsonIgnoreCondition.WhenWritingNull)]
    public IReadOnlyList<string>? Exclude { get; set; }

    [JsonPropertyName("eventBufferCapacity")]
    [JsonIgnore(Condition = JsonIgnoreCondition.WhenWritingNull)]
    public int? EventBufferCapacity { get; set; }

    public string ToJson() => JsonSerializer.Serialize(this);

    public static OpenOptions Filesystem(
        string? symlinkMode = "nofollow",
        IReadOnlyList<string>? include = null,
        IReadOnlyList<string>? exclude = null,
        int? eventBufferCapacity = null) =>
        new()
        {
            Mode = "filesystem",
            SymlinkMode = symlinkMode,
            Include = include,
            Exclude = exclude,
            EventBufferCapacity = eventBufferCapacity,
        };
}
