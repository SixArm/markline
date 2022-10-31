pub struct MarkView {
    pub button: cursive::views::Button,
}

impl cursive::view::ViewWrapper for MarkView {
    cursive::wrap_impl!(self.button: cursive::views::Button);

    fn wrap_on_event(&mut self, event: cursive::event::Event) -> cursive::event::EventResult {
        match event {
            cursive::event::Event::Char(c) => {
                self.button.set_label_raw(c);
                cursive::event::EventResult::Consumed(None)
            },
            _ => cursive::event::EventResult::Ignored,
        }
    }
}
