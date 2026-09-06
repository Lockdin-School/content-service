use crate::core::concepts::handlers::{
    create_concept, delete_concept, get_all_concepts, get_concept_by_id, get_concepts_by_topic_id,
    update_concept,
};
use crate::core::lessons::handlers::{create_lesson, get_lesson_by_id};
use crate::core::materials::handlers::{get_all_materials, get_material_by_id};
use crate::core::quizzes::handlers::{
    create_option, create_question, create_quiz, get_option_by_id, get_options_by_question_id,
    get_quiz_by_id,
};
use actix_web::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    log::info!("Configuring routes...");
    cfg.service(
        // ------------- configure routes ------------
        web::scope("/api/v1")
            .service(
                web::scope("/topics")
                    .service(get_all_materials)
                    .service(get_concepts_by_topic_id),
            )
            .service(web::scope("/materials").service(get_material_by_id))
            .service(
                web::scope("/lessons")
                    .service(create_lesson)
                    .service(get_lesson_by_id),
            )
            .service(
                web::scope("/concepts")
                    .service(get_all_concepts)
                    .service(create_concept)
                    .service(get_concept_by_id)
                    .service(update_concept)
                    .service(delete_concept),
            )
            .service(
                web::scope("/quizzes")
                    .service(get_quiz_by_id)
                    .service(create_quiz),
            )
            .service(
                web::scope("/questions")
                    .service(get_options_by_question_id)
                    .service(create_question),
            )
            .service(
                web::scope("/options")
                    .service(create_option)
                    .service(get_option_by_id),
            ),
    );
}
