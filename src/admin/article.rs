use salvo::prelude::*;

use crate::fs_helper;

#[handler]
pub async fn upload(req: &mut Request, res: &mut Response) {
    let file = req.file("file").await;
    if let Some(file) = file {
        let path = file.path().to_str().unwrap();
        let filename = file.name().unwrap();
        let keyname = filename.rsplitn(2, '.').skip(1).next().unwrap();
        if fs_helper::md_helper::add_catalog_by_upload_file(keyname, path).is_ok() {
            let dest = format!("assert/md/{}", filename);
            if let Err(e) = std::fs::copy(path, dest) {
                tracing::error!("Failed to copy file: {:?}", e);
                res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
                res.render(format!("fail to copy file: {:?}", e));
            } else {
                res.render(Redirect::found("/"));
            };
        } else {
            res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
            res.render("fail to add catalog by upload file");
        }
    } else {
        res.status_code(StatusCode::BAD_REQUEST);
        res.render("file not found in request");
    }
}
