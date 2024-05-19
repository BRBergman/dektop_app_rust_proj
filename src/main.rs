use druid::piet::InterpolationMode;
use druid::widget::{Button, FillStrat, Flex, Image, Label};
use druid::{AppLauncher, LocalizedString, PlatformError, Widget, WindowDesc};
use druid::{ImageBuf, WidgetExt};
use std::path::PathBuf;

mod get_image;

fn main() -> Result<(), PlatformError> {
    let main_window = WindowDesc::new(ui_builder()).title("Hallo");
    let data = 0_u32;
    AppLauncher::with_window(main_window)
        .log_to_console()
        .launch(data)
}

fn ui_builder() -> impl Widget<u32> {
    // The label text will be computed dynamically based on the current locale and count
    let text =
        LocalizedString::new("hello-counter").with_arg("count", |data: &u32, _env| (*data).into());
    let ibutton = Button::new("increment")
        .on_click(|_ctx, data, _env| *data += 1)
        .padding(5.0);
    let path = PathBuf::from("images/fuckass dog.png");
    println!("is it there?????? {}", path.exists());
    let pic = ImageBuf::from_file(path);
    let img = Image::new(match pic {
        Ok(image) => image,
        Err(_) => ImageBuf::empty(),
    })
    .fill_mode(FillStrat::Contain)
    .interpolation_mode(InterpolationMode::Bilinear);

    return Flex::column()
        .with_child(Label::new("welcome to my program").padding(5.0).center())
        .with_child(Label::new(text))
        .with_child(ibutton)
        .with_child(img);
}

//uh oh this is good!
