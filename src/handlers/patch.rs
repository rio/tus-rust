pub struct PatchHandler;


fn handle_patch_method(request: Request, response: &mut Response) {
    let uri = match request.uri {
        RequestUri::AbsolutePath(uri) => uri,
        _ => {
            *response.status_mut() = StatusCode::NotFound;
            return;
        }
    };

    let file_name = match Path::new(&uri).file_name() {
        Some(file_name) => file_name,
        None => {
            *response.status_mut() = StatusCode::NotFound;
            return;
        }
    };

    response.headers_mut().set(TusResumable::new());
}
