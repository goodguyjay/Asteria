using ASN.MC.UI.ViewModels;
using Avalonia.Controls;

namespace ASN.MC.UI.Views;

public sealed partial class MainWindow : Window
{
    public MainWindow()
    {
        InitializeComponent();
        DataContext = new MainWindowViewModel();
    }
}