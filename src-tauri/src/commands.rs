use crate::{
    categorizer_service::{
        self,
        library::{Library, LIBRARY},
    },
    config::MANGO_CONFIG,
};

#[tauri::command]
pub fn get_library() -> Library {
    (*categorizer_service::library::LIBRARY).clone()
}

#[tauri::command]
pub fn get_resource_server_url() -> String {
    MANGO_CONFIG.resource_server_url()
}

#[tauri::command]
pub fn get_chapter_list(series: String) -> Vec<String> {
	let mut ret = vec![];

	let series = LIBRARY.series_by_name(series).unwrap();

	for chap in &series.chapters {
		ret.push(format!("Chapter {}", chap.chapter_number));
	}

	ret
}

pub fn get_chapter_images(series: String) -> Vec<Vec<u8>> {
    let mut ret: Vec<Vec<u8>> = vec![];
    let series = LIBRARY.series_by_name(series);

    match series {
        Some(series) => {
            for chap in &series.chapters {
                for path in &chap.image_paths {
                    let img = image::io::Reader::open(path);

                    match img {
                        Ok(reader) => {
							match reader.decode() {
								Ok(dyn_img) => {
									let bytes = dyn_img.as_bytes().to_vec();

								},
								Err(e) => {
									eprintln!("Error decoding image {}:\n{:?}", path.to_str().unwrap(), e);
								},
							}
						},
                        Err(e) => {
                            eprintln!("Error reading image {}:\n{:?}", path.to_str().unwrap(), e);
                        }
                    }
                }
            }
        }
        None => (),
    }

	ret
}
