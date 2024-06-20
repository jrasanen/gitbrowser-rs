use git2::{Commit, Repository, ObjectType};

use ratatui::{
    prelude::Modifier,
    style::{Color, Style},
    text::{Span},
    layout::Rect,
    widgets::{
        block::{Padding, Title},
        Block, Borders,
    },
    Frame,
};

use crate::traits::{Drawable, Navigable};

mod pagination;
mod refs_page;
mod tree_page;

use crate::app::{
    refs_page::RefsPage,
    tree_page::TreePage,
};

pub struct App<'repo> {
    pub search_input: String,
    repo: &'repo Repository,
    commit: Option<Commit<'repo>>,
    refs_page: RefsPage<'repo>,
    tree_pages: Vec<TreePage<'repo>>,
}

impl<'repo> App<'repo> {
    pub fn new(repo: &'repo Repository) -> App<'repo> {
        App {
            search_input: String::new(),
            repo: repo,
            refs_page: RefsPage::new(repo),
            commit: None,
            tree_pages: vec![],
        }
    }

    pub fn title(&self) -> Vec<Span> {
        let mut parts = vec![
            Span::from(" "),
        ];

        let mut repo_name = vec![self.refs_page.title()];
        if let Some(commit) = &self.commit {
            repo_name.push(format!("@{}", commit.id()));
        }
        if self.tree_pages.len() > 1 {
            repo_name.push(": ".to_string());
        }

        parts.push(
            Span::styled(
                repo_name.join(""),
                Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
        );

        for (ix, page) in self.tree_pages.iter().enumerate() {
            let sep = if ix > 0 { "/" } else { "" };
            parts.push(
                Span::styled(
                    format!("{}{}", page.title(), sep),
                    Style::default().fg(Color::Blue).add_modifier(Modifier::BOLD)),
            );
        }
        parts.push(Span::from(" "));
        return parts;
    }

    pub fn draw(&self, f: &mut Frame, area: Rect, reserved_rows: u16) {
        let title = Title::from(self.title());
        let content_block = Block::default()
            .padding(Padding::horizontal(1))
            .borders(Borders::ALL)
            .style(Style::default())
            .title(title);

        let page: Box<&dyn Drawable> = if let Some(p) = self.tree_pages.last() {
            Box::new(p)
        } else {
            Box::new(&self.refs_page)
        };

        page.draw(f, area, content_block, reserved_rows);
    }

    pub fn next_selection(&mut self) {
        if let Some(page) = self.tree_pages.last_mut() {
            page.next_selection();
        } else {
            self.refs_page.next_selection();
        }
    }

    pub fn previous_selection(&mut self) {
        if let Some(page) = self.tree_pages.last_mut() {
            page.previous_selection();
        } else {
            self.refs_page.previous_selection();
        }
    }

    pub fn select(&mut self) {
        let page: Box<&dyn Navigable> = if let Some(p) = self.tree_pages.last() {
            Box::new(p)
        } else {
            Box::new(&self.refs_page)
        };
        let (object, name) = page.select();
        match object.kind() {
            Some(ObjectType::Blob) => {},
            Some(ObjectType::Tree) => {
                self.tree_pages.push(
                    TreePage::new(
                        self.repo,
                        object,
                        name,
                    ),
                );
            }
            Some(ObjectType::Commit) => {
                match object.peel_to_commit() {
                    Ok(commit) => {
                        self.commit = Some(commit);
                        self.tree_pages.push(
                            TreePage::new(
                                self.repo,
                                object,
                                name,
                            ),
                        );
                    }
                    Err(e) => panic!("Unable to peel commit? {}", e)
                }
            }
            _ => {}
        }
    }

    pub fn back(&mut self) {
        self.tree_pages.pop();
        if self.tree_pages.len() == 0 {
            self.commit = None;
        }
    }

    // pub fn save_key_value(&mut self) {
    //     self.pairs
    //         .insert(self.key_input.clone(), self.value_input.clone());

    //     self.key_input = String::new();
    //     self.value_input = String::new();
    //     self.currently_editing = None;
    // }

    // pub fn toggle_editing(&mut self) {
    //     if let Some(edit_mode) = &self.currently_editing {
    //         match edit_mode {
    //             CurrentlyEditing::Key => self.currently_editing = Some(CurrentlyEditing::Value),
    //             CurrentlyEditing::Value => self.currently_editing = Some(CurrentlyEditing::Key),
    //         };
    //     } else {
    //         self.currently_editing = Some(CurrentlyEditing::Key);
    //     }
    // }
}
