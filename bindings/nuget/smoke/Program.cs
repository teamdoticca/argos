using System.Diagnostics;
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

static async Task<int> RunOpenSmokeAsync(string root)
{
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
}

static async Task<int> RunWatchSmokeAsync()
{
    var tmp = Path.Combine(Path.GetTempPath(), "argos-nuget-smoke-" + Guid.NewGuid().ToString("n"));
    Directory.CreateDirectory(tmp);
    Console.WriteLine($"SMOKE watch root={tmp}");

    try
    {
        var git = Process.Start(new ProcessStartInfo
        {
            FileName = "git",
            Arguments = "init",
            WorkingDirectory = tmp,
            RedirectStandardOutput = true,
            RedirectStandardError = true,
            UseShellExecute = false,
        });
        if (git is null)
        {
            return Fail("failed to start git init");
        }
        await git.WaitForExitAsync();
        if (git.ExitCode != 0)
        {
            return Fail($"git init failed: {await git.StandardError.ReadToEndAsync()}");
        }

        await File.WriteAllTextAsync(
            Path.Combine(tmp, "package.json"),
            """{"name":"argos-smoke","private":true}""");

        await using var workspace = await ArgosWorkspace.OpenAsync(tmp);
        var scopesJson = workspace.ListScopes();
        using var scopesDoc = JsonDocument.Parse(scopesJson);
        if (scopesDoc.RootElement.ValueKind != JsonValueKind.Array || scopesDoc.RootElement.GetArrayLength() == 0)
        {
            return Fail("ListScopes returned empty array in watch smoke");
        }

        using var cts = new CancellationTokenSource(TimeSpan.FromSeconds(45));
        var gotEvent = false;
        var watchTask = Task.Run(async () =>
        {
            await foreach (var _ in workspace.WatchAsync(cts.Token))
            {
                gotEvent = true;
                cts.Cancel();
                break;
            }
        }, CancellationToken.None);

        await Task.Delay(750, CancellationToken.None);
        var trigger = Path.Combine(tmp, "argos-smoke-trigger.txt");
        await File.WriteAllTextAsync(trigger, DateTime.UtcNow.ToString("O"));

        try
        {
            await watchTask;
        }
        catch (OperationCanceledException)
        {
            // expected when we cancel after first event or timeout
        }

        if (!gotEvent)
        {
            return Fail("WatchAsync produced no event after file change (timeout)");
        }

        Console.WriteLine("SMOKE WATCH OK");
        return 0;
    }
    finally
    {
        try
        {
            Directory.Delete(tmp, recursive: true);
        }
        catch
        {
            // best-effort cleanup
        }
    }
}

var watch = args.Any(a => string.Equals(a, "--watch", StringComparison.OrdinalIgnoreCase));
if (watch)
{
    return await RunWatchSmokeAsync();
}

var pathArgs = args.Where(a => !a.StartsWith("--", StringComparison.Ordinal)).ToArray();
var root = pathArgs.Length > 0
    ? Path.GetFullPath(pathArgs[0])
    : FindFixtureFrom(Directory.GetCurrentDirectory())
        ?? FindFixtureFrom(AppContext.BaseDirectory)
        ?? "";

if (string.IsNullOrWhiteSpace(root) || !Directory.Exists(root))
{
    return Fail($"fixture path does not exist: '{root}'. Pass fixtures/small-pnpm explicitly.");
}

return await RunOpenSmokeAsync(root);
