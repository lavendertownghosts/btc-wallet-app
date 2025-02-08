use iced::{
    self, Sandbox, Settings, Length, Font, Color, Background, Border, Shadow, Padding, Alignment, Gradient, theme, window, Size,
    widget::{container, container::Appearance, Svg, column, row, text, horizontal_rule, text::Shaping, button, Theme},
    font,
    alignment::Vertical,
    gradient::{Linear, ColorStop},
    border::Radius
};

mod create_plan;
use crate::create_plan::select_plan::PlanCard;

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

enum Page {
    A,
    B,
}

#[derive(Debug, Clone)]
enum MyAppMessage {
    // GoToSelectPlanType(SelectPlanTypeMessage),
    // GoToSetupPlan(SetupPlanTypeMessage),
    TimeSafePressed,
    FailSafePressed
}

struct MyApp {
    time_safe_selected: bool,
    fail_safe_selected: bool
}

impl Sandbox for MyApp {
    type Message = MyAppMessage;

    fn new() -> Self {
        // Self { page: Page::A }
        Self {
            time_safe_selected: false,
            fail_safe_selected: false
        }
    }

    fn title(&self) -> String {
        String::from("My App")
    }

    fn update(&mut self, message: Self::Message) {
        // self.page = match message {
        //     MyAppMessage::GoToBButtonPressed => Page::B,
        //     MyAppMessage::GoToAButtonPressed => Page::A,
        // }
        match message {
            MyAppMessage::TimeSafePressed => {
                self.time_safe_selected = !self.time_safe_selected;
                self.fail_safe_selected = false;
            }
            _ => {
                self.time_safe_selected = false;
                self.fail_safe_selected = !self.fail_safe_selected;
            }
        }
    }

    fn view(&self) -> iced::Element<Self::Message> {
        // match self.page {
        //     Page::A => column![
        //         text("Page A"),
        //         button("Go to B").on_press(MyAppMessage::GoToBButtonPressed),
        //     ],
        //     Page::B => column![
        //         text("Page B"),
        //         button("Go to A").on_press(MyAppMessage::GoToAButtonPressed),
        //     ],
        // }
        // .into()
        // PlanCard::new().into()
        // container(PlanCard::new()).into()
        let time_safe_content = container(column![
            Svg::from_path("assets/create-plan/guard.svg").width(Length::Fixed(40.)).height(Length::Fixed(40.)),
            column![
                text("Time Safe").size(16).font(Font {
                    weight: font::Weight::Bold,
                    ..Font::DEFAULT
                }),
                text("Lock Bitcoin until a future date").size(14).style(Color::from_rgb(113. / 255., 121. / 255., 142. / 255.))
            ].spacing(6.),
            column![
                text("Protected against").size(12).style(Color::from_rgb(113. / 255., 121. / 255., 142. / 255.)),
                row![
                    container(row![
                            Svg::from_path("assets/create-plan/checks.svg").width(Length::Fixed(16.)).height(Length::Fixed(16.)),
                            text("Wrench attacks").size(12)
                        ].spacing(4).align_items(Alignment::Center)
                    ).style(Appearance {
                        text_color: Some(Color::from_rgb(113. / 255., 121. / 255., 142. / 255.)),
                        background: Some(Background::Color(Color::from_rgba(0., 149. / 255., 42. / 255., 0.12))),
                        border: Border {
                            color: Color::from_rgba(0. / 255., 149. / 255., 42. / 255., 0.2),
                            width: 1.0,
                            radius: 100.0.into()
                        },
                        shadow: Shadow::default()
                    }).padding([4.5, 10.]),
                    container(row![
                            Svg::from_path("assets/create-plan/checks.svg").width(Length::Fixed(16.)).height(Length::Fixed(16.)),
                            text("Unforeseen risks").size(12)
                        ].spacing(4).align_items(Alignment::Center)
                    ).style(Appearance {
                        text_color: Some(Color::from_rgb(113. / 255., 121. / 255., 142. / 255.)),
                        background: Some(Background::Color(Color::from_rgba(0. / 255., 149. / 255., 42. / 255., 0.12))),
                        border: Border {
                            color: Color::from_rgba(0. / 255., 149. / 255., 42. / 255., 0.2),
                            width: 1.0,
                            radius: 100.0.into()
                        },
                        shadow: Shadow::default()
                    }).padding([4.5, 10.]),
                    container(row![
                            Svg::from_path("assets/create-plan/checks.svg").width(Length::Fixed(16.)).height(Length::Fixed(16.)),
                            text("Emotional selling").size(12)
                        ].spacing(4).align_items(Alignment::Center)
                    ).style(Appearance {
                        text_color: Some(Color::from_rgb(113. / 255., 121. / 255., 142. / 255.)),
                        background: Some(Background::Color(Color::from_rgba(0. / 255., 149. / 255., 42. / 255., 0.12))),
                        border: Border {
                            color: Color::from_rgba(0., 149., 42., 0.2),
                            width: 1.0,
                            radius: 100.0.into()
                        },
                        shadow: Shadow::default()
                    }).padding([4.5, 10.]),
                ].spacing(8)
            ].spacing(8),
            horizontal_rule(0),
            column![
                row![
                    Svg::from_path("assets/create-plan/colored-check.svg").width(Length::Fixed(16.)).height(Length::Fixed(16.)),
                    column![
                        text("Time-locked access").size(12).font(Font {
                            weight: font::Weight::Medium,
                            ..Font::DEFAULT
                        }),
                        text("Keeps your funds securely locked until a specific future date, so they can’t be accessed early")
                        .size(12)
                        .style(Color::from_rgb(113. / 255., 121. / 255., 142. / 255.))
                    ]
                ].spacing(8.),
                row![
                    Svg::from_path("assets/create-plan/colored-check.svg").width(Length::Fixed(16.)).height(Length::Fixed(16.)),
                    column![
                        text("Unbreakable protection").size(12).font(Font {
                            weight: font::Weight::Medium,
                            ..Font::DEFAULT
                        }),
                        text("Ensures your funds are protected with advanced safeguards, so only the right person can claim them at the right time")
                        .size(12)
                        .style(Color::from_rgb(113. / 255., 121. / 255., 142. / 255.))
                    ]
                ].spacing(8.),
                row![
                    Svg::from_path("assets/create-plan/colored-check.svg").width(Length::Fixed(16.)).height(Length::Fixed(16.)),
                    column![
                        text("Claimable funds after expiry").size(12).font(Font {
                            weight: font::Weight::Medium,
                            ..Font::DEFAULT
                        }),
                        text("Allows you to claim the funds after the lock period ends")
                        .size(12)
                        .style(Color::from_rgb(113. / 255., 121. / 255., 142. / 255.))
                    ]
                ].spacing(8.)
            ].spacing(15)
        ].spacing(24.)).width(438.);

        let fail_safe_content = container(column![
            Svg::from_path("assets/create-plan/guard.svg").width(Length::Fixed(40.)).height(Length::Fixed(40.)),
            column![
                text("Fail Safe Recovery").size(16).font(Font {
                    weight: font::Weight::Bold,
                    ..Font::DEFAULT
                }),
                text("A dead man’s switch transferring assets on missed check-ins")
                .size(14)
                .style(Color::from_rgb(113. / 255., 121. / 255., 142. / 255.))
            ].spacing(6.),
            column![
                text("Protected against")
                .size(12)
                .style(Color::from_rgb(113. / 255., 121. / 255., 142. / 255.)),
                row![
                    container(row![
                            Svg::from_path("assets/create-plan/checks.svg").width(Length::Fixed(16.)).height(Length::Fixed(16.)),
                            text("Private key loss").size(12)
                        ].spacing(4).align_items(Alignment::Center)
                    ).style(Appearance {
                        text_color: Some(Color::from_rgb(113. / 255., 121. / 255., 142. / 255.)),
                        background: Some(Background::Color(Color::from_rgba(0. / 255., 149. / 255., 42. / 255., 0.12))),
                        border: Border {
                            color: Color::from_rgba(0. / 255., 149. / 255., 42. / 255., 0.2),
                            width: 1.0,
                            radius: 100.0.into()
                        },
                        shadow: Shadow::default()
                    }).padding([4.5, 10.]),
                    container(row![
                            Svg::from_path("assets/create-plan/checks.svg").width(Length::Fixed(16.)).height(Length::Fixed(16.)),
                            text("Unexpected life events").size(12)
                        ].spacing(4).align_items(Alignment::Center)
                    ).style(Appearance {
                        text_color: Some(Color::from_rgb(113. / 255., 121. / 255., 142. / 255.)),
                        background: Some(Background::Color(Color::from_rgba(0. / 255., 149. / 255., 42. / 255., 0.12))),
                        border: Border {
                            color: Color::from_rgba(0. / 255., 149. / 255., 42. / 255., 0.2),
                            width: 1.0,
                            radius: 100.0.into()
                        },
                        shadow: Shadow::default()
                    }).padding([4.5, 10.]),
                ].spacing(8)
            ].spacing(8),
            horizontal_rule(0),
            column![
                row![
                    Svg::from_path("assets/create-plan/colored-check.svg").width(Length::Fixed(16.)).height(Length::Fixed(16.)),
                    column![
                        text("Backup access plan").size(12).font(Font {
                            weight: font::Weight::Medium,
                            ..Font::DEFAULT
                        }),
                        text("Ensures your BTC can be recovered through predefined steps if you lose access to your private keys")
                        .size(12)
                        .style(Color::from_rgb(113. / 255., 121. / 255., 142. / 255.))
                    ]
                ].spacing(8.),
                row![
                    Svg::from_path("assets/create-plan/colored-check.svg").width(Length::Fixed(16.)).height(Length::Fixed(16.)),
                    column![
                        text("Time-based status check-ins").size(12).font(Font {
                            weight: font::Weight::Medium,
                            ..Font::DEFAULT
                        }),
                        text("Allows your beneficiary to recover BTC if you don’t confirm your status before the check-in period ends")
                        .size(12)
                        .style(Color::from_rgb(113. / 255., 121. / 255., 142. / 255.))
                    ]
                ].spacing(8.),
                row![
                    Svg::from_path("assets/create-plan/colored-check.svg").width(Length::Fixed(16.)).height(Length::Fixed(16.)),
                    column![
                        text("Beneficiary designation").size(12).font(Font {
                            weight: font::Weight::Medium,
                            ..Font::DEFAULT
                        }),
                        text("Allows you to assign trusted beneficiaries to claim your BTC if you miss a status check-in")
                        .size(12)
                        .style(Color::from_rgb(113. / 255., 121. / 255., 142. / 255.))
                    ]
                ].spacing(8.)
            ].spacing(15)
        ].spacing(24.)).width(438.);

        // row![
        //     PlanCard::new(self.time_safe_selected, time_safe_content, MyAppMessage::TimeSafePressed),
        //     PlanCard::new(self.fail_safe_selected, fail_safe_content, MyAppMessage::FailSafePressed),
        // ].spacing(24).into()

        container(column![
            text("Create new plan").size(32).font(Font {
                weight: font::Weight::Bold,
                ..Font::DEFAULT
            }),
            text("Select plan type").size(14),
            row![
                PlanCard::new(self.time_safe_selected, time_safe_content, MyAppMessage::TimeSafePressed),
                PlanCard::new(self.fail_safe_selected, fail_safe_content, MyAppMessage::FailSafePressed),
            ].spacing(24),
            button("Continue").style(
                theme::Button::Custom(Box::new(ButtonColor {}))
                // Color::from_rgb(255., 0., 0.)
            ).padding([12., 83.])
        ].align_items(Alignment::Center).spacing(24))
        .style(Appearance {
            background: Some(Background::Gradient(Gradient::Linear(Linear {
                angle: 2.6.into(),
                stops: [
                    Some(ColorStop {
                        offset: 0.0733,
                        color: Color {
                            r: 53.0 / 255.0,
                            g: 229.0 / 255.0,
                            b: 171.0 / 255.0,
                            a: 1.0,
                        },
                    }),
                    Some(ColorStop {
                        offset: 0.5191,
                        color: Color {
                            r: 62.0 / 255.0,
                            g: 112.0 / 255.0,
                            b: 253.0 / 255.0,
                            a: 1.0,
                        },
                    }),
                    Some(ColorStop {
                        offset: 0.939,
                        color: Color {
                            r: 135.0 / 255.0,
                            g: 85.0 / 255.0,
                            b: 241.0 / 255.0,
                            a: 1.0,
                        },
                    }),
                    None, None, None, None, None,
                ]
            }))),
            ..Appearance::default()
        }).width(Length::Fill).height(Length::Fill).center_x().into()

        // PlanCard::new(fail_safe_content).into()
    }
}

struct ButtonColor {}

impl<> button::StyleSheet for ButtonColor {
    type Style = Theme;

    fn active(&self, style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(Background::Gradient(Gradient::Linear(Linear {
                angle: 0.778.into(),
                stops: [
                    Some(ColorStop {
                        offset: 0.0,
                        color: Color {
                            r: 4.0 / 255.0,
                            g: 47.0 / 255.0,
                            b: 104.0 / 255.0,
                            a: 1.0,
                        },
                    }),
                    Some(ColorStop {
                        offset: 0.951,
                        color: Color {
                            r: 2.0 / 255.0,
                            g: 84.0 / 255.0,
                            b: 191.0 / 255.0,
                            a: 1.0,
                        },
                    }),
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    // Some(ColorStop { offset: 1., color: Color { r: 2., g: 84., b: 191., a: 1. } }),
                ]
            }))),
            border: Border::with_radius(Radius::from(12.0)),
            ..style.active(&theme::Button::Primary)
        }
    }
}