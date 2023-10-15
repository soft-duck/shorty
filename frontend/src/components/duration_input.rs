use std::mem;

use strum::FromRepr;
use web_sys::{DragEvent, Event, FocusEvent, HtmlInputElement, KeyboardEvent, MouseEvent};
use yew::{
    classes,
    html,
    AttrValue,
    Callback,
    Classes,
    Component,
    Context,
    Html,
    NodeRef,
    Properties,
};

use super::TEXT_INPUT;
use crate::{
    types::duration::{Duration, Parts},
    util::AsClasses,
};

fn cursor_location(input: &HtmlInputElement) -> u32 {
    let direction = input.selection_direction().unwrap().unwrap();

    let mut start = input.selection_start().unwrap().unwrap();
    let mut end = input.selection_end().unwrap().unwrap();

    if direction == "forward" {
        mem::swap(&mut start, &mut end);
    }

    start
}

#[derive(FromRepr, Copy, Clone, PartialEq)]
pub enum Selection {
    Days = 0,
    Hours = 1,
    Minutes = 2,
    Seconds = 3,
}

impl Selection {
    fn select(&self, input: &HtmlInputElement) {
        let start = (*self as usize * 3) as u32;
        input.set_selection_start(Some(start)).unwrap();
        input.set_selection_end(Some(start + 2)).unwrap();
    }

    fn from_cursor(cursor: u32) -> Self {
        if cursor > 11 {
            unreachable!("should not occur as the input field gets reset on input");
        }

        Self::from_repr(cursor as usize / 3).unwrap()
    }

    fn left(&self) -> Self {
        if *self == Self::Days {
            return *self;
        }

        Self::from_repr(*self as usize - 1).unwrap()
    }

    fn right(&self) -> Self {
        if *self == Self::Seconds {
            return *self;
        }

        Self::from_repr(*self as usize + 1).unwrap()
    }

    fn part_with(&self, n: i64) -> Parts {
        let mut parts = Parts {
            days: 0,
            hours: 0,
            minutes: 0,
            seconds: 0,
        };

        match self {
            Selection::Days => parts.days = n,
            Selection::Hours => parts.hours = n,
            Selection::Minutes => parts.minutes = n,
            Selection::Seconds => parts.seconds = n,
        }

        parts
    }
}

#[derive(PartialEq)]
pub enum DurationInputMessage {
    Key(Key),
    MouseUp,
    Focus(bool),
}

#[derive(PartialEq)]
pub enum Key {
    Arrow(Arrow),
    Digit(i64),
    Backspace,
}

#[derive(PartialEq)]
pub enum Arrow {
    Right,
    Left,
    Up,
    Down,
}

#[derive(Properties, PartialEq)]
pub struct DurationInputProps {
    pub input_ref: NodeRef,
    #[prop_or_default]
    pub class: Option<Classes>,
    pub id: Option<AttrValue>,
}

pub struct DurationInput {
    duration: Duration,
    selection: Option<Selection>,
    backspace: bool,
    pub sub_cursor: bool,
}

impl DurationInput {
    fn handle_key(&mut self, ctx: &Context<Self>, key: Key) {
        match key {
            Key::Arrow(arrow) => self.handle_arrow(ctx, arrow),
            Key::Digit(d) => self.handle_digit(d),
            Key::Backspace => self.handle_backspace(),
        }
    }

    // TODO maybe handle the first digit as a ones digit if no further input occurs and else handle it as tens digit
    fn handle_digit(&mut self, mut n: i64) {
        let sub_cursor = self.sub_cursor;

        if !sub_cursor {
            n *= 10;
        }

        let parts = self.selection.unwrap().part_with(n);

        if parts.valid() {
            if !sub_cursor {
                let mut zero_parts = self.duration.to_parts();
                zero_parts.zero_selection(self.selection.unwrap());
                self.duration.seconds = zero_parts.to_seconds();
            }

            self.duration.add_parts(parts);
            self.sub_cursor = !self.sub_cursor;

            if sub_cursor {
                self.selection = Some(self.selection.unwrap().right());
            }
        }
    }

    fn handle_backspace(&mut self) {
        self.sub_cursor = false;

        if self.backspace {
            self.backspace = false;
            self.sub_cursor = false;
            self.duration.reset();
        } else {
            self.backspace = true;

            let mut parts = self.duration.to_parts();
            parts.zero_selection(self.selection.unwrap());
            self.duration.seconds = parts.to_seconds();
        }
    }

    fn handle_arrow(&mut self, ctx: &Context<Self>, arrow: Arrow) {
        let input = ctx.props().input_ref.cast::<HtmlInputElement>().unwrap();
        self.sub_cursor = false;

        match arrow {
            Arrow::Right => {
                self.selection = Some(self.selection.unwrap().right());
                self.selection.unwrap().select(&input);
            },
            Arrow::Left => {
                self.selection = Some(self.selection.unwrap().left());
                self.selection.unwrap().select(&input);
            },
            Arrow::Up => {
                let parts = self.selection.unwrap().part_with(1);
                self.duration.add_parts(parts);
            },
            Arrow::Down => {
                let parts = self.selection.unwrap().part_with(-1);
                self.duration.add_parts(parts);
            },
        }
    }

    fn handle_mouseup(&mut self, ctx: &Context<Self>) {
        let input = ctx.props().input_ref.cast::<HtmlInputElement>().unwrap();

        let cursor = cursor_location(&input);
        let selection = Selection::from_cursor(cursor);
        self.selection = Some(selection);
        self.sub_cursor = false;
    }

    fn handle_focus(&mut self, ctx: &Context<Self>, focus: bool) {
        let input = ctx.props().input_ref.cast::<HtmlInputElement>().unwrap();

        if focus {
            if self.selection.is_none() {
                let cursor = cursor_location(&input);
                let selection = Selection::from_cursor(cursor);
                self.selection = Some(selection);
            }
        } else {
            self.selection = None;
        }
    }
}

impl Component for DurationInput {
    type Message = DurationInputMessage;
    type Properties = DurationInputProps;

    fn create(_: &Context<Self>) -> Self {
        Self {
            duration: Duration { seconds: 0 },
            selection: None,
            sub_cursor: false,
            backspace: false,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        use DurationInputMessage as Msg;

        if msg != Msg::Key(Key::Backspace) {
            self.backspace = false;
        }

        match msg {
            Msg::MouseUp => self.handle_mouseup(ctx),
            Msg::Focus(focus) => self.handle_focus(ctx, focus),
            Msg::Key(key) if self.selection.is_some() => self.handle_key(ctx, key),
            _ => (),
        }

        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        /*
            TODO
                - make tab focus better
                - maybe find a way to overwrite onmousedown
                - prevent dragging stuff out
                - add up/down buttons
        */

        let send_key = ctx.link().callback(|key| DurationInputMessage::Key(key));

        // TODO allow numpad and other numbers
        let onkeydown = Callback::from(move |event: KeyboardEvent| {
            match event.code().as_str() {
                "ArrowLeft" => send_key.emit(Key::Arrow(Arrow::Left)),
                "ArrowRight" => send_key.emit(Key::Arrow(Arrow::Right)),
                "ArrowUp" => send_key.emit(Key::Arrow(Arrow::Up)),
                "ArrowDown" => send_key.emit(Key::Arrow(Arrow::Down)),
                "Backspace" => send_key.emit(Key::Backspace),
                d if d.starts_with("Digit") => {
                    let n = d.chars().last().unwrap().to_digit(10).unwrap();
                    send_key.emit(Key::Digit(n as i64));
                },
                _ => (),
            }

            event.prevent_default();
        });

        let onmouseup = ctx.link().callback(|event: MouseEvent| {
            event.prevent_default();
            DurationInputMessage::MouseUp
        });

        let onpaste = Callback::from(|event: Event| {
            event.prevent_default();
        });

        let onfocus = ctx
            .link()
            .callback(|_: FocusEvent| DurationInputMessage::Focus(true));

        let onblur = ctx
            .link()
            .callback(|_: FocusEvent| DurationInputMessage::Focus(false));

        let ondrop = Callback::from(|event: DragEvent| {
            event.prevent_default();
        });

        html! {
            <>
                <input
                    { onblur }
                    { onfocus }
                    { ondrop }
                    { onpaste }
                    { onkeydown }
                    { onmouseup }
                    class={ classes!(TEXT_INPUT.as_classes(), "duration", ctx.props().class.clone()) }
                    ref={ ctx.props().input_ref.clone() }
                    id={ ctx.props().id.clone() }
                    pattern="^\\d{2}:(0\\d|1\\d|2[0-3]):[0-5]\\d:[0-5]\\d$"
                    style="text-align: right;"
                    type="text"
                    value={ format!("{}", self.duration) }/>
            </>
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, _: bool) {
        let input = ctx.props().input_ref.cast::<HtmlInputElement>().unwrap();

        if let Some(selection) = self.selection {
            selection.select(&input);
        }
    }
}
