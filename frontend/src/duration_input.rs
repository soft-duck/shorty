use std::fmt::{Display, Formatter};
use std::mem;
use gloo_timers::callback::Timeout;

use strum::FromRepr;
use tracing::debug;
use web_sys::{Event, FocusEvent, HtmlInputElement, InputEvent, KeyboardEvent, MouseEvent};
use yew::{AttrValue, Callback, Component, Context, Html, html, NodeRef, Properties};

pub struct DurationInput {
    duration: Duration,
}

struct Duration {
    hours: u32,
    minutes: u32,
    seconds: u32,
    selection: Selection,
    sub_cursor: bool,
}

impl Duration {
    const MAX: u32 = 99 * 60 * 60 + 59 * 60 + 59;

    fn add_selected(&mut self, n: i32) -> bool {
        let secs = (self.hours * 60 * 60 + self.minutes * 60 + self.seconds) as i32;
        let add = self.selection.to_seconds() as i32 * n;
        let sum = secs + add;

        if sum.is_negative() || sum as u32 > Self::MAX || (self.get_selection_field(self.selection).unwrap() as i32 + n).is_negative() {
            return false;
        }

        self.add_to(self.selection, n);

        true
    }

    fn add_to(&mut self, to: Selection, n: i32) {
        let sum = self.get_selection_field(to).unwrap() as i32 + n;
        let reminder = sum % (to.max() + 1);
        let next = sum / (to.max() + 1);

        self.set_selection(to, reminder as u32);

        if sum > to.max() {
            self.add_to(to.left(), next);
        };
    }

    fn get_selection_field(&self, s: Selection) -> Option<u32> {
        match s {
            Selection::Hour => Some(self.hours),
            Selection::Minute => Some(self.minutes),
            Selection::Second => Some(self.seconds),
            Selection::Empty => None,
        }
    }

    fn set_selection(&mut self, s: Selection, n: u32) {
        match s {
            Selection::Hour => self.hours = n,
            Selection::Minute => self.minutes = n,
            Selection::Second => self.seconds = n,
            _ => (),
        }
    }

    fn update_selection(&mut self, s: Selection) {
        if self.selection != s {
            self.selection = s;
            self.sub_cursor = false;
        }
    }

    fn get_selection(&self) -> Selection {
        self.selection
    }

    fn type_digit(&mut self, n: u32) {
        let number = self.get_selection_field(self.selection).unwrap();

        // if self.sub_cursor {
        //     number += n;
        // } else {
        //     number += n * 10;
        // }

        self.sub_cursor != self.sub_cursor;
    }
}

#[derive(FromRepr, Copy, Clone, PartialEq)]
enum Selection {
    Hour = 0,
    Minute = 1,
    Second = 2,
    Empty = 4,
}

impl Selection {
    fn from_cursor_position(cursor: u32) -> Self {
        if cursor > 8 {
            panic!()
        }

        Self::from_repr((cursor / 3) as usize).unwrap()
    }

    fn selection_start(&self) -> u32 {
        if *self == Self::Empty {
            panic!()
        }

        (*self as usize * 3) as u32
    }

    fn selection_end(&self) -> u32 {
        self.selection_start() + 2
    }

    fn left(&self) -> Self {
        if *self == Self::Hour {
            return *self;
        }

        Self::from_repr(*self as usize - 1).unwrap()
    }

    fn right(&self) -> Self {
        if *self == Self::Second {
            return *self;
        }

        Self::from_repr(*self as usize + 1).unwrap()
    }

    fn max(&self) -> i32 {
        match self {
            Selection::Hour => 99,
            Selection::Minute | Selection::Second => 59,
            _ => panic!()
        }
    }

    fn to_seconds(&self) -> u32 {
        match self {
            Selection::Hour => 60 * 60,
            Selection::Minute => 60,
            Selection::Second => 1,
            Selection::Empty => 0
        }
    }
}

impl Display for Duration {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:02}:{:02}:{:02}", self.hours, self.minutes, self.seconds))
    }
}

#[derive(Properties, PartialEq)]
pub struct DurationInputProps {
    pub name: Option<AttrValue>,
    pub input_ref: NodeRef,
}

fn unselect(input: HtmlInputElement) -> u32 {
    let direction = input.selection_direction().unwrap().unwrap();

    let mut start = input.selection_start().unwrap().unwrap();
    let mut end = input.selection_end().unwrap().unwrap();

    if direction == "forward" {
        input.set_selection_end(Some(start)).unwrap();
        mem::swap(&mut start, &mut end);
    } else {
        input.set_selection_start(Some(end)).unwrap();
    };

    start
}

pub enum DurationInputMessage {
    Select(Selection),
    Key(Key),
}

enum Key {
    Right,
    Left,
    Up,
    Down,
    Digit(i32),
}

impl Component for DurationInput {
    type Message = DurationInputMessage;
    type Properties = DurationInputProps;

    fn create(_: &Context<Self>) -> Self {
        Self {
            duration: Duration {
                hours: 0,
                minutes: 0,
                seconds: 0,
                selection: Selection::Empty,
                sub_cursor: false
            }
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let input = ctx.props().input_ref.cast::<HtmlInputElement>().unwrap();

        // TODO remove this and figure out a better way, but this works for now
        fn remove_me(a: &DurationInput, ctx: &Context<DurationInput>) {
            let s = a.duration.get_selection();
            let c = ctx.link().callback(move |_| {
                DurationInputMessage::Select(s)
            });
            let t = Timeout::new(1, move || {
                c.emit(());
            });
            t.forget();
        }

        match msg {
            DurationInputMessage::Select(s) => {
                self.duration.selection = s;
            }
            DurationInputMessage::Key(key) => match key {
                Key::Right => self.duration.update_selection(self.duration.selection.right()),
                Key::Left => self.duration.update_selection(self.duration.selection.left()),
                Key::Up => {
                    self.duration.add_selected(1);
                    remove_me(&self, ctx);
                },
                Key::Down => {
                    self.duration.add_selected(-1);
                    remove_me(&self, ctx);
                },
                Key::Digit(n) => debug!("Pressed: {}", n),
            },
            _ => (),
        }

        input.set_selection_start(Some(self.duration.get_selection().selection_start())).unwrap();
        input.set_selection_end(Some(self.duration.get_selection().selection_end())).unwrap();

        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        /*
            inputBox.addEventListener('keydown', handleKeydown);
            // selects a block of hours, minutes etc (useful when focused by keyboard: Tab)
            inputBox.addEventListener('focus', handleInputFocus);
            // selects a block of hours, minutes etc (useful when clicked on PC or tapped on mobile)
            inputBox.addEventListener('mouseup', handleClickFocus);
            inputBox.addEventListener('change', insertAndApplyValidations);
            // prefer 'input' event over 'keyup' for soft keyboards on mobile
            inputBox.addEventListener('input', handleUserInput);
            inputBox.addEventListener('blur', handleInputBlur);
            inputBox.addEventListener('drop', cancelDefaultEvent);
        */

        let send_key = ctx.link().callback(|key| {
            DurationInputMessage::Key(key)
        });

        let onkeydown = Callback::from(move |event: KeyboardEvent| {
            match event.code().as_str() {
                "ArrowLeft" => send_key.emit(Key::Left),
                "ArrowRight" => send_key.emit(Key::Right),
                "ArrowUp" => send_key.emit(Key::Up),
                "ArrowDown" => send_key.emit(Key::Down),
                d if d.starts_with("Digit") => {
                    let n = d.chars().last().unwrap().to_digit(10).unwrap();
                    send_key.emit(Key::Digit(n as i32));
                },
                _ => (),
            }

            event.prevent_default();
        });

        let select = ctx.link().callback(|selection| {
            DurationInputMessage::Select(selection)
        });

        let input_ref = ctx.props().input_ref.clone();
        let onmouseup = Callback::from(move |event: MouseEvent| {
            let input = input_ref.cast::<HtmlInputElement>().unwrap();
            let cursor = unselect(input);

            select.emit(Selection::from_cursor_position(cursor));

            event.prevent_default();
        });

        let onpaste = Callback::from(|event: Event| {
            event.prevent_default();
        });

        html! {
            <>
                <input onpaste={ onpaste } onkeydown={ onkeydown } onmouseup={ onmouseup } ref={ ctx.props().input_ref.clone() } name={ ctx.props().name.clone() } pattern="^[0-9]{1,2}:[0-5][0-9]:[0-5][0-9]$" style="text-align: right;" type="text" value={ format!("{}", self.duration) }/>
            </>
        }
        // pattern=r#"^(?:\d{1,2}:){0,2}(:?\d{1,2})?$"#
    }
}