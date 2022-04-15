/// Higher level model of the application
pub struct Model {
    /// Application construct
    pub app: Application<Id, Message, NoUserEvent>,

    /// Configuration on the model
    pub configuration: Configuration,

    /// State for quit / close
    pub quit: bool,

    /// State for redraw
    pub redraw: bool,

    /// Used to draw to terminal
    pub terminal: TerminalBridge,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            app: Self::init_app(),
        }
    }
}

impl Model {
    pub fn request_redraw(&mut self) {
        self.redraw = true;
    }

    pub fn request_quit(&mut self) {
        self.quit = true;
    }

    pub fn request_focus_on(&mut self, id: &Id) -> Result<()> {
        return self.app.active(id);
    }

    pub fn init_app() -> Application<Id, Message, NoUserEvent> {
        let mut app: Application<Id, Msg, NoUserEvent> = Application::init(
            EventListenerCfg::default()
                .default_input_listener(Duration::from_millis(20))
                .poll_timeout(Duration::from_millis(10))
                .tick_interval(Duration::from_secs(1)),
        );
    }
}
