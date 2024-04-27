use async_openai::{
    error::{ApiError, OpenAIError},
    types::{
        AudioResponseFormat, ChatCompletionRequestSystemMessageArgs,
        ChatCompletionRequestUserMessageArgs, CreateChatCompletionRequestArgs,
        CreateTranscriptionRequestArgs,
    },
    Client,
};

pub async fn transcribe(uri: &str) -> Result<String, OpenAIError> {
    let client = Client::new();
    let request = CreateTranscriptionRequestArgs::default()
        .file(uri)
        .model("whisper-1")
        .response_format(AudioResponseFormat::Json)
        .build()?;

    println!("Transcribing...");
    Ok(client.audio().transcribe(request).await?.text)
}

pub async fn summarize_recipe(transcription: &str) -> Result<String, OpenAIError> {
    let client = Client::new();
    let request = CreateChatCompletionRequestArgs::default()
        .model("gpt-3.5-turbo")
        .temperature(0_f32)
        // .response_format(ChatCompletionResponseFormat { r#type: JsonObject })
        .messages([
            ChatCompletionRequestSystemMessageArgs::default()
                .content(get_system_message())
                .build()?
                .into(),
            ChatCompletionRequestUserMessageArgs::default()
                .content(transcription)
                .build()?
                .into(),
        ])
        .build()?;

    println!("Summarizing...");
    let response = client.chat().create(request).await?;
    response
        .choices
        .first()
        .ok_or(OpenAIError::ApiError(ApiError {
            message: String::from("Model did not respond."),
            r#type: None,
            param: None,
            code: None,
        }))?
        .message
        .content
        .clone()
        .ok_or(OpenAIError::ApiError(ApiError {
            message: String::from("Model did not respond."),
            r#type: None,
            param: None,
            code: None,
        }))
}

fn get_system_message() -> String {
    String::from(
        r#"You will be provided with the transcription of a video. Presumably, the
    video is a chef explaining how to cook one or more dishes. For each dish,
    identify its name, ingredients with measurements, and detailed step-by-step
    instructions. Provide your output as an array of JSON objects where each
    object is a dish. Follow the template I give you and replace the values of
    each key-value pair with the appropriate information:

    [
        {
            "name": "name of dish",
            "ingredients": [
                "ingredient and its measurements",
                "ingredient and its measurements"
            ],
            "instructions": [
                "first step",
                "second step"
            ]
        }
    ]
    
    Remember your output must be an array of valid JSON objects and follow the
    template I gave you. If the transcript is not related to cooking dishes,
    output an empty array, []. Do not wrap your response in a markdown block."#,
    )
}
