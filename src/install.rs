use std::path::Path;
use zip;

use ashpd::Result;
use ashpd::desktop::file_chooser::SelectedFiles;

use homedir::my_home;

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


pub async fn install_for_steam() -> std::result::Result<(), &'static str> {
    let home_directory_result = my_home();

    if home_directory_result.is_err() {
        return Err("Error getting home directory");
    }

    let home_directory = home_directory_result.unwrap().unwrap();
    let gd_pfx_directory = home_directory.join(".steam/steam/steamapps/compatdata/322170/pfx");
    let gd_directory = home_directory.join(".steam/steam/steamapps/common/Geometry Dash");

    println!("Found Geometry Dash PFX directory: {}", gd_pfx_directory.display());
    println!("Found Geometry Dash directory: {}", gd_directory.display());

    let _result = extract_geode(&gd_directory);
    // TODO: handle it somehow

    let user_reg_file = gd_pfx_directory.join("user.reg");

    let user_reg_file_content = std::fs::read_to_string(&user_reg_file).unwrap();
    let new_content = user_reg_file_content.replace(
        "\"vcruntime140_1\"=\"native,builtin\"", 
        "\"vcruntime140_1\"=\"native,builtin\"\n\"xinput1_4\"=\"native,builtin\"",
    );
    std::fs::write(&user_reg_file, new_content).unwrap();

    return Ok(());
}