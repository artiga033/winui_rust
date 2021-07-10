fn main() {
    windows::build! {
        Microsoft::UI::Dispatching::DispatcherQueue,
        Microsoft::UI::Xaml::Controls::{Button,Grid,StackPanel,RelativePanel,UIElementCollection,TextBlock},
        Microsoft::UI::Xaml::Media::{SolidColorBrush},
        Microsoft::UI::Xaml::{Application,ApplicationInitializationCallback, Window,LaunchActivatedEventArgs,UIElement,
            DragEventHandler,DragEventArgs,
        },
        // Windows::Win32::UI::HiDpi::GetDpiForWindow,
        // Windows::Win32::UI::WindowsAndMessaging::{GetWindowRect, SetWindowPos, GetSystemMetrics},
        Windows::Foundation::{IAsyncOperation,AsyncStatus,TypedEventHandler},
        Windows::Storage::{StorageFile,
            Pickers::FileOpenPicker},
        Windows::UI::Color,
        Windows::Win32::{
            UI::Shell::IInitializeWithWindow,
            Foundation::HWND,
        }
    };
}
