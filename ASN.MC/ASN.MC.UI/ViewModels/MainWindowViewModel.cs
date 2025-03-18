using System.Collections.ObjectModel;
using ASN.MC.UI.Models.Core;
using CommunityToolkit.Mvvm.ComponentModel;

namespace ASN.MC.UI.ViewModels;

public sealed partial class MainWindowViewModel : ViewModelBase
{
    [ObservableProperty]
    private SidebarItem _selectedSidebarItem;
    
    public ObservableCollection<SidebarItem> SidebarItems { get; }

    public MainWindowViewModel()
    {
        SidebarItems =
        [
            new SidebarItem("Dashboard"),
            new SidebarItem("Telemetry"),
            new SidebarItem("Logs"),
            new SidebarItem("Commands"),
            new SidebarItem("Diagnostics"),
            new SidebarItem("Settings")
        ];
        
        SelectedSidebarItem = SidebarItems[0];
    }
}