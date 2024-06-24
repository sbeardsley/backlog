// async fn handle_login<T>(
//     service: web::Data<T>
//     login: web::Json<Login>
//   ) -> Result<impl Responder, Error>
//   where
//     T: AuthenticationContract
//   {
//     let login_data = login.0;
//     let session = service.login(login_data)?;
//     HttpResponseBuilder::new(StatusCode::OK).json(session)
//   }
