use orbclient::Renderer;
use std::cell::{Cell, RefCell};
use std::sync::Arc;
use orbimage;

use cell::{CheckSet, CloneCell};
use widgets::{Image, Widget};
use draw::draw_box;
use event::Event;
use rect::Rect;
use point::Point;
use theme::{Selector, Theme};
use traits::{Place, Text, Style};

static TOGGLE_ICON: &'static [u8; 703] = include_bytes!("../../res/icon-down-black.png");
static TOGGLE_ICON_ACTIVE: &'static [u8; 706] = include_bytes!("../../res/icon-down-white.png");

struct Entry {
    pub rect: Cell<Rect>,
    pub selector: CloneCell<Selector>,
    pub text: CloneCell<String>,
    pub text_offset: Cell<Point>,
    hover: Cell<bool>,
    pressed: Cell<bool>,
    index: u32,
    active: Cell<bool>,
}

impl Entry {
    fn new(text: &str, index: u32) -> Arc<Self> {
        Arc::new(Entry {
            rect: Cell::new(Rect::default()),
            selector: CloneCell::new(Selector::new(Some("combo-box-entry"))),
            text: CloneCell::new(String::from(text)),
            text_offset: Cell::new(Point::default()),
            hover: Cell::new(false),
            pressed: Cell::new(false),
            index,
            active: Cell::new(false),
        })
    }
}

impl Text for Entry {
    fn text<S: Into<String>>(&self, text: S) -> &Self {
        self.text.set(text.into());
        self
    }

    fn text_offset(&self, x: i32, y: i32) -> &Self {
        self.text_offset.set(Point::new(x, y));
        self
    }
}

impl Style for Entry {
    fn with_class<S: Into<String>>(&self, class: S) -> &Self {
        self.selector.set(self.selector.get().with_class(class));
        self
    }

    fn with_pseudo_class<S: Into<String>>(&self, pseudo_class: S) -> &Self {
        self.selector.set(self.selector.get().with_pseudo_class(pseudo_class));
        self
    }
}

impl Widget for Entry {
    fn rect(&self) -> &Cell<Rect> {
        &self.rect
    }
    fn draw(&self, renderer: &mut Renderer, _focused: bool, theme: &Theme) {
        let rect = self.rect.get();
        let offset = self.text_offset.get();

        if self.hover.get() || self.active.get() {
            let mut selector = Selector::new(Some("combo-box-entry"));
            
            
            if self.active.get() {
                selector = selector.with_pseudo_class("active");
            } else {
                selector = selector.with_pseudo_class("hover");
            }

            draw_box(
                renderer,
                Rect::new(rect.x, rect.y, rect.width, rect.height),
                theme,
                &selector,
            );
        }

        let mut point = Point::new(rect.x + offset.x, rect.y + rect.height as i32 / 2 - 8);
        for c in self.text.get().chars() {
            if point.x + 8 <= rect.width as i32 - 2 * offset.x {
                //let mut selector = Selector::new(Some("combo-box-entry"));
                let selector = &self.selector.get();

                //if self.active.get() {
                //    selector = selector.with_pseudo_class("active");
                //}

                renderer.char(point.x, point.y, c, theme.color("color", selector));
            }
            point.x += 8;
        }
    }
    fn event(&self, event: Event, _focused: bool, redraw: &mut bool) -> bool {
        match event {
            Event::Mouse {
                point, left_button, ..
            } => {
                let mut click = false;

                let rect = self.rect.get();
                if rect.contains(point) {
                    if self.hover.check_set(true) {
                        *redraw = true;
                    }

                    if left_button {
                        if self.pressed.check_set(true) {
                            *redraw = true;
                        }
                    } else {
                        if self.pressed.check_set(false) {
                            click = true;
                            *redraw = true;
                        }
                    }
                } else {
                    if self.hover.check_set(false) {
                        *redraw = true;
                    }

                    if !left_button {
                        if self.pressed.check_set(false) {
                            *redraw = true;
                        }
                    }
                }

                if click {
                    return true;
                }
            }
            _ => (),
        }

        _focused
    }

    fn name(&self) -> &str {
        "ComboBoxEntry"
    }

    fn visible(&self, flag: bool){
        !flag;
    }
}

pub struct ComboBox {
    pub rect: Cell<Rect>,
    pub selector: CloneCell<Selector>,
    pressed: Cell<bool>,
    activated: Cell<bool>,
    pub offset: Cell<Point>,
    selected: Cell<Option<u32>>,
    entries: RefCell<Vec<Arc<Entry>>>,
    text: CloneCell<String>,
    flyout_height: Cell<u32>,
    toggle_icon: RefCell<Option<Arc<Image>>>,
    toggle_icon_active: RefCell<Option<Arc<Image>>>,
    visible: Cell<bool>,
}

impl ComboBox {
    pub fn new() -> Arc<ComboBox> {
        let toggle_icon = RefCell::new(None);
        if let Ok(icon) = orbimage::parse_png(TOGGLE_ICON) {
            *toggle_icon.borrow_mut() = Some(Image::from_image(icon))
        };

        let toggle_icon_active = RefCell::new(None);
        if let Ok(icon) = orbimage::parse_png(TOGGLE_ICON_ACTIVE) {
            *toggle_icon_active.borrow_mut() = Some(Image::from_image(icon))
        };

        Arc::new(ComboBox {
            rect: Cell::new(Rect::new(0, 0, 332, 28)),
            selector: CloneCell::new(Selector::new(Some("combo-box"))),
            pressed: Cell::new(false),
            activated: Cell::new(false),
            offset: Cell::new(Point::new(4, 4)),
            selected: Cell::new(None),
            entries: RefCell::new(vec![]),
            text: CloneCell::new(String::new()),
            flyout_height: Cell::new(0),
            toggle_icon,
            toggle_icon_active,
            visible: Cell::new(true),
        })
    }

    pub fn selected(&self) -> i32 {
        if let Some(selected) = self.selected.get() {
            return selected as i32;
        };

        -1
    }

    pub fn push(&self, text: &str) {
        let rect = self.rect().get();
        let entry = Entry::new(text, self.entries.borrow().len() as u32);
        entry.rect.set(Rect::new(
            rect.x + 1,
            rect.y + rect.height as i32 * (self.entries.borrow().len() as i32 + 1),
            rect.width - 2,
            rect.height,
        ));
        entry.text_offset(self.offset.get().x, self.offset.get().y);
        self.flyout_height
            .set(self.flyout_height.get() + rect.height);

        self.entries.borrow_mut().push(entry);

        if self.entries.borrow().len() == 1 {
            self.change_selection(0);
        }
    }

    pub fn pop(&self) -> String {
        if let Some(entry) = self.entries.borrow_mut().pop() {
            self.change_selection(0);
            return entry.text.get();
        }

        String::new()
    }

    pub fn change_selection(&self, i: u32) {
        if let Some(index) = self.selected.get() {
            if let Some(entry) = self.entries.borrow().get(index as usize) {
                entry.active.set(false)
            }
        }

        self.selected.set(Some(i));

        if let Some(index) = self.selected.get() {
            if let Some(entry) = self.entries.borrow().get(index as usize) {
                entry.active.set(true);
                self.text.set(entry.text.get());
            }
        }
    }

    pub fn text_offset(&self, x: i32, y: i32) -> &Self {
        self.offset.set(Point::new(x, y));
        self
    }
}

impl Style for ComboBox {
    fn with_class<S: Into<String>>(&self, class: S) -> &Self {
        self.selector.set(self.selector.get().with_class(class));
        self
    }

    fn with_pseudo_class<S: Into<String>>(&self, pseudo_class: S) -> &Self {
        self.selector.set(self.selector.get().with_pseudo_class(pseudo_class));
        self
    }
}

impl Widget for ComboBox {
    fn rect(&self) -> &Cell<Rect> {
        &self.rect
    }

    fn draw(&self, renderer: &mut Renderer, _focused: bool, theme: &Theme) {
        if self.visible.get() {
            let rect = self.rect.get();
            let activated = self.activated.get();
            let offset = self.offset.get();

            // draw flyout
            if activated {
                let selector = Selector::new(Some("combo-box-flyout"));

                let flyout_rect = Rect::new(
                    rect.x,
                    rect.y + rect.height as i32 - 2,
                    rect.width,
                    self.flyout_height.get() + 2,
                );
                draw_box(renderer, flyout_rect, theme, &selector);

                // draw entries
                for entry in self.entries.borrow().iter() {
                    let mut point = Point::new(entry.rect.get().x, entry.rect.get().y);

                    if point.y >= rect.y
                        && point.y + rect.height as i32 <= flyout_rect.y + flyout_rect.height as i32
                    {
                        entry.draw(renderer, _focused, theme);
                    }
                }
            }

            // draw the combobox
            let mut selector = Selector::new(Some("combo-box"));

            if activated {
                selector = selector.with_pseudo_class("active");
            }

            draw_box(renderer, rect, theme, &selector);

            // draw toggle indicator
            selector = Selector::new(Some("combo-box-toggle"));

            if activated {
                selector = selector.with_pseudo_class("active");
            }

            let toggle_size = rect.height - 2 * offset.y as u32;

            let toggle_rect = Rect::new(
                rect.x + rect.width as i32 - toggle_size as i32 - offset.y,
                rect.y + offset.y,
                toggle_size,
                toggle_size,
            );

            draw_box(renderer, toggle_rect, theme, &selector);

            // draw the toggle icon
            if activated {
                if let Some(ref icon) = *self.toggle_icon_active.borrow() {
                    icon.position(toggle_rect.x, toggle_rect.y);
                    icon.draw(renderer, _focused, theme)
                }
            } else {
                if let Some(ref icon) = *self.toggle_icon.borrow() {
                    icon.position(toggle_rect.x, toggle_rect.y);
                    icon.draw(renderer, _focused, theme)
                }
            }

            // draw selected text
            let mut point = Point::new(rect.x + offset.x - 8, rect.y + rect.height as i32 / 2 - 8);
            for c in self.text.get().chars() {
                if point.x + 8 <= rect.width as i32 - toggle_rect.width as i32 - 2 * offset.x {
                    renderer.char(
                        point.x + rect.x,
                        point.y,
                        c,
                        theme.color("color", &"label".into()),
                    );
                }
                point.x += 8;
            }
        }
    }

    fn event(&self, event: Event, focused: bool, redraw: &mut bool) -> bool {
        if self.visible.get() {
            match event {
                Event::Mouse {
                    point, left_button, ..
                } => {
                    let mut ignore_event = false;
                    if self.activated.get() {
                        for entry in self.entries.borrow().iter() {
                            if entry.event(event, focused, redraw) {
                                ignore_event = true;

                                self.change_selection(entry.index);
                                if self.activated.check_set(false) {
                                    *redraw = true;
                                }
                            }
                        }
                    }

                    let rect = self.rect.get();
                    if rect.contains(point) {
                        if left_button {
                            self.pressed.set(!self.pressed.get());

                            if self.activated.check_set(true) {
                                *redraw = true;
                            }
                        } else {
                            if !self.pressed.get() {
                                if self.activated.check_set(false) {
                                    *redraw = true;
                                }
                            }
                        }
                    } else {
                        if !ignore_event {
                            if left_button {
                                self.pressed.set(false);
                            } else {
                                if !self.pressed.get() {
                                    if self.activated.check_set(false) {
                                        *redraw = true;
                                    }
                                }
                            }
                        }
                    }
                }
                Event::UpArrow => match self.selected.get() {
                    None => {
                        self.change_selection(0);
                        *redraw = true;
                    }
                    Some(i) => {
                        if i > 0 {
                            self.change_selection(i - 1);
                            *redraw = true;
                        }
                    }
                },
                Event::DownArrow => {
                    if self.activated.get() {
                        match self.selected.get() {
                            None => {
                                self.change_selection(0);
                                *redraw = true;
                            }
                            Some(i) => {
                                if i < self.entries.borrow().len() as u32 - 1 {
                                    self.change_selection(i + 1);
                                    *redraw = true;
                                }
                            }
                        }
                    }
                }
                Event::Enter => {
                    if self.activated.check_set(false) {
                        self.pressed.set(false);
                        *redraw = true;
                    }
                }
                _ => {}
            }
        }
        focused
    }

    fn name(&self) -> &str {
        if self.activated.get() {
            "ComboBoxActivated"
        } else {
            "ComboBox"
        }
    }

    fn visible(&self, flag: bool) {
        self.visible.set(flag);
    }
}

impl Place for ComboBox {}
