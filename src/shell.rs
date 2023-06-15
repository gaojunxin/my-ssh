use iced::widget::{column as col, TextInput, Button, Column, Row};
use iced::widget::{
    button, checkbox, container, horizontal_space, pick_list, row, slider, svg, text, text_input,
    toggler, vertical_slider,
};
use iced::{alignment, theme, Color, Sandbox};

use iced_aw::menu::{menu_tree::MenuTree, CloseCondition, ItemHeight, ItemWidth, PathHighlight};
use iced_aw::{quad, TabLabel};
use iced_aw::{helpers::menu_tree, menu_bar, menu_tree};

use iced::{
    widget::{Container, Text},
    Element, Length, Settings, Theme,
};
use crate::{Icon, Message, Tab};
use std::process::Command;


pub struct ShellViewTab {
    output: String,
    input:  String,
    submit_button_state: String,
}

#[derive(Debug, Clone)]
pub enum ShellMessage {
    SubmitInput,
    InputChanged(String),
    DataChanged(String),
}

impl ShellViewTab {
    pub fn new() -> Self {
        ShellViewTab {
            output: String::new(),
            input: String::new(),
            submit_button_state: String::new(),
        }
    }

    pub fn update(&mut self, message: ShellMessage) {
        match message {
            ShellMessage::SubmitInput => {
                // 处理用户输入并模拟终端命令执行
                let output = Command::new(&self.input)
                    .output()
                    .expect("Failed to execute command");

                if output.status.success() {
                    let stdout = String::from_utf8_lossy(&output.stdout);
                    self.output.push_str(&format!("$ {}\n", stdout));
                } else {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    self.output.push_str(&format!("$ {}\n", stderr));
                }
               
                // 清空输入框
                self.input.clear();
            }
            ShellMessage::InputChanged(value) => {
                self.input = value;
            }
            ShellMessage::DataChanged(mut data) => {
                data.truncate(100);
                self.input = data;
            }
        }
    }
}

impl Tab for ShellViewTab {
    type Message = Message;

    fn title(&self) -> String {
        String::from("Terminal")
    }

    fn tab_label(&self) -> TabLabel {
        //TabLabel::Text(self.title())
        TabLabel::IconText(Icon::CogAlt.into(), self.title())
    }

    fn content(&self) -> Element<'_, Self::Message> {
        // 显示终端输出的区域
        let output_text = Text::new(&self.output).size(20);

        // 输入区域
        // let input_field = TextInput::new(
        //     "",
        //     "Enter command",
        // )
        // .size(30)
        // .padding(10)
        // .on_submit(Message::SubmitInput);
        let input_field =
            text_input("Type the data of your QR code here...", &self.input)
                .on_input(ShellMessage::DataChanged)
                .size(30)
                .padding(15)
                .on_submit(ShellMessage::SubmitInput);

        let out_view = Column::new()
            .spacing(10)
            .push(output_text)
            .push(Row::new().spacing(10).push(input_field));

        // 将内容放入居中的容器中
        let content: Element<'_, ShellMessage> = Container::new(out_view).into();

        content.map(Message::Shell)
    }
}
