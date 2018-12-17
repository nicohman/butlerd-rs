use Responses::BError;
error_chain! {
    types {
        Error, ErrorKind, ResultExt, Result;
    }
    links {
    }
    foreign_links {
        Fmt(::std::fmt::Error);
        Io(::std::io::Error);
        Network(::reqwest::Error);
        Parse(::serde_json::error::Error);
    }

    errors {
        StartUpError(reason: String) {
            description("error starting butler")
            display("couldn't start butler: '{}'", reason)
        }
        ButlerError(berror: BError) {
            description("butler encountered an error")
            display("error message: {}", berror.message)
        }
        MissingField(field: String) {
            description("missing field in response")
            display("missing field {} in response", field)
        }
    }
}
