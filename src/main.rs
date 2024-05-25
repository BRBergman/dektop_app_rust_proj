use druid::piet::InterpolationMode;
use druid::widget::{Align, Button, FillStrat, Flex, Image, Label};
use druid::{AppLauncher, LocalizedString, PlatformError, Widget, WindowDesc};
use druid::{ImageBuf, WidgetExt};

mod colors;
use colors::Theme;

fn main() -> Result<(), PlatformError> {
    let main_window = WindowDesc::new(ui_builder())
        .title("Hallo")
        .window_size((1280.0, 720.0))
        .with_min_size((854.0, 480.0));
    let data = 0_u32;
    AppLauncher::with_window(main_window)
        .log_to_console()
        .launch(data)
}

fn ui_builder() -> impl Widget<u32> {
    let mut colorscheme = Theme::ROSE_PINE_MOON;
    colorscheme.next_theme();
    
    //colorscheme.next_theme();
    // The label text will be computed dynamically based on the current locale and count
    let text =
        LocalizedString::new("hello-counter").with_arg("count", |data: &u32, _env| (*data).into());
    let ibutton = Button::new("increment")
        .on_click(|_ctx, data, _env| *data += 1)
        .padding(5.0);
    let fuckassdog = Image::new(match ImageBuf::from_file("images/fuckass_dog.png") {
        Ok(image) => image,
        Err(_) => ImageBuf::empty(),
    })
    .fill_mode(FillStrat::ScaleDown)
    .interpolation_mode(InterpolationMode::Bilinear)
    .border(colorscheme.overlay, 3.0)
    .rounded(2.0);
    let layout = Flex::column()
        .with_child(Label::new("welcome to my program").with_text_color(colorscheme.text))
        .with_child(Label::new(text).with_text_color(colorscheme.text))
        .with_child(ibutton)
        .with_child(fuckassdog)
        .scroll()
        .disable_scrollbars()
        .center()
        .padding(5.0)
        .background(colorscheme.base);
    Align::centered(layout)
}
