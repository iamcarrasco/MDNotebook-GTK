
use base64::Engine as _;
use gtk::pango;
use gtk::prelude::*;

pub const TAG_H1: &str = "h1";
pub const TAG_H2: &str = "h2";
pub const TAG_H3: &str = "h3";
pub const TAG_H4: &str = "h4";
pub const TAG_H5: &str = "h5";
pub const TAG_H6: &str = "h6";
pub const TAG_BOLD: &str = "bold";
pub const TAG_ITALIC: &str = "italic";
pub const TAG_UNDERLINE: &str = "underline";
pub const TAG_STRIKE: &str = "strike";
pub const TAG_CODE: &str = "code";
pub const TAG_QUOTE: &str = "quote";
pub const TAG_LIST: &str = "list";
pub const TAG_RULE: &str = "rule";
pub const TAG_RAW_BLOCK: &str = "raw-block";
pub const TAG_CODE_BLOCK: &str = "code-block";
pub const TAG_CODE_FENCE: &str = "code-fence";

pub const CODE_LANGUAGES: &[&str] = &[
    "text", "javascript", "python", "rust", "bash",
    "html", "css", "json", "yaml", "go", "c", "cpp", "java", "sql",
];
pub const TAG_TABLE_CELL: &str = "table-cell";
pub const TAG_TABLE_SEP: &str = "table-sep";
pub const TAG_LINK: &str = "link";
pub const TAG_IMAGE: &str = "image";
pub const IMAGE_ALT_TAG_PREFIX: &str = "image-alt-||-";
const TAG_TASK_MARKER: &str = "task-marker";
const BLOCK_PLACEHOLDER: char = '\u{200b}';

#[derive(Copy, Clone)]
pub enum BlockType {
    Paragraph,
    Heading1,
    Heading2,
    Heading3,
    Heading4,
    Heading5,
    Heading6,
    Quote,
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum ListKind {
    Bullet,
    Ordered,
    Task,
}

enum MarkdownShortcut {
    Heading1,
    Heading2,
    Heading3,
    Heading4,
    Heading5,
    Heading6,
    Quote,
    Bullet,
    Ordered(String),
    TaskUnchecked,
    TaskChecked,
}

impl MarkdownShortcut {
    fn display_prefix(&self) -> Option<String> {
        match self {
            MarkdownShortcut::Heading1 => None,
            MarkdownShortcut::Heading2 => None,
            MarkdownShortcut::Heading3 => None,
            MarkdownShortcut::Heading4 => None,
            MarkdownShortcut::Heading5 => None,
            MarkdownShortcut::Heading6 => None,
            MarkdownShortcut::Quote => None,
            MarkdownShortcut::Bullet => Some("\u{2022} ".to_string()),
            MarkdownShortcut::Ordered(number) => Some(format!("{number}. ")),
            MarkdownShortcut::TaskUnchecked => Some("\u{2610} ".to_string()),
            MarkdownShortcut::TaskChecked => Some("\u{2611} ".to_string()),
        }
    }
}

#[derive(Copy, Clone, Default)]
struct InlineState {
    bold: bool,
    italic: bool,
    underline: bool,
    strike: bool,
    code: bool,
}



struct SerializeCtx {
    h1: Option<gtk::TextTag>,
    h2: Option<gtk::TextTag>,
    h3: Option<gtk::TextTag>,
    h4: Option<gtk::TextTag>,
    h5: Option<gtk::TextTag>,
    h6: Option<gtk::TextTag>,
    quote: Option<gtk::TextTag>,
    list: Option<gtk::TextTag>,
    rule: Option<gtk::TextTag>,
    bold: Option<gtk::TextTag>,
    italic: Option<gtk::TextTag>,
    underline: Option<gtk::TextTag>,
    strike: Option<gtk::TextTag>,
    code: Option<gtk::TextTag>,
}

pub fn install_tags(buffer: &gtk::TextBuffer) {
    let table = buffer.tag_table();

    let tags = [
        gtk::TextTag::builder()
            .name(TAG_H1)
            .weight(700)
            .scale(1.42)
            .pixels_above_lines(14)
            .pixels_below_lines(8)
            .build(),
        gtk::TextTag::builder()
            .name(TAG_H2)
            .weight(700)
            .scale(1.22)
            .pixels_above_lines(12)
            .pixels_below_lines(6)
            .build(),
        gtk::TextTag::builder()
            .name(TAG_H3)
            .weight(700)
            .scale(1.12)
            .pixels_above_lines(10)
            .pixels_below_lines(4)
            .build(),
        gtk::TextTag::builder()
            .name(TAG_H4)
            .weight(700)
            .scale(1.06)
            .pixels_above_lines(8)
            .pixels_below_lines(4)
            .build(),
        gtk::TextTag::builder()
            .name(TAG_H5)
            .weight(700)
            .pixels_above_lines(6)
            .pixels_below_lines(2)
            .build(),
        gtk::TextTag::builder()
            .name(TAG_H6)
            .weight(700)
            .scale(0.92)
            .style(pango::Style::Italic)
            .pixels_above_lines(6)
            .pixels_below_lines(2)
            .build(),
        gtk::TextTag::builder().name(TAG_BOLD).weight(700).build(),
        gtk::TextTag::builder()
            .name(TAG_ITALIC)
            .style(pango::Style::Italic)
            .build(),
        gtk::TextTag::builder()
            .name(TAG_UNDERLINE)
            .underline(pango::Underline::Single)
            .build(),
        gtk::TextTag::builder()
            .name(TAG_STRIKE)
            .strikethrough(true)
            .build(),
        gtk::TextTag::builder()
            .name(TAG_CODE)
            .family("monospace")
            .scale(0.96)
            .build(),
        gtk::TextTag::builder()
            .name(TAG_QUOTE)
            .left_margin(18)
            .style(pango::Style::Italic)
            .build(),
        gtk::TextTag::builder()
            .name(TAG_LIST)
            .left_margin(12)
            .build(),
        gtk::TextTag::builder().name(TAG_RULE).weight(700).build(),
        gtk::TextTag::builder()
            .name(TAG_RAW_BLOCK)
            .family("monospace")
            .scale(0.92)
            .left_margin(8)
            .right_margin(8)
            .pixels_above_lines(4)
            .pixels_below_lines(4)
            .editable(false)
            .build(),
        gtk::TextTag::builder()
            .name(TAG_CODE_BLOCK)
            .family("monospace")
            .scale(0.92)
            .left_margin(14)
            .right_margin(8)
            .build(),
        gtk::TextTag::builder()
            .name(TAG_CODE_FENCE)
            .family("monospace")
            .scale(0.85)
            .left_margin(8)
            .editable(false)
            .build(),
        gtk::TextTag::builder()
            .name(TAG_TABLE_CELL)
            .family("monospace")
            .build(),
        gtk::TextTag::builder()
            .name(TAG_TABLE_SEP)
            .family("monospace")
            .style(pango::Style::Italic)
            .build(),
        gtk::TextTag::builder()
            .name(TAG_LINK)
            .underline(pango::Underline::Single)
            .build(),
        gtk::TextTag::builder()
            .name(TAG_IMAGE)
            .style(pango::Style::Italic)
            .build(),
        gtk::TextTag::builder()
            .name(TAG_TASK_MARKER)
            .family("monospace")
            .scale(1.50)
            .build(),
    ];

    for tag in tags {
        table.add(&tag);
    }
}

pub fn load_markdown(buffer: &gtk::TextBuffer, markdown: &str) {
    buffer.set_text("");

    let mut iter = buffer.end_iter();
    let parser = pulldown_cmark::Parser::new_ext(markdown, pulldown_cmark::Options::all());

    let mut block_tags: Vec<&'static str> = Vec::new();
    let mut state = InlineState::default();
    let mut current_link: Option<String> = None;
    let mut list_index_stack: Vec<Option<u64>> = Vec::new(); // None for bullet, Some for ordered
    let mut in_image = false;
    let mut image_alt = String::new();
    let mut table_alignments: Vec<pulldown_cmark::Alignment> = Vec::new();

    let mut pending_item_prefix: Option<String> = None;

    let ensure_newline = |buffer: &gtk::TextBuffer, iter: &mut gtk::TextIter| {
        if !iter.starts_line() {
            buffer.insert(iter, "\n");
        }
    };
    let ensure_double_newline = |buffer: &gtk::TextBuffer, iter: &mut gtk::TextIter| {
        if iter.offset() == 0 {
            return;
        }
        
        let mut prev = *iter;
        if prev.backward_char() {
            if buffer.text(&prev, iter, true) == "\n" {
                let mut prev2 = prev;
                if prev2.backward_char() && buffer.text(&prev2, &prev, true) == "\n" {
                    return; // Already ends with \n\n
                }
                buffer.insert(iter, "\n"); // Add second \n
            } else {
                buffer.insert(iter, "\n\n"); // Add both \n
            }
        }
    };

    for event in parser {
        match event {
            pulldown_cmark::Event::Start(tag) => match tag {
                pulldown_cmark::Tag::Heading { level, .. } => {
                    let level_tag = match level {
                        pulldown_cmark::HeadingLevel::H1 => TAG_H1,
                        pulldown_cmark::HeadingLevel::H2 => TAG_H2,
                        pulldown_cmark::HeadingLevel::H3 => TAG_H3,
                        pulldown_cmark::HeadingLevel::H4 => TAG_H4,
                        pulldown_cmark::HeadingLevel::H5 => TAG_H5,
                        pulldown_cmark::HeadingLevel::H6 => TAG_H6,
                    };
                    block_tags.push(level_tag);
                }
                pulldown_cmark::Tag::BlockQuote(_) => block_tags.push(TAG_QUOTE),
                pulldown_cmark::Tag::CodeBlock(pulldown_cmark::CodeBlockKind::Fenced(lang)) => {
                    let lang_str = lang.as_ref();
                    let open_fence = if lang_str.is_empty() {
                        "```".to_string()
                    } else {
                        format!("```{}", lang_str)
                    };
                    let mut fence_tags: Vec<&str> = block_tags.clone();
                    fence_tags.push(TAG_CODE_FENCE);
                    buffer.insert_with_tags_by_name(&mut iter, &open_fence, &fence_tags);
                    buffer.insert(&mut iter, "\n");
                    block_tags.push(TAG_CODE_BLOCK);
                }
                pulldown_cmark::Tag::CodeBlock(pulldown_cmark::CodeBlockKind::Indented) => {
                    let mut fence_tags: Vec<&str> = block_tags.clone();
                    fence_tags.push(TAG_CODE_FENCE);
                    buffer.insert_with_tags_by_name(&mut iter, "```", &fence_tags);
                    buffer.insert(&mut iter, "\n");
                    block_tags.push(TAG_CODE_BLOCK);
                }
                pulldown_cmark::Tag::List(first_num) => {
                    ensure_newline(buffer, &mut iter);
                    list_index_stack.push(first_num);
                }
                pulldown_cmark::Tag::Item => {
                    ensure_newline(buffer, &mut iter);
                    let depth = list_index_stack.len().saturating_sub(1);
                    let mut indent_spaces = 0;
                    for entry in list_index_stack.iter().take(depth) {
                        if entry.is_some() {
                            indent_spaces += 3;
                        } else {
                            indent_spaces += 2;
                        }
                    }
                    let indent = " ".repeat(indent_spaces);
                    if let Some(Some(ref mut n)) = list_index_stack.last_mut() {
                        pending_item_prefix = Some(format!("{}{}. ", indent, n));
                        *n += 1;
                    } else {
                        pending_item_prefix = Some(format!("{}\u{2022} ", indent));
                    }
                }
                pulldown_cmark::Tag::Strong => state.bold = true,
                pulldown_cmark::Tag::Emphasis => state.italic = true,
                pulldown_cmark::Tag::Strikethrough => state.strike = true,
                pulldown_cmark::Tag::Link { dest_url, .. } => {
                    current_link = Some(dest_url.to_string());
                }
                pulldown_cmark::Tag::Image { dest_url, .. } => {
                    current_link = Some(dest_url.to_string());
                    in_image = true;
                    image_alt.clear();
                }
                pulldown_cmark::Tag::FootnoteDefinition(label) => {
                    ensure_newline(buffer, &mut iter);
                    let prefix = format!("[^{label}]: ");
                    insert_chunk(buffer, &mut iter, &prefix, &[TAG_RAW_BLOCK], state);
                    block_tags.push(TAG_RAW_BLOCK);
                }
                pulldown_cmark::Tag::Table(alignments) => {
                    table_alignments = alignments;
                }
                pulldown_cmark::Tag::TableHead => {
                    block_tags.push(TAG_TABLE_CELL);
                    buffer.insert_with_tags_by_name(&mut iter, "| ", &[TAG_TABLE_CELL]);
                }
                pulldown_cmark::Tag::TableRow => {
                    block_tags.push(TAG_TABLE_CELL);
                    buffer.insert_with_tags_by_name(&mut iter, "| ", &[TAG_TABLE_CELL]);
                }
                pulldown_cmark::Tag::TableCell => {}
                _ => {}
            },
            pulldown_cmark::Event::End(tag) => match tag {
                pulldown_cmark::TagEnd::Heading(_) => {
                    block_tags.pop();
                    ensure_double_newline(buffer, &mut iter);
                }
                pulldown_cmark::TagEnd::BlockQuote(_) => {
                    block_tags.pop();
                    ensure_double_newline(buffer, &mut iter);
                }
                pulldown_cmark::TagEnd::CodeBlock => {
                    block_tags.pop(); // removes TAG_CODE_BLOCK
                    let mut fence_tags: Vec<&str> = block_tags.clone();
                    fence_tags.push(TAG_CODE_FENCE);
                    buffer.insert_with_tags_by_name(&mut iter, "```", &fence_tags);
                    ensure_double_newline(buffer, &mut iter);
                }
                pulldown_cmark::TagEnd::List(_) => {
                    list_index_stack.pop();
                    if list_index_stack.is_empty() {
                        ensure_double_newline(buffer, &mut iter);
                    } else {
                        ensure_newline(buffer, &mut iter);
                    }
                }
                pulldown_cmark::TagEnd::Item => {
                    if let Some(prefix) = pending_item_prefix.take() {
                        let mut t = block_tags.clone();
                        t.push(TAG_LIST);
                        insert_chunk(buffer, &mut iter, &prefix, &t, state);
                    }
                    ensure_newline(buffer, &mut iter);
                }
                pulldown_cmark::TagEnd::Strong => state.bold = false,
                pulldown_cmark::TagEnd::Emphasis => state.italic = false,
                pulldown_cmark::TagEnd::Strikethrough => state.strike = false,
                pulldown_cmark::TagEnd::Link => current_link = None,
                pulldown_cmark::TagEnd::Image => {
                    // Always insert an image token; real bitmap rendering is applied later.
                    let url = current_link.clone().unwrap_or_default();
                    let tag_name = format!("image-||-{}", url);
                    if buffer.tag_table().lookup(&tag_name).is_none() {
                        let tag = gtk::TextTag::builder().name(&tag_name).build();
                        buffer.tag_table().add(&tag);
                    }
                    let alt_tag_name = if image_alt.is_empty() {
                        None
                    } else {
                        let encoded_alt = base64::engine::general_purpose::URL_SAFE_NO_PAD
                            .encode(image_alt.as_bytes());
                        let name = format!("{IMAGE_ALT_TAG_PREFIX}{encoded_alt}");
                        if buffer.tag_table().lookup(&name).is_none() {
                            let tag = gtk::TextTag::builder().name(&name).build();
                            buffer.tag_table().add(&tag);
                        }
                        Some(name)
                    };

                    let mut current_tags: Vec<&str> = block_tags.to_vec();
                    current_tags.push(TAG_IMAGE);
                    current_tags.push(&tag_name);
                    if let Some(name) = alt_tag_name.as_deref() {
                        current_tags.push(name);
                    }

                    // Placeholder glyph; gets replaced by an inline paintable texture.
                    buffer.insert_with_tags_by_name(&mut iter, "\u{1f5bc}", &current_tags);

                    current_link = None;
                    in_image = false;
                    image_alt.clear();
                }
                pulldown_cmark::TagEnd::FootnoteDefinition => {
                    block_tags.pop();
                    ensure_double_newline(buffer, &mut iter);
                }
                pulldown_cmark::TagEnd::Table => {
                    ensure_double_newline(buffer, &mut iter);
                }
                pulldown_cmark::TagEnd::TableHead => {
                    block_tags.pop();
                    let cols = table_alignments.len().max(1);
                    let mut sep = String::from("\n");
                    for i in 0..cols {
                        let align = table_alignments.get(i).copied()
                            .unwrap_or(pulldown_cmark::Alignment::None);
                        let cell = match align {
                            pulldown_cmark::Alignment::Left => "| :--- ",
                            pulldown_cmark::Alignment::Center => "| :---: ",
                            pulldown_cmark::Alignment::Right => "| ---: ",
                            pulldown_cmark::Alignment::None => "| --- ",
                        };
                        sep.push_str(cell);
                    }
                    sep.push_str("|\n");
                    buffer.insert_with_tags_by_name(&mut iter, &sep, &[TAG_TABLE_SEP]);
                }
                pulldown_cmark::TagEnd::TableRow => {
                    block_tags.pop();
                    ensure_newline(buffer, &mut iter);
                }
                pulldown_cmark::TagEnd::TableCell => {
                    insert_chunk(buffer, &mut iter, " | ", &block_tags, state);
                }
                pulldown_cmark::TagEnd::Paragraph => {
                    if list_index_stack.is_empty() && !block_tags.contains(&TAG_QUOTE) {
                        ensure_double_newline(buffer, &mut iter);
                    } else {
                        ensure_newline(buffer, &mut iter);
                    }
                }
                _ => {}
            },
            pulldown_cmark::Event::Text(text) => {
                if let Some(prefix) = pending_item_prefix.take() {
                    let mut t = block_tags.clone();
                    t.push(TAG_LIST);
                    insert_chunk(buffer, &mut iter, &prefix, &t, state);
                }

                if in_image {
                    image_alt.push_str(text.as_ref());
                } else if let Some(url) = current_link.as_ref() {
                    let tag_name = format!("link-||-{}", url);

                    if buffer.tag_table().lookup(&tag_name).is_none() {
                        let tag = gtk::TextTag::builder().name(&tag_name).build();
                        buffer.tag_table().add(&tag);
                    }

                    let mut current_tags: Vec<&str> = block_tags.to_vec();
                    current_tags.push(TAG_LINK);
                    current_tags.push(&tag_name);

                    if state.bold { current_tags.push(TAG_BOLD); }
                    if state.italic { current_tags.push(TAG_ITALIC); }
                    if state.strike { current_tags.push(TAG_STRIKE); }
                    if state.code { current_tags.push(TAG_CODE); }

                    let text_str = text.as_ref();
                    buffer.insert_with_tags_by_name(&mut iter, text_str, &current_tags);
                } else {
                    let mut current_tags = block_tags.clone();
                    if !list_index_stack.is_empty() {
                        current_tags.push(TAG_LIST);
                    }
                    insert_chunk(buffer, &mut iter, &text, &current_tags, state);
                }
            }
            pulldown_cmark::Event::Code(code) => {
                if let Some(prefix) = pending_item_prefix.take() {
                    let mut t = block_tags.clone();
                    t.push(TAG_LIST);
                    insert_chunk(buffer, &mut iter, &prefix, &t, state);
                }
                let mut temp_state = state;
                temp_state.code = true;

                let mut current_tags = block_tags.clone();
                if !list_index_stack.is_empty() { current_tags.push(TAG_LIST); }
                insert_chunk(buffer, &mut iter, &code, &current_tags, temp_state);
            }
            pulldown_cmark::Event::InlineHtml(html) => {
                let tag = html.trim();
                let tag_lower = tag.to_ascii_lowercase();
                if tag_lower == "<u>" || tag_lower.starts_with("<u ") {
                    state.underline = true;
                } else if tag_lower == "</u>" {
                    state.underline = false;
                } else {
                    if let Some(prefix) = pending_item_prefix.take() {
                        let mut t = block_tags.clone();
                        t.push(TAG_LIST);
                        insert_chunk(buffer, &mut iter, &prefix, &t, state);
                    }
                    let mut current_tags = block_tags.clone();
                    if !list_index_stack.is_empty() { current_tags.push(TAG_LIST); }
                    insert_chunk(buffer, &mut iter, &html, &current_tags, state);
                }
            }
            pulldown_cmark::Event::Html(html) => {
                let trimmed = html.trim();
                if trimmed.eq_ignore_ascii_case("<u>") {
                    state.underline = true;
                } else if trimmed.eq_ignore_ascii_case("</u>") {
                    state.underline = false;
                } else {
                    if let Some(prefix) = pending_item_prefix.take() {
                        let mut t = block_tags.clone();
                        t.push(TAG_LIST);
                        insert_chunk(buffer, &mut iter, &prefix, &t, state);
                    }
                    let mut current_tags = block_tags.clone();
                    if !list_index_stack.is_empty() { current_tags.push(TAG_LIST); }
                    insert_chunk(buffer, &mut iter, &html, &current_tags, state);
                }
            }
            pulldown_cmark::Event::SoftBreak | pulldown_cmark::Event::HardBreak => {
                ensure_newline(buffer, &mut iter);
            }
            pulldown_cmark::Event::Rule => {
                let rule_str = format!("\n{}\n\n", "\u{2500}".repeat(60));
                insert_chunk(buffer, &mut iter, &rule_str, &[TAG_RULE], state);
            }
            pulldown_cmark::Event::TaskListMarker(checked) => {
                if let Some(_prefix) = pending_item_prefix.take() {}
                let mut t = block_tags.clone();
                t.push(TAG_LIST);
                t.push(TAG_TASK_MARKER);
                let depth = list_index_stack.len().saturating_sub(1);
                let mut indent_spaces = 0;
                for entry in list_index_stack.iter().take(depth) {
                    if entry.is_some() {
                        indent_spaces += 3;
                    } else {
                        indent_spaces += 2;
                    }
                }
                let indent = " ".repeat(indent_spaces);

                if checked {
                    buffer.insert_with_tags_by_name(&mut iter, &format!("{}\u{2611} ", indent), &t);
                } else {
                    buffer.insert_with_tags_by_name(&mut iter, &format!("{}\u{2610} ", indent), &t);
                }
            }
            pulldown_cmark::Event::FootnoteReference(label) => {
                if let Some(prefix) = pending_item_prefix.take() {
                    let mut t = block_tags.clone();
                    t.push(TAG_LIST);
                    insert_chunk(buffer, &mut iter, &prefix, &t, state);
                }
                let mut current_tags = block_tags.clone();
                if !list_index_stack.is_empty() {
                    current_tags.push(TAG_LIST);
                }
                let rendered = format!("[^{label}]");
                insert_chunk(buffer, &mut iter, &rendered, &current_tags, state);
            }
            _ => {}
        }
    }

    let mut end = buffer.end_iter();
    let mut last = end;
    if last.backward_char() {
        while buffer.text(&last, &end, true) == "\n" {
            buffer.delete(&mut last, &mut end);
            if !last.backward_char() { break; }
        }
    }

}



pub fn to_markdown(buffer: &gtk::TextBuffer) -> String {
    let table = buffer.tag_table();
    let raw_tag = table.lookup(TAG_RAW_BLOCK);
    let code_fence_tag = table.lookup(TAG_CODE_FENCE);
    let code_block_tag = table.lookup(TAG_CODE_BLOCK);
    let table_cell_tag = table.lookup(TAG_TABLE_CELL);
    let table_sep_tag = table.lookup(TAG_TABLE_SEP);
    let tags = SerializeCtx {
        h1: table.lookup(TAG_H1),
        h2: table.lookup(TAG_H2),
        h3: table.lookup(TAG_H3),
        h4: table.lookup(TAG_H4),
        h5: table.lookup(TAG_H5),
        h6: table.lookup(TAG_H6),
        quote: table.lookup(TAG_QUOTE),
        list: table.lookup(TAG_LIST),
        rule: table.lookup(TAG_RULE),
        bold: table.lookup(TAG_BOLD),
        italic: table.lookup(TAG_ITALIC),
        underline: table.lookup(TAG_UNDERLINE),
        strike: table.lookup(TAG_STRIKE),
        code: table.lookup(TAG_CODE),
    };

    let mut lines: Vec<String> = Vec::new();
    let mut line_start = buffer.start_iter();

    loop {
        let mut line_end = line_start;
        if !line_end.ends_line() {
            line_end.forward_to_line_end();
        }

        // Verbatim tags: raw-block, code-fence, code-block, table-cell, table-sep
        let is_verbatim = has_resolved_tag(&line_start, raw_tag.as_ref())
            || has_resolved_tag(&line_start, code_fence_tag.as_ref())
            || has_resolved_tag(&line_start, code_block_tag.as_ref())
            || has_resolved_tag(&line_start, table_cell_tag.as_ref())
            || has_resolved_tag(&line_start, table_sep_tag.as_ref());

        if is_verbatim {
            let mut raw = buffer.text(&line_start, &line_end, true).to_string();
            if has_resolved_tag(&line_start, tags.quote.as_ref()) {
                raw = format!("> {raw}");
            }
            lines.push(raw);
        } else {
            lines.push(serialize_line(buffer, &line_start, &line_end, &tags));
        }

        if !line_start.forward_line() {
            break;
        }
    }

    lines.join("\n")
}

pub fn toggle_inline_tag(buffer: &gtk::TextBuffer, tag_name: &str, _placeholder: &str) {
    if let Some((start, end)) = buffer.selection_bounds() {
        if start.offset() == end.offset() {
            if let Some((word_start, word_end)) = current_word_range(buffer) {
                toggle_inline_range(buffer, tag_name, &word_start, &word_end);
            }
            return;
        }

        toggle_inline_range(buffer, tag_name, &start, &end);
        return;
    }

    if let Some((word_start, word_end)) = current_word_range(buffer) {
        toggle_inline_range(buffer, tag_name, &word_start, &word_end);
    }
}

pub fn set_block_type(buffer: &gtk::TextBuffer, block: BlockType) {
    let (start, end) = selected_line_range(buffer);
    let lines = collect_line_ranges(&start, &end);

    // Collect offsets of non-empty lines; process in reverse so text
    // removals (list-prefix stripping) don't shift later offsets.
    let offsets: Vec<i32> = lines
        .iter()
        .filter(|(ls, le)| !line_is_empty(buffer, ls, le))
        .map(|(ls, _)| ls.offset())
        .collect();

    for offset in offsets.into_iter().rev() {
        // Strip any list-prefix characters (bullet, checkbox, number)
        remove_any_list_prefix(buffer, offset);

        // Recompute line range after potential text deletion
        let line_start = buffer.iter_at_offset(offset);
        let mut line_end = line_start;
        line_end.forward_to_line_end();

        // Clear ALL block tags (heading, quote, list)
        clear_block_tags(buffer, &line_start, &line_end);

        match block {
            BlockType::Paragraph => {}
            BlockType::Heading1 => buffer.apply_tag_by_name(TAG_H1, &line_start, &line_end),
            BlockType::Heading2 => buffer.apply_tag_by_name(TAG_H2, &line_start, &line_end),
            BlockType::Heading3 => buffer.apply_tag_by_name(TAG_H3, &line_start, &line_end),
            BlockType::Heading4 => buffer.apply_tag_by_name(TAG_H4, &line_start, &line_end),
            BlockType::Heading5 => buffer.apply_tag_by_name(TAG_H5, &line_start, &line_end),
            BlockType::Heading6 => buffer.apply_tag_by_name(TAG_H6, &line_start, &line_end),
            BlockType::Quote => buffer.apply_tag_by_name(TAG_QUOTE, &line_start, &line_end),
        }
    }
}

pub fn toggle_heading(buffer: &gtk::TextBuffer, level: u8) {
    let target = match level {
        1 => TAG_H1,
        2 => TAG_H2,
        3 => TAG_H3,
        4 => TAG_H4,
        5 => TAG_H5,
        _ => TAG_H6,
    };
    let (start, end) = selected_line_range(buffer);
    let lines = collect_line_ranges(&start, &end);

    // Collect non-empty line info before modifying the buffer
    let non_empty: Vec<(i32, bool)> = lines
        .iter()
        .filter(|(ls, le)| !line_is_empty(buffer, ls, le))
        .map(|(ls, le)| (ls.offset(), line_has_tag(buffer, ls, le, target)))
        .collect();

    let should_remove = !non_empty.is_empty() && non_empty.iter().all(|(_, has)| *has);

    for (offset, _) in non_empty.into_iter().rev() {
        remove_any_list_prefix(buffer, offset);

        let line_start = buffer.iter_at_offset(offset);
        let mut line_end = line_start;
        line_end.forward_to_line_end();

        clear_block_tags(buffer, &line_start, &line_end);

        if !should_remove {
            buffer.apply_tag_by_name(target, &line_start, &line_end);
        }
    }
}

pub fn toggle_quote(buffer: &gtk::TextBuffer) {
    let (start, end) = selected_line_range(buffer);
    let lines = collect_line_ranges(&start, &end);

    let non_empty: Vec<(i32, bool)> = lines
        .iter()
        .filter(|(ls, le)| !line_is_empty(buffer, ls, le))
        .map(|(ls, le)| (ls.offset(), line_has_tag(buffer, ls, le, TAG_QUOTE)))
        .collect();

    let should_remove = !non_empty.is_empty() && non_empty.iter().all(|(_, has)| *has);

    for (offset, _) in non_empty.into_iter().rev() {
        remove_any_list_prefix(buffer, offset);

        let line_start = buffer.iter_at_offset(offset);
        let mut line_end = line_start;
        line_end.forward_to_line_end();

        clear_block_tags(buffer, &line_start, &line_end);

        if !should_remove {
            buffer.apply_tag_by_name(TAG_QUOTE, &line_start, &line_end);
        }
    }
}

pub fn toggle_bullet_list(buffer: &gtk::TextBuffer) {
    toggle_list_kind(buffer, ListKind::Bullet);
}

pub fn toggle_ordered_list(buffer: &gtk::TextBuffer) {
    toggle_list_kind(buffer, ListKind::Ordered);
}

pub fn toggle_task_list(buffer: &gtk::TextBuffer) {
    toggle_list_kind(buffer, ListKind::Task);
}

pub fn continue_list_on_enter(buffer: &gtk::TextBuffer) -> bool {
    if let Some((start, end)) = buffer.selection_bounds() {
        if start.offset() != end.offset() {
            return false;
        }
    }

    let cursor_offset = buffer.cursor_position();
    let cursor = buffer.iter_at_offset(cursor_offset);
    let mut line_start = cursor;
    line_start.set_line_offset(0);
    let mut line_end = line_start;
    line_end.forward_to_line_end();

    let line_text = buffer.text(&line_start, &line_end, true).to_string();
    let Some((prefix_chars, next_prefix)) = list_continuation_prefix(&line_text) else {
        return false;
    };

    // If the line is just the prefix with no content, remove it to end the list
    let content_after_prefix = strip_block_placeholders(
        &line_text[line_text.char_indices().nth(prefix_chars).map_or(line_text.len(), |(i, _)| i)..]
    );
    if content_after_prefix.trim().is_empty() {
        let line_start_offset = line_start.offset();
        let mut del_start = buffer.iter_at_offset(line_start_offset);
        let mut del_end = buffer.iter_at_offset(line_start_offset);
        del_end.forward_chars(prefix_chars as i32);
        buffer.delete(&mut del_start, &mut del_end);
        let ls = buffer.iter_at_offset(line_start_offset);
        let mut le = ls;
        le.forward_to_line_end();
        buffer.remove_tag_by_name(TAG_LIST, &ls, &le);
        return true;
    }

    let is_task = is_task_prefix(&next_prefix);

    // If caret is inside the list marker/prefix (common after checkbox clicks),
    // continue from end-of-line to avoid splitting item text unexpectedly.
    let line_start_offset = line_start.offset();
    let line_end_offset = line_end.offset();
    let cursor_col = cursor_offset - line_start_offset;
    let insert_offset = if cursor_col <= prefix_chars as i32 {
        line_end_offset
    } else {
        cursor_offset
    };

    let mut insert_at = buffer.iter_at_offset(insert_offset);
    buffer.insert(&mut insert_at, "\n");

    let prefix_start_offset = insert_offset + 1;
    let mut prefix_at = buffer.iter_at_offset(prefix_start_offset);
    if is_task {
        buffer.insert_with_tags_by_name(&mut prefix_at, &next_prefix, &[TAG_LIST, TAG_TASK_MARKER]);
    } else {
        buffer.insert(&mut prefix_at, &next_prefix);
    }

    let new_line_start = buffer.iter_at_offset(prefix_start_offset);
    let mut new_line_end = new_line_start;
    new_line_end.forward_to_line_end();
    buffer.apply_tag_by_name(TAG_LIST, &new_line_start, &new_line_end);

    true
}

pub fn apply_markdown_shortcut_on_space(buffer: &gtk::TextBuffer) -> bool {
    if let Some((start, end)) = buffer.selection_bounds() {
        if start.offset() != end.offset() {
            return false;
        }
    }

    let cursor_offset = buffer.cursor_position();
    let cursor = buffer.iter_at_offset(cursor_offset);
    let mut line_start = cursor;
    line_start.set_line_offset(0);

    let prefix = buffer.text(&line_start, &cursor, true).to_string();
    let Some(shortcut) = parse_markdown_shortcut(&prefix) else {
        return false;
    };

    let line_start_offset = line_start.offset();
    let mut delete_start = buffer.iter_at_offset(line_start_offset);
    let mut delete_end = buffer.iter_at_offset(cursor_offset);
    buffer.delete(&mut delete_start, &mut delete_end);

    let mut cursor_after = buffer.iter_at_offset(line_start_offset);
    let is_task = matches!(
        shortcut,
        MarkdownShortcut::TaskUnchecked | MarkdownShortcut::TaskChecked
    );
    if let Some(display_prefix) = shortcut.display_prefix() {
        if is_task {
            buffer.insert_with_tags_by_name(
                &mut cursor_after,
                &display_prefix,
                &[TAG_LIST, TAG_TASK_MARKER],
            );
        } else {
            buffer.insert(&mut cursor_after, &display_prefix);
        }
    }
    buffer.place_cursor(&cursor_after);

    let line_start = buffer.iter_at_offset(line_start_offset);
    let mut line_end = line_start;
    line_end.forward_to_line_end();
    clear_block_tags(buffer, &line_start, &line_end);

    match shortcut {
        MarkdownShortcut::Heading1 => {
            apply_block_tag_with_placeholder(buffer, line_start_offset, TAG_H1)
        }
        MarkdownShortcut::Heading2 => {
            apply_block_tag_with_placeholder(buffer, line_start_offset, TAG_H2)
        }
        MarkdownShortcut::Heading3 => {
            apply_block_tag_with_placeholder(buffer, line_start_offset, TAG_H3)
        }
        MarkdownShortcut::Heading4 => {
            apply_block_tag_with_placeholder(buffer, line_start_offset, TAG_H4)
        }
        MarkdownShortcut::Heading5 => {
            apply_block_tag_with_placeholder(buffer, line_start_offset, TAG_H5)
        }
        MarkdownShortcut::Heading6 => {
            apply_block_tag_with_placeholder(buffer, line_start_offset, TAG_H6)
        }
        MarkdownShortcut::Quote => {
            apply_block_tag_with_placeholder(buffer, line_start_offset, TAG_QUOTE)
        }
        MarkdownShortcut::Bullet
        | MarkdownShortcut::Ordered(_)
        | MarkdownShortcut::TaskUnchecked
        | MarkdownShortcut::TaskChecked => {
            buffer.apply_tag_by_name(TAG_LIST, &line_start, &line_end)
        }
    }

    true
}

pub fn toggle_task_marker_at_cursor(buffer: &gtk::TextBuffer, cursor: &gtk::TextIter) -> bool {
    let mut line_start = *cursor;
    line_start.set_line_offset(0);
    let mut line_end = line_start;
    line_end.forward_to_line_end();

    let text = buffer.text(&line_start, &line_end, true).to_string();
    let stripped = text.trim_start_matches(' ');
    let indent = text.len() - stripped.len();

    let (from_len, to) = if stripped.starts_with("\u{2610} ") {
        (indent + 2, format!("{}\u{2611} ", &text[..indent]))
    } else if stripped.starts_with("\u{2611} ") {
        (indent + 2, format!("{}\u{2610} ", &text[..indent]))
    } else if stripped.starts_with("- [ ] ") {
        (indent + 6, format!("{}- [x] ", &text[..indent]))
    } else if stripped.starts_with("* [ ] ") {
        (indent + 6, format!("{}* [x] ", &text[..indent]))
    } else if stripped.starts_with("- [x] ") || stripped.starts_with("- [X] ") {
        (indent + 6, format!("{}- [ ] ", &text[..indent]))
    } else if stripped.starts_with("* [x] ") || stripped.starts_with("* [X] ") {
        (indent + 6, format!("{}* [ ] ", &text[..indent]))
    } else {
        return false;
    };

    let line_start_offset = line_start.offset();
    let mut delete_start = buffer.iter_at_offset(line_start_offset);
    let mut delete_end = buffer.iter_at_offset(line_start_offset);
    delete_end.forward_chars(from_len as i32);
    buffer.delete(&mut delete_start, &mut delete_end);

    let mut insert_at = buffer.iter_at_offset(line_start_offset);
    buffer.insert_with_tags_by_name(&mut insert_at, &to, &[TAG_LIST, TAG_TASK_MARKER]);

    true
}

pub fn indent_list_item(buffer: &gtk::TextBuffer) -> bool {
    const LIST_INDENT: &str = "  ";

    let offsets = list_line_offsets_for_indent(buffer);
    if offsets.is_empty() {
        return false;
    }

    for offset in offsets.into_iter().rev() {
        let mut insert_at = buffer.iter_at_offset(offset);
        buffer.insert(&mut insert_at, LIST_INDENT);
    }
    true
}

pub fn dedent_list_item(buffer: &gtk::TextBuffer) -> bool {
    const LIST_INDENT_STEP: usize = 2;

    let offsets = list_line_offsets_for_indent(buffer);
    if offsets.is_empty() {
        return false;
    }

    let mut changed = false;
    for offset in offsets.into_iter().rev() {
        let line_start = buffer.iter_at_offset(offset);
        let mut line_end = line_start;
        line_end.forward_to_line_end();
        let text = buffer.text(&line_start, &line_end, true).to_string();
        let spaces = text
            .chars()
            .take_while(|c| *c == ' ')
            .count()
            .min(LIST_INDENT_STEP);
        if spaces == 0 {
            continue;
        }

        let mut del_start = buffer.iter_at_offset(offset);
        let mut del_end = buffer.iter_at_offset(offset);
        del_end.forward_chars(spaces as i32);
        buffer.delete(&mut del_start, &mut del_end);
        changed = true;
    }

    changed
}

fn list_line_offsets_for_indent(buffer: &gtk::TextBuffer) -> Vec<i32> {
    let (start, end) = selected_line_range(buffer);
    let lines = collect_line_ranges(&start, &end);

    let mut offsets = Vec::new();
    for (line_start, line_end) in lines {
        if line_is_empty(buffer, &line_start, &line_end) {
            continue;
        }

        let raw = buffer.text(&line_start, &line_end, true).to_string();
        let text = strip_block_placeholders(&raw);

        if line_has_tag(buffer, &line_start, &line_end, TAG_LIST)
            || detect_list_prefix_for_markdown(&text).is_some()
        {
            offsets.push(line_start.offset());
        }
    }

    offsets
}

/// Check for completed inline markdown patterns at the cursor and auto-convert them.
/// Returns true if a conversion was applied.
/// Patterns: `**text**` → bold, `*text*` → italic, `` `text` `` → code, `~~text~~` → strike.
pub fn check_inline_markdown_shortcuts(buffer: &gtk::TextBuffer) -> bool {
    let cursor_offset = buffer.cursor_position();
    let cursor = buffer.iter_at_offset(cursor_offset);
    let mut line_start = cursor;
    line_start.set_line_offset(0);

    let text_before = buffer.text(&line_start, &cursor, true).to_string();

    // Try patterns from longest marker to shortest
    if let Some(result) = try_inline_pattern(&text_before, "**", TAG_BOLD) {
        return apply_inline_convert(buffer, &line_start, &result);
    }
    if let Some(result) = try_inline_pattern(&text_before, "~~", TAG_STRIKE) {
        return apply_inline_convert(buffer, &line_start, &result);
    }
    // Single * for italic — but only if not part of **
    if let Some(result) = try_inline_pattern_single_star(&text_before) {
        return apply_inline_convert(buffer, &line_start, &result);
    }
    if let Some(result) = try_inline_pattern(&text_before, "`", TAG_CODE) {
        return apply_inline_convert(buffer, &line_start, &result);
    }

    false
}

struct InlineConvertResult {
    /// Byte offset from line_start where the opening marker begins
    open_start: usize,
    /// The marker string (e.g., "**", "~~", "`", "*")
    marker_len: usize,
    /// The text between markers
    content: String,
    /// Tag to apply
    tag: &'static str,
}

fn try_inline_pattern(text: &str, marker: &str, tag: &'static str) -> Option<InlineConvertResult> {
    // Text must end with the marker (the closing marker the user just typed)
    if !text.ends_with(marker) {
        return None;
    }
    // Look for opening marker before the closing one
    let before_close = &text[..text.len() - marker.len()];
    let open_pos = before_close.rfind(marker)?;
    let content = &before_close[open_pos + marker.len()..];
    if content.is_empty() || content.trim().is_empty() {
        return None;
    }
    Some(InlineConvertResult {
        open_start: open_pos,
        marker_len: marker.len(),
        content: content.to_string(),
        tag,
    })
}

fn try_inline_pattern_single_star(text: &str) -> Option<InlineConvertResult> {
    if !text.ends_with('*') {
        return None;
    }
    // Make sure the closing * is not part of **
    if text.ends_with("**") {
        return None;
    }
    let before_close = &text[..text.len() - 1];
    // Find the opening *, but skip ** sequences
    let mut search = before_close.len();
    loop {
        let pos = before_close[..search].rfind('*')?;
        // Check if this * is part of a **
        if pos > 0 && before_close.as_bytes().get(pos - 1) == Some(&b'*') {
            search = pos.saturating_sub(1);
            if search == 0 {
                return None;
            }
            continue;
        }
        if pos + 1 < before_close.len() && before_close.as_bytes().get(pos + 1) == Some(&b'*') {
            search = pos;
            if search == 0 {
                return None;
            }
            continue;
        }
        let content = &before_close[pos + 1..];
        if content.is_empty() || content.trim().is_empty() {
            return None;
        }
        return Some(InlineConvertResult {
            open_start: pos,
            marker_len: 1,
            content: content.to_string(),
            tag: TAG_ITALIC,
        });
    }
}

fn apply_inline_convert(
    buffer: &gtk::TextBuffer,
    line_start: &gtk::TextIter,
    result: &InlineConvertResult,
) -> bool {
    let line_offset = line_start.offset();

    buffer.begin_user_action();

    // Delete the entire range from opening marker to end of closing marker
    let full_start = line_offset + result.open_start as i32;
    let full_end = full_start
        + result.marker_len as i32
        + result.content.len() as i32
        + result.marker_len as i32;

    let mut del_start = buffer.iter_at_offset(full_start);
    let mut del_end = buffer.iter_at_offset(full_end);
    buffer.delete(&mut del_start, &mut del_end);

    // Insert the content with the tag applied
    let mut ins = buffer.iter_at_offset(full_start);
    buffer.insert_with_tags_by_name(&mut ins, &result.content, &[result.tag]);

    buffer.end_user_action();
    true
}

/// Renumber consecutive ordered list items in the buffer.
/// Called after operations that affect list structure (enter, toggle, indent/dedent).
pub fn renumber_ordered_list(buffer: &gtk::TextBuffer) {
    let total_lines = buffer.line_count();
    let mut i = 0;

    while i < total_lines {
        let ls = buffer.iter_at_line(i).unwrap_or(buffer.start_iter());
        let mut le = ls;
        if !le.ends_line() {
            le.forward_to_line_end();
        }
        let text = buffer.text(&ls, &le, true).to_string();
        let stripped = text.trim_start();
        let indent = text.len() - stripped.len();

        if let Some((num_str, _, _prefix_chars)) = parse_ordered_prefix(stripped) {
            // Found start of an ordered list run at this indent level
            let delim = if stripped.as_bytes().get(num_str.len()) == Some(&b')') {
                ')'
            } else {
                '.'
            };

            let mut expected = 1u64;
            let mut j = i;

            while j < total_lines {
                let jls = buffer.iter_at_line(j).unwrap_or(buffer.start_iter());
                let mut jle = jls;
                if !jle.ends_line() {
                    jle.forward_to_line_end();
                }
                let jtext = buffer.text(&jls, &jle, true).to_string();
                let jstripped = jtext.trim_start();
                let jindent = jtext.len() - jstripped.len();

                if jindent != indent {
                    break;
                }

                if let Some((cur_num, _, cur_prefix)) = parse_ordered_prefix(jstripped) {
                    let cur_val: u64 = cur_num.parse().unwrap_or(0);
                    if cur_val != expected {
                        // Need to renumber this line
                        let new_prefix = format!("{expected}{delim} ");
                        let old_prefix_byte_len = jstripped.len() - jstripped[cur_prefix..].len();
                        let abs_start = jls.offset() + jindent as i32;
                        let abs_end = abs_start + old_prefix_byte_len as i32;
                        let mut del_start = buffer.iter_at_offset(abs_start);
                        let mut del_end = buffer.iter_at_offset(abs_end);
                        buffer.delete(&mut del_start, &mut del_end);
                        let mut ins = buffer.iter_at_offset(abs_start);
                        buffer.insert(&mut ins, &new_prefix);
                    }
                    expected += 1;
                    j += 1;
                } else {
                    break;
                }
            }
            i = j;
        } else {
            i += 1;
        }
    }
}

pub fn insert_horizontal_rule(buffer: &gtk::TextBuffer) {
    let mut cursor = buffer.iter_at_offset(buffer.cursor_position());
    buffer.insert(&mut cursor, "\n");

    let rule_chars = "\u{2500}".repeat(60);
    let start = cursor;
    buffer.insert_with_tags_by_name(
        &mut cursor,
        &rule_chars,
        &[TAG_RULE],
    );
    buffer.insert(&mut cursor, "\n");

    let mut end = start;
    end.forward_chars(60);
    buffer.apply_tag_by_name(TAG_RULE, &start, &end);
}

/// Insert a code block with proper tagged structure.
pub fn insert_code_block(buffer: &gtk::TextBuffer, language: &str) {
    let mut iter = buffer.iter_at_offset(buffer.cursor_position());
    // Ensure we start on a new line
    if iter.line_offset() != 0 {
        buffer.insert(&mut iter, "\n");
    }
    let fence_open = if language.is_empty() {
        "```".to_string()
    } else {
        format!("```{language}")
    };
    buffer.insert_with_tags_by_name(&mut iter, &fence_open, &[TAG_CODE_FENCE]);
    buffer.insert(&mut iter, "\n");
    buffer.insert_with_tags_by_name(&mut iter, "", &[TAG_CODE_BLOCK]);
    let cursor_offset = iter.offset();
    buffer.insert(&mut iter, "\n");
    buffer.insert_with_tags_by_name(&mut iter, "```", &[TAG_CODE_FENCE]);
    buffer.insert(&mut iter, "\n");
    // Place cursor on the empty code line
    let cursor_iter = buffer.iter_at_offset(cursor_offset);
    buffer.place_cursor(&cursor_iter);
}


/// Returns true if the cursor is at a fence boundary and the given key action
/// (Backspace or Delete) would cross into or out of a non-editable fence region.
pub fn would_cross_fence_boundary(buffer: &gtk::TextBuffer, is_backspace: bool) -> bool {
    let table = buffer.tag_table();
    let fence_tag = match table.lookup(TAG_CODE_FENCE) {
        Some(t) => t,
        None => return false,
    };

    let cursor = buffer.iter_at_offset(buffer.cursor_position());

    if is_backspace {
        // Backspace: check the char before cursor
        if cursor.offset() == 0 {
            return false;
        }
        let mut prev = cursor;
        prev.backward_char();
        prev.has_tag(&fence_tag) != cursor.has_tag(&fence_tag)
    } else {
        // Delete: check the char after cursor
        let mut next = cursor;
        if !next.forward_char() {
            return false;
        }
        next.has_tag(&fence_tag) != cursor.has_tag(&fence_tag)
    }
}

pub fn handle_bulk_delete_cross_block(buffer: &gtk::TextBuffer) -> bool {
    if let Some((mut start, mut end)) = buffer.selection_bounds() {
        if start.line() != end.line() {
            let start_tags = block_tags_at(buffer, &start);
            let end_tags = block_tags_at(buffer, &end);
            
            if start_tags != end_tags && !start_tags.is_empty() && !end_tags.is_empty() {
                buffer.delete(&mut start, &mut end);
                
                let mut line_start = start;
                line_start.set_line_offset(0);
                let mut line_end = start;
                if !line_end.ends_line() {
                    line_end.forward_to_line_end();
                }
                
                for tag_name in [TAG_H1, TAG_H2, TAG_H3, TAG_H4, TAG_H5, TAG_H6, TAG_QUOTE, TAG_LIST, TAG_CODE_BLOCK, TAG_RAW_BLOCK] {
                    buffer.remove_tag_by_name(tag_name, &line_start, &line_end);
                }
                return true;
            }
        }
    }
    false
}

fn block_tags_at(_buffer: &gtk::TextBuffer, iter: &gtk::TextIter) -> Vec<String> {
    iter.tags().iter().filter_map(|t| {
        if let Some(n) = t.name() {
            let name = n.as_str();
            if [TAG_H1, TAG_H2, TAG_H3, TAG_H4, TAG_H5, TAG_H6, TAG_QUOTE, TAG_LIST, TAG_CODE_BLOCK, TAG_RAW_BLOCK].contains(&name) {
                return Some(name.to_string());
            }
        }
        None
    }).collect()
}

/// Insert a table with proper tagged structure.
pub fn insert_table(buffer: &gtk::TextBuffer) {
    let mut iter = buffer.iter_at_offset(buffer.cursor_position());
    if iter.line_offset() != 0 {
        buffer.insert(&mut iter, "\n");
    }
    buffer.insert_with_tags_by_name(&mut iter, "| Column 1 | Column 2 |", &[TAG_TABLE_CELL]);
    buffer.insert(&mut iter, "\n");
    buffer.insert_with_tags_by_name(&mut iter, "| --- | --- |", &[TAG_TABLE_SEP]);
    buffer.insert(&mut iter, "\n");
    let cursor_offset = iter.offset();
    buffer.insert_with_tags_by_name(&mut iter, "| Value | Value |", &[TAG_TABLE_CELL]);
    buffer.insert(&mut iter, "\n");
    let cursor_iter = buffer.iter_at_offset(cursor_offset);
    buffer.place_cursor(&cursor_iter);
}

/// Add a row to the table at the cursor position.
pub fn table_add_row(buffer: &gtk::TextBuffer) {
    let cursor = buffer.iter_at_offset(buffer.cursor_position());
    // Find the end of the current line
    let mut line_end = cursor;
    if !line_end.ends_line() {
        line_end.forward_to_line_end();
    }
    let text = {
        let mut ls = cursor;
        ls.set_line_offset(0);
        ls.text(&line_end)
    };
    // Count columns from pipe characters
    let col_count = text.matches('|').count().saturating_sub(1).max(1);
    let new_row = format!("\n|{}|", " Value |".repeat(col_count));
    let mut insert_at = line_end;
    buffer.insert_with_tags_by_name(&mut insert_at, &new_row, &[TAG_TABLE_CELL]);
}

/// Add a column to every row in the table block at cursor.
pub fn table_add_column(buffer: &gtk::TextBuffer) {
    let cursor = buffer.iter_at_offset(buffer.cursor_position());
    let table = buffer.tag_table();
    let cell_tag = table.lookup(TAG_TABLE_CELL);
    let sep_tag = table.lookup(TAG_TABLE_SEP);

    // Find all lines in the table block
    let mut line = cursor.line();
    // Scan up to find start of table
    while line > 0 {
        let ls = buffer.iter_at_line(line - 1).unwrap_or(buffer.start_iter());
        if has_resolved_tag(&ls, cell_tag.as_ref()) || has_resolved_tag(&ls, sep_tag.as_ref()) {
            line -= 1;
        } else {
            break;
        }
    }
    let start_line = line;
    // Scan down to find end
    let total_lines = buffer.line_count();
    line = cursor.line();
    while line + 1 < total_lines {
        let ls = buffer.iter_at_line(line + 1).unwrap_or(buffer.end_iter());
        if has_resolved_tag(&ls, cell_tag.as_ref()) || has_resolved_tag(&ls, sep_tag.as_ref()) {
            line += 1;
        } else {
            break;
        }
    }
    let end_line = line;

    // Process lines in reverse to preserve offsets
    for l in (start_line..=end_line).rev() {
        let mut le = buffer.iter_at_line(l).unwrap_or(buffer.end_iter());
        if !le.ends_line() {
            le.forward_to_line_end();
        }
        let ls = buffer.iter_at_line(l).unwrap_or(buffer.start_iter());
        let text = buffer.text(&ls, &le, true).to_string();
        let trimmed = text.trim_end();
        if trimmed.ends_with('|') {
            let is_sep = has_resolved_tag(&ls, sep_tag.as_ref());
            let append = if is_sep { " --- |" } else { " New |" };
            let insert_off = ls.offset() + trimmed.len() as i32;
            let mut insert_iter = buffer.iter_at_offset(insert_off);
            let tag_name = if is_sep { TAG_TABLE_SEP } else { TAG_TABLE_CELL };
            buffer.insert_with_tags_by_name(&mut insert_iter, append, &[tag_name]);
        }
    }
}



fn insert_chunk(
    buffer: &gtk::TextBuffer,
    iter: &mut gtk::TextIter,
    text: &str,
    block_tags: &[&'static str],
    state: InlineState,
) {
    let mut tags: Vec<&str> = block_tags.to_vec();

    if state.bold {
        tags.push(TAG_BOLD);
    }
    if state.italic {
        tags.push(TAG_ITALIC);
    }
    if state.underline {
        tags.push(TAG_UNDERLINE);
    }
    if state.strike {
        tags.push(TAG_STRIKE);
    }
    if state.code {
        tags.push(TAG_CODE);
    }

    if tags.is_empty() {
        buffer.insert(iter, text);
    } else {
        buffer.insert_with_tags_by_name(iter, text, &tags);
    }
}

fn serialize_line(
    buffer: &gtk::TextBuffer,
    line_start: &gtk::TextIter,
    line_end: &gtk::TextIter,
    ctx: &SerializeCtx,
) -> String {
    let mut content_start = *line_start;
    let line_text = buffer.text(line_start, line_end, true).to_string();

    if line_has_resolved_tag(line_start, line_end, ctx.rule.as_ref())
        || is_rule_display_line(&line_text)
    {
        return "---".to_string();
    }

    let mut prefix = String::new();

    if line_has_resolved_tag(line_start, line_end, ctx.list.as_ref())
        || detect_list_prefix_for_markdown(&line_text).is_some()
    {
        if let Some((list_prefix, consumed_chars)) = detect_list_prefix_for_markdown(&line_text) {
            prefix.push_str(&list_prefix);
            if consumed_chars > 0 {
                content_start.forward_chars(consumed_chars as i32);
            }
        } else {
            prefix.push_str("- ");
        }
    } else if line_has_resolved_tag(line_start, line_end, ctx.h1.as_ref()) {
        prefix.push_str("# ");
    } else if line_has_resolved_tag(line_start, line_end, ctx.h2.as_ref()) {
        prefix.push_str("## ");
    } else if line_has_resolved_tag(line_start, line_end, ctx.h3.as_ref()) {
        prefix.push_str("### ");
    } else if line_has_resolved_tag(line_start, line_end, ctx.h4.as_ref()) {
        prefix.push_str("#### ");
    } else if line_has_resolved_tag(line_start, line_end, ctx.h5.as_ref()) {
        prefix.push_str("##### ");
    } else if line_has_resolved_tag(line_start, line_end, ctx.h6.as_ref()) {
        prefix.push_str("###### ");
    } else if line_has_resolved_tag(line_start, line_end, ctx.quote.as_ref()) {
        prefix.push_str("> ");
    }

    let mut output = String::new();
    let mut iter = content_start;
    let mut state = InlineState::default();

    while iter.offset() < line_end.offset() {
        // Check for link/image tag start
        if let Some((tag_name, is_image)) = find_link_or_image_tag_start(&iter) {
            if let Some(tag) = buffer.tag_table().lookup(&tag_name) {
                let mut link_start = iter;
                if is_image && !link_start.starts_tag(Some(&tag)) {
                    let mut candidate = iter;
                    if candidate.backward_to_tag_toggle(Some(&tag)) && candidate.has_tag(&tag) {
                        link_start = candidate;
                    }
                }

                let mut link_end = link_start;
                if !link_end.forward_to_tag_toggle(Some(&tag)) {
                    link_end = *line_end;
                }
                if link_end.offset() > line_end.offset() {
                    link_end = *line_end;
                }

                let url = tag_name.split("-||-").nth(1).unwrap_or("").to_string();

                if is_image {
                    let display = buffer.text(&link_start, &link_end, true).to_string();
                    let alt = image_alt_from_iter(&link_start).unwrap_or_else(|| {
                        display
                            .replace(['\u{fffc}', '\u{1f5bc}'], "")
                            .trim()
                            .to_string()
                    });
                    output.push_str(&format!("![{alt}]({url})"));
                } else {
                    // Serialize link display text with its own inline markers,
                    // relative to the surrounding state. This preserves styles
                    // that wrap around the link and styles inside the link.
                    let display_text = serialize_inline_range(
                        buffer, &link_start, &link_end, ctx, state,
                    );
                    if url.starts_with("wiki:") {
                        output.push_str(&format!("[[{display_text}]]"));
                    } else {
                        output.push_str(&format!("[{display_text}]({url})"));
                    }
                }

                // State is unchanged — surrounding inline styles continue
                iter = link_end;
                continue;
            }
        }

        let mut next = iter;
        if !next.forward_char() {
            break;
        }

        let next_state = InlineState {
            bold: has_resolved_tag(&iter, ctx.bold.as_ref()),
            italic: has_resolved_tag(&iter, ctx.italic.as_ref()),
            underline: has_resolved_tag(&iter, ctx.underline.as_ref()),
            strike: has_resolved_tag(&iter, ctx.strike.as_ref()),
            code: has_resolved_tag(&iter, ctx.code.as_ref()),
        };

        transition_inline_state(&mut output, state, next_state);

        let chunk = buffer.text(&iter, &next, true).to_string();
        if chunk.chars().all(|ch| ch == BLOCK_PLACEHOLDER || ch == '\u{fffc}') {
            iter = next;
            state = next_state;
            continue;
        }
        output.push_str(&chunk);

        iter = next;
        state = next_state;
    }

    transition_inline_state(&mut output, state, InlineState::default());

    format!("{prefix}{output}")
}

/// Serialize a sub-range of the buffer (e.g. link display text) with inline markers,
/// relative to the surrounding inline state. Styles already open in `surrounding` are
/// not re-emitted; styles active inside but not outside get their own markers.
fn serialize_inline_range(
    buffer: &gtk::TextBuffer,
    start: &gtk::TextIter,
    end: &gtk::TextIter,
    ctx: &SerializeCtx,
    surrounding: InlineState,
) -> String {
    let mut out = String::new();
    let mut state = surrounding;
    let mut iter = *start;

    while iter.offset() < end.offset() {
        let next_offset = (iter.offset() + 1).min(end.offset());
        let next = buffer.iter_at_offset(next_offset);

        let next_state = InlineState {
            bold: has_resolved_tag(&iter, ctx.bold.as_ref()),
            italic: has_resolved_tag(&iter, ctx.italic.as_ref()),
            underline: has_resolved_tag(&iter, ctx.underline.as_ref()),
            strike: has_resolved_tag(&iter, ctx.strike.as_ref()),
            code: has_resolved_tag(&iter, ctx.code.as_ref()),
        };

        transition_inline_state(&mut out, state, next_state);

        let chunk = buffer.text(&iter, &next, true).to_string();
        if !chunk.chars().all(|ch| ch == BLOCK_PLACEHOLDER || ch == '\u{fffc}') {
            out.push_str(&chunk);
        }

        iter = next;
        state = next_state;
    }

    // Close back to surrounding state so the caller's context is preserved
    transition_inline_state(&mut out, state, surrounding);
    out
}

fn transition_inline_state(out: &mut String, from: InlineState, to: InlineState) {
    if from.code && !to.code {
        out.push('`');
    }
    if from.bold && !to.bold {
        out.push_str("**");
    }
    if from.strike && !to.strike {
        out.push_str("~~");
    }
    if from.italic && !to.italic {
        out.push('*');
    }
    if from.underline && !to.underline {
        out.push_str("</u>");
    }

    if !from.underline && to.underline {
        out.push_str("<u>");
    }
    if !from.italic && to.italic {
        out.push('*');
    }
    if !from.strike && to.strike {
        out.push_str("~~");
    }
    if !from.bold && to.bold {
        out.push_str("**");
    }
    if !from.code && to.code {
        out.push('`');
    }
}

fn has_resolved_tag(iter: &gtk::TextIter, tag: Option<&gtk::TextTag>) -> bool {
    if let Some(tag) = tag {
        iter.has_tag(tag)
    } else {
        false
    }
}

fn line_has_resolved_tag(
    line_start: &gtk::TextIter,
    line_end: &gtk::TextIter,
    tag: Option<&gtk::TextTag>,
) -> bool {
    let Some(tag) = tag else {
        return false;
    };

    if line_start.offset() == line_end.offset() {
        return false;
    }

    let mut iter = *line_start;
    while iter.offset() < line_end.offset() {
        if iter.has_tag(tag) {
            return true;
        }

        if !iter.forward_char() {
            break;
        }
    }

    false
}

fn toggle_inline_range(
    buffer: &gtk::TextBuffer,
    tag_name: &str,
    start: &gtk::TextIter,
    end: &gtk::TextIter,
) {
    if range_fully_tagged(buffer, start, end, tag_name) {
        buffer.remove_tag_by_name(tag_name, start, end);
    } else {
        buffer.apply_tag_by_name(tag_name, start, end);
    }
}

fn current_word_range(buffer: &gtk::TextBuffer) -> Option<(gtk::TextIter, gtk::TextIter)> {
    let cursor_offset = buffer.cursor_position();
    let cursor = buffer.iter_at_offset(cursor_offset);

    if !(cursor.inside_word() || cursor.starts_word() || cursor.ends_word()) {
        return None;
    }

    let mut start = cursor;
    if !start.starts_word() && !start.backward_word_start() {
        return None;
    }

    let mut end = cursor;
    if !end.ends_word() && !end.forward_word_end() {
        return None;
    }

    if start.offset() == end.offset() {
        None
    } else {
        Some((start, end))
    }
}

fn range_fully_tagged(
    buffer: &gtk::TextBuffer,
    start: &gtk::TextIter,
    end: &gtk::TextIter,
    tag_name: &str,
) -> bool {
    let table = buffer.tag_table();
    let Some(tag) = table.lookup(tag_name) else {
        return false;
    };

    if start.offset() == end.offset() {
        return false;
    }

    let mut iter = *start;
    while iter.offset() < end.offset() {
        if !iter.has_tag(&tag) {
            return false;
        }

        if !iter.forward_char() {
            break;
        }
    }

    true
}

fn selected_line_range(buffer: &gtk::TextBuffer) -> (gtk::TextIter, gtk::TextIter) {
    if let Some((mut start, mut end)) = buffer.selection_bounds() {
        if end.starts_line() && end.offset() > start.offset() {
            end.backward_char();
        }
        start.set_line_offset(0);
        end.forward_to_line_end();
        return (start, end);
    }

    let mut start = buffer.iter_at_offset(buffer.cursor_position());
    start.set_line_offset(0);
    let mut end = start;
    end.forward_to_line_end();
    (start, end)
}

fn collect_line_ranges(
    start: &gtk::TextIter,
    end: &gtk::TextIter,
) -> Vec<(gtk::TextIter, gtk::TextIter)> {
    let mut ranges: Vec<(gtk::TextIter, gtk::TextIter)> = Vec::new();
    let mut line_start = *start;

    loop {
        let mut line_end = line_start;
        line_end.forward_to_line_end();
        ranges.push((line_start, line_end));

        if line_end.offset() >= end.offset() {
            break;
        }

        if !line_end.forward_line() {
            break;
        }

        line_start = line_end;
    }

    ranges
}

fn strip_block_placeholders(text: &str) -> String {
    text.chars().filter(|ch| *ch != BLOCK_PLACEHOLDER).collect()
}

fn line_is_empty(
    buffer: &gtk::TextBuffer,
    line_start: &gtk::TextIter,
    line_end: &gtk::TextIter,
) -> bool {
    let raw = buffer.text(line_start, line_end, true).to_string();
    strip_block_placeholders(&raw).trim().is_empty()
}

fn line_has_tag(
    buffer: &gtk::TextBuffer,
    line_start: &gtk::TextIter,
    line_end: &gtk::TextIter,
    tag_name: &str,
) -> bool {
    let table = buffer.tag_table();
    let Some(tag) = table.lookup(tag_name) else {
        return false;
    };

    if line_start.offset() == line_end.offset() {
        return false;
    }

    let mut iter = *line_start;
    while iter.offset() < line_end.offset() {
        if iter.has_tag(&tag) {
            return true;
        }

        if !iter.forward_char() {
            break;
        }
    }

    false
}

fn toggle_list_kind(buffer: &gtk::TextBuffer, kind: ListKind) {
    let (start, end) = selected_line_range(buffer);
    let line_ranges = collect_line_ranges(&start, &end);

    let non_empty: Vec<(i32, String)> = line_ranges
        .iter()
        .filter_map(|(line_start, line_end)| {
            let raw = buffer.text(line_start, line_end, true).to_string();
            let text = strip_block_placeholders(&raw);
            if text.trim().is_empty() {
                None
            } else {
                Some((line_start.offset(), text))
            }
        })
        .collect();

    let should_remove = !non_empty.is_empty()
        && non_empty
            .iter()
            .all(|(_, text)| line_matches_list_kind(text, kind));

    let mut operations: Vec<(i32, Option<String>)> = Vec::new();
    let mut ordered_index = 1;

    for (offset, _) in non_empty {
        if should_remove {
            operations.push((offset, None));
        } else {
            let prefix = match kind {
                ListKind::Bullet => "\u{2022} ".to_string(),
                ListKind::Ordered => {
                    let out = format!("{ordered_index}. ");
                    ordered_index += 1;
                    out
                }
                ListKind::Task => "\u{2610} ".to_string(),
            };
            operations.push((offset, Some(prefix)));
        }
    }

    for (offset, prefix) in operations.into_iter().rev() {
        remove_any_list_prefix(buffer, offset);

        if let Some(prefix) = prefix {
            let mut line_start = buffer.iter_at_offset(offset);
            if kind == ListKind::Task {
                buffer.insert_with_tags_by_name(
                    &mut line_start,
                    &prefix,
                    &[TAG_LIST, TAG_TASK_MARKER],
                );
            } else {
                buffer.insert(&mut line_start, &prefix);
            }
        }

        let line_start = buffer.iter_at_offset(offset);
        let mut line_end = line_start;
        line_end.forward_to_line_end();

        if line_is_empty(buffer, &line_start, &line_end) {
            continue;
        }

        if should_remove {
            buffer.remove_tag_by_name(TAG_LIST, &line_start, &line_end);
        } else {
            buffer.apply_tag_by_name(TAG_LIST, &line_start, &line_end);
        }
    }
}

fn line_matches_list_kind(text: &str, kind: ListKind) -> bool {
    match kind {
        ListKind::Bullet => {
            text.starts_with("\u{2022} ") || text.starts_with("- ") || text.starts_with("* ")
        }
        ListKind::Ordered => parse_ordered_prefix(text).is_some(),
        ListKind::Task => {
            text.starts_with("\u{2610} ")
                || text.starts_with("\u{2611} ")
                || text.starts_with("- [ ] ")
                || text.starts_with("* [ ] ")
                || text.starts_with("- [x] ")
                || text.starts_with("* [x] ")
                || text.starts_with("- [X] ")
                || text.starts_with("* [X] ")
        }
    }
}

fn remove_any_list_prefix(buffer: &gtk::TextBuffer, line_start_offset: i32) {
    let line_start = buffer.iter_at_offset(line_start_offset);
    let mut line_end = line_start;
    line_end.forward_to_line_end();
    let text = buffer.text(&line_start, &line_end, true).to_string();

    let stripped = text.trim_start_matches(' ');
    let indent = text.len() - stripped.len();

    let inner_prefix = if stripped.starts_with("- [ ] ")
        || stripped.starts_with("* [ ] ")
        || stripped.starts_with("- [x] ")
        || stripped.starts_with("* [x] ")
        || stripped.starts_with("- [X] ")
        || stripped.starts_with("* [X] ")
    {
        6
    } else if stripped.starts_with("\u{2610} ")
        || stripped.starts_with("\u{2611} ")
        || stripped.starts_with("\u{2022} ")
        || stripped.starts_with("- ")
        || stripped.starts_with("* ")
    {
        2
    } else if let Some((_, _, chars)) = parse_ordered_prefix(stripped) {
        chars
    } else {
        0
    };

    let prefix_chars = indent + inner_prefix;

    if prefix_chars == 0 {
        return;
    }

    let mut delete_start = buffer.iter_at_offset(line_start_offset);
    let mut delete_end = buffer.iter_at_offset(line_start_offset);
    delete_end.forward_chars(prefix_chars as i32);
    buffer.delete(&mut delete_start, &mut delete_end);
}

fn parse_markdown_shortcut(prefix: &str) -> Option<MarkdownShortcut> {
    match prefix {
        "#" => return Some(MarkdownShortcut::Heading1),
        "##" => return Some(MarkdownShortcut::Heading2),
        "###" => return Some(MarkdownShortcut::Heading3),
        "####" => return Some(MarkdownShortcut::Heading4),
        "#####" => return Some(MarkdownShortcut::Heading5),
        "######" => return Some(MarkdownShortcut::Heading6),
        ">" => return Some(MarkdownShortcut::Quote),
        "-" | "*" => return Some(MarkdownShortcut::Bullet),
        "- [ ]" | "* [ ]" => return Some(MarkdownShortcut::TaskUnchecked),
        "- [x]" | "* [x]" | "- [X]" | "* [X]" => return Some(MarkdownShortcut::TaskChecked),
        _ => {}
    }

    parse_ordered_shortcut(prefix).map(MarkdownShortcut::Ordered)
}

fn is_task_prefix(text: &str) -> bool {
    let stripped = text.trim_start();
    stripped.starts_with('\u{2610}')
        || stripped.starts_with('\u{2611}')
        || stripped.starts_with("- [ ] ")
        || stripped.starts_with("* [ ] ")
        || stripped.starts_with("- [x] ")
        || stripped.starts_with("* [x] ")
        || stripped.starts_with("- [X] ")
        || stripped.starts_with("* [X] ")
}

fn parse_ordered_shortcut(text: &str) -> Option<String> {
    let bytes = text.as_bytes();
    let mut index = 0;

    while index < bytes.len() && bytes[index].is_ascii_digit() {
        index += 1;
    }

    if index == 0 || index + 1 != bytes.len() || (bytes[index] != b'.' && bytes[index] != b')') {
        return None;
    }

    Some(text[..index].to_string())
}

fn clear_block_tags(
    buffer: &gtk::TextBuffer,
    line_start: &gtk::TextIter,
    line_end: &gtk::TextIter,
) {
    for tag_name in [TAG_H1, TAG_H2, TAG_H3, TAG_H4, TAG_H5, TAG_H6, TAG_QUOTE, TAG_LIST] {
        buffer.remove_tag_by_name(tag_name, line_start, line_end);
    }
}

fn apply_block_tag_with_placeholder(
    buffer: &gtk::TextBuffer,
    line_start_offset: i32,
    tag_name: &str,
) {
    let line_start = buffer.iter_at_offset(line_start_offset);
    let mut line_end = line_start;
    line_end.forward_to_line_end();

    if line_start.offset() == line_end.offset() {
        let mut insert_at = line_start;
        let marker = BLOCK_PLACEHOLDER.to_string();
        buffer.insert_with_tags_by_name(&mut insert_at, &marker, &[tag_name]);
        buffer.place_cursor(&insert_at);
        return;
    }

    buffer.apply_tag_by_name(tag_name, &line_start, &line_end);
}

fn detect_list_prefix_for_markdown(text: &str) -> Option<(String, usize)> {
    let stripped = text.trim_start_matches(' ');
    let indent = text.len() - stripped.len();
    let indent_str = &text[..indent];

    if stripped.starts_with("\u{2610} ") {
        return Some((format!("{indent_str}- [ ] "), indent + 2));
    }
    if stripped.starts_with("\u{2611} ") {
        return Some((format!("{indent_str}- [x] "), indent + 2));
    }
    if stripped.starts_with("- [ ] ") {
        return Some((format!("{indent_str}- [ ] "), indent + 6));
    }
    if stripped.starts_with("* [ ] ") {
        return Some((format!("{indent_str}* [ ] "), indent + 6));
    }
    if stripped.starts_with("- [x] ") || stripped.starts_with("- [X] ") {
        return Some((format!("{indent_str}- [x] "), indent + 6));
    }
    if stripped.starts_with("* [x] ") || stripped.starts_with("* [X] ") {
        return Some((format!("{indent_str}* [x] "), indent + 6));
    }

    if let Some((number, _, chars)) = parse_ordered_prefix(stripped) {
        let delim = if stripped.as_bytes().get(number.len()) == Some(&b')') {
            ')'
        } else {
            '.'
        };
        return Some((format!("{indent_str}{number}{delim} "), indent + chars));
    }

    if stripped.starts_with("\u{2022} ") || stripped.starts_with("- ") {
        return Some((format!("{indent_str}- "), indent + 2));
    }
    if stripped.starts_with("* ") {
        return Some((format!("{indent_str}* "), indent + 2));
    }

    None
}

fn list_continuation_prefix(text: &str) -> Option<(usize, String)> {
    let stripped = text.trim_start_matches(' ');
    let indent = text.len() - stripped.len();
    let indent_str = &text[..indent];

    if stripped.starts_with("\u{2610} ") || stripped.starts_with("\u{2611} ") {
        return Some((indent + 2, format!("{indent_str}\u{2610} ")));
    }

    if stripped.starts_with("- [ ] ")
        || stripped.starts_with("- [x] ")
        || stripped.starts_with("- [X] ")
    {
        return Some((indent + 6, format!("{indent_str}- [ ] ")));
    }

    if stripped.starts_with("* [ ] ")
        || stripped.starts_with("* [x] ")
        || stripped.starts_with("* [X] ")
    {
        return Some((indent + 6, format!("{indent_str}* [ ] ")));
    }

    if let Some((number, _, chars)) = parse_ordered_prefix(stripped) {
        let next = number.parse::<u64>().unwrap_or(0).saturating_add(1);
        let delim = if stripped.as_bytes().get(number.len()) == Some(&b')') {
            ')'
        } else {
            '.'
        };
        return Some((indent + chars, format!("{indent_str}{next}{delim} ")));
    }

    if stripped.starts_with("\u{2022} ") {
        return Some((indent + 2, format!("{indent_str}\u{2022} ")));
    }
    if stripped.starts_with("- ") {
        return Some((indent + 2, format!("{indent_str}- ")));
    }
    if stripped.starts_with("* ") {
        return Some((indent + 2, format!("{indent_str}* ")));
    }

    None
}

fn is_rule_display_line(line: &str) -> bool {
    let trimmed = line.trim();
    !trimmed.is_empty() && trimmed.chars().all(|c| c == '\u{2500}')
}

fn parse_ordered_prefix(text: &str) -> Option<(&str, &str, usize)> {
    let bytes = text.as_bytes();
    let mut index = 0;

    while index < bytes.len() && bytes[index].is_ascii_digit() {
        index += 1;
    }

    if index == 0 || index + 1 >= bytes.len() {
        return None;
    }

    if (bytes[index] != b'.' && bytes[index] != b')') || bytes[index + 1] != b' ' {
        return None;
    }

    let number = &text[..index];
    let rest = &text[(index + 2)..];
    Some((number, rest, index + 2))
}



fn find_link_or_image_tag_start(iter: &gtk::TextIter) -> Option<(String, bool)> {
    for tag in iter.tags() {
        if let Some(name) = tag.name() {
            if name.starts_with("link-||-") && iter.starts_tag(Some(&tag)) {
                return Some((name.to_string(), false));
            }
            if name.starts_with("image-||-")
                && (iter.starts_tag(Some(&tag)) || iter.has_tag(&tag))
            {
                return Some((name.to_string(), true));
            }
        }
    }
    None
}

fn image_alt_from_iter(iter: &gtk::TextIter) -> Option<String> {
    for tag in iter.tags() {
        let Some(name) = tag.name() else {
            continue;
        };
        if let Some(encoded) = name.strip_prefix(IMAGE_ALT_TAG_PREFIX) {
            let bytes = base64::engine::general_purpose::URL_SAFE_NO_PAD
                .decode(encoded.as_bytes())
                .ok()?;
            let alt = String::from_utf8(bytes).ok()?;
            return Some(alt);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup() -> bool {
        gtk::init().is_ok()
    }

    #[test]
    fn test_markdown_idempotency() {
        if !setup() {
            eprintln!("Skipping GTK-dependent test_markdown_idempotency (no display/GTK init)");
            return;
        }
        let buffer = gtk::TextBuffer::new(None::<&gtk::TextTagTable>);
        install_tags(&buffer);

        let input = "\
# Header

Some text with **bold** and *italic* and `code`.

> A quote
> with multiple lines

- List item 1
  - Nested list item
    - Deeply nested
- List item 2

1. Ordered 1
   1. Nested ordered
2. Ordered 2

```rust
fn main() {
    println!(\"hello\");
}
```

Link test: [My link](https://example.com)";

        load_markdown(&buffer, input);
        
        let output = to_markdown(&buffer);

        assert_eq!(input.trim(), output.trim());
    }
}
