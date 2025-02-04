use crate::{
    components::{
        popup_paragraph, visibility_blocking, CommandBlocking,
        CommandInfo, Component, DrawableComponent,
    },
    keys::SharedKeyConfig,
    queue::{Action, InternalEvent, Queue},
    strings, ui,
};
use anyhow::Result;
use crossterm::event::Event;
use std::borrow::Cow;
use tui::{
    backend::Backend, layout::Rect, text::Span, widgets::Clear, Frame,
};
use ui::style::SharedTheme;

///
pub struct ResetComponent {
    target: Option<Action>,
    visible: bool,
    queue: Queue,
    theme: SharedTheme,
    key_config: SharedKeyConfig,
}

impl DrawableComponent for ResetComponent {
    fn draw<B: Backend>(
        &self,
        f: &mut Frame<B>,
        _rect: Rect,
    ) -> Result<()> {
        if self.visible {
            let (title, msg) = self.get_text();

            let txt = vec![Span::styled(
                Cow::from(msg),
                self.theme.text_danger(),
            )];

            let area = ui::centered_rect(30, 20, f.size());
            f.render_widget(Clear, area);
            f.render_widget(
                popup_paragraph(&title, txt, &self.theme, true),
                area,
            );
        }

        Ok(())
    }
}

impl Component for ResetComponent {
    fn commands(
        &self,
        out: &mut Vec<CommandInfo>,
        _force_all: bool,
    ) -> CommandBlocking {
        out.push(CommandInfo::new(
            strings::commands::reset_confirm(&self.key_config),
            true,
            self.visible,
        ));
        out.push(CommandInfo::new(
            strings::commands::close_popup(&self.key_config),
            true,
            self.visible,
        ));

        visibility_blocking(self)
    }

    fn event(&mut self, ev: Event) -> Result<bool> {
        if self.visible {
            if let Event::Key(e) = ev {
                if e == self.key_config.exit_popup {
                    self.hide();
                } else if e == self.key_config.enter {
                    self.confirm();
                }

                return Ok(true);
            }
        }

        Ok(false)
    }

    fn is_visible(&self) -> bool {
        self.visible
    }

    fn hide(&mut self) {
        self.visible = false
    }

    fn show(&mut self) -> Result<()> {
        self.visible = true;

        Ok(())
    }
}

impl ResetComponent {
    ///
    pub fn new(
        queue: Queue,
        theme: SharedTheme,
        key_config: SharedKeyConfig,
    ) -> Self {
        Self {
            target: None,
            visible: false,
            queue,
            theme,
            key_config,
        }
    }
    ///
    pub fn open(&mut self, a: Action) -> Result<()> {
        self.target = Some(a);
        self.show()?;

        Ok(())
    }
    ///
    pub fn confirm(&mut self) {
        if let Some(a) = self.target.take() {
            self.queue
                .borrow_mut()
                .push_back(InternalEvent::ConfirmedAction(a));
        }

        self.hide();
    }

    fn get_text(&self) -> (String, String) {
        if let Some(ref a) = self.target {
            return match a {
                Action::Reset(_) => (
                    strings::confirm_title_reset(&self.key_config),
                    strings::confirm_msg_reset(&self.key_config),
                ),
                Action::StashDrop(_) => (
                    strings::confirm_title_stashdrop(
                        &self.key_config,
                    ),
                    strings::confirm_msg_stashdrop(&self.key_config),
                ),
                Action::ResetHunk(_, _) => (
                    strings::confirm_title_reset(&self.key_config),
                    strings::confirm_msg_resethunk(&self.key_config),
                ),
            };
        }

        ("".to_string(), "".to_string())
    }
}
