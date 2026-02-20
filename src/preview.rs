
#![allow(dead_code)]
use gtk::pango;
use gtk::prelude::*;

#[derive(Clone, Debug)]
pub struct LinkSpan {
    pub start: i32,
    pub end: i32,
    pub target: String,
    pub is_wiki: bool,
}

#[derive(Default, Clone, Debug)]
pub struct RenderResult {
    pub links: Vec<LinkSpan>,
}

#[derive(Clone, Debug)]
enum LinkKind {
    Markdown(String),
    Wiki(String),
}

#[derive(Copy, Clone, Default)]
struct InlineState {
    bold: bool,
    italic: bool,
    strike: bool,
    code: bool,
}

pub fn install_tags(buffer: &gtk::TextBuffer) {
    let table = buffer.tag_table();

    let tags = [
        gtk::TextTag::builder()
            .name("h1")
            .weight(700)
            .scale(1.45)
            .pixels_above_lines(14)
            .pixels_below_lines(8)
            .build(),
        gtk::TextTag::builder()
            .name("h2")
            .weight(700)
            .scale(1.25)
            .pixels_above_lines(12)
            .pixels_below_lines(6)
            .build(),
        gtk::TextTag::builder()
            .name("h3")
            .weight(700)
            .scale(1.12)
            .pixels_above_lines(10)
            .pixels_below_lines(4)
            .build(),
        gtk::TextTag::builder()
            .name("h4")
            .weight(700)
            .scale(1.06)
            .pixels_above_lines(8)
            .pixels_below_lines(4)
            .build(),
        gtk::TextTag::builder()
            .name("h5")
            .weight(700)
            .pixels_above_lines(6)
            .pixels_below_lines(2)
            .build(),
        gtk::TextTag::builder()
            .name("h6")
            .weight(700)
            .scale(0.92)
            .style(pango::Style::Italic)
            .pixels_above_lines(6)
            .pixels_below_lines(2)
            .build(),
        gtk::TextTag::builder().name("bold").weight(700).build(),
        gtk::TextTag::builder()
            .name("italic")
            .style(pango::Style::Italic)
            .build(),
        gtk::TextTag::builder()
            .name("strike")
            .strikethrough(true)
            .build(),
        gtk::TextTag::builder()
            .name("code")
            .family("monospace")
            .scale(0.96)
            .build(),
        gtk::TextTag::builder()
            .name("quote")
            .left_margin(18)
            .style(pango::Style::Italic)
            .build(),
        gtk::TextTag::builder()
            .name("link")
            .underline(pango::Underline::Single)
            .build(),
        gtk::TextTag::builder()
            .name("wikilink")
            .underline(pango::Underline::Single)
            .weight(600)
            .build(),
        gtk::TextTag::builder().name("rule").weight(700).build(),
        gtk::TextTag::builder()
            .name("codeblock")
            .family("monospace")
            .left_margin(14)
            .right_margin(8)
            .build(),
        gtk::TextTag::builder()
            .name("table")
            .family("monospace")
            .build(),
        gtk::TextTag::builder()
            .name("task-done")
            .strikethrough(true)
            .build(),
    ];

    for tag in tags {
        table.add(&tag);
    }
}

pub fn render_markdown(buffer: &gtk::TextBuffer, markdown: &str) -> RenderResult {
    buffer.set_text("");
    let mut out = RenderResult::default();
    let mut iter = buffer.end_iter();

    let parser = pulldown_cmark::Parser::new_ext(markdown, pulldown_cmark::Options::all());

    let mut block_tags: Vec<&'static str> = Vec::new();
    let mut state = InlineState::default();
    let mut current_link: Option<LinkKind> = None;
    let mut list_index_stack: Vec<Option<u64>> = Vec::new(); // None for bullet, Some(number) for ordered
    let mut in_image = false;

    let mut pending_item_prefix = None;

    for event in parser {
        match event {
            pulldown_cmark::Event::Start(tag) => match tag {
                pulldown_cmark::Tag::Heading { level, .. } => {
                    let level_tag = match level {
                        pulldown_cmark::HeadingLevel::H1 => "h1",
                        pulldown_cmark::HeadingLevel::H2 => "h2",
                        pulldown_cmark::HeadingLevel::H3 => "h3",
                        pulldown_cmark::HeadingLevel::H4 => "h4",
                        pulldown_cmark::HeadingLevel::H5 => "h5",
                        pulldown_cmark::HeadingLevel::H6 => "h6",
                    };
                    block_tags.push(level_tag);
                }
                pulldown_cmark::Tag::BlockQuote(_) => block_tags.push("quote"),
                pulldown_cmark::Tag::CodeBlock(pulldown_cmark::CodeBlockKind::Fenced(lang)) => {
                    block_tags.push("codeblock");
                    let lang_str = lang.as_ref();
                    let prefix = if lang_str.is_empty() {
                        "\u{27ea} code block \u{27eb}\n".to_string()
                    } else {
                        format!("\u{27ea} code block: {lang_str} \u{27eb}\n")
                    };
                    insert(buffer, &mut iter, &prefix, &block_tags, state, None, &mut out.links);
                }
                pulldown_cmark::Tag::CodeBlock(pulldown_cmark::CodeBlockKind::Indented) => {
                    block_tags.push("codeblock");
                    insert(buffer, &mut iter, "\u{27ea} code block \u{27eb}\n", &block_tags, state, None, &mut out.links);
                }
                pulldown_cmark::Tag::List(first_num) => {
                    list_index_stack.push(first_num);
                }
                pulldown_cmark::Tag::Item => {
                    if let Some(Some(ref mut n)) = list_index_stack.last_mut() {
                        pending_item_prefix = Some(format!("{n}. "));
                        *n += 1;
                    } else {
                        pending_item_prefix = Some("\u{2022} ".to_string());
                    }
                }
                pulldown_cmark::Tag::Strong => state.bold = true,
                pulldown_cmark::Tag::Emphasis => state.italic = true,
                pulldown_cmark::Tag::Strikethrough => state.strike = true,
                pulldown_cmark::Tag::Link { dest_url, .. } => {
                    let url = dest_url.to_string();
                    if let Some(wiki) = url.strip_prefix("wiki:") {
                        current_link = Some(LinkKind::Wiki(wiki.to_string()));
                    } else {
                        current_link = Some(LinkKind::Markdown(url));
                    }
                }
                pulldown_cmark::Tag::Image { .. } => {
                    in_image = true;
                }
                pulldown_cmark::Tag::Table(_) => {
                    block_tags.push("table");
                }
                pulldown_cmark::Tag::TableHead | pulldown_cmark::Tag::TableRow => {
                    insert(buffer, &mut iter, "| ", &block_tags, state, None, &mut out.links);
                }
                pulldown_cmark::Tag::TableCell => {
                    // Cell start
                }
                _ => {}
            },
            pulldown_cmark::Event::End(tag) => match tag {
                pulldown_cmark::TagEnd::Heading(_) => {
                    block_tags.pop();
                    insert(buffer, &mut iter, "\n", &block_tags, state, None, &mut out.links);
                }
                pulldown_cmark::TagEnd::BlockQuote(_) => {
                    block_tags.pop();
                    insert(buffer, &mut iter, "\n", &block_tags, state, None, &mut out.links);
                }
                pulldown_cmark::TagEnd::CodeBlock => {
                    insert(buffer, &mut iter, "\u{27ea} end code block \u{27eb}\n", &block_tags, state, None, &mut out.links);
                    block_tags.pop();
                }
                pulldown_cmark::TagEnd::List(_) => {
                    list_index_stack.pop();
                    insert(buffer, &mut iter, "\n", &block_tags, state, None, &mut out.links);
                }
                pulldown_cmark::TagEnd::Item => {
                    if let Some(prefix) = pending_item_prefix.take() {
                        insert(buffer, &mut iter, &prefix, &block_tags, state, None, &mut out.links);
                    }
                    insert(buffer, &mut iter, "\n", &block_tags, state, None, &mut out.links);
                }
                pulldown_cmark::TagEnd::Strong => state.bold = false,
                pulldown_cmark::TagEnd::Emphasis => state.italic = false,
                pulldown_cmark::TagEnd::Strikethrough => state.strike = false,
                pulldown_cmark::TagEnd::Link => current_link = None,
                pulldown_cmark::TagEnd::Image => in_image = false,
                pulldown_cmark::TagEnd::Table => {
                    block_tags.pop();
                    insert(buffer, &mut iter, "\n", &block_tags, state, None, &mut out.links);
                }
                pulldown_cmark::TagEnd::TableHead => {
                    insert(buffer, &mut iter, "\n\u{2504}\u{2504}\u{2504}\u{2504}\u{2504}\u{2504}\u{2504}\u{2504}\u{2504}\u{2504}\u{2504}\u{2504}\n", &block_tags, state, None, &mut out.links);
                }
                pulldown_cmark::TagEnd::TableRow => {
                    insert(buffer, &mut iter, "\n", &block_tags, state, None, &mut out.links);
                }
                pulldown_cmark::TagEnd::TableCell => {
                    insert(buffer, &mut iter, " | ", &block_tags, state, None, &mut out.links);
                }
                pulldown_cmark::TagEnd::Paragraph => {
                    if list_index_stack.is_empty() && !block_tags.contains(&"quote") {
                        insert(buffer, &mut iter, "\n\n", &block_tags, state, None, &mut out.links);
                    }
                }
                _ => {}
            },
            pulldown_cmark::Event::Text(text) => {
                if let Some(prefix) = pending_item_prefix.take() {
                    insert(buffer, &mut iter, &prefix, &block_tags, state, None, &mut out.links);
                }
                if in_image {
                    insert(buffer, &mut iter, &format!("\u{1f5bc} {}", text), &block_tags, state, None, &mut out.links);
                } else {
                    let mut tags = block_tags.clone();
                    if let Some(LinkKind::Wiki(_)) = current_link {
                        tags.push("wikilink");
                    } else if current_link.is_some() {
                        tags.push("link");
                    }
                    insert(buffer, &mut iter, &text, &tags, state, current_link.clone(), &mut out.links);
                }
            }
            pulldown_cmark::Event::Code(code) => {
                if let Some(prefix) = pending_item_prefix.take() {
                    insert(buffer, &mut iter, &prefix, &block_tags, state, None, &mut out.links);
                }
                let mut temp_state = state;
                temp_state.code = true;
                let mut tags = block_tags.clone();
                if let Some(LinkKind::Wiki(_)) = current_link {
                    tags.push("wikilink");
                } else if current_link.is_some() {
                    tags.push("link");
                }
                insert(buffer, &mut iter, &code, &tags, temp_state, current_link.clone(), &mut out.links);
            }
            pulldown_cmark::Event::Html(html) => {
                if let Some(prefix) = pending_item_prefix.take() {
                    insert(buffer, &mut iter, &prefix, &block_tags, state, None, &mut out.links);
                }
                insert(buffer, &mut iter, &html, &block_tags, state, None, &mut out.links);
            }
            pulldown_cmark::Event::SoftBreak | pulldown_cmark::Event::HardBreak => {
                insert(buffer, &mut iter, "\n", &block_tags, state, None, &mut out.links);
            }
            pulldown_cmark::Event::Rule => {
                insert(buffer, &mut iter, "\n\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\n\n", &["rule"], state, None, &mut out.links);
            }
            pulldown_cmark::Event::TaskListMarker(checked) => {
                if let Some(_prefix) = pending_item_prefix.take() {
                }
                if checked {
                    let mut temp_state = state;
                    temp_state.strike = true; 
                    let mut tags = block_tags.clone();
                    tags.push("task-done");
                    insert(buffer, &mut iter, "\u{2611} ", &tags, temp_state, None, &mut out.links);
                } else {
                    insert(buffer, &mut iter, "\u{2610} ", &block_tags, state, None, &mut out.links);
                }
            }
            _ => {}
        }
    }

    // Strip trailing newlines if the buffer matches exactly what we wrote plus newlines
    let mut end_iter = buffer.end_iter();
    let mut last_char = end_iter;
    if last_char.backward_char() {
        while buffer.text(&last_char, &end_iter, true) == "\n" {
            buffer.delete(&mut last_char, &mut end_iter);
            if !last_char.backward_char() {
                break;
            }
        }
    }

    out
}



fn insert(
    buffer: &gtk::TextBuffer,
    iter: &mut gtk::TextIter,
    text: &str,
    block_tags: &[&'static str],
    state: InlineState,
    link: Option<LinkKind>,
    links: &mut Vec<LinkSpan>,
) {
    let mut tags: Vec<&str> = block_tags.to_vec();

    if state.bold {
        tags.push("bold");
    }
    if state.italic {
        tags.push("italic");
    }
    if state.strike {
        tags.push("strike");
    }
    if state.code {
        tags.push("code");
    }
    match &link {
        Some(LinkKind::Markdown(_)) => tags.push("link"),
        Some(LinkKind::Wiki(_)) => {
            tags.push("link");
            tags.push("wikilink");
        }
        None => {}
    }

    let start = iter.offset();
    if tags.is_empty() {
        buffer.insert(iter, text);
    } else {
        buffer.insert_with_tags_by_name(iter, text, &tags);
    }
    let end = iter.offset();

    if let Some(link_kind) = link {
        match link_kind {
            LinkKind::Markdown(target) => links.push(LinkSpan {
                start,
                end,
                target,
                is_wiki: false,
            }),
            LinkKind::Wiki(target) => links.push(LinkSpan {
                start,
                end,
                target,
                is_wiki: true,
            }),
        }
    }
}


