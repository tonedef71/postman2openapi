#[cfg(test)]
#[cfg(not(target_arch = "wasm32"))]
mod unit_tests {
    use postman2openapi::openapi::v3_0::{ObjectOrReference, Parameter, Schema};
    use postman2openapi::openapi::OpenApi;
    use postman2openapi::postman::Spec;
    use postman2openapi::Transpiler;

    #[test]
    fn it_preserves_order_on_paths() {
        let spec: Spec = serde_json::from_str(get_fixture("echo.postman.json").as_ref()).unwrap();
        let oas = Transpiler::transpile(spec);
        let ordered_paths = [
            "/get",
            "/post",
            "/put",
            "/patch",
            "/delete",
            "/headers",
            "/response-headers",
            "/basic-auth",
            "/digest-auth",
            "/auth/hawk",
            "/oauth1",
            "/cookies/set",
            "/cookies",
            "/cookies/delete",
            "/status/200",
            "/stream/5",
            "/delay/2",
            "/encoding/utf8",
            "/gzip",
            "/deflate",
            "/ip",
            "/time/now",
            "/time/valid",
            "/time/format",
            "/time/unit",
            "/time/add",
            "/time/subtract",
            "/time/start",
            "/time/object",
            "/time/before",
            "/time/after",
            "/time/between",
            "/time/leap",
            "/transform/collection",
            "/{method}/hello",
        ];
        if let OpenApi::V3_0(s) = oas {
            let keys = s.paths.keys().enumerate();
            for (i, k) in keys {
                assert_eq!(k, ordered_paths[i])
            }
        }
    }

    #[test]
    fn it_uses_the_correct_content_type_for_form_urlencoded_data() {
        let spec: Spec = serde_json::from_str(get_fixture("echo.postman.json").as_ref()).unwrap();
        let oas = Transpiler::transpile(spec);
        if let OpenApi::V3_0(oas) = oas {
            let b = oas
                .paths
                .get("/post")
                .unwrap()
                .post
                .as_ref()
                .unwrap()
                .request_body
                .as_ref()
                .unwrap();
            if let ObjectOrReference::Object(b) = b {
                assert!(b.content.contains_key("application/x-www-form-urlencoded"));
            }
        }
    }

    #[test]
    fn it_generates_headers_from_the_request() {
        let spec: Spec = serde_json::from_str(get_fixture("echo.postman.json").as_ref()).unwrap();
        let oas = Transpiler::transpile(spec);
        if let OpenApi::V3_0(oas) = oas {
            let params = oas
                .paths
                .get("/headers")
                .unwrap()
                .get
                .as_ref()
                .unwrap()
                .parameters
                .as_ref()
                .unwrap();
            let header = params
                .iter()
                .find(|p| {
                    if let ObjectOrReference::Object(p) = p {
                        p.location == "header"
                    } else {
                        false
                    }
                })
                .unwrap();
            let expected = ObjectOrReference::Object(Parameter {
                name: "my-sample-header".to_owned(),
                location: "header".to_owned(),
                description: Some("My Sample Header".to_owned()),
                schema: Some(Schema {
                    schema_type: Some("string".to_owned()),
                    example: Some(serde_json::Value::String(
                        "Lorem ipsum dolor sit amet".to_owned(),
                    )),
                    ..Schema::default()
                }),
                ..Parameter::default()
            });
            assert_eq!(header, &expected);
        }
    }

    fn get_fixture(filename: &str) -> String {
        use std::fs;

        let filename: std::path::PathBuf =
            [env!("CARGO_MANIFEST_DIR"), "./tests/fixtures/", filename]
                .iter()
                .collect();
        let file = filename.into_os_string().into_string().unwrap();
        fs::read_to_string(file).unwrap()
    }
}