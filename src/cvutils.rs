use anyhow::Result;
use opencv::core::Vector;
use opencv::types;
use opencv::{
    core::{self},
    imgcodecs,
    prelude::*,
};
use std::collections::HashMap;
use std::path::Path;

use crate::user_config;
use crate::{fileutils::cleanup_tmp, himawaridt::HimawariDatetime, tiles::LocalTile};

const ROWMAX: u32 = 20;
const COLMAX: u32 = 20;

pub(crate) enum Axis {
    X,
    Y,
}
/// Save image with opencv  
/// #Arguments:
/// * `opencv::core::Mat` a Mat of the image you want to save
/// * `&str` path to save it to
#[allow(dead_code)]
pub(crate) async fn cv_save_image(m: &core::Mat, p: &str) -> Result<()> {
    let _ = imgcodecs::imwrite(p, &m, &types::VectorOfi32::new()).unwrap();
    Ok(())
}
/// load image with opencv
/// #Arguments:
/// * `&str` path to load image from
pub(crate) async fn cv_load_image(p: &str) -> Result<core::Mat> {
    let mat = imgcodecs::imread(p, imgcodecs::IMREAD_COLOR)?;
    Ok(mat)
}
/// Helper to build the entire disk, first with vertical strips which are stored in tmp, then in horizontal.
/// it calls cv_concat_array which, could do vertical or horizontal, vertical is preferred to ensure the way the CPP Vector
/// which is being wrapped will be sorted.
/// Then it concats the other axis.
/// The FullDisc is returned in a struct wrapping its location on disk, height, width size and has methods callable on itself that're kinda useful.
/// #Arguments:
/// * `m: HashMap<(u32,u32), LocalTile>`
/// * `hwdt` the HimawariDatetime used to create all the tiles the disc will be made up of
pub(crate) async fn assemble_full_disc(
    m: HashMap<(u32, u32), LocalTile>,
    hwdt: HimawariDatetime,
    user_config: crate::user_config::Config,
) -> Result<crate::wallpaperutils::FullDisc> {
    let mut working_vec = vec![];
    let mut range: Vector<Mat> = Vector::new();

    for r in 0..ROWMAX {
        working_vec.push(m.get(&(r, 0)).unwrap().clone());
        for c in 1..COLMAX {
            working_vec.push(m.get(&(r, c)).unwrap().clone());
        }
        range.push(cv_concat_array(&working_vec, Axis::X, Some(true)).await?); //NOTE: is this Some(bool) sloppy?
        working_vec.clear();
    }
    concat_off_axis(range, Axis::Y, hwdt).await?;

    let p = std::path::Path::new(&user_config.completed).join(hwdt.pretty_filename());
    assert!(cleanup_tmp()?); //NOTE Cleanup the completed too?

    crate::wallpaperutils::FullDisc::new(&p)
}

// TODO: Consider removing the Axis args, as you're not giving the user a choice really...
/// Wrapping OpenCV's array/matrix based concats, to build disks from LocalTiles
/// #Arguments:
/// * `v` a Vector<LocalTile> which represent a tile on disk you're looking to concatenate.
/// * `axis` the axis you want to concat on.
/// * `keep_tmps` - a bool to determine if you keep intermediate files or not during the concatenation.
async fn cv_concat_array(v: &Vec<LocalTile>, axis: Axis, keep_tmps: Option<bool>) -> Result<Mat> {
    let mut range: Vector<Mat> = Vector::new(); //opencv requires you use their vector type, which is NOT a rust Vec.
    let mut mat = core::Mat::default();

    for lt in v.iter() {
        //TODO this could be a oneliner no? .iter().map(|lt| lt.....
        range.push(cv_load_image(lt.path_as_str()).await?);
    }
    match axis {
        Axis::X => core::vconcat(&range, &mut mat).unwrap(),
        Axis::Y => core::hconcat(&range, &mut mat).unwrap(),
    }
    // keep the intermediate representations on disk...
    if keep_tmps.unwrap_or(false) {
        let filename = format!("completed/{}complete.png", v[0].get_xy().await.0);
        imgcodecs::imwrite(&filename, &mat, &types::VectorOfi32::new()).unwrap();
    }
    Ok(mat)
}

/// Wrapping OpenCV's array/matrix based concat but taking a native cpp vector of Mat's, as opporesed to a rust Vec. (see: cv_concat_array)
/// #Arguments:
/// * `range` a Vector<Mat> of the images you want to concat, NOTE: this is the cpp wrapped Vector, not the rust one
/// * `axis` the axis you want to concat on
/// * `hwdt` the HimawariDatetime used to create all the tiles this img will be made up of
async fn concat_off_axis(range: Vector<Mat>, axis: Axis, hwdt: HimawariDatetime) -> Result<()> {
    let mut mat = core::Mat::default();
    match axis {
        Axis::X => core::vconcat(&range, &mut mat).unwrap(),
        Axis::Y => core::hconcat(&range, &mut mat).unwrap(),
    }
    let filename = format!("completed/{}", hwdt.pretty_filename());
    let _ = imgcodecs::imwrite(&filename, &mat, &types::VectorOfi32::new()).unwrap();
    Ok(())
}

/// Retrieve the (width, height) of an image from its path.
pub(crate) fn get_dims(p: &Path) -> Result<(i32, i32)> {
    let img = imgcodecs::imread(
        p.to_str()
            .expect("unable to parse filepath to read dimensions. in cv_get_dims"),
        imgcodecs::IMREAD_COLOR,
    )?;
    Ok((img.rows(), img.cols()))
}
