use iced::widget::{
    column, container, image, pane_grid, row, scrollable, text, Column, Image, PaneGrid, Row,
};
use iced::{alignment, executor, Application, Command, Element, Length, Theme};

#[derive(Debug)]
pub enum PaneKind {
    ImageList,
    SpriteSheet,
}

#[derive(Clone, Debug)]
pub enum Message {
    PaneResized(pane_grid::ResizeEvent),
}

pub struct SpritePack {
    panes: pane_grid::State<PaneKind>,
}

impl Application for SpritePack {
    type Message = Message;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        let pane_config = pane_grid::Configuration::Split {
            axis: pane_grid::Axis::Vertical,
            ratio: 0.25,
            a: Box::new(pane_grid::Configuration::Pane(PaneKind::ImageList)),
            b: Box::new(pane_grid::Configuration::Pane(PaneKind::SpriteSheet)),
        };

        let panes = pane_grid::State::with_configuration(pane_config);

        (Self { panes }, Command::none())
    }

    fn title(&self) -> String {
        "Sprite Pack".to_string()
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::PaneResized(event) => {
                self.panes.resize(&event.split, event.ratio);
            }
            _ => (),
        }

        Command::none()
    }

    fn view(&self) -> Element<'_, Self::Message> {
        PaneGrid::new(&self.panes, |pane, kind, is_maximized| {
            let content = match kind {
                PaneKind::ImageList => image_list_pane_content(100),
                PaneKind::SpriteSheet => sprite_sheet_pane_content(),
            };

            pane_grid::Content::new(content)
        })
        .on_resize(8, Message::PaneResized)
        .into()
    }
}

fn image_list_pane_content<'a>(count: u32) -> Element<'a, Message> {
    let images = (0..count)
        .map(|i| Element::new(text(format!("image_{i}.png"))))
        .collect::<Vec<_>>();

    container(
        scrollable(Column::with_children(images).spacing(4))
            .width(Length::Fill)
            .height(Length::Fill),
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .padding([0, 8])
    .into()
}

fn sprite_sheet_pane_content<'a>() -> Element<'a, Message> {
    column![
        image("test-data/sprite-sheet.png")
            .width(Length::Fill)
            .height(Length::Fill),
        text("Buttons...").height(Length::Shrink)
    ]
    .into()
}

fn main() -> iced::Result {
    SpritePack::run(Default::default())
}
