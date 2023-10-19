use actix_web::{
    web,
    web::block,
    HttpRequest,
    HttpResponse,
    error::InternalError,
    http::StatusCode,
};
use crate::errors::Error;
use actix_web_httpauth::headers::authorization::{Authorization, Bearer};
use crate::models::{
    Categories,
    Item,
    User,
};
use sailfish::TemplateOnce;
use diesel::{
    RunQueryDsl,
    ExpressionMethods,
    QueryDsl,
    PgConnection,
    Connection,
};
use actix_multipart::{Field, Multipart};
use futures::StreamExt;
use serde::{Deserialize, Serialize};
use std::{
    io::Write,
    fs::create_dir_all,
    str,
};
use crate::schema;
use crate::utils::establish_connection;


pub fn test_routes(config: &mut web::ServiceConfig) {
    config.route("/all-blogs/", web::get().to(all_blogs_page));
    config.route("/blog/{id}", web::get().to(blog_page));
    config.route("/create-blog/", web::post().to(create_blog));
    config.route("/edit-blog/{id}/", web::post().to(edit_blog));
    config.route("/delete-blog/{id}/", web::post().to(delete_blog));
}

async fn get_request_user_id(req: &HttpRequest) -> Option<i32> { 
    match Authorization::<Bearer>::parse(req) {
        Ok(ok) => {
            let token = ok.as_ref().token().to_string();
            return match verify_jwt(token, "MYSECRETKEY").await {
                Ok(ok) => ok.id,
                Err(_) => None,
            }
        },
        Err(_) => return None,
    }
}

fn get_user(pk: i32) -> User {
    use crate::schema::users::dsl::users;
    let _connection = establish_connection();
    return users
        .filter(schema::users::id.eq(pk))
        .first::<User>(&_connection)
        .expect("E");
}

fn get_content_type<'a>(req: &'a HttpRequest) -> Option<&'a str> {
    return req.headers().get("user-agent")?.to_str().ok();
}
pub fn is_desctop(req: &HttpRequest) -> bool {
    if get_content_type(req).unwrap().contains("Mobile") {
        return false;
    };
    return true;
} 

pub async fn all_blogs_page(req: HttpRequest) -> actix_web::Result<HttpResponse> {
    let is_desctop = is_desctop(&req);
    let cats = block(move || Categories::get_categories()).await?;
    let tags = block(move || Categories::get_tags()).await?;
    let user_id = get_request_user_id(&req);
    if user_id.is_some() {
        let _request_user = get_user(user_id.unwrap());
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/blogs/categories.stpl")]
            struct Template {
                request_user:   User,
                cats:           Vec<Cat>,
                all_tags:       Vec<SmallTag>,
            }
            let body = Template {
                request_user:   _request_user,
                cats:           cats,
                all_tags:       tags,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/blogs/categories.stpl")]
            struct Template {
                request_user:   User,
                cats:           Vec<Cat>,
                all_tags:       Vec<SmallTag>,
            }
            let body = Template {
                request_user:   _request_user,
                cats:           cats,
                all_tags:       tags,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
    else {
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/blogs/anon_categories.stpl")]
            struct Template {
                request_user:   User,
                cats:           Vec<Cat>,
                all_tags:       Vec<SmallTag>,
            }
            let body = Template {
                request_user:   _request_user,
                cats:           cats,
                all_tags:       tags,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/blogs/anon_categories.stpl")]
            struct Template {
                request_user:   User,
                cats:           Vec<Cat>,
                all_tags:       Vec<SmallTag>,
            }
            let body = Template {
                request_user:   _request_user,
                cats:           cats,
                all_tags:       tags,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
}
pub async fn get_blog_page(req: HttpRequest, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    let is_desctop = is_desctop(&req);
    let cats = block(move || Categories::get_categories()).await?;
    let tags = block(move || Categories::get_tags()).await?;
    let blog = block(move || Blogs::get_object(*_id)).await?;
    let user_id = get_request_user_id(&req);
    if user_id.is_some() {
        let _request_user = get_user(user_id.unwrap());
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/blogs/categories.stpl")]
            struct Template {
                request_user:   User,
                cats:           Vec<Cat>,
                all_tags:       Vec<SmallTag>,
                blog:           Blog,
            }
            let body = Template {
                request_user:   _request_user,
                cats:           cats,
                all_tags:       tags,
                blog:           blog,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/blogs/categories.stpl")]
            struct Template {
                request_user:   User,
                cats:           Vec<Cat>,
                all_tags:       Vec<SmallTag>,
                blog:           Blog,
            }
            let body = Template {
                request_user:   _request_user,
                cats:           cats,
                all_tags:       tags,
                blog:           blog,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
    else {
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/blogs/anon_categories.stpl")]
            struct Template {
                request_user:   User,
                cats:           Vec<Cat>,
                all_tags:       Vec<SmallTag>,
                blog:           Blog,
            }
            let body = Template {
                request_user:   _request_user,
                cats:           cats,
                all_tags:       tags,
                blog:           blog,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/blogs/anon_categories.stpl")]
            struct Template {
                request_user:   User,
                cats:           Vec<Cat>,
                all_tags:       Vec<SmallTag>,
                blog:           Blog,
            }
            let body = Template {
                request_user:   _request_user,
                cats:           cats,
                all_tags:       tags,
                blog:           blog,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
}


#[derive(Debug, Clone)]
pub struct UploadedFiles {
    pub name: String,
    pub path: String,
}
impl UploadedFiles {
    fn new(filename: String, owner_id: i32) -> UploadedFiles {
        use chrono::Datelike;

        let now = chrono::Local::now().naive_utc();
        let format_folder = format!(
            "./media/{}/{}/{}/{}/",
            owner_id.to_string(),
            now.year().to_string(),
            now.month().to_string(),
            now.day().to_string(),
        );
        let format_path = format_folder.clone() + &filename.to_string();
        // вариант для https
        let create_path = format_folder.replace("./", "/my/");
        // вариант для debug
        //let create_path = format_folder.replace("./", "/");
        create_dir_all(create_path).unwrap();

        UploadedFiles {
            name: filename.to_string(),
            path: format_path.to_string(),
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CategoriesForm {
    pub name:        String,
    pub description: String,
    pub image:       String,
}

pub async fn category_form(payload: &mut Multipart, owner_id: i32) -> CategoriesForm {
    let mut form: CategoriesForm = CategoriesForm {
        name:        "".to_string(),
        description: "".to_string(),
        image:       "".to_string(),
    };

    while let Some(item) = payload.next().await {
        let mut field: Field = item.expect("split_payload err");
        let name = field.name();

        if name == "image" {
            let _new_path = field.content_disposition().get_filename().unwrap();
            if _new_path != "" {
                let file = UploadedFiles::new(_new_path.to_string(), owner_id);
                let file_path = file.path.clone();
                let mut f = web::block(move || std::fs::File::create(&file_path).expect("Failed to open hello.txt"))
                    .await
                    .unwrap();
                while let Some(chunk) = field.next().await {
                    let data = chunk.unwrap();
                    f = web::block(move || f.write_all(&data).map(|_| f))
                        .await
                        .unwrap()
                        .expect("Failed to open hello.txt");
                }
                form.image = file.path.clone().replace("./","/");
            }
        }
        else {
            while let Some(chunk) = field.next().await {
                let data = chunk.expect("split_payload err chunk");
                if let Ok(s) = str::from_utf8(&data) {
                    let data_string = s.to_string();
                    if field.name() == "name" {
                        form.name = data_string
                    } else if field.name() == "description" {
                        form.description = data_string
                    }
                }
            }
        }
    }
    form
}
pub async fn create_blog(req: HttpRequest, mut payload: Multipart) -> impl Responder {
    let user_id = get_request_user_id(&req);
    if user_id.is_some() {
        let _request_user = get_user(user_id.unwrap());
        if _request_user.is_admin() {
            let form = item_form(payload.borrow_mut(), _request_user.id).await;
            Blog::create_blog(form);
        }
    };
    HttpResponse::Ok()
}
pub async fn edit_blog(req: HttpRequest, _id: web::Path<i32>) -> impl Responder {
    let user_id = get_request_user_id(&req);
    if user_id.is_some() {
        let _request_user = get_user(user_id.unwrap());
        let blog = block(move || Blogs::get_object(*_id)).await?; 
        if _request_user.id == blog.user_id {
            let form = item_form(payload.borrow_mut(), _request_user.id).await;
            blog.edit_blog(form);
        }
    };
    HttpResponse::Ok()
}
pub async fn delete_blog(req: HttpRequest, _id: web::Path<i32>) -> impl Responder {
    let user_id = get_request_user_id(&req);
    if user_id.is_some() {
        let _request_user = get_user(user_id.unwrap());
        let blog = block(move || Blogs::get_object(*_id)).await?; 
        if _request_user.id == blog.user_id {
            let form = item_form(payload.borrow_mut(), _request_user.id).await;
            blog.delete_blog();
        }
    };
    HttpResponse::Ok()
}

