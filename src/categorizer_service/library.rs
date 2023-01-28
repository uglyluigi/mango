use std::{
    fmt::Display,
    fs::{DirEntry},
    io::{self, BufWriter, Write},
    path::{PathBuf},
};

use derive_builder::Builder;
use lazy_static::lazy_static;
use regex::{Captures, Regex};
use serde::{Deserialize, Serialize};

use crate::config;

lazy_static! {
    pub static ref LIBRARY: Library = init_library();
}

#[derive(Serialize, Deserialize, Clone)]
pub enum Rating {
    OutOfTen(i32),
    OutOfFive(i32),
    OutOfTenDecimal(f32),
    OutOfFiveDecimal(f32),
}

#[derive(Serialize, Deserialize, Clone)]
pub enum Status {
    InProgress,
    Haitus,
    Complete,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Tag {
    tag: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Tags {
    tags: Vec<Tag>,
}

impl Tags {
    pub fn new() -> Self {
        Self { tags: Vec::new() }
    }
}

#[derive(Serialize, Deserialize, Builder)]
pub struct Series {
    pub title: String,
    pub rating: Rating,
    pub number_of_chapters: i32,
    pub status: Status,
    pub tags: Tags,
    pub chapters: Vec<Chapter>,
    pub covers: Vec<Cover>,
}

#[derive(Serialize, Deserialize, Clone, Builder)]
pub struct Chapter {
    chapter_number: i32,
    image_paths: Vec<PathBuf>,
}

impl Chapter {
    pub fn new(chapter_number: i32, image_paths: Vec<PathBuf>) -> Self {
        Self {
            chapter_number,
            image_paths,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Library {
    series: Vec<Series>,
}

impl Display for Library {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // May be kind of slow. Maybe memoize somehow later
        write!(f, "{}", serde_json::to_string_pretty(&self).unwrap())
    }
}

impl Library {
    pub fn new() -> Self {
        Self { series: Vec::new() }
    }

    pub fn add_series(&mut self, series: Series) {
        self.series.push(series);
    }

    pub fn series(&self) -> &Vec<Series> {
        &self.series
    }
}

impl Series {
    pub fn new(
        title: String,
        rating: Rating,
        number_of_chapters: i32,
        status: Status,
        tags: Tags,
        chapters: Vec<Chapter>,
        covers: Vec<Cover>,
    ) -> Self {
        Self {
            title,
            rating,
            number_of_chapters,
            status,
            tags,
            chapters,
            covers,
        }
    }

    pub fn chapters(&self) -> &Vec<Chapter> {
        &self.chapters
    }

    pub fn covers(&self) -> &Vec<Cover> {
        &self.covers
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Cover {
    pub path: PathBuf,
}

impl Cover {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }
}

const CHAPTER_FOLDER_REGEX: &'static str = r"((c|Ch(.*)( |\.))?([0-9]+))(.+)?";
const COVER_IMAGE_REGEX: &'static str = r"([cC]over)(\.(jpe?g|png))?";

pub fn build_library(library_path: &PathBuf) -> Library {
    let mut library = Library::new();

    // Get pathbufs of all files inside library_path
    match get_pathbufs_in(&library_path, Some(pathbuf_filter)) {
        Some(series) => {
            // Begin building the "Series" struct for this directory
            for buf in series {
                let mut series_builder = SeriesBuilder::default();

                series_builder
                    .title(buf.file_name().unwrap().to_str().unwrap().to_string())
                    .rating(Rating::OutOfTen(0))
                    .status(Status::InProgress)
                    .tags(Tags::new())
                    .number_of_chapters(0);

                match get_pathbufs_in(&buf, Some(pathbuf_filter)) {
                    Some(chapters) => {
                        let mut chapters_but_structs = vec![];

                        for chapter in chapters {
                            if deconstruct_buf(&chapter, COVER_IMAGE_REGEX).is_some() {
                                series_builder.covers(vec![Cover::new(chapter.clone())]);
                                continue;
                            }

                            let mut chapter_builder = ChapterBuilder::default();

                            match deconstruct_buf(&chapter, CHAPTER_FOLDER_REGEX) {
                                Some(caps) => match caps.get(5) {
                                    Some(chapter_number) => {
                                        chapter_builder.chapter_number(
                                            chapter_number.as_str().parse::<i32>().unwrap(),
                                        );
                                        match get_pathbufs_in(&chapter, Some(pathbuf_filter)) {
                                            Some(chapter_images) => {
                                                chapter_builder.image_paths(chapter_images);
                                            }
                                            None => println!(
                                                "Skipping folder {:?} (does not contain a number)",
                                                chapter.as_path()
                                            ),
                                        }
                                    }
                                    None => (),
                                },
                                None => {
                                    println!(
                                        "Skipping folder {:?} (does not appear to be a chapter of a series)",
                                        chapter.as_path()
                                    );
                                }
                            };
                            chapters_but_structs.push(chapter_builder.build().unwrap());
                        }
                        series_builder.chapters(chapters_but_structs);
                    }
                    None => (),
                }
                library.add_series(series_builder.build().unwrap());
            }
        }
        None => (),
    }

    library
}

fn deconstruct_buf<'a>(path: &'a PathBuf, regex: &'static str) -> Option<Captures<'a>> {
    match path.file_name() {
        Some(name) => match name.to_str() {
            Some(name) => match Regex::new(regex) {
                Ok(matcher) => matcher.captures(name),
                Err(_) => None,
            },
            None => None,
        },
        None => None,
    }
}

// Returns a pathbuf representing each file inside of the provided path
fn get_pathbufs_in<F>(path: &PathBuf, filter: Option<F>) -> Option<Vec<PathBuf>>
where
    F: FnOnce(&Result<DirEntry, std::io::Error>) -> bool + Copy,
{
    let mut ret = Vec::new();

    match path.read_dir() {
        Ok(read) => {
            read.filter(|dir| match filter {
                Some(f) => f(dir),
                None => true,
            })
            .for_each(|x| match x {
                Ok(x) => ret.push(x.path()),
                _ => (),
            });

            Some(ret)
        }
        Err(e) => {
            eprintln!("Error reading dir {:?}", e);
            None
        }
    }
}

fn pathbuf_filter(entry: &Result<DirEntry, std::io::Error>) -> bool {
    match entry {
        Ok(dir) => {
            return match dir.file_name().into_string() {
                Ok(name) => !name.starts_with("."),
                _ => false,
            };
        }
        _ => false,
    }
}

const LIBRARY_FILE_NAME: &'static str = "library.json";

fn serialize_to_disk(library: &Library) -> io::Result<()> {
    let f = std::fs::File::create(LIBRARY_FILE_NAME)?;
    let mut f = BufWriter::new(f);

    match serde_json::to_string(&library) {
        Ok(library_json) => f.write_all(library_json.as_bytes()),
        Err(e) => Err(io::Error::from(e)),
    }
}

pub fn serialize_library(library: &Library) {
    match serialize_to_disk(&library) {
        Ok(_) => {
            println!("Successfully wrote library to disk");
        }
        Err(e) => {
            eprintln!("{:?}", e);
        }
    }
}

pub fn deserialize_from_disk() -> io::Result<Library> {
    let bytes = std::fs::read("library.json")?;
    let library: Library = serde_json::from_str(
        String::from_utf8(bytes)
            .expect("Failed to read library file into utf8 string")
            .as_str(),
    )
    .expect("Failed to deserialize library struct from library file");

    Ok(library)
}

pub fn init_library() -> Library {
    match deserialize_from_disk() {
        Ok(lib) => lib,
        Err(e) => {
            match e.kind() {
                io::ErrorKind::NotFound => {
                    println!("Initializing new library");
                    let default_library = build_library(config::MANGO_CONFIG.library_path());
                    serialize_library(&default_library);
                    default_library
                }
                _ => panic!("Failed to load library: {:?}", e),
            }
        }
    }
}
