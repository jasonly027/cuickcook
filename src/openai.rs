use async_openai::{
    error::OpenAIError,
    types::{
        AudioResponseFormat, ChatCompletionRequestSystemMessageArgs, ChatCompletionRequestUserMessageArgs,
        CreateChatCompletionRequestArgs, CreateTranscriptionRequestArgs,
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

    let response = client.chat().create(request).await?;
    response.choices.first().and_then(
        |msg| msg.message.content.clone().and_then(
            |content| Some(content)
        )).ok_or(OpenAIError::InvalidArgument("Missing response".to_owned()))
}

fn get_system_message() -> String {
    r#"You will be provided with the transcription of a video. Presumably, the
    video is a chef explaining to the viewers how to cook one or more dishes.
    Find out the ingredients, measurements for the ingredients, and the detailed
    instructions for cooking the dishes. Provide your output by following the
    template I'm going to give you. The template is within "///"
    delimiters and words wrapped in angle brackets are placeholders. Follow the
    template for each dish in the transcript.

    ///
    <Dish 1>
    Ingredients:
    <ingredient and measurement>
    <ingredient and measurement>
    ...
    <ingredient and measurement>

    Instructions:
    <Step 1>
    <Step 2>
    ...
    <Step 3>
    ///
    
    There is a chance the transcription is not for a cooking video. In that
    case, disregard the template and simply output
    "This does not appear to be a cooking video.""#.to_owned()
}
