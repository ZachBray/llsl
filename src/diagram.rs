use std::cmp::min;
use bit_set::BitSet;

static HEADER: &'static str = "
 0                   1                   2                   3
 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
";

static WORD_CHAR_COUNT: usize = 65;

struct Word {
    content: String,
    splits: BitSet,
}

impl Word {
    fn new() -> Self {
        Word {
            content: "|".to_owned(),
            splits: BitSet::new(),
        }
    }

    fn append<'a>(&mut self, segment: &'a str) -> Option<&'a str> {
        let available_size = WORD_CHAR_COUNT - min(self.content.len(), WORD_CHAR_COUNT);
        if available_size == 0 {
            Some(segment)
        } else {
            let consumed_size = min(available_size, segment.len());
            let (consumed, remaining) = segment.split_at(consumed_size);
            self.content += consumed;
            let is_eow = self.content.len() == WORD_CHAR_COUNT;
            let has_remaining = remaining.len() != 0;
            let requires_separator = is_eow || !has_remaining;
            if requires_separator {
                self.content.pop(); // Padding character is ignored and replaced with a pipe
                if !has_remaining {
                    self.splits.insert(self.content.len());
                }
                self.content += "|";
            }
            if has_remaining { Some(remaining) } else { None }
        }
    }

    fn is_full(&self) -> bool {
        self.content.len() >= WORD_CHAR_COUNT
    }

    fn suggest_max_title_size(&self) -> usize {
        if self.is_full() {
            WORD_CHAR_COUNT - 2 // Will go into new word
        } else {
            WORD_CHAR_COUNT - self.content.len()
        }
    }
}

pub struct Diagram {
    words: Vec<Word>,
}

impl Diagram {
    pub fn new() -> Self {
        Diagram { words: vec![Word::new()] }
    }

    fn truncate_section(section: &mut String, size: usize) {
        if section.len() > size {
            if size > 2 {
                section.truncate(size - 1);
                *section += "…";
            } else {
                section.truncate(size)
            }
        }
    }

    fn pad_section(section: &mut String, desired_title_size: usize, desired_size: usize) {
        if section.len() < desired_size {
            let padding = desired_title_size - section.len();
            let padding_before = padding / 2;
            let non_title_padding = desired_size - desired_title_size;
            let padding_after = non_title_padding + padding - padding_before;
            for _ in 0..padding_before {
                section.insert(0, ' ');
            }
            for _ in 0..padding_after {
                section.push(' ');
            }
        }
    }

    fn current_word(&mut self) -> &mut Word {
        let index = self.words.len() - 1;
        &mut self.words[index]
    }

    pub fn align_word(&mut self) {
        self.words.push(Word::new());
    }

    pub fn append(&mut self, mut section: String, bits: u32) {
        let section_size = bits as usize * 2;
        let title_size = min(
            section_size - 1,
            self.current_word().suggest_max_title_size(),
        );
        Diagram::truncate_section(&mut section, title_size);
        Diagram::pad_section(&mut section, title_size, section_size);
        let mut remaining: &str = &section;
        while let Some(r) = self.current_word().append(remaining) {
            remaining = r;
            self.words.push(Word::new());
        }
    }

    pub fn draw(&self) -> String {
        let mut diagram = HEADER.to_owned();
        let mut last_word: Option<&Word> = None;
        for current_word in &self.words {
            if let Some(last_w) = last_word {
                diagram += "+";
                let last_split = last_w.splits.iter().max().unwrap_or(0);
                let first_split = current_word.splits.iter().min().unwrap_or(65);
                for i in 1..64 {
                    let is_joined_above = i > last_split && i < first_split &&
                        i < last_w.content.len();
                    if is_joined_above {
                        diagram += " ";
                    } else {
                        let has_split = last_w.splits.contains(i) ||
                            current_word.splits.contains(i);
                        diagram += if has_split { "+" } else { "-" };
                    }
                }
                diagram += "+\n";
            }
            last_word = Some(current_word);
            diagram += &current_word.content;
            diagram += "\n";
        }
        if let Some(last_w) = last_word {
            diagram += "+";
            for i in 1..last_w.content.len() - 1 {
                let has_split = last_w.splits.contains(i);
                diagram += if has_split { "+" } else { "-" };
            }
            diagram += "+\n";
        }
        diagram
    }
}
