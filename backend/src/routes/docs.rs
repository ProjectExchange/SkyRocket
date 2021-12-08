use rocket::fairing::AdHoc;
use rocket_okapi::rapidoc::{make_rapidoc, GeneralConfig, HideShowConfig, RapiDocConfig};
use rocket_okapi::settings::UrlObject;
use rocket_okapi::swagger_ui::{make_swagger_ui, SwaggerUIConfig};

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Docs Stage", |rocket| async {
        rocket
            .mount(
                "/swagger-ui/",
                make_swagger_ui(&SwaggerUIConfig {
                    url: "../v1/openapi.json".to_owned(),
                    ..Default::default()
                }),
            )
            .mount(
                "/rapidoc/",
                make_rapidoc(&RapiDocConfig {
                    general: GeneralConfig {
                        spec_urls: vec![UrlObject::new("General", "../v1/openapi.json")],
                        ..Default::default()
                    },
                    hide_show: HideShowConfig {
                        allow_spec_url_load: false,
                        allow_spec_file_load: false,
                        ..Default::default()
                    },
                    ..Default::default()
                }),
            )
    })
}
