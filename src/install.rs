use std::path::Path;
use zip;
use ashpd::desktop::file_chooser::OpenFileRequest;
use ashpd::Result;

pub fn install_geode(target_directory: &Path) -> std::result::Result<bool, &'static str> {
    let modloader_zip_bytes = include_bytes!("../resources/geode/modloader.zip");

    let zip_notsure = zip::ZipArchive::new(std::io::Cursor::new(modloader_zip_bytes));

    if !zip_notsure.is_ok() {
        return Err("Error creating zip archive");
    }

    let mut zip = zip_notsure.unwrap();

    let extract_result = zip.extract(target_directory);

    if !extract_result.is_ok() {
        return Err("Error extracting the archive");
    }


    return Ok(true);
}

pub async fn install_using_wine() -> ashpd::Result<()> {
    let open_file_request = OpenFileRequest::default().directory(true).send().await?;

    if !open_file_request.response().is_ok() {
        return Err(open_file_request.response().unwrap_err());
    }

    let response = open_file_request.response();

    println!("File: {:#?}", response);

    return Ok(());
}