use serenity::async_trait;
use serenity::prelude::*;
use serenity::model::channel::Message;
use evalexpr::eval_int;
use scraper::*;


pub async fn get_weather() -> Result<String, reqwest::Error> {
    
    let url = "https://weather.com/weather/today/l/857a297816e489427e3c6b82d20cb5cd974d1c6132de1e63e139c32f1bfca2bd";
    

    let weather_page = reqwest::get(url).await?.text().await?;
     
    let document = scraper::Html::parse_document(&weather_page);
    
    let fragment = document.select(&Selector::parse("body").unwrap()).next().unwrap();

    let fragment = fragment.select(&Selector::parse("div#todayDetails").unwrap()).next().unwrap();

    // &Selector::parse we used a css selector. 
    let fragment = fragment.select(&Selector::parse("#todayDetails > section:nth-child(1) > div:nth-child(2)").unwrap()) 
        .flat_map(|ele| ele.text())
        .collect::<Vec<&str>>();

    let mut output : String = String::new();
    
    // put spaces in between data. 
    for ele in fragment.iter() {
        output = output + ele.as_ref();
        output = output + " ";
    }
    
    Ok(output)
    
}


struct Handler;


#[async_trait]
impl EventHandler for Handler {

    async fn message(&self, ctx : Context, msg : Message)  {

        let userinp : Vec<&str> = msg.content.split(" ").collect();

        match userinp.get(0).unwrap().as_ref() {
            "!ping" => {
                msg.channel_id.say(&ctx.http, "pong").await.expect("error");
            },
            
            "!solve" => {
                let user_equa = userinp.get(1).unwrap().as_ref();
                let user_equa = eval_int(user_equa).unwrap_or(-255);

                if user_equa == -255 {
                    msg.channel_id.say(&ctx.http, "error bad operator").await.expect("error");
                } else {
                    msg.channel_id.say(&ctx, user_equa.to_string()).await.expect("error");
                }
            }

            "!help" => {
                let msssg = "commands :      !solve- command solves basic arithmetic expressions. Ex !solve 5+8 ;;; !ping checks the bot, making sure it is running ;;; !weather tell you the weather of largo florida";
                msg.channel_id.say(&ctx.http, msssg).await.expect("error"); 
            }, 
            
            "!weather" => {
                let weather = get_weather().await;
                msg.channel_id.say(&ctx.http, weather.unwrap()).await.expect("error");
            },

            _ => println!("std msg")
        } 
        
    }
}


#[tokio::main]
async fn main() {

    let token = "TOKEN"; 
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .await
        .expect("Error creating client");
    
    client.start().await.expect("An error occurred");

}

