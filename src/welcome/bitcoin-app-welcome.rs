// use iced::{
//     advanced::{
//         layout, mouse,
//         renderer::{self, Quad},
//         widget::Tree,
//         Layout, Widget,
//     },
//     widget::{button, container},
//     Alignment, Border, Color, Element, Length, Rectangle, Sandbox, Settings, Shadow, Size, Theme,
// };

// fn main() -> iced::Result {
//     MyApp::run(Settings::default())
// }

// #[derive(Debug, Clone)]
// enum WelcomeMessage {
//     GetStartedButtonPressed,
// }

// struct MyApp {
//     counter: u8
// }

// impl Sandbox for MyApp {
//     type Message = WelcomeMessage;

//     fn new() -> Self {
//         Self { counter: 0 }
//     }

//     fn title(&self) -> String {
//         String::from("My App")
//     }

//     fn update(&mut self, message: Self::Message) {
//         match message {
//             WelcomeMessage::GetStartedButtonPressed => self.counter = 1,
//         }
//     }

//     fn view(&self) -> iced::Element<Self::Message> {
//         match self.counter {
//             0 => container(MyWidgetOuter::new(button("Other widget").on_press(WelcomeMessage::GetStartedButtonPressed).into()))
//             .width(Length::Fill)
//             .height(Length::Fill)
//             .center_x()
//             .center_y()
//             .into(),
//             // 0 => button("Other widget").on_press(WelcomeMessage::GetStartedButtonPressed).into(),
//             _ => container("").into()
//         }
//     }
// }

// struct MyWidgetOuter<'a, Message, Renderer> {
//     inner_widget: Element<'a, Message, Theme, Renderer>,
// }

// impl<'a, Message, Renderer> MyWidgetOuter<'a, Message, Renderer>
// where
//     Renderer: iced::advanced::Renderer,
// {
//     fn new(inner_widget: Element<'a, Message, Theme, Renderer>) -> Self {
//         Self { inner_widget }
//     }
// }

// impl<Message, Renderer> Widget<Message, Theme, Renderer> for MyWidgetOuter<'_, Message, Renderer>
// where
//     Renderer: iced::advanced::Renderer,
// {
//     fn size(&self) -> Size<Length> {
//         Size {
//             width: Length::Shrink,
//             height: Length::Shrink,
//         }
//     }

//     fn diff(&self, tree: &mut Tree) {
//         tree.diff_children(std::slice::from_ref(&self.inner_widget));
//     }

//     fn layout(
//         &self,
//         tree: &mut Tree,
//         renderer: &Renderer,
//         limits: &layout::Limits,
//     ) -> layout::Node {
//         let mut child_node =
//             self.inner_widget
//                 .as_widget()
//                 .layout(&mut tree.children[0], renderer, limits);

//         let size_of_this_node = child_node.size().expand(Size::new(50., 50.));

//         child_node = child_node.align(Alignment::Center, Alignment::Center, size_of_this_node);

//         layout::Node::with_children(size_of_this_node, vec![child_node])
//     }

//     fn children(&self) -> Vec<Tree> {
//         vec![Tree::new(self.inner_widget.as_widget())]
//     }

//     fn draw(
//         &self,
//         state: &Tree,
//         renderer: &mut Renderer,
//         theme: &Theme,
//         style: &renderer::Style,
//         layout: Layout<'_>,
//         cursor: mouse::Cursor,
//         viewport: &Rectangle,
//     ) {
//         renderer.fill_quad(
//             Quad {
//                 bounds: layout.bounds(),
//                 border: Border {
//                     color: Color::from_rgb(0.6, 0.93, 1.0),
//                     width: 1.0,
//                     radius: 10.0.into(),
//                 },
//                 shadow: Shadow::default(),
//             },
//             Color::from_rgb(0.0, 0.33, 0.4),
//         );

//         self.inner_widget.as_widget().draw(
//             &state.children[0],
//             renderer,
//             theme,
//             style,
//             layout.children().next().unwrap(),
//             cursor,
//             viewport,
//         );
//     }
// }

// impl<'a, Message, Renderer> From<MyWidgetOuter<'a, Message, Renderer>>
//     for Element<'a, Message, Theme, Renderer>
// where
//     Message: 'a,
//     Renderer: iced::advanced::Renderer + 'a,
// {
//     fn from(widget: MyWidgetOuter<'a, Message, Renderer>) -> Self {
//         Self::new(widget)
//     }
// }

use iced::{
    advanced::{
        self, layout, mouse, overlay,
        renderer::{self, Quad},
        widget::{Tree, Operation, self},
        Layout, Widget, Shell, Clipboard
    },
    widget::{container, image::Handle, Theme, column, Image, Svg, text, row, button},
    Border, Color, Element, Length, Rectangle, Sandbox, Settings, Shadow, Size, Alignment, Vector, Font, Background, theme, Point, Event,
    font::Weight, Gradient, gradient::{ Linear, ColorStop }, Radians, window, event,
    alignment::{ Horizontal }
};

#[derive(Debug, Clone)]
enum WelcomeMessage {
    GetStartedButtonPressed,
}

fn main() -> iced::Result {
    MyApp::run(Settings {
        window: window::Settings {
            size: Size {
                width: 1440.,
                height: 920.,
            },
            ..window::Settings::default()
        },
        ..Settings::default()
    })
}

struct MyApp {
    card_content: u8
}

impl Sandbox for MyApp {
    type Message = WelcomeMessage;

    fn new() -> Self {
        Self { card_content: 0 }
    }

    fn title(&self) -> String {
        String::from("My App")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            WelcomeMessage::GetStartedButtonPressed => self.card_content = 1,
        }
    }

    fn view(&self) -> iced::Element<Self::Message> {
        match self.card_content {
            0 => {
                let background = container(
                    MyWidgetWithImage::new()
                );
        
                let card_content = column![
                    container("").height(Length::Fixed(208.69)),
                    container(
                        Svg::from_path("assets/welcome/Union.svg").width(Length::Fixed(95.49)).height(Length::Fixed(71.62))
                    )
                        .width(Length::Fill)
                        .align_x(Horizontal::Center),
                    text("inherit").font(Font {
                        weight: Weight::Bold,
                        ..Font::DEFAULT
                    })
                        .size(48)
                        .width(Length::Fill)
                        .horizontal_alignment(Horizontal::Center),
                    container("").height(Length::Fixed(82.)),
                    text("Protect your legacy").font(Font {
                        weight: Weight::Semibold,
                        ..Font::DEFAULT
                    })
                        .size(32)
                        .width(Length::Fill)
                        .horizontal_alignment(Horizontal::Center),
                    container(text("Please connect your wallet to continue")
                        .width(Length::Fill)
                        .horizontal_alignment(Horizontal::Center))
                        .padding(8.),
                    container(
                        button("Get started")
                        .on_press(WelcomeMessage::GetStartedButtonPressed)
                        .style(
                            theme::Button::Custom(Box::new(ButtonColor {} ))
                        ).width(Length::Fixed(131.)).height(Length::Fixed(48.))
                    )
                        .width(Length::Fill)
                        .align_x(Horizontal::Center),
                    container("").height(Length::Fixed(195.)),
                    container(row![
                        text("Inherit 2025"),
                        text("*"),
                        text("Terms of service"),
                        text("Privacy policy")
                    ]
                        .spacing(10))
                        .width(Length::Fill)
                        .align_x(Horizontal::Center)
                ];

                let card = container(
                    MyWidgetInner::new(card_content)
                );

                // card.into()
        
                GetStarted::new(background, card).into()

                // button("Get started")
                //         .on_press(WelcomeMessage::GetStartedButtonPressed)
                //         .style(
                //             theme::Button::Custom(Box::new(ButtonColor {} ))
                //         ).width(Length::Fixed(100.)).height(Length::Fixed(50.)).into()
            },

            _ => {
                container("").into()
            }
        }
        
    }
}

struct MyWidgetWithImage {
    handle: Handle,
}

impl MyWidgetWithImage {
    fn new() -> Self {
        Self {
            handle: Handle::from_path("assets/welcome/Inherit_LookDev_02.png"),
        }
    }
}

impl<Message, Renderer> Widget<Message, Theme, Renderer> for MyWidgetWithImage
where
    Renderer: iced::advanced::Renderer + iced::advanced::image::Renderer<Handle = Handle>,
{
    fn size(&self) -> Size<Length> {
        Size {
            width: Length::Fill,
            height: Length::Fill,
        }
    }

    fn layout(
        &self,
        _tree: &mut Tree,
        renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        iced::widget::image::layout(
            renderer,
            limits,
            &self.handle,
            Length::Fill,
            Length::Fill,
            iced::ContentFit::Fill,
        )
    }

    fn draw(
        &self,
        _state: &Tree,
        renderer: &mut Renderer,
        _theme: &Theme,
        _style: &renderer::Style,
        layout: Layout<'_>,
        _cursor: mouse::Cursor,
        _viewport: &Rectangle,
    ) {
        renderer.fill_quad(
            Quad {
                bounds: layout.bounds(),
                border: Border {
                    color: Color::from_rgb(1.0, 0.66, 0.6),
                    width: 1.0,
                    radius: 10.0.into(),
                },
                shadow: Shadow::default(),
            },
            Color::BLACK,
        );

        iced::widget::image::draw(
            renderer,
            layout,
            &self.handle,
            iced::ContentFit::Fill,
            iced::widget::image::FilterMethod::Linear,
        );
    }
}

impl<'a, Message, Renderer> From<MyWidgetWithImage> for Element<'a, Message, Theme, Renderer>
where
    Renderer: iced::advanced::Renderer + iced::advanced::image::Renderer<Handle = Handle>,
{
    fn from(widget: MyWidgetWithImage) -> Self {
        Self::new(widget)
    }
}

struct MyWidgetInner<'a, Message, Renderer> {
    content: Element<'a, Message, Theme, Renderer>
}

impl<'a, Message, Renderer> MyWidgetInner<'a, Message, Renderer> {
    fn new(content: impl Into<Element<'a, Message, Theme, Renderer>>) -> Self {
        Self { content: content.into() }
    }
}

impl<'a, Message, Renderer> Widget<Message, Theme, Renderer> for MyWidgetInner<'a, Message, Renderer>
where
    Renderer: iced::advanced::Renderer,
{
    fn size(&self) -> Size<Length> {
        Size {
            width: Length::Shrink,
            height: Length::Shrink,
        }
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(&[&self.content]);
    }

    fn layout(
        &self,
        tree: &mut Tree,
        renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        match tree.children.len() {
            0 => layout::Node::new(Size::new(585., 820.)),
            _ => {
                let mut child_node = self.content.as_widget().layout(&mut tree.children[0], renderer, limits);

                let size_of_this_node = Size::new(585., 820.);

                child_node = child_node.align(Alignment::Center, Alignment::Center, size_of_this_node);

                // child_node = child_node.move_to(Point { x: 100., y: 0. });

                layout::Node::with_children(size_of_this_node, vec![child_node])
            },
        }
    }

    fn on_event(
        &mut self,
        state: &mut Tree,
        event: Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle
    ) -> event::Status {
        self.content.as_widget_mut().on_event(
            &mut state.children[0],
            event,
            layout,
            cursor,
            renderer,
            clipboard,
            shell,
            viewport,
        )
    }

    fn mouse_interaction(
        &self,
        state: &Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        self.content.as_widget().mouse_interaction(
            &state.children[0],
            layout,
            cursor,
            viewport,
            renderer,
        )
    }

    fn draw(
        &self,
        state: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        renderer.fill_quad(
            Quad {
                bounds: layout.bounds(),
                border: Border {
                    color: Color::from_rgb(0.6, 0.8, 1.0),
                    width: 0.0,
                    radius: 32.0.into(),
                },
                shadow: Shadow::default(),
            },
            Color::from_rgb(205., 220., 241.),
        );

        self.content.as_widget().draw(
            &state.children[0],
            renderer,
            theme,
            style,
            layout.children().next().unwrap(),
            cursor,
            viewport,
        )
    }

    fn operate(
        &self,
        state: &mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn widget::Operation<Message>,
    ) {
        self.content
            .as_widget()
            .operate(&mut state.children[0], layout, renderer, operation);
    }
}

impl<'a, Message, Renderer> From<MyWidgetInner<'a, Message, Renderer>> for Element<'a, Message, Theme, Renderer>
where
    Renderer: 'a + iced::advanced::Renderer,
    Message: 'a + Clone,
{
    fn from(widget: MyWidgetInner<'a, Message, Renderer>) -> Self {
        Self::new(widget)
    }
}

struct GetStarted<'a, Message, Renderer> {
    background: Element<'a, Message, Theme, Renderer>,
    card: Element<'a, Message, Theme, Renderer>,
}

impl<'a, Message, Renderer> GetStarted<'a, Message, Renderer> {
    fn new(
        background: impl Into<Element<'a, Message, Theme, Renderer>>,
        card: impl Into<Element<'a, Message, Theme, Renderer>>
    ) -> Self {
        Self {
            background: background.into(),
            card: card.into()
        }
    }
}

impl<'a, Message, Renderer> Widget<Message, Theme, Renderer> for GetStarted<'a, Message, Renderer>
where
    Renderer: iced::advanced::Renderer,
{
    fn children(&self) -> Vec<Tree> {
        vec![
            Tree::new(&self.background),
            Tree::new(&self.card)
        ]
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(&[&self.background, &self.card]);
    }

    fn size(&self) -> Size<Length> {
        Size {
            width: Length::Fill,
            height: Length::Fill,
        }
    }

    fn layout(
        &self,
        tree: &mut Tree,
        renderer: &Renderer,
        limits: &layout::Limits
    ) -> layout::Node {
        self.background.as_widget().layout(tree, renderer, limits)
    }

    fn draw(
        &self,
        state: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle
    ) {
        self.background.as_widget().draw(
            &state.children[0],
            renderer,
            theme,
            style,
            layout,
            cursor,
            viewport
        )
    }

    fn overlay<'b>(
        &'b mut self,
        state: &'b mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
        translation: Vector
    ) -> Option<overlay::Element<'b, Message, Theme, Renderer>> {
        Some(overlay::Element::new(
            Box::new(Overlay {
                content: &mut self.card,
                tree: &mut state.children[1],
                size: layout.bounds().size()
            })
        ))
    }

    fn operate(
        &self,
        state: &mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn Operation<Message>,
    ) {
        self.background
            .as_widget()
            .operate(&mut state.children[0], layout, renderer, operation);
    }
}

struct Overlay<'a, 'b, Message, Theme, Renderer> {
    content: &'b mut Element<'a, Message, Theme, Renderer>,
    tree: &'b mut Tree,
    size: Size,
}

impl <'a, 'b, Message, Theme, Renderer> iced::advanced::Overlay<Message, Theme, Renderer>
    for Overlay<'a, 'b, Message, Theme, Renderer>
where
    Renderer: iced::advanced::Renderer
{
    fn layout(&mut self, renderer: &Renderer, _bounds: Size) -> layout::Node {
        let limits = layout::Limits::new(Size::ZERO, self.size)
            .width(Length::Fill)
            .height(Length::Fill);
        
        let mut child = self.content.as_widget().layout(self.tree, renderer, &limits);
        // child = child.align(Alignment::Start, Alignment::Center, limits.max());
        child = child.move_to(Point { x: 100., y: 50. });

        let mut node = layout::Node::with_children(self.size, vec![child]);

         node
    }

    fn draw(
        &self,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
    ) {
        self.content.as_widget().draw(
            self.tree,
            renderer,
            theme,
            style,
            layout.children().next().unwrap(),
            cursor,
            &layout.bounds(),
        )
    }

    fn operate(
        &mut self,
        layout: Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn Operation<Message>,
    ) {
        self.content.as_widget().operate(
            self.tree,
            layout.children().next().unwrap(),
            renderer,
            operation,
        );
    }

    fn overlay<'c>(
        &'c mut self,
        layout: Layout<'_>,
        renderer: &Renderer,
    ) -> Option<overlay::Element<'c, Message, Theme, Renderer>> {
        self.content.as_widget_mut().overlay(
            self.tree,
            layout.children().next().unwrap(),
            renderer,
            Vector {
                x: 200.,
                y: 200.
            }
        )
    }
}

impl<'a, Message, Renderer> From<GetStarted<'a, Message, Renderer>> for Element<'a, Message, Theme, Renderer>
where
    Renderer: 'a + iced::advanced::Renderer,
    Message: 'a + Clone,
{
    fn from(widget: GetStarted<'a, Message, Renderer>) -> Self {
        Self::new(widget)
    }
}

struct ButtonColor {}

impl<> button::StyleSheet for ButtonColor {
    type Style = Theme;

    fn active(&self, style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(Background::Gradient(Gradient::Linear(Linear {
                angle: Radians(0.778),
                stops: [
                    Some(ColorStop { offset: 0., color: Color { r: 4., g: 47., b: 104., a: 1. } }),
                    Some(ColorStop { offset: 1., color: Color { r: 2., g: 84., b: 191., a: 1. } }),
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    // Some(ColorStop { offset: 1., color: Color { r: 2., g: 84., b: 191., a: 1. } }),
                ]
            }))),
            ..style.active(&theme::Button::Primary)
        }
    }
}