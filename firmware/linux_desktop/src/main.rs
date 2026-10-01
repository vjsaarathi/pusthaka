use iced::{
    alignment,
    widget::{button, column, container, row, scrollable, text, Space, horizontal_space},
    Alignment, Application, Command, Element, Length, Settings, Theme, Font, Color,
};
use reader_os::reader::format::{SemanticBlock, ReaderBook};
use reader_os::reader::ingestion::{IngestionEngine, IngestionFormat};
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;

mod hal_impl;

const ICON_FONT: Font = Font::with_name("Material Icons");
macro_rules! icon {
    ($u:expr) => {
        text($u.to_string()).font(ICON_FONT).size(24)
    };
}

pub fn main() -> iced::Result {
    ReaderApp::run(Settings {
        fonts: vec![include_bytes!("../assets/material-icons.ttf").as_slice().into()],
        window: iced::window::Settings {
            size: iced::Size::new(1200.0, 900.0),
            min_size: Some(iced::Size::new(800.0, 600.0)),
            ..iced::window::Settings::default()
        },
        default_font: Font::with_name("Georgia"),
        ..Settings::default()
    })
}

pub struct PageStyle { is_dark: bool }
impl iced::widget::container::StyleSheet for PageStyle {
    type Style = Theme;
    fn appearance(&self, _style: &Self::Style) -> iced::widget::container::Appearance {
        iced::widget::container::Appearance {
            background: Some(iced::Background::Color(if self.is_dark { Color::from_rgb(0.12, 0.12, 0.12) } else { Color::WHITE })),
            text_color: Some(if self.is_dark { Color::from_rgb(0.9, 0.9, 0.9) } else { Color::BLACK }),
            border: iced::Border {
                color: if self.is_dark { Color::from_rgb(0.2, 0.2, 0.2) } else { Color::from_rgb(0.85, 0.85, 0.85) },
                width: 1.0,
                radius: 4.0.into(),
            },
            shadow: iced::Shadow {
                color: Color::from_rgba(0.0, 0.0, 0.0, 0.05),
                offset: iced::Vector::new(0.0, 4.0),
                blur_radius: 10.0,
            },
        }
    }
}

pub struct SidebarStyle { is_dark: bool }
impl iced::widget::container::StyleSheet for SidebarStyle {
    type Style = Theme;
    fn appearance(&self, _style: &Self::Style) -> iced::widget::container::Appearance {
        iced::widget::container::Appearance {
            background: Some(iced::Background::Color(if self.is_dark { Color::from_rgb(0.08, 0.08, 0.08) } else { Color::from_rgb(0.97, 0.97, 0.97) })),
            border: iced::Border {
                color: if self.is_dark { Color::from_rgb(0.15, 0.15, 0.15) } else { Color::from_rgb(0.9, 0.9, 0.9) },
                width: 1.0,
                radius: 0.0.into(),
            },
            ..Default::default()
        }
    }
}


pub struct CoverStyle;
impl iced::widget::container::StyleSheet for CoverStyle {
    type Style = Theme;
    fn appearance(&self, _style: &Self::Style) -> iced::widget::container::Appearance {
        iced::widget::container::Appearance {
            background: Some(iced::Background::Color(Color::from_rgb(0.2, 0.2, 0.25))),
            border: iced::Border {
                color: Color::from_rgb(0.3, 0.3, 0.35),
                width: 1.0,
                radius: 4.0.into(),
            },
            shadow: iced::Shadow {
                color: Color::from_rgba(0.0, 0.0, 0.0, 0.3),
                offset: iced::Vector::new(2.0, 4.0),
                blur_radius: 8.0,
            },
            ..Default::default()
        }
    }
}

pub struct TopBarStyle { is_dark: bool }
impl iced::widget::container::StyleSheet for TopBarStyle {
    type Style = Theme;
    fn appearance(&self, _style: &Self::Style) -> iced::widget::container::Appearance {
        iced::widget::container::Appearance {
            background: Some(iced::Background::Color(if self.is_dark { Color::from_rgb(0.1, 0.1, 0.1) } else { Color::WHITE })),
            border: iced::Border {
                color: if self.is_dark { Color::from_rgb(0.15, 0.15, 0.15) } else { Color::from_rgb(0.9, 0.9, 0.9) },
                width: 1.0,
                radius: 0.0.into(),
            },
            ..Default::default()
        }
    }
}

enum AppState {
    Library { books: Vec<(String, PathBuf)> },
    Loading { message: String },
    Reading { 
        book: Arc<ReaderBook>, 
        current_block: usize,
        page_history: Vec<usize> 
    },
    Error(String),
}

struct ReaderApp {
    state: AppState,
    is_dark: bool,
}

#[derive(Debug, Clone)]
enum Message {
    LoadLibrary,
    LibraryLoaded(Vec<(String, PathBuf)>),
    OpenImportDialog,
    ImportFileSelected(Option<PathBuf>),
    BookImported,
    OpenBook(PathBuf),
    BookParsed(Result<Arc<ReaderBook>, String>),
    NextPage,
    PrevPage,
    GoToBlock(usize),
    ReturnToLibrary,
    ToggleTheme,
}

fn library_dir() -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
    let path = PathBuf::from(home).join(".local").join("share").join("reader-os").join("books");
    fs::create_dir_all(&path).ok();
    path
}

fn determine_format(path: &PathBuf) -> Option<IngestionFormat> {
    let ext = path.extension()?.to_str()?.to_lowercase();
    match ext.as_str() {
        "epub" => Some(IngestionFormat::Epub),
        "pdf" => Some(IngestionFormat::Pdf),
        "txt" => Some(IngestionFormat::Txt),
        _ => None,
    }
}

impl Application for ReaderApp {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            Self { 
                state: AppState::Loading { message: "Loading Library...".to_string() },
                is_dark: false,
            },
            Command::perform(async {}, |_| Message::LoadLibrary),
        )
    }

    fn title(&self) -> String {
        match &self.state {
            AppState::Reading { book, .. } => format!("{} - Reader OS", book.title),
            _ => String::from("Reader OS"),
        }
    }

    fn theme(&self) -> Theme {
        if self.is_dark { Theme::Dark } else { Theme::Light }
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::LoadLibrary => {
                return Command::perform(
                    async move {
                        let mut books = Vec::new();
                        let dir = library_dir();
                        if let Ok(entries) = fs::read_dir(dir) {
                            for entry in entries.flatten() {
                                let path = entry.path();
                                if path.extension().and_then(|s| s.to_str()) == Some("rkyv") {
                                    let name = path.file_stem().unwrap().to_string_lossy().to_string();
                                    books.push((name, path));
                                }
                            }
                        }
                        books.sort_by(|a, b| a.0.cmp(&b.0));
                        books
                    },
                    |books| Message::LibraryLoaded(books),
                );
            }
            Message::LibraryLoaded(books) => {
                self.state = AppState::Library { books };
            }
            Message::OpenImportDialog => {
                let path = rfd::FileDialog::new()
                    .add_filter("Documents", &["epub", "pdf", "txt"])
                    .set_title("Import Document to Library")
                    .pick_file();
                return Command::perform(async move { path }, Message::ImportFileSelected);
            }
            Message::ImportFileSelected(Some(path)) => {
                self.state = AppState::Loading { message: "Parsing Document (This may take a moment)...".to_string() };
                return Command::perform(
                    async move {
                        if let Some(name) = path.file_name() {
                            let format = determine_format(&path).unwrap_or(IngestionFormat::Txt);
                            if let Ok(bytes) = fs::read(&path) {
                                let name_str = name.to_string_lossy().to_string();
                                
                                // Offload the heavy, synchronous CPU parsing to a background thread
                                // so the UI event loop doesn't block!
                                let _ = tokio::task::spawn_blocking(move || {
                                    if let Ok(book) = IngestionEngine::import_in_memory(format, &bytes) {
                                        if let Ok(rkyv_bytes) = rkyv::to_bytes::<rkyv::rancor::Error>(&book) {
                                            let clean_name = name_str.replace(".epub", "").replace(".pdf", "").replace(".txt", "");
                                            let dest = library_dir().join(format!("{}.rkyv", clean_name));
                                            fs::write(&dest, rkyv_bytes).ok();
                                        }
                                    }
                                }).await;
                            }
                        }
                    },
                    |_| Message::BookImported,
                );
            }
            Message::ImportFileSelected(None) => {}
            Message::BookImported => {
                return Command::perform(async {}, |_| Message::LoadLibrary);
            }
            Message::OpenBook(path) => {
                self.state = AppState::Loading { message: "Loading RKYV...".to_string() };
                return Command::perform(
                    async move {
                        let bytes = fs::read(&path).map_err(|e| e.to_string())?;
                        let archived = unsafe { rkyv::access_unchecked::<<ReaderBook as rkyv::Archive>::Archived>(&bytes) };
                        let book: ReaderBook = rkyv::deserialize::<ReaderBook, rkyv::rancor::Error>(archived)
                            .map_err(|_| "Failed to deserialize cached .rkyv binary".to_string())?;
                        Ok(Arc::new(book))
                    },
                    Message::BookParsed,
                );
            }
            Message::BookParsed(Ok(book)) => {
                let safe_title = book.title.replace("/", "_");
                let saved_block = fs::read_to_string(library_dir().join(format!("{}.progress", safe_title)))
                    .unwrap_or_default().parse::<usize>().unwrap_or(0);
                
                // If we load a saved block, push 0 so we can at least go back to the beginning if we hit Prev
                let mut page_history = Vec::new();
                if saved_block > 0 { page_history.push(0); }
                
                self.state = AppState::Reading { book, current_block: saved_block, page_history };
            }
            Message::BookParsed(Err(err)) => {
                self.state = AppState::Error(err);
            }
            Message::NextPage => {
                if let AppState::Reading { book, current_block, page_history } = &mut self.state {
                    let mut char_count = 0;
                    let mut next_idx = *current_block;
                    while next_idx < book.blocks.len() && char_count < 3500 {
                        match &book.blocks[next_idx] {
                            SemanticBlock::Paragraph { text, .. } => char_count += text.len() + 100,
                            SemanticBlock::Heading { text, .. } => char_count += (text.len() * 3) + 300,
                            SemanticBlock::Image { .. } => char_count += 1500,
                            SemanticBlock::PageBreak => char_count += 150,
                        }
                        next_idx += 1;
                        if next_idx - *current_block > 100 { break; }
                    }
                    if next_idx < book.blocks.len() {
                        page_history.push(*current_block);
                        *current_block = next_idx;
                        let safe_title = book.title.replace("/", "_");
                        let _ = fs::write(library_dir().join(format!("{}.progress", safe_title)), next_idx.to_string());
                    }
                }
            }
            Message::PrevPage => {
                if let AppState::Reading { book, current_block, page_history, .. } = &mut self.state {
                    *current_block = page_history.pop().unwrap_or(0);
                    let safe_title = book.title.replace("/", "_");
                    let _ = fs::write(library_dir().join(format!("{}.progress", safe_title)), current_block.to_string());
                }
            }
            Message::GoToBlock(target_idx) => {
                if let AppState::Reading { book, current_block, page_history, .. } = &mut self.state {
                    page_history.push(*current_block);
                    *current_block = target_idx;
                    let safe_title = book.title.replace("/", "_");
                    let _ = fs::write(library_dir().join(format!("{}.progress", safe_title)), target_idx.to_string());
                }
            }
            Message::ReturnToLibrary => {
                return Command::perform(async {}, |_| Message::LoadLibrary);
            }
            Message::ToggleTheme => {
                self.is_dark = !self.is_dark;
            }
        }
        Command::none()
    }

    
    fn subscription(&self) -> iced::Subscription<Message> {
        iced::event::listen_with(|event, status| match (event, status) {
            (
                iced::Event::Keyboard(iced::keyboard::Event::KeyPressed { key, .. }),
                iced::event::Status::Ignored,
            ) => match key {
                iced::keyboard::Key::Named(iced::keyboard::key::Named::ArrowRight) 
                | iced::keyboard::Key::Named(iced::keyboard::key::Named::Space) 
                | iced::keyboard::Key::Named(iced::keyboard::key::Named::PageDown) => Some(Message::NextPage),
                
                iced::keyboard::Key::Named(iced::keyboard::key::Named::ArrowLeft) 
                | iced::keyboard::Key::Named(iced::keyboard::key::Named::PageUp) => Some(Message::PrevPage),
                
                iced::keyboard::Key::Named(iced::keyboard::key::Named::Escape) => Some(Message::ReturnToLibrary),
                _ => None,
            },
            _ => None,
        })
    }

    fn view(&self) -> Element<Message> {

        let text_color = if self.is_dark { Color::from_rgb(0.9, 0.9, 0.9) } else { Color::from_rgb(0.1, 0.1, 0.1) };
        let subtle_color = if self.is_dark { Color::from_rgb(0.5, 0.5, 0.5) } else { Color::from_rgb(0.4, 0.4, 0.4) };

        match &self.state {
            AppState::Library { books } => {
                let header = row![
                    text("Reader OS").size(36).style(iced::theme::Text::Color(text_color)),
                    horizontal_space(),
                    button(row![icon!(if self.is_dark { '\u{e518}' } else { '\u{e51c}' }), Space::with_width(8), text(if self.is_dark { "Light Mode" } else { "Dark Mode" })].align_items(Alignment::Center))
                        .on_press(Message::ToggleTheme)
                        .style(iced::theme::Button::Text),
                    Space::with_width(20),
                    button(row![icon!('\u{e145}'), Space::with_width(8), text("Import Document")].align_items(Alignment::Center)).on_press(Message::OpenImportDialog).padding(10)
                ].align_items(Alignment::Center).padding(40);

                let mut grid_col: iced::widget::Column<'_, Message, Theme, iced::Renderer> = column![].spacing(30);
                
                if books.is_empty() {
                    grid_col = grid_col.push(
                        container(text("Your library is elegantly empty.
Import an EPUB, PDF, or TXT file to begin.").size(20).horizontal_alignment(alignment::Horizontal::Center).style(iced::theme::Text::Color(subtle_color)))
                            .width(Length::Fill)
                            .center_x()
                            .padding(100)
                    );
                } else {
                    for chunk in books.chunks(4) {
                        let mut row_widget: iced::widget::Row<'_, Message, Theme, iced::Renderer> = row![].spacing(30);
                        for (name, path) in chunk {
                            let cover_content = container(
                                text(name.clone()).size(18).style(iced::theme::Text::Color(Color::from_rgb(0.9, 0.9, 0.9)))
                            )
                            .width(Length::Fixed(160.0))
                            .height(Length::Fixed(240.0))
                            .padding(15)
                            .style(iced::theme::Container::Custom(Box::new(CoverStyle)));
                            
                            let btn = button(cover_content)
                                .style(iced::theme::Button::Text)
                                .on_press(Message::OpenBook(path.clone()));
                                
                            row_widget = row_widget.push(btn);
                        }
                        grid_col = grid_col.push(row_widget);
                    }
                }

                column![header, container(scrollable(grid_col).width(Length::Fixed(800.0))).width(Length::Fill).center_x()].into()
            }
            AppState::Loading { message } => {
                container(text(message).size(24).style(iced::theme::Text::Color(subtle_color)))
                    .width(Length::Fill).height(Length::Fill).center_x().center_y().into()
            }
            AppState::Error(err) => {
                let err_txt = text(format!("Error:\n{} ", err)).size(18).style(iced::theme::Text::Color(Color::from_rgb(0.8, 0.2, 0.2)));
                let back_btn = button("Return to Library").on_press(Message::ReturnToLibrary).padding(15);
                container(column![err_txt, back_btn].spacing(30).align_items(Alignment::Center))
                    .width(Length::Fill).height(Length::Fill).center_x().center_y().into()
            }
            AppState::Reading { book, current_block, .. } => {
                let top_bar = container(
                    row![
                        button(row![icon!('\u{e5c4}'), Space::with_width(8), text("Library")].align_items(Alignment::Center)).on_press(Message::ReturnToLibrary).style(iced::theme::Button::Text),
                        horizontal_space(),
                        text(&book.title).size(16).style(iced::theme::Text::Color(subtle_color)),
                        horizontal_space(),
                        button(icon!(if self.is_dark { '\u{e518}' } else { '\u{e51c}' })).on_press(Message::ToggleTheme).style(iced::theme::Button::Text)
                    ].align_items(Alignment::Center).padding(20)
                ).style(iced::theme::Container::Custom(Box::new(TopBarStyle { is_dark: self.is_dark })));

                let mut content_col: iced::widget::Column<'_, Message, Theme, iced::Renderer> = column![].spacing(25);
                let mut char_count = 0;
                let mut render_idx = *current_block;

                while render_idx < book.blocks.len() && char_count < 3500 {
                    match &book.blocks[render_idx] {
                        SemanticBlock::Heading { level, text: t } => {
                            let size = match level { 1 => 38, 2 => 28, 3 => 24, _ => 22 };
                            content_col = content_col.push(text(t).size(size).width(Length::Fill));
                            char_count += (t.len() * 3) + 300;
                        }
                        SemanticBlock::Paragraph { text: t, .. } => {
                            content_col = content_col.push(
                                text(t).size(20).line_height(iced::widget::text::LineHeight::Relative(1.6)).width(Length::Fill)
                            );
                            char_count += t.len() + 100;
                        }
                        SemanticBlock::Image { caption, .. } => {
                            if let Some(cap) = caption {
                                content_col = content_col.push(
                                    text(format!("[ Image: {} ]", cap)).size(16).style(iced::theme::Text::Color(subtle_color))
                                );
                            }
                            char_count += 1500;
                        }
                        SemanticBlock::PageBreak => {
                            content_col = content_col.push(Space::with_height(40));
                            char_count += 150; 
                        }
                    }
                    render_idx += 1;
                    if render_idx - *current_block > 100 { break; }
                }

                let progress_percent = if book.blocks.is_empty() { 0.0 } else { (*current_block as f32 / book.blocks.len() as f32) * 100.0 };

                let page_controls = row![
                    button("PREVIOUS").on_press(Message::PrevPage).style(iced::theme::Button::Text),
                    horizontal_space(),
                    text(format!("{:.0}%", progress_percent)).size(14).style(iced::theme::Text::Color(subtle_color)),
                    horizontal_space(),
                    button("NEXT").on_press(Message::NextPage).style(iced::theme::Button::Text),
                ].align_items(Alignment::Center).padding(20);

                let reader_pane = column![
                    container(content_col)
                        .height(Length::Fill)
                        .width(Length::Fill)
                        .padding(iced::Padding { top: 60.0, bottom: 60.0, left: 80.0, right: 80.0 })
                        .style(iced::theme::Container::Custom(Box::new(PageStyle { is_dark: self.is_dark }))),
                    page_controls
                ].width(Length::Fixed(800.0));

                let mut toc_col: iced::widget::Column<'_, Message, Theme, iced::Renderer> = column![
                    text("CONTENTS").size(12).style(iced::theme::Text::Color(subtle_color)), 
                    Space::with_height(20)
                ].spacing(8);
                
                for toc in &book.table_of_contents {
                    let indent = " ".repeat((toc.level as usize) * 2);
                    let btn = button(text(format!("{}{}", indent, toc.title)).size(14))
                        .style(iced::theme::Button::Text)
                        .on_press(Message::GoToBlock(toc.target_block_index as usize));
                    toc_col = toc_col.push(btn);
                }
                
                let sidebar = container(scrollable(toc_col))
                    .width(Length::Fixed(300.0))
                    .height(Length::Fill)
                    .padding(30)
                    .style(iced::theme::Container::Custom(Box::new(SidebarStyle { is_dark: self.is_dark })));

                container(
                    column![
                        top_bar,
                        row![
                            sidebar, 
                            container(reader_pane).padding(40).center_x().width(Length::Fill)
                        ].height(Length::Fill)
                    ]
                )
                .width(Length::Fill)
                .height(Length::Fill)
                .style(iced::theme::Container::Custom(Box::new(SidebarStyle { is_dark: self.is_dark })))
                .into()
            }
        }
    }
}

fn dummy_sub() -> iced::Subscription<Message> {
    iced::event::listen_with(|event, status| match (event, status) {
        (
            iced::Event::Keyboard(iced::keyboard::Event::KeyPressed { key, .. }),
            iced::event::Status::Ignored,
        ) => match key {
            iced::keyboard::Key::Named(iced::keyboard::key::Named::ArrowRight) => Some(Message::NextPage),
            iced::keyboard::Key::Named(iced::keyboard::key::Named::ArrowLeft) => Some(Message::PrevPage),
            _ => None,
        },
        _ => None,
    })
}
