use git2::{Blob, Object, Repository};

use ratatui::{
    layout::Rect,
    prelude::Line,
    widgets::{Block, Paragraph},
    Frame,
};

use crate::traits::{Drawable, Navigable};

pub struct BlobPager<'repo> {
    top: usize,
    // repo: &'repo Repository,
    // blob: Blob<'repo>,
    name: String,
    lines: Vec<Line<'repo>>,
}

impl<'repo> BlobPager<'repo> {
    pub fn new(_repo: &'repo Repository, blob: Blob<'repo>, name: String) -> BlobPager<'repo> {
        let content = match std::str::from_utf8(blob.content()) {
            Ok(v) => v,
            Err(e) => panic!("unable to decode utf8 {}", e),
        };
        let lines = content
            .lines()
            .map(|line| Line::from(line.to_string()))
            .collect();
        BlobPager {
            top: 0,
            // repo: repo,
            // blob: blob.clone(),
            name: name,
            lines: lines,
        }
    }

    pub fn from_object(repo: &'repo Repository, object: Object<'repo>, name: String) -> Self {
        match object.into_blob() {
            Ok(blob) => {
                return BlobPager::new(repo, blob, name);
            }
            Err(_) => panic!("peeling blob"),
        }
    }
}

impl<'repo> Drawable<'repo> for BlobPager<'repo> {
    fn draw(&self, f: &mut Frame, area: Rect, content_block: Block, reserved_rows: u16) {
        let viewport: usize = (f.size().height - reserved_rows).into();
        let bottom = if self.top + viewport > self.lines.len() {
            self.lines.len()
        } else {
            self.top + viewport
        };
        let lines = self.lines[self.top..bottom].to_vec();
        let content = Paragraph::new(lines.into_iter().collect::<Vec<Line>>()).block(content_block);
        f.render_widget(content, area);
    }

    fn title(&self) -> String {
        return format!("{}", self.name);
    }
}

impl<'repo> Navigable<'repo> for BlobPager<'repo> {
    fn pagedown(&mut self, page_size: u16) {
        let h: usize = page_size.into();
        let top = self.top + h;
        self.top = if top > self.lines.len() {
            self.lines.len() - 1
        } else {
            top
        }
    }

    fn pageup(&mut self, page_size: u16) {
        let h: usize = page_size.into();
        if self.top < h {
            self.top = 0;
        } else {
            self.top = self.top - h;
        }
    }

    fn next_selection(&mut self) {
        // Always keep the last line on the screen
        if self.top < self.lines.len() - 1 {
            self.top += 1;
        }
    }

    fn previous_selection(&mut self) {
        if self.top > 0 {
            self.top -= 1;
        }
    }

    fn select(&self) -> Option<(Object<'repo>, String)> {
        return None;
    }

    fn selected_item(&self) -> String {
        return "".to_string();
    }
}
