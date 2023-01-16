use std::{collections::HashMap, fs::DirEntry, io::BufWriter, path::Path};

use regex::Regex;
use serde::{ser::SerializeStruct, Serialize};

pub enum Rating {
    OutOfTen(i32),
    OutOfFive(i32),
    OutOfTenDecimal(f32),
    OutOfFiveDecimal(f32),
}

impl Serialize for Rating {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Rating::OutOfTen(x) => serializer.serialize_newtype_variant("Rating", 0, "OutOfTen", x),
            Rating::OutOfFive(x) => {
                serializer.serialize_newtype_variant("Rating", 1, "OutOfFive", x)
            }
            Rating::OutOfTenDecimal(x) => {
                serializer.serialize_newtype_variant("Rating", 2, "OutOfTenDecimal", x)
            }
            Rating::OutOfFiveDecimal(x) => {
                serializer.serialize_newtype_variant("Rating", 3, "OutOfFiveDecimal", x)
            }
        }
    }
}

pub enum Status {
    IN_PROGRESS,
    HAITUS,
    COMPLETE,
}

impl Serialize for Status {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Status::IN_PROGRESS => serializer.serialize_unit_variant("Status", 0, "IN_PROGRESS"),
            Status::HAITUS => serializer.serialize_unit_variant("Status", 1, "HAITUS"),
            Status::COMPLETE => serializer.serialize_unit_variant("Status", 2, "COMPLETE"),
        }
    }
}

pub struct Tag {
    tag: String,
}

pub struct Tags {
    tags: Vec<Tag>,
}

impl Tags {
    pub fn new() -> Self {
        Self { tags: Vec::new() }
    }
}

pub struct Series {
    title: String,
    rating: Rating,
    number_of_chapters: i32,
    status: Status,
    tags: Tags,
    chapters: Vec<Chapter>,
}

pub struct Chapter {
    chapter_number: i32,
}

impl Serialize for Chapter {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("Chapter", 1)?;
        state.serialize_field("chapter_number", &self.chapter_number)?;
        state.end()
    }
}

pub struct Library {
    series: Vec<Series>,
}

impl Serialize for Library {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("Library", 1)?;
        state.serialize_field("series", &self.series)?;
        state.end()
    }
}

impl Library {
    pub fn new() -> Self {
        Self { series: Vec::new() }
    }

    pub fn add_series(&mut self, series: Series) {
        self.series.push(series);
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
    ) -> Self {
        Self {
            title,
            rating,
            number_of_chapters,
            status,
            tags,
            chapters,
        }
    }
}

impl Serialize for Series {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("Series", 6)?;
        state.serialize_field("title", &self.title)?;
        state.serialize_field("rating", &self.rating)?;
        state.serialize_field("number_of_chapters", &self.number_of_chapters)?;
        state.serialize_field("status", &self.status)?;
        //state.serialize_field("tags", self.)
        state.serialize_field("chapters", &self.chapters)?;
        state.end()
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
                    } || match dir.file_type() {
                        Ok(file_type) => (file_type.is_dir()),
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

                        match dir_entry.path().read_dir() {
                            Ok(chapter_dirs) => {
                                let chapter_dirs = chapter_dirs.filter(|dir| match dir {
                                    Ok(dir) => {
                                        return (match dir.file_name().into_string() {
                                            Ok(name) => !(name.starts_with(".")),
                                            _ => false,
                                        } || match dir.file_type() {
                                            Ok(file_type) => (file_type.is_dir()),
                                            _ => false,
                                        });
                                    }
                                    _ => false,
                                });

                                for chapter_dir in chapter_dirs {
                                    match chapter_dir {
                                        Ok(file) => match file.file_name().into_string() {
                                            Ok(chapter_folder_name) => {
                                                println!(
                                                    "chapter_folder_name: {}",
                                                    chapter_folder_name
                                                );
                                                let re =
                                                    Regex::new(r"((c|Ch(.*)( |\.))?([0-9]+))(.+)?")
                                                        .unwrap();

                                                let caps =
                                                    re.captures(&chapter_folder_name).unwrap();

                                                let chapter_number = caps
                                                    .get(5)
                                                    .unwrap()
                                                    .as_str()
                                                    .parse::<i32>()
                                                    .unwrap();

                                                println!(
                                                    "Logging chapter {} of series {}",
                                                    chapter_number, series_name
                                                );

                                                chapters.push(Chapter { chapter_number })
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
                        ));
                    }
                    Err(e) => todo!(),
                }
            }
        }
        Err(e) => println!("{:?}", e),
    }
    library
}

pub fn serialize_to_disk(library: Library) -> bincode::Result<()> {
    match std::fs::File::create("library.mlf") {
        Ok(f) => {
            let mut f = BufWriter::new(f);
            bincode::serialize_into(&mut f, &library)
        },
        Err(e) => bincode::Result::Err(Box::new(bincode::ErrorKind::Io(e))),
    }
}
