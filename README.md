# rgpt

A rust Chat GPT-3 CLI tool.

This implementation of the `main` function calls the appropriate subcommand based on the user's input, using the provided `auth_token` and `model` values to create a `Client` instance for sending requests to the OpenAI API.

When the `chat` subcommand is called, the `CompletionResponse` struct is used to deserialize the JSON response from the OpenAI API, and the generated chat response is printed to the console.

When the `code` subcommand is called, the `CodeCompletionResponse` struct is used to deserialize the JSON response from the OpenAI API, and the generated code completion is printed to the console.

When the `image` subcommand is called, a `Vec<String>` of URLs for the generated images is returned from the OpenAI API, and each URL is printed to the console using a `for` loop.

Overall, this implementation provides a full-featured CLI tool for working with OpenAI's GPT-3 API in Rust.
