#![windows_subsystem = "windows"]

use std::{convert::TryFrom};

use bindings::{
    Microsoft,
    Microsoft::UI::Dispatching::{DispatcherQueue, DispatcherQueueHandler},
    Microsoft::UI::Xaml::{
        Application, ApplicationInitializationCallback,
        Controls::{Button, StackPanel ,TextBlock, UIElementCollection},
         DragEventArgs, DragEventHandler,
        LaunchActivatedEventArgs,
        Media::SolidColorBrush,
        RoutedEventHandler, Window,
    },
    Windows::{
        Foundation::{AsyncStatus, IAsyncOperation},
        Storage::{Pickers::FileOpenPicker, StorageFile},
        Win32::{Foundation::HWND, UI::Shell::IInitializeWithWindow},
        UI::Color,
    },
};

use windows::{implement, IInspectable, Interface};

#[implement(extend Microsoft::UI::Xaml::Application, override OnLaunched)]
struct App {
    _window: Option<Window>,
}

#[allow(non_snake_case)]
impl App {
    fn OnLaunched(&mut self, _: &Option<LaunchActivatedEventArgs>) -> windows::Result<()> {
        //initialize windows and its controls
        let window = Window::new()?;
        window.SetTitle("Rust using WinUI")?;
        let inspectable = &IInspectable::from(&window);
        let hwnd = HWND(
            windows_app::window_handle(inspectable).expect("Failed to get native window handle"),
        );

        let stackpanel = StackPanel::new()?;
        stackpanel.SetAllowDrop(true)?;
        // stackpanel.DragOver(DragEventHandler::new(|sender,_args|{
        //     if let Some(sp) = sender {
        //         let sp= sp.cast::<StackPanel>()?;
        //         sp.DispatcherQueue()?.TryEnqueue(DispatcherQueueHandler::new(move ||{
        //           sp.Children()?.GetAt(0)?.cast::<TextBlock>()?.SetText("DISPLAY")
        //         }))?;
        //     }
        //     Ok(())
        // }))?;

        let brush1 = SolidColorBrush::new()?;
        brush1.SetColor(Color {
            A: 255,
            B: 230,
            G: 230,
            R: 230,
        })?;
        stackpanel.SetBackground(&brush1)?;

        let textblock = TextBlock::new()?;
        textblock.SetFontSize(33.0)?;
        textblock.SetText("Path to File")?;
        let button = Button::new()?;
        button.SetFontSize(33.0)?;
        button.SetContent(IInspectable::try_from("Browse")?)?;
        button.Click(RoutedEventHandler::new(move |sender, _args| {
            let picker = FileOpenPicker::new()?;

            //see https://docs.microsoft.com/en-us/uwp/api/windows.storage.pickers.fileopenpicker.-ctor?view=winrt-19041#remarks
            let initializewithWindow = picker.cast::<IInitializeWithWindow>()?;
            unsafe {
                initializewithWindow.Initialize(hwnd)?;
            }
            let file_op = picker.PickSingleFileAsync()?;
            while file_op.Status()? == AsyncStatus::Started {}
            // if let AsyncStatus::Completed = file_op.Status()? {
            //     textblock
            //         .DispatcherQueue()?
            //         .TryEnqueue(DispatcherQueueHandler::new(move || {
            //             textblock.SetText(file_op.GetResults()?.Path()?);
            //             Ok(())
            //         }))?;
            // }
            Ok(())
        }))?;
        stackpanel.Children()?.Append(&button)?;
        stackpanel.Children()?.Append(&textblock)?;
        window.SetContent(&stackpanel)?;
        let result = window.Activate();
        self._window = Some(window);
        result
    }
}

fn main() -> windows::Result<()> {
    windows_app::bootstrap::initialize().and_then(|_| {
        Application::Start(ApplicationInitializationCallback::new(|_| {
            App { _window: None }.new()?;
            Ok(())
        }))
    })
}
