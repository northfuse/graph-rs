use rust_onedrive::drive::intoitem::{IntoFetch, MutateDownload};
use rust_onedrive::drive::Drive;
use rust_onedrive::drive::EP;
use rust_onedrive::from_to::*;
use rust_onedrive::oauth::OAuth;
use rust_onedrive::prelude::{Collection, DriveItem, ItemMe};
use std::convert::TryFrom;
use std::ffi::OsString;
use std::path::PathBuf;

fn main() {
    download();
    download_with_drive_item();
    download_and_format();
    // Rename the downloaded file
    download_and_rename("FILE_NAME");
}

// Set the name of the file you want to download here. The file must be in
// the root of your drive. This is the same place where your normal Documents,
// Attachments, Pictures, Desktop, etc. folders are.
static DRIVE_FILE: &str = "YOUR_DRIVE_FILE_NAME";

pub fn download() {
    // Get the access token from OAuth for the Drive API.
    let oauth: OAuth = OAuth::from_json_file("./examples/example_files/web_oauth.json").unwrap();
    let drive = Drive::try_from(oauth).unwrap();

    // Call the API. drive_root_child is the files in the users main documents folder.
    let mut collection: Collection<DriveItem> = drive.v1().drive_root_child().send().unwrap();
    // Save the metadata of the files.
    collection
        .to_json_file("./examples/example_files/drive_root_child.json")
        .unwrap();

    // Get the Values vec that lists the files.
    let drive_item = collection.find_by_name(DRIVE_FILE).unwrap();
    let item_id = drive_item.id().unwrap();

    // Download the file. The file will be downloaded with the same name.
    let mut req = drive
        .v1()
        .me()
        .download(item_id.as_str(), "./examples/example_files");

    let path_buf: PathBuf = req.send().unwrap();
    println!("{:#?}", path_buf);
}

pub fn download_with_drive_item() {
    // Get the access token from OAuth for the Drive API.
    let oauth: OAuth = OAuth::from_json_file("./examples/example_files/web_oauth.json").unwrap();
    let drive = Drive::try_from(oauth).unwrap();

    // Call the API. drive_root_child is the files in the users main documents folder.
    let mut collection: Collection<DriveItem> = drive.v1().drive_root_child().send().unwrap();
    // Save the metadata of the files.
    collection
        .to_json_file("./examples/example_files/drive_root_child.json")
        .unwrap();

    // Get the Values vec that lists the files.
    let mut drive_item = collection.find_by_name(DRIVE_FILE).unwrap();

    // Download the file. The file will be downloaded with the same name.
    let mut req = drive
        .v1()
        .me()
        .download_drive_item(&mut drive_item, "./examples/example_files")
        .unwrap();

    let path_buf: PathBuf = req.send().unwrap();

    println!("{:#?}", path_buf.metadata());
}

// You can convert a file to a different format using the download_format() method.
// There are 4 formats: glb, html, jpg, and pdf that an item can be converted to.
// This uses the PDF conversion which can be converted from: doc, docx, epub,
// eml, htm, html, md, msg, odp, ods, odt, pps, ppsx, ppt, pptx, rtf, tif, tiff, xls, xlsm, and xlsx.
//
// For more info on download formats see:
// https://docs.microsoft.com/en-us/onedrive/developer/rest-api/api/driveitem_get_content_format?view=odsp-graph-online
pub fn download_and_format() {
    // Get the access token from OAuth for the Drive API.
    let oauth: OAuth = OAuth::from_json_file("./examples/example_files/web_oauth.json").unwrap();
    let drive = Drive::try_from(oauth).unwrap();

    // Call the API. drive_root_child is the files in the users main documents folder.
    let mut collection: Collection<DriveItem> = drive.v1().drive_root_child().send().unwrap();
    // Save the metadata of the files.
    collection
        .to_json_file("./examples/example_files/drive_root_child.json")
        .unwrap();

    // Get the Values vec that lists the files.
    let mut drive_item = collection.find_by_name(DRIVE_FILE).unwrap();

    // Create the download request.
    let mut req = drive
        .v1()
        .me()
        .download_drive_item(&mut drive_item, "./examples/example_files")
        .unwrap();

    // Select the format.
    req.format_pdf();

    // Send the request and download the file.
    let path_buf: PathBuf = req.send().unwrap();

    println!("{:#?}", path_buf.metadata());
}

fn download_and_rename(name: &str) {
    // Get the access token from OAuth for the Drive API.
    let oauth: OAuth = OAuth::from_json_file("./examples/example_files/web_oauth.json").unwrap();
    let drive = Drive::try_from(oauth).unwrap();

    // Call the API. drive_root_child is the files in the users main documents folder.
    let mut collection: Collection<DriveItem> = drive.v1().drive_root_child().send().unwrap();
    // Save the metadata of the files.
    collection
        .to_json_file("./examples/example_files/drive_root_child.json")
        .unwrap();

    // Get the Values vec that lists the files.
    let mut drive_item = collection.find_by_name(DRIVE_FILE).unwrap();

    // Create the download request.
    let mut req = drive
        .v1()
        .me()
        .download_drive_item(&mut drive_item, "./examples/example_files")
        .unwrap();

    // Rename the file
    req.rename(OsString::from(name));

    // Send the request and download the file.
    let path_buf: PathBuf = req.send().unwrap();

    println!("{:#?}", path_buf.metadata());
}
