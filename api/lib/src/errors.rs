use error_chain::error_chain;
pub use error_chain::bail;

// All systems use these errors, except for GraphQL resolvers, because they have to return a particular kind of error
error_chain! {
    // The custom errors for this crate
    errors {
        InvalidToolchainName(t: String) {
            description("invalid toolchain name")
            display("invalid toolchain name: '{}'", t)
        }

        // You can also add commas after description/display.
        // This may work better with some editor auto-indentation modes:
        UnknownToolchainVersion(v: String) {
            description("unknown toolchain version"), // note the ,
            display("unknown toolchain version: '{}'", v), // trailing comma is allowed
        }

        // For when an environment variable has an invalid type
        // For example if a port is given as a hex string for some reason
        InvalidEnvVarType(var_name: String, expected: String) {
            description("invalid environment variable type")
            display(
                "invalid environment variable type for variable '{var_name}', expected '{expected}'",
                var_name=var_name,
                expected=expected
            )
        }

        // For if the required part of the GraphQL context object is not found
        GraphQLContextNotFound(elem_name: String) {
            description("required graphql context element not found")
            display("required graphql context element '{}' not found", elem_name)
        }

        // For when some Mutex is poisoned
        // This error is used if `.lock()` fails on a Mutex
        MutexPoisoned(mutex_name: String) {
            description("mutex poisoned")
            display("mutex '{}' poisoned", mutex_name)
        }

        OidSerializationFailed {
            description("failed to serialize string as object id")
            display("failed to serialize string as object id")
        }

        // For when the subscriptions server fails to publish data it's been asked to
        // This is used in the GraphQL systems themselves for parsing responses from the subscriptions server
        SubscriptionDataPublishFailed {
            description("subscriptions server failed to publish data internally")
            display("subscriptions server failed to publish data internally")
        }
    }
    // We work with many external libraries, all of which have their own errors
    foreign_links {
        Io(::std::io::Error);
        EnvFile(::dotenv::Error);
        EnvVar(::std::env::VarError);
        Mongo(::mongodb::error::Error);
        BsonOid(::mongodb::bson::oid::Error);
        Reqwest(::reqwest::Error);
        Json(::serde_json::Error);
    }
}

// These will automatically convert all our custom errors into versions palatable for GraphQL
// They should only be used in resolvers!
// There may be use cases where it's helpful to use one of `async_graphql`'s custom error types in one of your own functions though
pub type GQLResult<T> = async_graphql::Result<T>;
pub type GQLError = async_graphql::Error; // This type can't be used with `error_chain`
