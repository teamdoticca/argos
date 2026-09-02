using System.Text.Json;
using Argos;

static int Fail(string message)
{
    Console.Error.WriteLine("SMOKE FAIL: " + message);
    return 1;
}

static string? FindFixtureFrom(string start)
{
    var dir = new DirectoryInfo(start);
    while (dir is not null)
    {
        var candidate = Path.Combine(dir.FullName, "fixtures", "small-pnpm");
        if (Directory.Exists(candidate))
        {
            return candidate;
        }
        dir = dir.Parent;
    }
    return null;
}

var root = args.Length > 0
    ? Path.GetFullPath(args[0])
    : FindFixtureFrom(Directory.GetCurrentDirectory())
        ?? FindFixtureFrom(AppContext.BaseDirectory)
        ?? "";

if (string.IsNullOrWhiteSpace(root) || !Directory.Exists(root))
{
    return Fail($"fixture path does not exist: '{root}'. Pass fixtures/small-pnpm explicitly.");
}

Console.WriteLine($"SMOKE OpenAsync({root})");

await using var workspace = await ArgosWorkspace.OpenAsync(root);
var snapshotJson = workspace.CurrentSnapshot;
if (string.IsNullOrWhiteSpace(snapshotJson))
{
    return Fail("CurrentSnapshot empty");
}

using var doc = JsonDocument.Parse(snapshotJson);
var rootEl = doc.RootElement;
foreach (var section in new[] { "identity", "model", "filesystem", "planning" })
{
    if (!rootEl.TryGetProperty(section, out _))
    {
        return Fail($"snapshot missing section '{section}'");
    }
}

var scopesJson = workspace.ListScopes();
using var scopesDoc = JsonDocument.Parse(scopesJson);
if (scopesDoc.RootElement.ValueKind != JsonValueKind.Array || scopesDoc.RootElement.GetArrayLength() == 0)
{
    return Fail("ListScopes returned empty array");
}

var nodesJson = workspace.ListNodes();
using var nodesDoc = JsonDocument.Parse(nodesJson);
if (nodesDoc.RootElement.ValueKind != JsonValueKind.Array || nodesDoc.RootElement.GetArrayLength() == 0)
{
    return Fail("ListNodes returned empty array");
}

Console.WriteLine($"SMOKE OK scopes={scopesDoc.RootElement.GetArrayLength()} nodes={nodesDoc.RootElement.GetArrayLength()}");
return 0;
