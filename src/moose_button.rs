//This module is supposed to house the custom widget,
// "MooseButton", which can be inputed by "enter" or mouse input.
//It is essentially a conrod button with a little extra functionality.
//Copy and paste is your friend.

#![macro_use]
extern crate conrod_derive;

use cmoose;
use xmoose;

use conrod::{Color, Colorable, FontSize, Borderable, Labelable, Positionable, Sizeable, UiCell, Widget};
//use image;
use color;
use conrod::position::{self, Align, Rect, Scalar};
use conrod::text;
use conrod::widget;
use self::conrod_derive::{WidgetCommon,WidgetStyle};
use std::sync::Mutex;

/// A pressable button widget whose reaction is triggered upon release.
#[derive(Clone, WidgetCommon)]
pub struct MooseButton<'a, S> {
    #[conrod(common_builder)]
    common: widget::CommonBuilder,
    maybe_label: Option<&'a str>,
    /// Whether the `Button` is a `Flat` color or an `Image`.
    pub show: S,
    /// Unique styling parameters for the Button.
    pub style: Style,
    /// Whether or not user input is enabled.
    //NB, here there is a small difference from the standard conrod button,
    // as this button can be enabled or disabled for keyboard.
    mouse_enabled: bool,
    enter_enabled: bool,
    //A reference to the Advanced Widget Cycler.
    //Because rust AND conrod are special.
    awc: &'a Mutex<cmoose::AdvWidgetCycler>,
}

/// Unique styling for the Button.
#[derive(Copy, Clone, Debug, Default, PartialEq, WidgetStyle)]
pub struct Style {
    /// Color of the Button's pressable area.
    #[conrod(default = "theme.shape_color")]
    pub color: Option<Color>,
    /// Width of the border surrounding the button
    #[conrod(default = "theme.border_width")]
    pub border: Option<Scalar>,
    /// The color of the border.
    #[conrod(default = "theme.border_color")]
    pub border_color: Option<Color>,
    /// The color of the Button's label.
    #[conrod(default = "theme.label_color")]
    pub label_color: Option<Color>,
    /// The font size of the Button's label.
    #[conrod(default = "theme.font_size_medium")]
    pub label_font_size: Option<FontSize>,
    /// The ID of the font used to display the label.
    #[conrod(default = "theme.font_id")]
    pub label_font_id: Option<Option<text::font::Id>>,
    /// The label's typographic alignment over the *x* axis.
    #[conrod(default = "text::Justify::Center")]
    pub label_justify: Option<text::Justify>,
    /// The position of the title bar's `Label` widget over the *x* axis.
    #[conrod(default = "position::Relative::Align(Align::Middle)")]
    pub label_x: Option<position::Relative>,
    /// The position of the title bar's `Label` widget over the *y* axis.
    #[conrod(default = "position::Relative::Align(Align::Middle)")]
    pub label_y: Option<position::Relative>,

    //This bit controls the style of the marker
    #[conrod(default = "position::Relative::Align(Align::Start)")]
    pub marker_x: Option<position::Relative>,
    /// The position of the title bar's `Label` widget over the *y* axis.
    #[conrod(default = "position::Relative::Align(Align::Middle)")]
    pub marker_y: Option<position::Relative>,
    #[conrod(default = "theme.shape_color.complement()")]
    pub marker_color: Option<Color>,
}


widget_ids! {
    /// Identifiers for a "flat" MooseButton.
    //NB- they all come with their own marker.
    //Which should be set only on hover.
    #[allow(missing_docs, missing_copy_implementations)]
    pub struct FlatIds {
        rectangle,
        label,
        marker,
    }
}

widget_ids! {
    /// Identifiers for an image MooseButton.
    //NB- they all come with their own marker.
    //which should be set only on hover.
    #[allow(missing_docs, missing_copy_implementations)]
    pub struct ImageIds {
        image,
        label,
        marker,
    }
}

/// The `Button` simply displays a flat color.
#[derive(Copy, Clone, Default, PartialEq, Debug)]
pub struct Flat {
    /// Allows specifying a color to use when the mouse hovers over the button.
    ///
    /// By default, this is `color.highlighted()` where `color` is the button's regular color.
    pub hover_color: Option<Color>,
    /// Allows specifying a color to use when the mouse presses the button.
    ///
    /// By default, this is `color.clicked()` where `color` is the button's regular color.
    pub press_color: Option<Color>,
}

/// The `Button` displays an `Image` on top.
#[derive(Copy, Clone)]
pub struct Image {
    /// The id of the `Image` to be used.
    pub image_id: conrod::image::Id,
    /// The image displayed when the mouse hovers over the button.
    pub hover_image_id: Option<conrod::image::Id>,
    /// The image displayed when the mouse has captured and is pressing the button.
    pub press_image_id: Option<conrod::image::Id>,
    /// If `Some`, maps the image's luminance to this `Color`.
    pub color: ImageColor,
    /// The rectangular area of the original source image that should be displayed.
    pub src_rect: Option<Rect>,
}

/// The coloring of the `Image`.
#[derive(Copy, Clone, Debug)]
pub enum ImageColor {
    /// The image's luminance will be mapped to this color.
    Normal(Color),
    /// The image's luminance will be mapped to this color.
    ///
    /// The color will change slightly upon interaction to provide visual feedback.
    WithFeedback(Color),
    /// The image's regular color will be used.
    None,
}


//This
#[derive(Copy, Clone,PartialEq)]
pub enum Interaction { Idle, MouseHover, EnterHover, MousePress, EnterPress }

/// The `Event` type yielded by the `Button` widget.
///
/// Represents the number of times that the `Button` has been clicked with the left mouse button
/// since the last update.
//NB, this is also slightly different from the standard button.
//NB2: Has a really unnecessarily cheesy name.
#[derive(Clone, Debug)]
#[allow(missing_copy_implementations)]
pub struct HeadButts {
	pub mouse_clicks: u16,
	pub enter_clicks: u16,
}

//IntarectionAndTimedTriggered...
impl HeadButts {
    /// `true` if the `Button` was clicked one or more times.
    pub fn was_clicked(self) -> bool { self.mouse_clicks > 0 }
    pub fn was_pressed(self) -> bool { self.enter_clicks > 0 }
}

//NB, this is also modified. NB- mouse clicks always take priority over keyboard.
impl Iterator for HeadButts {
    type Item = ();
    fn next(&mut self) -> Option<Self::Item> {
        if self.mouse_clicks > 0 {
            self.mouse_clicks-= 1;
            Some(())
        }else if self.enter_clicks > 0 {
            self.enter_clicks-= 1;
            Some(())
        } else {
            None
        }
    }
}


impl<'a> MooseButton<'a, Image> {

    /// Begin building a button displaying the given `Image` on top.
    pub fn image(image_id: conrod::image::Id,
				 awc:&'a Mutex<cmoose::AdvWidgetCycler>) -> Self {
        let image = Image {
            image_id: image_id,
            hover_image_id: None,
            press_image_id: None,
            src_rect: None,
            color: ImageColor::None,
        };
        Self::new_internal(image,awc)
    }

    /// The rectangular area of the image that we wish to display.
    ///
    /// If this method is not called, the entire image will be used.
    pub fn source_rectangle(mut self, rect: Rect) -> Self {
        self.show.src_rect = Some(rect);
        self
    }

    /// Map the `Image`'s luminance to the given color.
    pub fn image_color(mut self, color: Color) -> Self {
        self.show.color = ImageColor::Normal(color);
        self
    }

    /// Map the `Image`'s luminance to the given color.
    ///
    /// The color will change slightly when the button is highlighted or clicked to give the user
    /// some visual feedback.
    pub fn image_color_with_feedback(mut self, color: Color) -> Self {
        self.show.color = ImageColor::WithFeedback(color);
        self
    }

    /// The image displayed while the mouse hovers over the `Button`.
    pub fn hover_image(mut self, id: conrod::image::Id) -> Self {
        self.show.hover_image_id = Some(id);
        self
    }

    /// The image displayed while the `Button` is pressed.
    pub fn press_image(mut self, id: conrod::image::Id) -> Self {
        self.show.press_image_id = Some(id);
        self
    }

}

impl<'a> MooseButton<'a, Flat> {

    /// Begin building a flat-colored `Button` widget.
    pub fn new(awc:&'a Mutex<cmoose::AdvWidgetCycler>) -> Self {
        Self::new_internal(Flat::default(),awc)
    }

    /// Override the default button style
    pub fn with_style(mut self, s: Style) -> Self {
        self.style = s;
        self
    }

    /// Specify a color to use when the mouse hovers over the button.
    ///
    /// By default, this is `color.highlighted()` where `color` is the button's regular color.
    pub fn hover_color(mut self, color: Color) -> Self {
        self.show.hover_color = Some(color);
        self
    }

    /// Specify a color to use when the mouse presses the button.
    ///
    /// By default, this is `color.clicked()` where `color` is the button's regular color.
    pub fn press_color(mut self, color: Color) -> Self {
        self.show.press_color = Some(color);
        self
    }
}

impl<'a, S> MooseButton<'a, S> {

    /// Create a button context to be built upon.
    fn new_internal(show: S,awc:&'a Mutex<cmoose::AdvWidgetCycler>) -> Self {
        MooseButton {
            common: widget::CommonBuilder::default(),
            show: show,
            maybe_label: None,
            style: Style::default(),
            mouse_enabled: true,
            enter_enabled: true,
            awc:awc,
        }
    }

    /// Specify the font used for displaying the label.
    pub fn label_font_id(mut self, font_id: text::font::Id) -> Self {
        self.style.label_font_id = Some(Some(font_id));
        self
    }

    /// Align the label to the left of the `Button`'s surface.
    pub fn left_justify_label(mut self) -> Self {
        self.style.label_justify = Some(text::Justify::Left);
        self
    }

    /// Align the label to the mid-left of the `Button`'s surface.
    ///
    /// This is the default label alignment.
    pub fn center_justify_label(mut self) -> Self {
        self.style.label_justify = Some(text::Justify::Center);
        self
    }

    /// Align the label to the mid-left of the `Button`'s surface.
    pub fn right_justify_label(mut self) -> Self {
        self.style.label_justify = Some(text::Justify::Right);
        self
    }

    /// Specify the label's position relatively to `Button` along the *x* axis.
    pub fn label_x(mut self, x: position::Relative) -> Self {
        self.style.label_x = Some(x);
        self
    }

    /// Specify the label's position relatively to `Button` along the *y* axis.
    pub fn label_y(mut self, y: position::Relative) -> Self {
        self.style.label_y = Some(y);
        self
    }

    /// Change the marker's position on x-axis.
    pub fn marker_x(mut self, x: position::Relative) -> Self {
        self.style.marker_x = Some(x);
        self
    }

    /// Chnage the marker's position on y-axis.
    pub fn marker_y(mut self, y: position::Relative) -> Self {
        self.style.marker_y = Some(y);
        self
    }

	//Basically to set this kind of button into a shadow UiCell called "AdvWidgetCycler"
	pub fn set_outer
	(self, awc:&'a Mutex<cmoose::AdvWidgetCycler>,
				id: conrod::widget::Id,
				ui: &mut conrod::UiCell)
	-> <Self as Widget>::Event

	where MooseButton<'a, S>: conrod::Widget
	{
		//Stuff to do with the AdvWidgetCycler goes here...
		//SAFETIFY THIS.
		awc.lock().ok().unwrap().mark_as_set(id,cmoose::MooseWidgetType::MooseButton);
		self.set(id,ui)
	}

    builder_methods!{
        pub mouse_enabled { mouse_enabled = bool }
        pub enter_enabled { enter_enabled = bool }
    }
}

//the below implementations require a customised interaction and times triggered to change anything.
impl<'a> Widget for MooseButton<'a, Flat> {
    type State = FlatIds;
    type Style = Style;
    type Event = HeadButts;

    fn init_state(&self, id_gen: widget::id::Generator) -> Self::State {
        FlatIds::new(id_gen)
    }

    fn style(&self) -> Style {
        self.style.clone()
    }

    /// Update the state of the Button.
    fn update(self, args: widget::UpdateArgs<Self>) -> Self::Event {
        let widget::UpdateArgs { id, state, style, rect, ui, .. } = args;
        let MooseButton { show, maybe_label, awc, .. } = self;

        let (interaction, times_triggered_m, times_triggered_e) = interaction_and_times_triggered(id,awc,ui);
        let color = match interaction {
            Interaction::Idle|Interaction::EnterHover => style.color(&ui.theme),
            Interaction::MouseHover => show.hover_color.unwrap_or_else(|| style.color(&ui.theme).highlighted()),
            Interaction::MousePress|Interaction::EnterPress => show.press_color
                .unwrap_or_else(|| style.color(&ui.theme).clicked()),
        };

        bordered_rectangle(id, state.rectangle, rect, color, style, ui);

        // Label widget.
        if let Some(l) = maybe_label {
            label(id, state.label, l, style, ui);
        }

        HeadButts{
			mouse_clicks: times_triggered_m,
			enter_clicks: times_triggered_e,
		}
    }

}

impl<'a> Widget for MooseButton<'a, Image> {
    type State = ImageIds;
    type Style = Style;
    type Event = HeadButts;

    fn init_state(&self, id_gen: widget::id::Generator) -> Self::State {
        ImageIds::new(id_gen)
    }

    fn style(&self) -> Style {
        self.style.clone()
    }

    /// Update the state of the Button.
    fn update(self, args: widget::UpdateArgs<Self>) -> Self::Event {
        let widget::UpdateArgs { id, state, style, rect, ui, .. } = args;
        let MooseButton { show, maybe_label, awc, .. } = self;

        let (interaction, times_triggered_m, times_triggered_e) = interaction_and_times_triggered(id,awc,ui);

        // Instantiate the image.
        let Image { image_id, press_image_id, hover_image_id, src_rect, color } = show;

        // Determine the correct image to display.
        let image_id = match interaction {
            Interaction::Idle|Interaction::EnterHover => image_id,
            Interaction::MouseHover => hover_image_id.unwrap_or(image_id),
            Interaction::MousePress|Interaction::EnterPress =>
				press_image_id.or(hover_image_id).unwrap_or(image_id),
        };

        let (x, y, w, h) = rect.x_y_w_h();
        let mut image = widget::Image::new(image_id)
            .x_y(x, y)
            .w_h(w, h)
            .parent(id)
            .graphics_for(id);
        image.src_rect = src_rect;
        image.style.maybe_color = match color {
            ImageColor::Normal(color) => Some(Some(color)),
            ImageColor::WithFeedback(color) =>
                ui.widget_input(id).mouse()
                    .map(|mouse| if mouse.buttons.left().is_down() {
                        Some(color.clicked())
                    } else {
                        Some(color.highlighted())
                    })
                    .or(Some(Some(color))),
            ImageColor::None => None,
        };
        image.set(state.image, ui);

        if let Some(s) = maybe_label {
            label(id, state.label, s, style, ui);
        }

        HeadButts{
			mouse_clicks: times_triggered_m,
			enter_clicks: times_triggered_e,
		}
    }

}

//Currently only does things for mouse.
fn interaction_and_times_triggered(button_id: widget::Id,
								   awc: & Mutex<cmoose::AdvWidgetCycler>,
								   ui: &UiCell)
	-> (Interaction, u16,u16) {
	let global_events = ui.global_input().events();
    let input = ui.widget_input(button_id);

    //First phase- if interaction is mouse all is good and we are resolved.
    let mut interaction = input.mouse().map_or(Interaction::Idle, |mouse| {
        let is_pressed =
            mouse.buttons.left().is_down()
            || ui.global_input().current.touch.values()
                 .any(|t| t.start.widget == Some(button_id));
        if is_pressed { Interaction::MousePress } else { Interaction::MouseHover }
    });

    let mut keyboard_ui_presses:u16 = 0;
    let mut keyboard_raw_presses:u16 = 0;
    //Second phase- if we are idle, we query global input.
    if interaction!=Interaction::MousePress {
		//SAFETIFY THIS!!!
		match awc.lock().ok().unwrap().current() {
			Some(button_id) => {
				//interaction = Interaction::EnterHover;
				'e:for event in global_events {
					match event {
						conrod::event::Event::Raw(conrod::event::Input::Release(but)) => {
							match but {
								conrod::input::Button::Keyboard(conrod::input::Key::Return)|
								conrod::input::Button::Keyboard(conrod::input::Key::Return2) => {
									interaction = Interaction::EnterPress;
									keyboard_raw_presses+= 1;
								},
								_ => {},
							};
						},
						conrod::event::Event::Ui(conrod::event::Ui::Release(_,rel))  => {
							match rel.button {
								conrod::event::Button::Keyboard(conrod::input::Key::Return)|
								conrod::event::Button::Keyboard(conrod::input::Key::Return2) => {
									interaction = Interaction::EnterPress;
									keyboard_ui_presses+= 1;
								},
								_ => {},
							};
						},
						_ => {},
					};
				};
			},
			_  => {},
		};
	};


    let times_triggered_mouse = (input.clicks().left().count() + input.taps().count()) as u16;
    let times_triggered_keyboard = keyboard_raw_presses + keyboard_ui_presses;
    let times_triggered_keyboard = if times_triggered_keyboard > 0 {1}else{0};
    (interaction, times_triggered_mouse, times_triggered_keyboard)
}

fn bordered_rectangle(button_id: widget::Id, rectangle_id: widget::Id,
                      rect: Rect, color: Color, style: &Style, ui: &mut UiCell)
{
    // BorderedRectangle widget.
    let dim = rect.dim();
    let border = style.border(&ui.theme);
    let border_color = style.border_color(&ui.theme);
    widget::BorderedRectangle::new(dim)
        .middle_of(button_id)
        .graphics_for(button_id)
        .color(color)
        .border(border)
        .border_color(border_color)
        .set(rectangle_id, ui);
}

fn label(button_id: widget::Id, label_id: widget::Id,
         label: &str, style: &Style, ui: &mut UiCell)
{
    let color = style.label_color(&ui.theme);
    let font_size = style.label_font_size(&ui.theme);
    let x = style.label_x(&ui.theme);
    let y = style.label_y(&ui.theme);
    let justify = style.label_justify(&ui.theme);
    let font_id = style.label_font_id(&ui.theme).or(ui.fonts.ids().next());
    widget::Text::new(label)
        .and_then(font_id, widget::Text::font_id)
        .x_position_relative_to(button_id, x)
        .y_position_relative_to(button_id, y)
        .justify(justify)
        .parent(button_id)
        .graphics_for(button_id)
        .color(color)
        .font_size(font_size)
        .set(label_id, ui);
}

//Function to set the marker triangle.
//NB requires knowledge of the current refresh state for pulsing colour.
fn mark(button_id: widget::Id, marker_id: widget::Id,
		timer:usize,marker_active:bool,style: &Style, ui: &mut UiCell)
{
	let wh_of_button = ui.wh_of(button_id).unwrap_or([0.0,0.0]);
	let xy_of_button = ui.xy_of(button_id).unwrap_or([0.0,0.0]);

	//make the edge coordinate of the triangle.
	let edge = [xy_of_button[0],xy_of_button[1]];
	let coordinates = if marker_active {
		vec![[edge[0]+wh_of_button[0]*(-0.5)+1.0,edge[1]-wh_of_button[1]/4.0],
			 [edge[0]+wh_of_button[0]*(-0.5)+1.0,edge[1]+wh_of_button[1]/4.0],
			 [edge[0]+wh_of_button[0]*(-0.3)+1.0,edge[1]]]
	}else{
		vec![[0.0,0.0],[0.0,0.0],[0.0,0.0]]
	};

	//Make the pulsing colour.
	let pulsing_colour:color::Color = if marker_active {
		conrod::color::ORANGE.with_luminance(xmoose::sync_t(timer))
	}else{
		conrod::color::BLACK.with_alpha(0.0)
	};

	//set the thingy.

	widget::Polygon::fill_with(coordinates,pulsing_colour)
		.xy(xy_of_button)
		.floating(true)
		.set(marker_id,ui);
}


impl<'a, S> Colorable for MooseButton<'a, S> {
    builder_method!(color { style.color = Some(Color) });
}

impl<'a, S> Borderable for MooseButton<'a, S> {
    builder_methods!{
        border { style.border = Some(Scalar) }
        border_color { style.border_color = Some(Color) }
    }
}

impl<'a,S>Labelable<'a> for MooseButton<'a,S> {
	builder_methods!{
        label { maybe_label = Some(&'a str) }
        label_color { style.label_color = Some(Color) }
        label_font_size { style.label_font_size = Some(FontSize) }
	}
}
