namespace ASN.MC.UI.Models.Core;

public sealed class SidebarItem(string name, object? content = null)
{
    public string Name { get; } = name;
    public object? Content { get; } = content;
}