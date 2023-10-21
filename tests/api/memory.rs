use crate::helpers::{init_test, test_env, test_file, test_gpt};
use espionox::{
    agents::Agent,
    language_models::LanguageModel,
    memory::{CachingMechanism, Memory, MessageRole, MessageVector, ToMessage},
};

#[tokio::test]
async fn summarize_at_limit_works() {
    init_test();
    let limit = 4;
    let mech = CachingMechanism::SummarizeAtLimit {
        limit,
        save_to_lt: false,
    };
    let memory = Memory::build().caching_mechanism(mech).finished();
    let model = LanguageModel::from(test_gpt());
    let mut agent = Agent { memory, model };
    for _ in 0..=3 {
        agent
            .memory
            .push_to_message_cache("user", "Hello".to_string())
            .await;
        agent
            .memory
            .push_to_message_cache("assistant", "Hello! how can i help you?".to_string())
            .await;
    }
    assert!(limit >= agent.memory.cache().len_excluding_system_prompt());
}

#[tokio::test]
async fn forgetful_works() {
    init_test();
    let mech = CachingMechanism::Forgetful;
    let memory = Memory::build().caching_mechanism(mech.clone()).finished();
    let model = LanguageModel::from(test_gpt());
    let mut agent = Agent { memory, model };
    for _ in 0..=3 {
        agent
            .memory
            .push_to_message_cache("user", "Hello".to_string())
            .await;
        agent
            .memory
            .push_to_message_cache("assistant", "Hello! how can i help you?".to_string())
            .await;
    }
    assert!(mech.limit() >= agent.memory.cache().len_excluding_system_prompt());
}

#[tokio::test]
async fn long_term_memory_integration() {
    init_test();
    let mut memory = Memory::build()
        .env(test_env())
        .long_term_thread("testing")
        .finished();
    let file = test_file();
    memory.flatten_struct_to_cache(file);
    match memory.save_cache_to_long_term().await {
        Ok(_) => {
            tracing::info!("Succesfully saved cache to ltm");
            assert!(true)
        }
        Err(err) => {
            tracing::warn!("Error: {:?}", err);
            assert!(false)
        }
    }
}
