use iced::widget::{column as col};
use iced::widget::{
    button, checkbox, container, horizontal_space, pick_list, row, slider, svg, text, text_input,
    toggler, vertical_slider,
};
use iced::{alignment, theme, Color};

use iced_aw::menu::{menu_tree::MenuTree, CloseCondition, ItemHeight, ItemWidth, PathHighlight};
use iced_aw::{quad};
use iced_aw::{helpers::menu_tree, menu_bar, menu_tree};

use iced::{
    widget::{Container, Text},
    Application, Command, Element, Length, Settings, Theme,
};
use iced_aw::{split, Split};

mod login;
use iced::{
    alignment::{Horizontal, Vertical},
    widget::{Column},
    Font, Sandbox,
};
use iced_aw::{TabLabel, Tabs};
use login::{LoginMessage, LoginTab};

mod ferris;
use ferris::{FerrisMessage, FerrisTab};

mod counter;
use counter::{CounterMessage, CounterTab};

mod settings;
use settings::{SettingsMessage, SettingsTab, TabBarPosition};

mod shell;
use shell::ShellViewTab;
use shell::ShellMessage;

pub fn main() -> iced::Result {
    App::run(iced::Settings {
        default_text_size: 15.0,
        window: iced::window::Settings {
            size: (1600, 1000),
            // position: iced::window::Position::Default,
            ..Default::default()
        },
        ..Default::default()
    })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SizeOption {
    Uniform,
    Static,
}
impl SizeOption {
    const ALL: [SizeOption; 2] = [SizeOption::Uniform, SizeOption::Static];
}
impl std::fmt::Display for SizeOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Uniform => "Uniform",
                Self::Static => "Static",
            }
        )
    }
}

const HEADER_SIZE: u16 = 32;
const TAB_PADDING: u16 = 16;

const ICON_FONT: Font = iced::Font::External {
    name: "Icons",
    bytes: include_bytes!("../fonts/icons.ttf"),
};

enum Icon {
    User,
    Heart,
    Calc,
    CogAlt,
}

impl From<Icon> for char {
    fn from(icon: Icon) -> Self {
        match icon {
            Icon::User => '\u{E800}',
            Icon::Heart => '\u{E801}',
            Icon::Calc => '\u{F1EC}',
            Icon::CogAlt => '\u{E802}',
        }
    }
}


#[derive(Debug, Clone)]
enum Message {
    Debug(String),
    ValueChange(u8),
    CheckChange(bool),
    ToggleChange(bool),
    ColorChange(Color),
    FlipHorizontal,
    FlipVertical,
    ThemeChange(bool),
    TextChange(String),
    SizeOption(SizeOption),
    OnVerResize(u16),
    OnHorResize(u16),
    TabSelected(TabId),
    Login(LoginMessage),
    Ferris(FerrisMessage),
    Counter(CounterMessage),
    Settings(SettingsMessage),
    Shell(ShellMessage)
}

#[derive(Clone, PartialEq, Eq, Debug)]
enum TabId {
    Login,
    Ferris,
    Counter,
    Settings,
    Shell
}
struct App {
    title: String,
    value: u8,
    check: bool,
    toggle: bool,
    theme: iced::Theme,
    flip_h: bool,
    flip_v: bool,
    dark_mode: bool,
    text: String,
    size_option: SizeOption,
    ver_divider_position: Option<u16>,
    hor_divider_position: Option<u16>,
    active_tab: TabId,
    login_tab: LoginTab,
    ferris_tab: FerrisTab,
    counter_tab: CounterTab,
    settings_tab: SettingsTab,
    shell_tab: ShellViewTab
}
impl Application for App {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = iced::Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        let theme = iced::Theme::custom(theme::Palette {
            primary: Color::from([0.45, 0.25, 0.57]),
            ..iced::Theme::Light.palette()
        });

        (
            Self {
                title: "Menu Test".to_string(),
                value: 0,
                check: false,
                toggle: false,
                theme,
                flip_h: false,
                flip_v: false,
                dark_mode: false,
                text: "Text Input".into(),
                size_option: SizeOption::Static,
                ver_divider_position: None,
                hor_divider_position: Some(200),
                active_tab: TabId::Login,
                login_tab: LoginTab::new(),
                ferris_tab: FerrisTab::new(),
                counter_tab: CounterTab::new(),
                settings_tab: SettingsTab::new(),
                shell_tab: ShellViewTab::new()
            },
            iced::Command::none(),
        )
    }

    fn theme(&self) -> Self::Theme {
        self.theme.clone()
    }

    fn title(&self) -> String {
        self.title.clone()
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message {
            Message::Debug(s) => {
                self.title = s;
            }
            Message::ValueChange(v) => {
                self.value = v;
                self.title = v.to_string();
            }
            Message::CheckChange(c) => {
                self.check = c;
                self.title = c.to_string();
            }
            Message::ToggleChange(t) => {
                self.toggle = t;
                self.title = t.to_string();
            }
            Message::ColorChange(c) => {
                self.theme = iced::Theme::custom(theme::Palette {
                    primary: c,
                    ..self.theme.palette()
                });
                self.title = format!("[{:.2}, {:.2}, {:.2}]", c.r, c.g, c.b);
            }
            Message::FlipHorizontal => self.flip_h = !self.flip_h,
            Message::FlipVertical => self.flip_v = !self.flip_v,
            Message::ThemeChange(b) => {
                self.dark_mode = b;
                let primary = self.theme.palette().primary;
                if b {
                    self.theme = iced::Theme::custom(theme::Palette {
                        primary,
                        ..iced::Theme::Dark.palette()
                    })
                } else {
                    self.theme = iced::Theme::custom(theme::Palette {
                        primary,
                        ..iced::Theme::Light.palette()
                    })
                }
            }
            Message::TextChange(s) => {
                self.text = s.clone();
                self.title = s;
            }
            Message::SizeOption(so) => {
                self.size_option = so;
                self.title = self.size_option.to_string();
            }
            Message::OnVerResize(position) => self.ver_divider_position = Some(position),
            Message::OnHorResize(position) => self.hor_divider_position = Some(position),
            Message::TabSelected(selected) => self.active_tab = selected,
            Message::Login(message) => self.login_tab.update(message),
            Message::Ferris(message) => self.ferris_tab.update(message),
            Message::Counter(message) => self.counter_tab.update(message),
            Message::Settings(message) => self.settings_tab.update(message),
            Message::Shell(message) => self.shell_tab.update(message),
        }
        iced::Command::none()
    }

    fn view(&self) -> iced::Element<'_, Self::Message, iced::Renderer<Self::Theme>> {
        let pick_size_option = pick_list(
            &SizeOption::ALL[..],
            Some(self.size_option),
            Message::SizeOption,
        );

        let mb = match self.size_option {
            SizeOption::Uniform => {
                menu_bar!(menu_1(self), menu_2(self), menu_3(self), menu_4(self))
                    .item_width(ItemWidth::Uniform(180))
                    .item_height(ItemHeight::Uniform(25))
            }
            SizeOption::Static => menu_bar!(
                menu_1(self),
                menu_2(self),
                menu_3(self),
                menu_4(self),
                menu_5(self),
            )
            .item_width(ItemWidth::Static(180))
            .item_height(ItemHeight::Static(25)),
        }
        .spacing(4.0)
        .bounds_expand(30)
        .path_highlight(Some(PathHighlight::MenuActive))
        .close_condition(CloseCondition {
            leave: true,
            click_outside: false,
            click_inside: false,
        });

        let r = if self.flip_h {
            row!(pick_size_option, horizontal_space(Length::Fill), mb,)
        } else {
            row!(mb, horizontal_space(Length::Fill), pick_size_option)
        }
        .padding([2, 8])
        .align_items(alignment::Alignment::Center);

        let top_bar_style: fn(&iced::Theme) -> container::Appearance =
            |_theme| container::Appearance {
                background: Some(Color::TRANSPARENT.into()),
                ..Default::default()
            };
        let top_bar = container(r).width(Length::Fill).style(top_bar_style);

        let back_style: fn(&iced::Theme) -> container::Appearance = |theme| container::Appearance {
            background: Some(theme.extended_palette().primary.base.color.into()),
            ..Default::default()
        };


        let position = self
            .settings_tab
            .settings()
            .tab_bar_position
            .unwrap_or_default();
        let theme = self
            .settings_tab
            .settings()
            .tab_bar_theme
            .unwrap_or_default();

        let content_tabs = Tabs::new(Message::TabSelected)
            .push(
                TabId::Shell,
                self.shell_tab.tab_label(),
                self.shell_tab.view(),
            )
            .push(
                TabId::Login,
                self.login_tab.tab_label(),
                self.login_tab.view(),
            )
            .push(
                TabId::Ferris,
                self.ferris_tab.tab_label(),
                self.ferris_tab.view(),
            )
            .push(
                TabId::Counter,
                self.counter_tab.tab_label(),
                self.counter_tab.view(),
            )
            .push(
                TabId::Settings,
                self.settings_tab.tab_label(),
                self.settings_tab.view(),
            )
            .set_active_tab(&self.active_tab)
            .tab_bar_style(theme)
            .icon_font(ICON_FONT)
            .tab_bar_position(match position {
                TabBarPosition::Top => iced_aw::TabBarPosition::Top,
                TabBarPosition::Bottom => iced_aw::TabBarPosition::Bottom,
            });
        

        let left = Container::new(Text::new("First"))
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y();

        let right = Container::new(content_tabs)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y();

        let panel = Split::new(
            left,
            right,
            self.hor_divider_position,
            split::Axis::Vertical,
            Message::OnHorResize,
        );
        let back = container(col![panel])
            .width(Length::Fill)
            .height(Length::Fill)
            .style(back_style);


        let c = if self.flip_v {
            col![back, top_bar, ]
        } else {
            col![top_bar, back]
        };
        
        c.into()
    }
}

struct ButtonStyle;
impl button::StyleSheet for ButtonStyle {
    type Style = iced::Theme;

    fn active(&self, style: &Self::Style) -> button::Appearance {
        button::Appearance {
            text_color: style.extended_palette().background.base.text,
            border_radius: 4.0,
            background: Some(Color::TRANSPARENT.into()),
            ..Default::default()
        }
    }

    fn hovered(&self, style: &Self::Style) -> button::Appearance {
        let plt = style.extended_palette();

        button::Appearance {
            background: Some(plt.primary.weak.color.into()),
            text_color: plt.primary.weak.text,
            ..self.active(style)
        }
    }
}

fn base_button<'a>(
    content: impl Into<Element<'a, Message, iced::Renderer>>,
    msg: Message,
) -> button::Button<'a, Message, iced::Renderer> {
    button(content)
        .padding([4, 8])
        .style(iced::theme::Button::Custom(Box::new(ButtonStyle {})))
        .on_press(msg)
}

fn labeled_button<'a>(label: &str, msg: Message) -> button::Button<'a, Message, iced::Renderer> {
    base_button(
        text(label)
            .width(Length::Fill)
            .height(Length::Fill)
            .vertical_alignment(alignment::Vertical::Center),
        msg,
    )
}

fn debug_button<'a>(label: &str) -> button::Button<'a, Message, iced::Renderer> {
    labeled_button(label, Message::Debug(label.into()))
}

fn debug_item<'a>(label: &str) -> MenuTree<'a, Message, iced::Renderer> {
    menu_tree!(debug_button(label).width(Length::Fill).height(Length::Fill))
}

fn color_item<'a>(color: impl Into<Color>) -> MenuTree<'a, Message, iced::Renderer> {
    let color = color.into();
    menu_tree!(base_button(circle(color), Message::ColorChange(color)))
}

fn sub_menu<'a>(
    label: &str,
    msg: Message,
    children: Vec<MenuTree<'a, Message, iced::Renderer>>,
) -> MenuTree<'a, Message, iced::Renderer> {
    let handle = svg::Handle::from_path(format!(
        "{}/caret-right-fill.svg",
        env!("CARGO_MANIFEST_DIR")
    ));
    let arrow = svg(handle)
        .width(Length::Shrink)
        .style(theme::Svg::custom_fn(|theme| svg::Appearance {
            color: Some(theme.extended_palette().background.base.text),
        }));

    menu_tree(
        base_button(
            row![
                text(label)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .vertical_alignment(alignment::Vertical::Center),
                arrow
            ],
            msg,
        )
        .width(Length::Fill)
        .height(Length::Fill),
        children,
    )
}

fn debug_sub_menu<'a>(
    label: &str,
    children: Vec<MenuTree<'a, Message, iced::Renderer>>,
) -> MenuTree<'a, Message, iced::Renderer> {
    sub_menu(label, Message::Debug(label.into()), children)
}

fn separator<'a>() -> MenuTree<'a, Message, iced::Renderer> {
    menu_tree!(quad::Quad {
        color: [0.5; 3].into(),
        border_radius: 4.0.into(),
        inner_bounds: quad::InnerBounds::Ratio(0.98, 0.1),
        ..Default::default()
    })
}

fn dot_separator<'a>() -> MenuTree<'a, Message, iced::Renderer> {
    menu_tree!(text("·························")
        .size(30)
        .width(Length::Fill)
        .height(Length::Fill)
        .horizontal_alignment(alignment::Horizontal::Center)
        .vertical_alignment(alignment::Vertical::Center))
}

fn labeled_separator(label: &'_ str) -> MenuTree<'_, Message, iced::Renderer> {
    let q_1 = quad::Quad {
        color: [0.5; 3].into(),
        border_radius: 4.0.into(),
        inner_bounds: quad::InnerBounds::Ratio(0.98, 0.1),
        ..Default::default()
    };
    let q_2 = quad::Quad {
        color: [0.5; 3].into(),
        border_radius: 4.0.into(),
        inner_bounds: quad::InnerBounds::Ratio(0.98, 0.1),
        ..Default::default()
    };

    menu_tree!(row![
        q_1,
        text(label)
            .height(Length::Fill)
            .vertical_alignment(alignment::Vertical::Center),
        q_2,
    ])
}

fn circle(color: Color) -> quad::Quad {
    let radius = 10.0;

    quad::Quad {
        color,
        inner_bounds: quad::InnerBounds::Square(radius * 2.0),
        border_radius: radius.into(),
        ..Default::default()
    }
}

fn menu_1<'a>(_app: &App) -> MenuTree<'a, Message, iced::Renderer> {
    let root = menu_tree(
        debug_button("Nested Menus"),
        vec![
            debug_item("Item"),
            debug_item("Item"),
        ],
    )
    .width(110);

    root
}

fn menu_2<'a>(app: &App) -> MenuTree<'a, Message, iced::Renderer> {
    let sub_1 = menu_tree(
        container(toggler(
            Some("Or as a sub menu item".to_string()),
            app.toggle,
            Message::ToggleChange,
        ))
        .padding([0, 8])
        .height(Length::Fill)
        .align_y(alignment::Vertical::Center),
        vec![
            debug_item("Item"),
            debug_item("Item"),
            debug_item("Item"),
            debug_item("Item"),
        ],
    );

    let bt = menu_tree!(button(
        text("Button")
            .width(Length::Fill)
            .height(Length::Fill)
            .vertical_alignment(alignment::Vertical::Center),
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .on_press(Message::Debug("Button".into())));

    let cb = menu_tree!(checkbox("Checkbox", app.check, Message::CheckChange).width(Length::Fill));

    let sld = menu_tree!(row![
        "Slider",
        horizontal_space(Length::Fixed(8.0)),
        slider(0..=255, app.value, Message::ValueChange)
    ]);

    let txn = menu_tree!(text_input("", &app.text).on_input(Message::TextChange));

    let root = menu_tree(
        debug_button("Widgets"),
        vec![
            debug_item("You can use any widget"),
            debug_item("as a menu item"),
            bt,
            cb,
            sld,
            txn,
            sub_1,
            separator(),
            debug_item("Seperators are also widgets"),
            labeled_separator("Separator"),
            debug_item("Item"),
            debug_item("Item"),
            dot_separator(),
            debug_item("Item"),
            debug_item("Item"),
        ],
    );

    root
}

fn menu_3<'a>(app: &App) -> MenuTree<'a, Message, iced::Renderer> {
    let [r, g, b, _] = app.theme.palette().primary.into_rgba8();

    let primary = debug_sub_menu(
        "Primary",
        vec![
            menu_tree!(slider(0..=255, r, move |x| {
                Message::ColorChange(Color::from_rgb8(x, g, b))
            })),
            menu_tree!(slider(0..=255, g, move |x| {
                Message::ColorChange(Color::from_rgb8(r, x, b))
            })),
            menu_tree!(slider(0..=255, b, move |x| {
                Message::ColorChange(Color::from_rgb8(r, g, x))
            })),
        ],
    );

    let root = menu_tree(
        debug_button("Controls"),
        vec![
            menu_tree!(labeled_button("Flip Horizontal", Message::FlipHorizontal)
                .width(Length::Fill)
                .height(Length::Fill)),
            menu_tree!(labeled_button("Flip Vertical", Message::FlipVertical)
                .width(Length::Fill)
                .height(Length::Fill)),
            separator(),
            menu_tree!(row![toggler(
                Some("Dark Mode".into()),
                app.dark_mode,
                Message::ThemeChange
            )]
            .padding([0, 8])),
            color_item([0.45, 0.25, 0.57]),
            color_item([0.15, 0.59, 0.64]),
            color_item([0.76, 0.82, 0.20]),
            color_item([0.17, 0.27, 0.33]),
            primary,
        ],
    );

    root
}

fn menu_4<'a>(_app: &App) -> MenuTree<'a, Message, iced::Renderer> {
    
    let root = menu_tree(
        debug_button("Scroll"),
        vec![
            debug_item("ajrs"), // 0
            debug_item("bsdfho"),
            debug_item("clkjhbf"),
            debug_item("dekjdaud"),
        ],
    );

    root
}

fn menu_5<'a>(app: &App) -> MenuTree<'a, Message, iced::Renderer> {
    let slider_count = 3;
    let slider_width = 30;
    let spacing = 4;

    let [r, g, b, _] = app.theme.palette().primary.into_rgba8();

    let sliders = menu_tree!(row![
        vertical_slider(0..=255, r, move |x| Message::ColorChange(Color::from_rgb8(
            x, g, b
        )))
        .width(30),
        vertical_slider(0..=255, g, move |x| Message::ColorChange(Color::from_rgb8(
            r, x, b
        )))
        .width(30),
        vertical_slider(0..=255, b, move |x| Message::ColorChange(Color::from_rgb8(
            r, g, x
        )))
        .width(30),
    ]
    .spacing(4))
    .height(100);

    let root = menu_tree(
        debug_button("Static"),
        vec![labeled_separator("Primary"), sliders],
    )
    .width(slider_width * slider_count + (slider_count - 1) * spacing);

    root
}

trait Tab {
    type Message;

    fn title(&self) -> String;

    fn tab_label(&self) -> TabLabel;

    fn view(&self) -> Element<'_, Self::Message> {
        let column = Column::new()
            .spacing(20)
            // .push(Text::new(self.title()).size(HEADER_SIZE))
            .push(self.content());

        Container::new(column)
            .width(Length::Fill)
            .height(Length::Fill)
            // .align_x(Horizontal::Center)
            // .align_y(Vertical::Center)
            .padding(TAB_PADDING)
            .into()
    }

    fn content(&self) -> Element<'_, Self::Message>;
}
