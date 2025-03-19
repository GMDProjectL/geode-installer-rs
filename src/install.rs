use std::path::Path;
use zip;

use ashpd::Result;
use ashpd::desktop::file_chooser::SelectedFiles;

pub fn extract_geode(target_directory: &Path) -> std::result::Result<bool, &'static str> {
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

pub async fn install_using_wine() -> Result<()> {
    let files = SelectedFiles::open_file()
        .title("Select Geometry Dash Folder")
        .directory(true)
        .send()
        .await?
        .response()?;

    let path_str = files.uris()[0].path();

    let _result = extract_geode(Path::new(path_str));
    // TODO: handle it somehow

    let registry_bytes = include_bytes!("../resources/xinput_wine_override.reg");

    let registry_path = Path::new(path_str).join("xinput_wine_override.reg");
    let registry_path_string = registry_path.to_str().unwrap();

    std::fs::write(registry_path_string, registry_bytes).unwrap();
    std::process::Command::new("wine")
        .arg("regedit")
        //.arg("/s")
        .arg(registry_path_string)
        .spawn()
        .unwrap();

    return Ok(());
}