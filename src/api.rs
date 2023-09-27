use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        static CHARACTER_NAME: &str = "ASSISTANT:";
        static USER_NAME: &str = "USER:";

        use actix_web::web;
        use actix_web::HttpRequest;
        use actix_web::HttpResponse;
        use actix_web:ws:Message as Msg

        fn session_setup() {
            let persona = "A chat between a human and an assistant.";
            let history = format!(
                "{CHARACTER_NAME}:Hello - How may I help you today?\n\
                    {USER_NAME}:What is the capital of France?\n\
                    {CHARACTER_NAME}:Paris is the capital of France.\n"
            );
            history!
        }

        pub async fn ws(req: HttpRequest, body: Payload) -> Result<HttpResponse, Error>{
            let (respone, session, mut msg_stream) = actix_ws:handle(&req, body)?;
        }

    }
}
