use crate::AOCClient;
use openai_api_rust::{
    chat::{ChatApi, ChatBody},
    Auth, Message, OpenAI, Role,
};
use std::sync::Arc;

use anyhow::{Context, Result};

pub fn get_day_description(aoc_client: Arc<AOCClient>, day: usize) -> Result<String> {
    let aoc_description = aoc_client.get_description(day)?;
    let code = std::fs::read_to_string(format!("src/day{:02}/mod.rs", day))?;

    let auth = Auth::from_env().map_err(|e| anyhow::anyhow!(e))?;
    let openai = OpenAI::new(auth, "https://api.openai.com/v1/");
    let body = ChatBody {
        model: "gpt-4o-mini".to_string(),
        max_tokens: None,
        temperature: None,
        top_p: None,
        n: Some(1),
        stream: Some(false),
        stop: None,
        presence_penalty: None,
        frequency_penalty: None,
        logit_bias: None,
        user: None,
        messages: vec![
            Message {
                role: Role::System,
                content: r"
            You will be given an advent of code puzzle description and my 
            code to solve this puzzle in Rust. You must write a very short 
            (just 1 paragraph) but informative explanation of the approach 
            that I took to solve the puzzle written in first person. 
            
            Don't write any filler text or introduction such as stating the 
            name of the puzzle or that it is in Rust. Just launch into the 
            description of the approach. It should be brief and technical 
            and should focus on the choice of datastructures and algorithms
            that were used.

            The parsing stage is not interesting and not to be mentioned.
            
            Be sure to wrap any code in <code></code> tags and not to use
            markdown backticks.

            For example when saying I used a <code>HashMap</code> to store
            the values, you should write I used a <code>HashMap</code> to
            store the values and not I used a `HashMap` to store the values.

            REMEMBER: Never use backticks for code. Always use <code></code> tags.
            "
                .to_string(),
            },
            Message {
                role: Role::User,
                content: format!(
                    "Puzzle description: {}\n\nMy code:\n{}",
                    aoc_description, code
                ),
            },
        ],
    };
    let result = openai
        .chat_completion_create(&body)
        .map_err(|e| anyhow::anyhow!(e))?;

    let message = result
        .choices
        .first()
        .context("No completion returned")?
        .message
        .clone()
        .context("No message in completion")?;

    Ok(message.content)
}
