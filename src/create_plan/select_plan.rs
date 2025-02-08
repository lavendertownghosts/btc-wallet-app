use crate::iced::{
    advanced::{
        graphics::core::event,
        layout, mouse,
        renderer::{self, Quad},
        widget::Tree,
        Layout, Widget, Clipboard, Shell
    },
    Border, Color, Element, Length, Rectangle, Sandbox, Settings, Shadow, Size, Theme, Event, Alignment,
    mouse::{self as mouse_enum}
};

// #[derive(Debug, Clone)]
// enum PlanCardMessage {
//     SelectedPlan(bool)
// }

// type Pcm = PlanCardMessage

pub struct PlanCard<'a, Message, Renderer> {
    selected: bool,
    content: Element<'a, Message, Theme, Renderer>,
    message: Message
}

impl<'a, Message, Renderer> PlanCard<'a, Message, Renderer> {
    pub fn new(
        selected: bool,
        content: impl Into<Element<'a, Message, Theme, Renderer>>,
        message: Message
    ) -> Self {
        Self { 
            selected,
            content: content.into(),
            message
        }
    }
}

impl<'a, Message: Clone, Renderer> Widget<Message, Theme, Renderer> for PlanCard<'a, Message, Renderer>
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
        limits: &layout::Limits
    ) -> layout::Node {
        match tree.children.len() {
            0 => layout::Node::new(Size::new(502., 463.)),
            _ => {
                let mut child_node = self.content.as_widget().layout(&mut tree.children[0], renderer, limits);
                let size_of_this_node = Size::new(502., 463.);
                child_node = child_node.align(Alignment::Center, Alignment::Center, size_of_this_node);
                layout::Node::with_children(size_of_this_node, vec![child_node])
            }
        }
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

        match self.selected {
            true => {
                renderer.fill_quad(
                    Quad {
                        bounds: layout.bounds(),
                        border: Border {
                            color: Color::from_rgb(2. / 255., 84. / 255., 191. / 255.),
                            width: 1.,
                            radius: 16.0.into(),
                        },
                        shadow: Shadow::default(),
                    },
                    Color::from_rgb(227. / 255., 239. / 255., 1.),
                );
            }

            false => {
                renderer.fill_quad(
                    Quad {
                        bounds: layout.bounds(),
                        border: Border {
                            color: Color::from_rgb(205. / 255., 220. / 255., 241. / 255.),
                            width: 1.,
                            radius: 16.0.into(),
                        },
                        shadow: Shadow::default(),
                    },
                    Color::WHITE,
                );
            }
        }

        self.content.as_widget().draw(
            &state.children[0],
            renderer,
            theme,
            style,
            layout.children().next().unwrap(),
            cursor,
            viewport
        )
    }

    fn on_event(
        &mut self,
        _state: &mut Tree,
        event: Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        _renderer: &Renderer,
        _clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        _viewport: &Rectangle,
    ) -> event::Status {
        match event {
            Event::Mouse(
                mouse_enum::Event::ButtonPressed(mouse_enum::Button::Left)
            ) => {
                // self.selected = !self.selected;
                // event::Status::Captured
                if cursor.is_over(layout.bounds()) {
                    self.selected = !self.selected;
                    shell.publish(self.message.clone());
                    event::Status::Captured
                } else {
                    event::Status::Ignored
                }
            }

            _ => event::Status::Ignored,
        }
    }

    fn mouse_interaction(
        &self,
        _state: &Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        _viewport: &Rectangle,
        _renderer: &Renderer,
    ) -> mouse::Interaction {
        if cursor.is_over(layout.bounds()) {
            mouse::Interaction::Pointer
        } else {
            mouse::Interaction::Idle
        }
    }
}

impl<'a, Message, Renderer> From<PlanCard<'a, Message, Renderer>> for Element<'a, Message, Theme, Renderer>
where
    Renderer: iced::advanced::Renderer + 'a,
    Message: Clone + 'a,
{
    fn from(widget: PlanCard<'a, Message, Renderer>) -> Self {
        Self::new(widget)
    }
}