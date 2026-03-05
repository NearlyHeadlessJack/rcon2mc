/*
 * // Copyright (c) 2026 Jack Wang
 * //
 * // Permission is hereby granted, free of charge, to any person obtaining a copy
 * // of this software and associated documentation files (the "Software"), to deal
 * // in the Software without restriction, including without limitation the rights
 * // to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * // copies of the Software, and to permit persons to whom the Software is
 * // furnished to do so, subject to the following conditions:
 * //
 * // The above copyright notice and this permission notice shall be included in all
 * // copies or substantial portions of the Software.
 * //
 * // THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * // IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * // FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * // AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * // LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * // OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * // SOFTWARE.
 * //
 * // Author: Jack Wang <wang@rjack.cn>
 * // GitHub: https://github.com/nearlyheadlessjack/rcon2mc
 */
use rcon2mc::rcon_client::{RconClient, TargetStatus, TargetStatusSuccess};
use std::sync::{Mutex, MutexGuard, OnceLock};
use std::thread::sleep;
use std::time::Duration;
mod utils;

static GLOBAL_EXECUTOR: OnceLock<Mutex<rcon2mc::command::CommandExecutor>> = OnceLock::new();

fn get_executor() -> Option<MutexGuard<'static, rcon2mc::command::CommandExecutor>> {
    let mutex = GLOBAL_EXECUTOR.get_or_init(|| {
        let rcon = RconClient::builder()
            .host(utils::consts::host())
            .port(utils::consts::port())
            .password(utils::consts::password())
            .build()
            .expect("Fail to build rcon client connection");
        Mutex::new(rcon.command())
    });
    Some(mutex.lock().unwrap())
}

// #[test]
// fn test_docker_command_whitelist_add_not_found() {
//     let Some(rcon) = utils::rcon::get_rcon() else {
//         return;
//     };
//     let Ok(rcon) = rcon else {
//         assert!(false);
//         return;
//     };
//     let feedback = rcon
//         .command()
//         .whitelist_add("NOFbieufwbe3i32fdSWEATER99992f")
//         .expect("whitelist add command push fail");
//     dbg!(&feedback);
//     assert_eq!(feedback, TargetStatus::NotFound)
// }

#[test]
fn test_docker_command_kill_none() {
    let Some(mut executor) = get_executor() else {
        panic!("Fail to get rcon executor");
    };
    let feedback = executor.kill("Steve").expect("kill command push fail");
    dbg!(&feedback);
    assert_eq!(feedback, TargetStatus::NotFound)
}

#[test]
fn test_docker_command_kick_none() {
    let Some(mut executor) = get_executor() else {
        panic!("Fail to get rcon executor");
    };
    let feedback = executor
        .kick("Steve", Some("no reason"))
        .expect("kick command push fail");
    dbg!(&feedback);
    assert_eq!(feedback, TargetStatus::NotFound)
}

#[test]
fn test_docker_command_give_none() {
    let Some(mut executor) = get_executor() else {
        panic!("Fail to get rcon executor");
    };
    let feedback = executor
        .give("Steve", "minecraft:torch", 1)
        .expect("give command push fail");
    dbg!(&feedback);
    assert_eq!(feedback, TargetStatus::NotFound)
}

#[test]
fn test_docker_command_msg_none() {
    let Some(mut executor) = get_executor() else {
        panic!("Fail to get rcon executor");
    };
    let feedback = executor
        .msg("Steve", "my msg")
        .expect("msg / w / tell command push fail");
    dbg!(&feedback);
    assert_eq!(feedback, TargetStatus::NotFound)
}

#[test]
fn test_docker_command_say_none() {
    let Some(mut executor) = get_executor() else {
        panic!("Fail to get rcon executor");
    };
    let feedback = executor.say("my msg").expect("say command push fail");
    dbg!(&feedback);
    assert_eq!(feedback, ())
}

#[test]
fn test_docker_command_title_none() {
    let Some(mut executor) = get_executor() else {
        panic!("Fail to get rcon executor");
    };
    let feedback = executor
        .title("Steve", "title", "my msg")
        .expect("title command push fail");
    dbg!(&feedback);
    assert_eq!(feedback, TargetStatus::NotFound)
}

#[test]
fn test_docker_command_tp_none() {
    let Some(mut executor) = get_executor() else {
        panic!("Fail to get rcon executor");
    };
    let feedback = executor
        .tp("Steve", 1.0, 1.0, 0.0)
        .expect("tp command push fail");
    dbg!(&feedback);
    assert_eq!(feedback, TargetStatus::NotFound)
}

#[test]
fn test_docker_command_transfer_none() {
    use utils::rcon::is_not_available;
    if is_not_available("1.20.5") {
        return;
    }
    let Some(mut executor) = get_executor() else {
        panic!("Fail to get rcon executor");
    };
    let feedback = executor
        .transfer("192.168.1.1", "25565", "Steve")
        .expect("transfer command push fail");
    dbg!(&feedback);
    assert_eq!(feedback, TargetStatus::NotFound)
}

#[test]
fn test_docker_command_weather_none() {
    let Some(mut executor) = get_executor() else {
        panic!("Fail to get rcon executor");
    };
    let feedback = executor
        .weather("clear")
        .expect("weather command push fail");
    dbg!(&feedback);
    assert_eq!(feedback, ())
}

#[test]
fn test_docker_command_difficulty_none() {
    let Some(mut executor) = get_executor() else {
        panic!("Fail to get rcon executor");
    };
    let feedback = executor
        .difficulty("easy")
        .expect("difficulty command push fail");
    dbg!(&feedback);
    assert_eq!(feedback, ())
}
/// 1.12.2测试，重复封禁仍使用旧信息，不会提示重复信息
#[test]
fn test_docker_command_ban_none() {
    let Some(mut executor) = get_executor() else {
        panic!("Fail to get rcon executor");
    };
    let feedback = executor
        .ban("zi_min", Some("no reason"))
        .expect("ban command push fail");
    dbg!(&feedback);
    let result = if feedback == TargetStatus::Success(TargetStatusSuccess::Success)
        || feedback == TargetStatus::Success(TargetStatusSuccess::Duplicated)
    {
        true
    } else {
        false
    };
    assert!(result);
    sleep(Duration::from_secs(5));
    let feedback = executor
        .ban("zi_min", Some("no reason"))
        .expect("ban command push fail");
    dbg!(&feedback);
    let result = if feedback == TargetStatus::Success(TargetStatusSuccess::Success)
        || feedback == TargetStatus::Success(TargetStatusSuccess::Duplicated)
    {
        true
    } else {
        false
    };
    assert!(result);
    sleep(Duration::from_secs(5));
    let feedback = executor.pardon("zi_min").expect("pardon command push fail");
    dbg!(&feedback);
    assert_eq!(
        feedback,
        TargetStatus::Success(TargetStatusSuccess::Success)
    );
    sleep(Duration::from_secs(5));

    let feedback = executor.pardon("zi_min").expect("pardon command push fail");
    dbg!(&feedback);
    assert_eq!(
        feedback,
        TargetStatus::Success(TargetStatusSuccess::Duplicated)
    );
}

/// 1.12.2测试，重复封禁仍使用旧信息，不会提示重复信息
#[test]
fn test_docker_command_ban_ip_none() {
    let Some(mut executor) = get_executor() else {
        panic!("Fail to get rcon executor");
    };
    let feedback = executor
        .ban_ip("192.168.1.1", Some("no reason"))
        .expect("ban-ip command push fail");
    dbg!(&feedback);
    let result = if feedback == TargetStatus::Success(TargetStatusSuccess::Success)
        || feedback == TargetStatus::Success(TargetStatusSuccess::Duplicated)
    {
        true
    } else {
        false
    };
    assert!(result);
    sleep(Duration::from_secs(5));
    let feedback = executor
        .ban_ip("192.168.1.1", Some("no reason"))
        .expect("ban-ip command push fail");
    dbg!(&feedback);
    let result = if feedback == TargetStatus::Success(TargetStatusSuccess::Success)
        || feedback == TargetStatus::Success(TargetStatusSuccess::Duplicated)
    {
        true
    } else {
        false
    };
    assert!(result);
    sleep(Duration::from_secs(5));
    let feedback = executor
        .pardon_ip("192.168.1.1")
        .expect("pardon-ip command push fail");
    dbg!(&feedback);
    assert_eq!(
        feedback,
        TargetStatus::Success(TargetStatusSuccess::Success)
    );
    sleep(Duration::from_secs(5));

    let feedback = executor
        .pardon_ip("192.168.1.1")
        .expect("pardon-ip command push fail");
    dbg!(&feedback);
    // In 1.16.5 and below, pardon player-id has duplicated
    // but pardon-ip shares the same response for Duplicated.
    // Therefore, 1.12.2 parser only return Success
    let result = if feedback == TargetStatus::Success(TargetStatusSuccess::Success)
        || feedback == TargetStatus::Success(TargetStatusSuccess::Duplicated)
    {
        true
    } else {
        false
    };
    assert!(result);
}

#[test]
fn test_docker_command_whitelist_operation_none() {
    let Some(mut executor) = get_executor() else {
        panic!("Fail to get rcon executor");
    };
    let feedback = executor.whitelist().expect("whitelist command push fail");
    dbg!(&feedback);
    assert_eq!(feedback, None);
    sleep(Duration::from_secs(5));

    let feedback = executor
        .whitelist_add("zi_min")
        .expect("whitelist command push fail");
    dbg!(&feedback);
    assert_eq!(
        feedback,
        TargetStatus::Success(TargetStatusSuccess::Success)
    );
    sleep(Duration::from_secs(5));

    let feedback = executor
        .whitelist_add("zi_min")
        .expect("whitelist command push fail");
    dbg!(&feedback);
    let result = if feedback == TargetStatus::Success(TargetStatusSuccess::Success)
        || feedback == TargetStatus::Success(TargetStatusSuccess::Duplicated)
    {
        true
    } else {
        false
    };
    assert!(result);
    sleep(Duration::from_secs(5));

    let feedback = executor
        .whitelist_remove("zi_min")
        .expect("whitelist command push fail");
    dbg!(&feedback);
    assert_eq!(
        feedback,
        TargetStatus::Success(TargetStatusSuccess::Success)
    );
    sleep(Duration::from_secs(5));

    let feedback = executor
        .whitelist_remove("zi_min")
        .expect("whitelist command push fail");
    dbg!(&feedback);
    let result = if feedback == TargetStatus::Success(TargetStatusSuccess::Success)
        || feedback == TargetStatus::Success(TargetStatusSuccess::Duplicated)
    {
        true
    } else {
        false
    };
    assert!(result);
}

#[test]
fn test_docker_command_op_none() {
    let Some(mut executor) = get_executor() else {
        panic!("Fail to get rcon executor");
    };
    let feedback = executor.op("zi_min").expect("op command push fail");
    dbg!(&feedback);
    let result = if feedback == TargetStatus::Success(TargetStatusSuccess::Success)
        || feedback == TargetStatus::Success(TargetStatusSuccess::Duplicated)
    {
        true
    } else {
        false
    };
    assert!(result);
    sleep(Duration::from_secs(5));
    let feedback = executor.op("zi_min").expect("op command push fail");
    dbg!(&feedback);
    let result = if feedback == TargetStatus::Success(TargetStatusSuccess::Success)
        || feedback == TargetStatus::Success(TargetStatusSuccess::Duplicated)
    {
        true
    } else {
        false
    };
    assert!(result);
    sleep(Duration::from_secs(5));
    let feedback = executor.deop("zi_min").expect("op command push fail");
    dbg!(&feedback);
    assert_eq!(
        feedback,
        TargetStatus::Success(TargetStatusSuccess::Success)
    );
    sleep(Duration::from_secs(5));
    let feedback = executor.deop("zi_min").expect("op command push fail");
    dbg!(&feedback);
    assert_eq!(
        feedback,
        TargetStatus::Success(TargetStatusSuccess::Duplicated)
    );
}

#[test]
fn test_docker_command_gamemode_none() {
    let Some(mut executor) = get_executor() else {
        panic!("Fail to get rcon executor");
    };
    let feedback = executor
        .gamemode("creative", Some("player1"))
        .expect("gamemode command push fail");
    dbg!(&feedback);
    assert_eq!(feedback, TargetStatus::NotFound)
}

#[test]
fn test_docker_command_list_none() {
    let Some(mut executor) = get_executor() else {
        panic!("Fail to get rcon executor");
    };
    let feedback = executor.list().expect("list command push fail");
    dbg!(&feedback);
    assert_eq!(feedback, None)
}

#[test]
fn test_docker_command_list_uuid_none() {
    let Some(mut executor) = get_executor() else {
        panic!("Fail to get rcon executor");
    };
    let feedback = executor.list_uuid().expect("list uuid command push fail");
    dbg!(&feedback);
    assert_eq!(feedback, None)
}
