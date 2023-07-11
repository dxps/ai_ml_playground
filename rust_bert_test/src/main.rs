use std::io::stdin;

use rust_bert::{
    gpt_neo::{GptNeoConfigResources, GptNeoModelResources, GptNeoVocabResources},
    pipelines::{
        common::{ModelResource, ModelType},
        text_generation::{TextGenerationConfig, TextGenerationModel},
    },
    resources::RemoteResource,
};

fn main() {
    //
    // Note that the 1st time this app runs, it downloads the model (with the size around 10GB),
    // so it might take a while. Subsequent runs will use the locally cached/downloaded model.

    // The model.
    let model_resource = ModelResource::Torch(Box::new(RemoteResource::from_pretrained(
        GptNeoModelResources::GPT_NEO_2_7B,
    )));
    // The config part of the model.
    let config_resource = Box::new(RemoteResource::from_pretrained(
        GptNeoConfigResources::GPT_NEO_2_7B,
    ));
    // The vocab part of the model.
    let vocab_resource = Box::new(RemoteResource::from_pretrained(
        GptNeoVocabResources::GPT_NEO_2_7B,
    ));
    // The merges part of the model.
    let merges_resource = Some(Box::new(RemoteResource::from_pretrained(
        GptNeoModelResources::GPT_NEO_2_7B,
    )));

    let mut generate_config = TextGenerationConfig::new(
        ModelType::GPTNeo,
        model_resource,
        config_resource,
        vocab_resource,
        merges_resource,
    );
    generate_config.no_repeat_ngram_size = 2;
    generate_config.max_length = Some(100);

    let model = TextGenerationModel::new(generate_config).unwrap();

    loop {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        // Splitting the input into tokens, with '/' as separator.
        // The 1st token is the text generation prefix.
        // The other tokens are generation input.
        let input_split = input.split('/').collect::<Vec<&str>>();
        let input_slice = input_split.as_slice();

        let output = model.generate(&input_slice[1..], Some(input_slice[0]));
        for sentence in output {
            println!("{sentence}")
        }
    }
}
