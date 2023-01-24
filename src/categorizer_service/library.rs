use std::{
    collections::HashMap,
    fmt::Display,
    fs::{DirEntry, ReadDir},
    io::{self, BufRead, BufReader, BufWriter, Write},
    path::{Path, PathBuf},
};

use regex::{Captures, Regex};
use serde::{ser::SerializeStruct, Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum Rating {
    OutOfTen(i32),
    OutOfFive(i32),
    OutOfTenDecimal(f32),
    OutOfFiveDecimal(f32),
}

#[derive(Serialize, Deserialize)]
pub enum Status {
    IN_PROGRESS,
    HAITUS,
    COMPLETE,
}

#[derive(Serialize, Deserialize)]
pub struct Tag {
    tag: String,
}

#[derive(Serialize, Deserialize)]
pub struct Tags {
    tags: Vec<Tag>,
}

impl Tags {
    pub fn new() -> Self {
        Self { tags: Vec::new() }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Series {
    title: String,
    rating: Rating,
    number_of_chapters: i32,
    status: Status,
    tags: Tags,
    chapters: Vec<Chapter>,
    covers: Vec<Cover>,
}

#[derive(Serialize, Deserialize)]
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

#[derive(Serialize, Deserialize)]
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

pub fn build_library_2(library_path: PathBuf) -> Library {
    let mut library = Library::new();

    match get_pathbufs_in(&library_path) {
        Some(series) => {
            for buf in series {
                if deconstruct_buf(&buf, COVER_IMAGE_REGEX).is_some() {
                    //
                }

                match get_pathbufs_in(&buf) {
                    Some(chapters) => {
                        for chapter in chapters {
                            match deconstruct_buf(&chapter, CHAPTER_FOLDER_REGEX) {
                                Some(caps) => match caps.get(5) {
                                    Some(chapter_number) => match get_pathbufs_in(&chapter) {
                                        Some(chapter_images) => {
                                            
                                        }
                                        None => println!(
                                            "Skipping folder {} (does not contain a number)",
                                            chapter.file_name().unwrap().to_str().unwrap()
                                        ),
                                    },
                                    None => (),
                                },
                                None => {
                                    println!(
                                        "Skipping folder {} (does not appear to be a chapter of a series)",
                                        chapter.file_name().unwrap().to_str().unwrap()
                                    );
                                }
                            };
                        }
                    }
                    None => todo!(),
                }
            }
        }
        None => todo!(),
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

pub fn build_library() -> Library {
    let library_dir = Path::new("./MangaLibrary");
    let mut library = Library::new();

    match library_dir.read_dir() {
        Ok(read_dir) => {
            // Filter out non-directories, "dotfiles", and files with names that
            // cannot be converted from an OsString to a String
            let read_dir = read_dir.filter(|dir| match dir {
                Ok(dir) => {
                    return (match dir.file_name().into_string() {
                        Ok(name) => !(name.starts_with(".")),
                        _ => false,
                    });
                }
                _ => false,
            });

            for read_dir_result in read_dir {
                match read_dir_result {
                    Ok(dir_entry) => {
                        let series_name = dir_entry.file_name().into_string().unwrap();

                        println!("Logging series: {}", series_name);
                        let mut chapters = Vec::new();
                        let mut covers = Vec::new();

                        match dir_entry.path().read_dir() {
                            Ok(chapter_dirs) => {
                                let chapter_dirs = chapter_dirs.filter(|dir| match dir {
                                    Ok(dir) => {
                                        return (match dir.file_name().into_string() {
                                            Ok(name) => !(name.starts_with(".")),
                                            _ => false,
                                        });
                                    }
                                    _ => false,
                                });

                                for chapter_dir in chapter_dirs {
                                    match chapter_dir {
                                        Ok(file) => match file.file_name().into_string() {
                                            Ok(file_name) => {
                                                // this should probably just panic if my regexes are bad tbh
                                                let chapter_matcher =
                                                    Regex::new(CHAPTER_FOLDER_REGEX)
                                                        .expect("Failed to compile regex");

                                                let cover_matcher = Regex::new(COVER_IMAGE_REGEX)
                                                    .expect("Failed to compile regex");

                                                let chapter_caps =
                                                    chapter_matcher.captures(&file_name);

                                                if let Some(caps) = chapter_caps {
                                                    let chapter_number = caps
                                                        .get(5)
                                                        .unwrap()
                                                        .as_str()
                                                        .parse::<i32>()
                                                        .unwrap();

                                                    match file.path().read_dir() {
                                                        Ok(read) => {
                                                            for thing in read {
                                                                println!(
                                                                    "{}",
                                                                    thing
                                                                        .unwrap()
                                                                        .file_name()
                                                                        .to_str()
                                                                        .unwrap()
                                                                );
                                                            }
                                                        }
                                                        Err(err) => todo!(),
                                                    }

                                                    let chapter =
                                                        Chapter::new(chapter_number, Vec::new());

                                                    chapters.push(chapter);
                                                }

                                                let cover_caps = cover_matcher.captures(&file_name);

                                                if let Some(caps) = cover_caps {
                                                    covers.push(Cover::new(file.path()));
                                                }
                                            }
                                            Err(e) => println!("Err reading dir name: {:?}", e),
                                        },
                                        _ => (),
                                    }
                                }
                            }
                            Err(_) => todo!(),
                        }

                        library.add_series(Series::new(
                            series_name,
                            Rating::OutOfTen(0),
                            0,
                            Status::IN_PROGRESS,
                            Tags::new(),
                            chapters,
                            covers,
                        ));
                    }
                    Err(e) => println!("{:?}", e),
                }
            }
        }
        Err(e) => println!("{:?}", e),
    }
    library
}

// Returns a pathbuf representing each file inside of the provided path
fn get_pathbufs_in(path: &PathBuf) -> Option<Vec<PathBuf>> {
    let mut ret = Vec::new();

    match path.read_dir() {
        Ok(read) => {
            read.filter(|dir| match dir {
                Ok(dir) => {
                    return match dir.file_name().into_string() {
                        Ok(name) => !name.starts_with("."),
                        _ => false,
                    };
                }
                _ => false,
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

const LIBRARY_FILE_NAME: &'static str = "library.json";

pub fn serialize_to_disk(library: Library) -> io::Result<()> {
    let f = std::fs::File::create(LIBRARY_FILE_NAME)?;
    let mut f = BufWriter::new(f);

    match serde_json::to_string(&library) {
        Ok(library_json) => f.write_all(library_json.as_bytes()),
        Err(e) => Err(io::Error::from(e)),
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
