// It is recommended that you read the README file, it is very important to this example.
// This example will help us to use a sqlite database with our bot.

use std::fmt::Write as _;

use rand::{thread_rng, Rng};

use serenity::async_trait;
use serenity::model::prelude::*;
use serenity::prelude::*;

struct Bot {
    database: sqlx::SqlitePool,
}

const COWBOY_ROLE: u64 = 1037134876601753680;
const INDIAN_ROLE: u64 = 1037064094727032913;

#[async_trait]
impl EventHandler for Bot {
    async fn message(&self, ctx: Context, msg: Message) {
        let user_id = msg.author.id.0 as i64;
        let guild_id = msg.guild_id.unwrap().as_u64().to_owned();
        let roles = ctx.cache.guild_roles(guild_id).unwrap();
        let rng = thread_rng();

        if msg.content.trim() == "~p cowboy" {
            let role = ctx.cache.role(guild_id, COWBOY_ROLE);
            let user_has_role = msg
                .author
                .has_role(&ctx, guild_id, COWBOY_ROLE)
                .await
                .unwrap();
            let points: u64 = rng.gen_range(10..=20);
            if user_has_role {
                sqlx::query!(
                    "INSERT INTO score (points, role, role_id) VALUES (?, ?, ?)",
                    points,
                    role,
                    role_id,
                )
                .execute(&self.database)
                .await
                .unwrap();

                let response = format!("Successfully added `{}` points to your team!", points);
                msg.channel_id.say(&ctx, response).await.unwrap();
            }
        } else if msg.content.trim() == "~p indian" {
            let role = ctx.cache.role(guild_id, INDIAN_ROLE);
            let user_has_role = msg
                .author
                .has_role(&ctx, guild_id, INDIAN_ROLE)
                .await
                .unwrap();
            let points: u64 = rng.gen_range(10..=20);
            if user_has_role {
                sqlx::query!(
                    "INSERT INTO score (points, role, role_id) VALUES (?, ?, ?)",
                    points,
                    role,
                    role_id,
                )
                .execute(&self.database)
                .await
                .unwrap();

                let response = format!("Successfully added {} points to your team!", points);
                msg.channel_id.say(&ctx, response).await.unwrap();
            }
        } else if msg.content.trim() == "~p list" {
            let table = sqlx::query!("SELECT * FROM score",)
                .fetch_all(&self.database) // < All matched data will be sent to todos
                .await
                .unwrap();

            let mut response = format!("Cowboys vs. Indians");
            for t in table.iter() {
                println!(response, "{}. {}", t.role, t.points).unwrap();
            }

            msg.channel_id.say(&ctx, response).await.unwrap();
        }
    }
}

#[tokio::main]
async fn main() {
    // Configure the client with your Discord bot token in the environment.
    let token = std::env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    // Initiate a connection to the database file, creating the file if required.
    let database = sqlx::sqlite::SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(
            sqlx::sqlite::SqliteConnectOptions::new()
                .filename("database.sqlite")
                .create_if_missing(true),
        )
        .await
        .expect("Couldn't connect to database");

    // Run migrations, which updates the database's schema to the latest version.
    sqlx::migrate!("./migrations")
        .run(&database)
        .await
        .expect("Couldn't run database migrations");

    let bot = Bot { database };

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(&token, intents)
        .event_handler(bot)
        .await
        .expect("Err creating client");
    client.start().await.unwrap();
}

