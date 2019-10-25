use crate::{Align, Column, Justify, Length};

#[derive(Debug)]
pub struct Scrollable<'a, Element> {
    pub state: &'a mut State,
    pub height: Length,
    pub align_self: Option<Align>,
    pub align_items: Align,
    pub content: Column<Element>,
}

impl<'a, Element> Scrollable<'a, Element> {
    pub fn new(state: &'a mut State) -> Self {
        Scrollable {
            state,
            height: Length::Shrink,
            align_self: None,
            align_items: Align::Start,
            content: Column::new(),
        }
    }

    /// Sets the vertical spacing _between_ elements.
    ///
    /// Custom margins per element do not exist in Iced. You should use this
    /// method instead! While less flexible, it helps you keep spacing between
    /// elements consistent.
    pub fn spacing(mut self, units: u16) -> Self {
        self.content = self.content.spacing(units);
        self
    }

    /// Sets the padding of the [`Column`].
    ///
    /// [`Column`]: struct.Column.html
    pub fn padding(mut self, units: u16) -> Self {
        self.content = self.content.padding(units);
        self
    }

    /// Sets the width of the [`Column`].
    ///
    /// [`Column`]: struct.Column.html
    pub fn width(mut self, width: Length) -> Self {
        self.content = self.content.width(width);
        self
    }

    /// Sets the height of the [`Column`].
    ///
    /// [`Column`]: struct.Column.html
    pub fn height(mut self, height: Length) -> Self {
        self.height = height;
        self
    }

    /// Sets the maximum width of the [`Column`].
    ///
    /// [`Column`]: struct.Column.html
    pub fn max_width(mut self, max_width: Length) -> Self {
        self.content = self.content.max_width(max_width);
        self
    }

    /// Sets the maximum height of the [`Column`] in pixels.
    ///
    /// [`Column`]: struct.Column.html
    pub fn max_height(mut self, max_height: Length) -> Self {
        self.content = self.content.max_height(max_height);
        self
    }

    /// Sets the alignment of the [`Column`] itself.
    ///
    /// This is useful if you want to override the default alignment given by
    /// the parent container.
    ///
    /// [`Column`]: struct.Column.html
    pub fn align_self(mut self, align: Align) -> Self {
        self.align_self = Some(align);
        self
    }

    /// Sets the horizontal alignment of the contents of the [`Column`] .
    ///
    /// [`Column`]: struct.Column.html
    pub fn align_items(mut self, align_items: Align) -> Self {
        self.align_items = align_items;
        self
    }

    /// Sets the vertical distribution strategy for the contents of the
    /// [`Column`] .
    ///
    /// [`Column`]: struct.Column.html
    pub fn justify_content(mut self, justify: Justify) -> Self {
        self.content = self.content.justify_content(justify);
        self
    }

    /// Adds an element to the [`Column`].
    ///
    /// [`Column`]: struct.Column.html
    pub fn push<E>(mut self, child: E) -> Scrollable<'a, Element>
    where
        E: Into<Element>,
    {
        self.content = self.content.push(child);
        self
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct State {
    pub offset: u32,
}

impl State {
    pub fn new() -> Self {
        State::default()
    }
}
