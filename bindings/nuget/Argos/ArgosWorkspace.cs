using System.Runtime.InteropServices;
using System.Text;
using System.Text.Json;

namespace Argos;

public sealed class ArgosWorkspace : IAsyncDisposable, IDisposable
{
    private IntPtr _handle;
    private bool _disposed;

    private ArgosWorkspace(IntPtr handle) => _handle = handle;

    public static Task<ArgosWorkspace> OpenAsync(string root, string? optionsJson = null, CancellationToken ct = default)
    {
        ct.ThrowIfCancellationRequested();
        return Task.FromResult(Open(root, optionsJson));
    }

    public static ArgosWorkspace Open(string root, string? optionsJson = null)
    {
        var opts = string.IsNullOrEmpty(optionsJson) ? "{}" : optionsJson;
        var rc = Native.argos_workspace_open(root, opts, out var handle);
        if (rc != 0 || handle == IntPtr.Zero)
            throw new ArgosException(Native.LastError());
        return new ArgosWorkspace(handle);
    }

    /// <summary>Pointer to the latest immutable WorkspaceSnapshot JSON.</summary>
    public string CurrentSnapshot
    {
        get
        {
            EnsureAlive();
            var rc = Native.argos_workspace_current_snapshot(_handle, out var ptr);
            if (rc != 0) throw new ArgosException(Native.LastError());
            return Native.TakeString(ptr);
        }
    }

    public string RebuildSnapshot()
    {
        EnsureAlive();
        var rc = Native.argos_workspace_rebuild_snapshot(_handle, out var ptr);
        if (rc != 0) throw new ArgosException(Native.LastError());
        return Native.TakeString(ptr);
    }

    public string ExplainPath(string path)
    {
        EnsureAlive();
        var rc = Native.argos_explain_path(_handle, path, out var ptr);
        if (rc != 0) throw new ArgosException(Native.LastError());
        return Native.TakeString(ptr);
    }

    public string FindOwner(string path)
    {
        EnsureAlive();
        var rc = Native.argos_find_owner(_handle, path, out var ptr);
        if (rc != 0) throw new ArgosException(Native.LastError());
        return Native.TakeString(ptr);
    }

    public string ListScopes()
    {
        EnsureAlive();
        var rc = Native.argos_list_scopes(_handle, out var ptr);
        if (rc != 0) throw new ArgosException(Native.LastError());
        return Native.TakeString(ptr);
    }

    public string ListNodes()
    {
        EnsureAlive();
        var rc = Native.argos_list_nodes(_handle, out var ptr);
        if (rc != 0) throw new ArgosException(Native.LastError());
        return Native.TakeString(ptr);
    }

    public string Health()
    {
        EnsureAlive();
        var rc = Native.argos_workspace_health(_handle, out var ptr);
        if (rc != 0) throw new ArgosException(Native.LastError());
        return Native.TakeString(ptr);
    }

    public static string ComputeDelta(string oldSnapshotJson, string newSnapshotJson)
    {
        var rc = Native.argos_compute_delta(oldSnapshotJson, newSnapshotJson, out var ptr);
        if (rc != 0) throw new ArgosException(Native.LastError());
        return Native.TakeString(ptr);
    }

    public IReadOnlyList<string> GetAffectedScopes(string changeEventJson)
    {
        EnsureAlive();
        var rc = Native.argos_get_affected_scopes(_handle, changeEventJson, out var ptr);
        if (rc != 0) throw new ArgosException(Native.LastError());
        var json = Native.TakeString(ptr);
        return JsonSerializer.Deserialize<List<string>>(json) ?? new List<string>();
    }

    public async IAsyncEnumerable<string> WatchAsync([System.Runtime.CompilerServices.EnumeratorCancellation] CancellationToken ct = default)
    {
        EnsureAlive();
        var rc = Native.argos_watch_start(_handle);
        if (rc != 0) throw new ArgosException(Native.LastError());
        try
        {
            while (!ct.IsCancellationRequested)
            {
                var poll = Native.argos_watch_poll(_handle, out var ptr);
                if (poll != 0) throw new ArgosException(Native.LastError());
                var json = Native.TakeString(ptr);
                using var doc = JsonDocument.Parse(json);
                foreach (var el in doc.RootElement.EnumerateArray())
                    yield return el.GetRawText();
                await Task.Delay(50, ct).ConfigureAwait(false);
            }
        }
        finally
        {
            Native.argos_watch_stop(_handle);
        }
    }

    private void EnsureAlive()
    {
        ObjectDisposedException.ThrowIf(_disposed || _handle == IntPtr.Zero, this);
    }

    public void Dispose()
    {
        if (_disposed) return;
        if (_handle != IntPtr.Zero)
        {
            Native.argos_workspace_free(_handle);
            _handle = IntPtr.Zero;
        }
        _disposed = true;
    }

    public ValueTask DisposeAsync()
    {
        Dispose();
        return ValueTask.CompletedTask;
    }
}

public sealed class ArgosException : Exception
{
    public ArgosException(string message) : base(message) { }
}

internal static class Native
{
    private const string Lib = "argos";

    [DllImport(Lib, CallingConvention = CallingConvention.Cdecl)]
    public static extern int argos_workspace_open([MarshalAs(UnmanagedType.LPUTF8Str)] string root, [MarshalAs(UnmanagedType.LPUTF8Str)] string optionsJson, out IntPtr outWorkspace);

    [DllImport(Lib, CallingConvention = CallingConvention.Cdecl)]
    public static extern void argos_workspace_free(IntPtr workspace);

    [DllImport(Lib, CallingConvention = CallingConvention.Cdecl)]
    public static extern int argos_workspace_current_snapshot(IntPtr workspace, out IntPtr outJson);

    [DllImport(Lib, CallingConvention = CallingConvention.Cdecl)]
    public static extern int argos_workspace_rebuild_snapshot(IntPtr workspace, out IntPtr outJson);

    [DllImport(Lib, CallingConvention = CallingConvention.Cdecl)]
    public static extern int argos_explain_path(IntPtr workspace, [MarshalAs(UnmanagedType.LPUTF8Str)] string path, out IntPtr outJson);

    [DllImport(Lib, CallingConvention = CallingConvention.Cdecl)]
    public static extern int argos_find_owner(IntPtr workspace, [MarshalAs(UnmanagedType.LPUTF8Str)] string path, out IntPtr outJson);

    [DllImport(Lib, CallingConvention = CallingConvention.Cdecl)]
    public static extern int argos_list_scopes(IntPtr workspace, out IntPtr outJson);

    [DllImport(Lib, CallingConvention = CallingConvention.Cdecl)]
    public static extern int argos_list_nodes(IntPtr workspace, out IntPtr outJson);

    [DllImport(Lib, CallingConvention = CallingConvention.Cdecl)]
    public static extern int argos_workspace_health(IntPtr workspace, out IntPtr outJson);

    [DllImport(Lib, CallingConvention = CallingConvention.Cdecl)]
    public static extern int argos_compute_delta([MarshalAs(UnmanagedType.LPUTF8Str)] string oldJson, [MarshalAs(UnmanagedType.LPUTF8Str)] string newJson, out IntPtr outJson);

    [DllImport(Lib, CallingConvention = CallingConvention.Cdecl)]
    public static extern int argos_get_affected_scopes(IntPtr workspace, [MarshalAs(UnmanagedType.LPUTF8Str)] string eventJson, out IntPtr outJson);

    [DllImport(Lib, CallingConvention = CallingConvention.Cdecl)]
    public static extern int argos_watch_start(IntPtr workspace);

    [DllImport(Lib, CallingConvention = CallingConvention.Cdecl)]
    public static extern int argos_watch_stop(IntPtr workspace);

    [DllImport(Lib, CallingConvention = CallingConvention.Cdecl)]
    public static extern int argos_watch_poll(IntPtr workspace, out IntPtr outJson);

    [DllImport(Lib, CallingConvention = CallingConvention.Cdecl)]
    public static extern int argos_last_error(out IntPtr outMsg);

    [DllImport(Lib, CallingConvention = CallingConvention.Cdecl)]
    public static extern void argos_string_free(IntPtr s);

    public static string TakeString(IntPtr ptr)
    {
        if (ptr == IntPtr.Zero) return string.Empty;
        try
        {
            return Marshal.PtrToStringUTF8(ptr) ?? string.Empty;
        }
        finally
        {
            argos_string_free(ptr);
        }
    }

    public static string LastError()
    {
        if (argos_last_error(out var ptr) != 0) return "unknown error";
        return TakeString(ptr);
    }
}
