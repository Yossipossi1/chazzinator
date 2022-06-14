use poise::{serenity_prelude as serenity, Event};

use rusqlite::{params, Connection, Result};

use std::time;

use crate::config;
use crate::poise_ext::{Data, Error};

pub async fn event_listeners(
    ctx: &serenity::Context,
    event: &poise::Event<'_>,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    _user_data: &Data,
) -> Result<(), Error> {
    let current_time = time::SystemTime::now()
        .duration_since(time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let configuration = config::load();

    match event {
        Event::Ready {
            data_about_bot: bot,
        } => {
            println!("{} is connected!", bot.user.name)
        }

        // When a guild member is updated...
        Event::GuildMemberUpdate {
            old_if_available: pre_member_wrapped,
            new: post_member,
        } => {
            let visitor_role_id = serenity::RoleId(configuration.guild.roles.visitor);

            if let Some(pre_member) = pre_member_wrapped {
                // ...check if the update was the addition of the visitor role...
                if !pre_member.roles.contains(&visitor_role_id)
                    & post_member.roles.contains(&visitor_role_id)
                {
                    // ...and insert the relevant information into the database.
                    db_insert_visitor_addition(
                        u64::from(post_member.user.id),
                        post_member.joined_at.unwrap().unix_timestamp(),
                        current_time,
                    )?;
                }
            };

            if let Err(reason) = updater(ctx.to_owned(), current_time).await {
                println!("Unable to execute updater: {:?}", reason);
            };
        }

        _ => {}
    }

    Ok(())
}

// TODO: Create a less intense, looping function which executes these updates once every X time instead of per user update.
async fn updater(ctx: serenity::Context, current_time: u64) -> Result<(), Error> {
    let configuration = config::load();

    // Get the user IDs from the Visitor struct.
    let mut visitors_id_vec = db_get_visitor_additions(current_time)?
        .into_iter()
        .map(|x| x.user_id);

    // Filter members who do not have the Visitor role.
    let members_vec: Vec<serenity::Member> =
        poise::serenity_prelude::GuildId(configuration.guild.id)
            .members(&ctx.http, None, None)
            .await?
            .into_iter()
            .filter(|x| {
                x.roles
                    .contains(&serenity::RoleId(configuration.guild.roles.visitor))
            })
            .collect();

    // For every member who is eligible for the Member role... 
    for mut member in members_vec {
        if visitors_id_vec.any(|x| member.user.id == serenity::UserId(x)) {
            // ...add the Member role and remove the Visitor role...
            let _ = member
                .add_role(&ctx.http, configuration.guild.roles.member)
                .await;
            let _ = member
                .remove_role(&ctx.http, configuration.guild.roles.visitor)
                .await;
            // ...and remove the user from the database of people who are eligible to be assigned member.
            let _ = db_remove_visitor_addition(u64::from(member.user.id));
        }
    }

    Ok(())
}

/* TODO: Probably find a better place to put these. */

#[allow(dead_code)] // May need other fields for future use.
struct Visitors {
    user_id: u64,
    join_time: i64,
    role_added_time: u64,
}

// Get the list of people who got the visitor role.
fn db_get_visitor_additions(current_time: u64) -> Result<Vec<Visitors>, rusqlite::Error> {
    let connection = Connection::open("./database.db")?;

    let mut stmt = connection.prepare(
        "SELECT user_id, server_join_time, visitor_added_time 
        FROM user_time_roles
        WHERE visitor_added_time < ?",
    )?;

    // Execute query and return each row as a Visitors struct.
    // TODO: Change this from a hard-coded number to a configurable setting.
    let rows = stmt.query_map(params![current_time - 259200], |row| {
        Ok(Visitors {
            user_id: row.get(0)?,
            join_time: row.get(1)?,
            role_added_time: row.get(2)?,
        })
    })?;

    // Put the Visitor structs into a vector.
    let mut visitor_vec = Vec::new();

    for row in rows {
        visitor_vec.push(row?);
    }

    Ok(visitor_vec)
}

// Insert the addition of the visitor role into the database.
fn db_insert_visitor_addition(
    user_id: u64,
    join_time: i64,
    role_added_time: u64,
) -> Result<(), rusqlite::Error> {
    let connection = Connection::open("./database.db")?;

    let _ = connection.execute(
        "INSERT INTO user_time_roles(
            user_id,
            server_join_time,
            visitor_added_time
        ) 
        VALUES (
            ?1,
            ?2,
            ?3
        )",
        params![user_id, join_time, role_added_time],
    )?;

    Ok(())
}

// Remove a user from the visitor database.
fn db_remove_visitor_addition(user_id: u64) -> Result<(), rusqlite::Error> {
    let connection = Connection::open("./database.db")?;

    let _ = connection.execute(
        "DELETE FROM user_time_roles 
        WHERE user_id = ?1",
        params![user_id],
    )?;

    Ok(())
}
