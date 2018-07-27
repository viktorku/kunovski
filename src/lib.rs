#![feature(use_extern_macros)]

#[macro_use]
extern crate lazy_static;
extern crate wasm_bindgen;

use std::rc::Rc;
use std::sync::Mutex;
use wasm_bindgen::prelude::*;

const NAME: &'static str = "viktor";
const SURNAME: &'static str = "kunovski";
const EMAIL: &'static str = "viktor@kunovski.com";
const DELIMITER: &'static str = "---";

#[wasm_bindgen]
extern {
    type Performance;
    static performance: Performance;
    #[wasm_bindgen(method)]
    fn now(this: &Performance) -> f64;

    pub type HTMLDocument;
    static document: HTMLDocument;

    #[wasm_bindgen(method, js_name = getElementById)]
    fn get_element_by_id(this: &HTMLDocument, id: &str) -> Element;
    #[wasm_bindgen(method)]
    fn createElement(this: &HTMLDocument, tagName: &str) -> Element;

    pub type Element;
    #[wasm_bindgen(method, setter = innerHTML)]
    fn set_inner_html(this: &Element, html: &str);
    #[wasm_bindgen(method, js_name = appendChild)]
    fn append_child(this: &Element, other: Element);
    #[wasm_bindgen(method, js_name = appendChild)]
    fn append_child_ref(this: &Element, other: &Element);

    #[wasm_bindgen(method, getter = className)]
    fn class_name(this: &Element) -> String;
    #[wasm_bindgen(method, setter = className)]
    fn set_class_name(this: &Element, val: &str);

    #[wasm_bindgen(method, getter = id)]
    fn id(this: &Element) -> String;
    #[wasm_bindgen(method, setter = id)]
    fn set_id(this: &Element, val: &str);

    #[wasm_bindgen(method, js_name = addEventListener)]
    fn add_event_listener(this: &Element, event: &str, handler: &Closure<FnMut()>);
    #[wasm_bindgen(method, js_name = setAttribute)]
    fn set_attribute(this: &Element, name: &str, value: &str);

    #[wasm_bindgen(js_name = setInterval)]
    fn set_interval(cb: &Closure<FnMut()>, delay: u32) -> f64;
    #[wasm_bindgen(js_name = clearInterval)]
    fn clear_interval(interval: f64);

    #[wasm_bindgen(js_name = setTimeout)]
    fn set_timeout(cb: &Closure<FnMut()>, delay: u32) -> f64;

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen(module = "./index")]
extern {
    fn update(status: bool, pending: u32) -> bool;
    fn next_u32(min: u32, max: u32) -> u32;
    fn done();
}

lazy_static! {
    static ref DATA: Vec<String> = include_str!("../assets/mobydick.txt")
        .split_whitespace()
        .map(|s| s.to_lowercase())
        .collect();
    static ref DESCRIPTION: Vec<Vec<&'static str>> = include_str!("../assets/description.txt")
        .lines()
        .map(|l| l.split_whitespace().collect())
        .collect();
    static ref INTERVALS: Mutex<Vec<f64>> = Mutex::new(vec![0.; NAME.len() + SURNAME.len()]);
    static ref INTERVALS_WORD: Mutex<Vec<f64>> = Mutex::new(vec![0.; {
        DESCRIPTION.iter().fold(1, |acc, line| acc + line.len())
     }]);
    static ref INTERVALS_HOVER: Mutex<Vec<f64>> = Mutex::new(vec![0.; {
        DESCRIPTION.iter().fold(1, |acc, line| acc + line.len())
    }]);
    static ref ROLLING_SLOT: Mutex<usize> = Mutex::new(0);
    static ref PARTIAL_RUN: Mutex<bool> = Mutex::new(false);
}

#[wasm_bindgen]
pub fn init() -> bool {
    // lazy eval the large corpus; usually takes about 0.5-1sec
    DATA.len();
    DESCRIPTION.len();
    true
}

#[wasm_bindgen]
pub fn start(partial: bool, delay: u32) -> Vec<u32> {

    stop_all_pending_intervals(&INTERVALS);
    stop_all_pending_intervals(&INTERVALS_WORD);

    {
        let mut pr = PARTIAL_RUN.lock().unwrap();
        *pr = partial;

        if !partial {
            let email = document.get_element_by_id("email");
            email.set_inner_html("");
            let node = document.get_element_by_id("detail");
            node.set_inner_html("");
        }
    }

    let node = document.get_element_by_id("main");
    node.set_inner_html("");

    for (i, c) in NAME.chars().enumerate() {
        find(c, i, &node, delay);
    }

    let spacing = document.createElement("div");
    spacing.set_class_name("spacing");
    node.append_child(spacing);

    for (i, c) in SURNAME.chars().enumerate() {
        find(c, i + NAME.len(), &node, delay);
    }

    vec![delay, DATA.len() as u32]
}

struct WordResult {
    letter: char,
    word: String,
}

fn process_word(word_raw: &'static str)
    -> Result<WordResult, &'static str> {
    let mut chars = word_raw.chars();
    if let Some(letter) = chars.next() {
        let mut word: String = String::from("");
        for character in chars {
            if !character.is_ascii() {
                return Err("Unsupported character in word.");
            }
            if character.is_alphanumeric() {
                word.push(character);
            } else {
                break;
            }
        }
        Ok(WordResult {
            letter: letter,
            word: word
        })
    } else {
        Err("Word is empty.")
    }
}

pub fn find(letter: char, idx: usize, node: &Element, delay: u32) {

    let container = document.createElement("div");

    let child = document.createElement("div");
    let child_v = document.createElement("div");
    child_v.set_class_name("vertical");

    container.append_child_ref(&child);
    container.append_child_ref(&child_v);

    node.append_child_ref(&container);

    let ls = letter.to_string();
    let start = performance.now();
    let interval = Closure::new(move || {
        if let Ok(result) = process_word(get_random_word()) {
            if performance.now() - start >= 5000. {
                child.set_inner_html(&letter.to_string()[..]);
                child.set_class_name("timeout");
                stop_namechar_interval(&idx);
                return;
            }
            let letter = &result.letter.to_string()[..];
            child.set_inner_html(letter);
            if letter == ls && result.word.len() > 3 && result.word.len() <= 12 &&
               result.word != "sperm" /* it's moby dick after all :) */ {
                child_v.set_inner_html(&result.word);
                stop_namechar_interval(&idx);
            }
        }
    });

    let mut intervals = INTERVALS.lock().unwrap();
    (*intervals)[idx] = set_interval(&interval, delay);
    interval.forget();
}

fn stop_namechar_interval(idx: &usize) -> bool {
    let mut intervals = INTERVALS.lock().unwrap();
    clear_interval((*intervals)[*idx]);
    (*intervals)[*idx] = 0.;
    let pending = (*intervals).iter().filter(|&i| *i != 0.).count();
    let status = pending == 0;
    drop(intervals); // drop the mutex guard in case we transition in stop_all.

    // Let JS know the status of pending intervals
    // If we get negative feedback we stop all pending
    if !update(status, pending as u32) {
        stop_all_pending_intervals(&INTERVALS);
    }

    if status {
        if !*PARTIAL_RUN.lock().unwrap() {
            continue_description();
        }
    }

    status
}

fn continue_description() {
    describe(0, 0, true);
}

fn stop_interval(idx: &usize, intervals: &Mutex<Vec<f64>>) {
    let mut intervals_ = intervals.lock().unwrap();
    clear_interval((*intervals_)[*idx]);
    (*intervals_)[*idx] = 0.;
}

fn stop_all_pending_intervals(intervals: &Mutex<Vec<f64>>) {
    let mut intervals_ = intervals.lock().unwrap();
    for i in &mut *intervals_ {
        if *i != 0. {
            clear_interval(*i);
            *i = 0.;
        }
    }
}

fn get_random_word() -> &'static str {
    &DATA[next_u32(0, DATA.len() as u32) as usize]
}

fn describe(line_idx: usize, word_idx: usize, new_line: bool) {

    let node = document.get_element_by_id("detail");

    let slot = {
        let mut rs = ROLLING_SLOT.lock().unwrap();
        *rs += 1;
        *rs
    };

    let mut p: Element = document.createElement("p");

    let is_last_line = line_idx == DESCRIPTION.len() - 1;
    let is_last_word = word_idx == DESCRIPTION[line_idx].len() - 1;

    let closure = Closure::new(move || {
        let word = DESCRIPTION[line_idx][word_idx];

        let next = move || {
            if !is_last_line || !is_last_word {
                let new_line_idx = if is_last_word { line_idx + 1 } else { line_idx };
                let new_word_idx = if is_last_word { 0 } else { word_idx + 1 };
                describe(new_line_idx, new_word_idx, new_line_idx != line_idx);
            } else {
                set_email();

                // we're done: reset
                let mut rs = ROLLING_SLOT.lock().unwrap();
                *rs = 0;

                done();
            }
        };

        if word == DELIMITER {
            set_delimiter(&node);
            next();
        } else {
            let id = &format!("p-{}", line_idx);
            if new_line {
                p.set_id(id);
                node.append_child_ref(&p);
            } else {
                p = document.get_element_by_id(id);
            }
            roll(&p, word, is_last_word, slot, next);
        }

    });

    set_timeout(&closure, 0);
    closure.forget();
}

fn roll(
    p: &Element,
    word: &'static str,
    is_last_word: bool,
    slot: usize,
    next: impl Fn() + 'static) {

    let span = Rc::new(document.createElement("span"));
    attach_listeners(span.clone(), Rc::new(word.to_string()), slot, is_last_word);
    p.append_child_ref(&span);

    let mut i = 0;
    let mut went_next = false;
    let interval = Closure::new(move || {
        loop {
            let random_word = get_random_word();
            if random_word.len() == word.len() {
                span.set_inner_html(random_word);
                break;
            }
        }
        i += 1;
        // Allow crossfading
        if !went_next && i >= 45 {
            went_next = true;
            next();
        }
        if i >= 65 {
            if is_last_word {
                span.set_inner_html(&word);
            } else {
                span.set_inner_html(&(word.to_string() + "&nbsp;"));
            }
            stop_interval(&slot, &INTERVALS_WORD);
        }
    });

    let mut intervals = INTERVALS_WORD.lock().unwrap();
    (*intervals)[slot] = set_interval(&interval, 0);
    interval.forget();

}

fn attach_listeners(
    span: Rc<Element>,
    word: Rc<String>,
    slot: usize,
    last_in_line: bool) {

    let len = word.len();
    let span_ome = span.clone();
    let ome = Closure::new(move || {
        let span_ome_ = span_ome.clone();
        let interval = Closure::new(move || {
            if let Ok(random_word) = get_hover_word(len) {
                if last_in_line {
                    span_ome_.set_inner_html(&random_word);
                } else {
                    span_ome_.set_inner_html(&(random_word.to_string() + "&nbsp;"));
                }
            }
        });
        let mut intervals = INTERVALS_HOVER.lock().unwrap();
        (*intervals)[slot] = set_interval(&interval, 0);
        interval.forget();
    });
    span.add_event_listener("mouseenter", &ome);
    ome.forget();


    let span_oml = span.clone();
    let oml = Closure::new(move || {
        stop_interval(&slot, &INTERVALS_HOVER);
        if last_in_line {
            span_oml.set_inner_html(&word);
        } else {
            span_oml.set_inner_html(&(word.to_string() + "&nbsp;"));
        }
    });

    span.add_event_listener("mouseleave", &oml);
    oml.forget();
}

fn get_hover_word(len: usize) -> Result<String, &'static str> {
    match process_word(get_random_word()) {
        Ok(result) => {
            let mut random_word = result.letter.to_string() + &result.word;
            let l = random_word.len();
            let diff = (len as i32) - (l as i32);
            if diff > 0 {
                random_word += &get_hover_word(diff as usize)?;
            } else if diff < 0 {
                let mut boundary = diff.abs() as usize;
                let mut invalid_boundary = false;
                while !random_word.is_char_boundary(boundary) && boundary <= random_word.len() {
                    invalid_boundary = true;
                    boundary += 1;
                }
                random_word = {
                    let (lhs, rhs) = random_word.split_at(boundary);
                    (if invalid_boundary { lhs } else { rhs }).to_string()
                };
            }
            Ok(random_word)
        },
        Err(err) => Err(err)
    }
}

fn set_delimiter(node: &Element) {
    let hr = document.createElement("div");
    hr.set_id("hr");

    let borderline = document.createElement("div");
    borderline.set_id("borderline");
    hr.append_child(borderline);

    node.append_child(hr);
}

fn set_email() {
    let a = document.createElement("a");
    a.set_attribute("href", &("mailto:".to_string() + EMAIL));
    a.set_attribute("target", "_blank");
    a.set_inner_html(EMAIL);
    let email = document.get_element_by_id("email");
    email.append_child(a);
}
