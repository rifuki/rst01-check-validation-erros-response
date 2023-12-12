use actix_web::web;

use super::handler::{
    get_users, 
    create_user, 
    // get_user, 
    // update_user, 
    // delete_user
};

pub fn scoped_users(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .route("", web::get().to(get_users))
            .route("", web::post().to(create_user))
            // .service(
                // web::resource("/{user_id}")
                //     .get(get_user)
                //     .put(update_user)
                //     .delete(delete_user)
            // )
    );
}