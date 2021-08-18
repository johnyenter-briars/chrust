use druid::widget::{Button, Flex, Label};
use druid::{AppLauncher, LocalizedString, PlatformError, Widget, WidgetExt, WindowDesc};

use crate::game::chessgame::ChessGame;

pub struct Visualizer2<'a> {
    game: ChessGame<'a>,
}

impl<'a> Visualizer2<'a> {
    pub fn new(game: ChessGame<'a>) -> Self {
        Visualizer2 { game }
    }

    pub fn start_viz(&mut self) -> Result<(), PlatformError> {
        let main_window = WindowDesc::new(ui_builder);
        let data = 0_u32;
        AppLauncher::with_window(main_window)
            .use_simple_logger()
            .launch(data)
    }
}

fn ui_builder() -> impl Widget<u32> {
    // The label text will be computed dynamically based on the current locale and count
    let text =
        LocalizedString::new("hello-counter").with_arg("count", |data: &u32, _env| (*data).into());
    let label = Label::new(text).padding(5.0).center();
    let button = Button::new("increment")
        .on_click(|_ctx, data, _env| *data += 1)
        .padding(5.0);

    Flex::column().with_child(label).with_child(button)
}
