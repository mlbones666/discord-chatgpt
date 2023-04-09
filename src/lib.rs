use discord_flows::{create_text_message_in_channel, listen_to_channel, TextMessage};
use dotenv::dotenv;
use flowsnet_platform_sdk::write_error_log;
use openai_flows::{chat_completion, ChatModel, ChatOptions};
use std::env;

#[no_mangle]
pub fn run() {
    let guild_name: String = match env::var("server_name") {
        Err(_) => "myserver".to_string(),
        Ok(name) => name,
    };
    let channel_name: String = match env::var("channel_name") {
        Err(_) => "general".to_string(),
        Ok(name) => name,
    };

    let openai_key_name: String = match env::var("openai_key_name") {
        Err(_) => "jaykchen".to_string(),
        Ok(name) => name,
    };

    listen_to_channel(&guild_name, &channel_name, |sm| {
        let prompt = "Please answer questions based on the below FAQ:
How do we compete with SSV or Obol?
We don’t have to compete with them. We are here to work with staking providers (like Lido). They want to decentralize their node ops, and we will offer a unique way for them (and others) to do it with 8 ETH initiators.
Can I post my content about ParaState in the community?
We would prefer that you do not use our content word for a word anywhere unless it is quoted and referenced correctly. But you are welcome to discuss the project on your terms and use our materials for reference.
Will LIQUID STAKING be available on SafeStake?
Liquid staking will actually be coming in Stage 2! The 'initiator' will make an 8 ETH deposit to create a mini-pool and the remaining 24 ETH will come from the LSD staking pool.
What is the conversation ratio?
10 STATE : 1 DVT
What is the new token supply ?
If the current max supply of state token is 1B, then the max supply of DVT token is 100M. If the product needs to increase max supply for new demand, we will have another governance vote.
More details : Besides the governance function, the major token use case will be the native payment token on SafeStake mainnet. Such a major function will change the supply and demand dramatically after mainnet launch, driving us to tweak the outdated token economy. Why? Let’s assume around 100K validators onboard SafeStake in one to two years, and the subscription fee to the Operator committee is 120 tokens for one validator per month. The balance tokens will be consumed in less than 3.5 years even if we release all eco treasury token portions for the validators since the subscription fee is an ARR fee.
Token price after the new token release ?
If 10 State :1 DVT, the DVT token listing price should be 10x of the closing State spot price. However, we can’t control how the market will react.
Do I need to unstake my STATE token to convert STATE into the new DVT token?
Staked $STATE will be able to be converted to a new token without the need to unstake it. If your STATE tokens are under the staking contract, please keep them as it is. We are still on the governance vote & we will provide more details through a complete article about the swapping process. Therefore no actions are needed to be taken for now.
With the team having control of 80% of the supply, will they be voting with that?
The team portion is also vested and linear release. We don’t even have to use the released amount to vote. Given the current status, it seems the majority community supports it. (While the team does not believe we will need to use the released portion to vote, the team does reserve the right to use that portion to vote if necessary
What is the voting power for stakers?
The weight of State tokens in the governance staking contract is 2x of the one in circulation.";

        if !sm.author.bot && !sm.content.trim().is_empty() {
            let msg = sm.content;
            let co = ChatOptions {
                model: ChatModel::GPT35Turbo,
                restart: false,
                restarted_sentence: Some(prompt),
            };
            if let Some(r) =
                chat_completion(&openai_key_name, &format!("chat_id#{}", sm.author.username), &msg, &co)
            {
                create_text_message_in_channel(&guild_name, &channel_name, r.choice, Some(sm.id));
            }
        };
    });
}
