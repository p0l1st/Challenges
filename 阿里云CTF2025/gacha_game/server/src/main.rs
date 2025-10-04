use anchor_lang::{prelude::AccountMeta, AccountDeserialize, InstructionData, ToAccountMetas};
use solana_sdk::{
    instruction::Instruction,
    pubkey::Pubkey,
    compute_budget::ComputeBudgetInstruction,
    signature::{Keypair, Signer},
};

use sol_ctf_framework::ChallengeBuilder;

use solana_program::system_instruction;

use std::{
    error::Error,
    env,
    io::{BufReader, BufRead, Write},
    net::{TcpListener, TcpStream},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind("0.0.0.0:5000")?;

    println!("Server listening on port 5000");

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();

        tokio::spawn(async move {
            if let Err(err) = handle_connection(&mut stream).await {
                writeln!(stream, "error: {:?}", err).ok();
            }
        });
    }
    Ok(())
}

async fn handle_connection(socket: &mut TcpStream) -> Result<(), Box<dyn Error>> {
    let mut builder = ChallengeBuilder::try_from(socket.try_clone()?)?;

    let chall_id = builder.add_program("./challenge.so", Some(challenge::id()));
    let solve_id = builder.input_program()?;

    let mut chall = builder.build().await;

    // -------------------------------------------------------------------------
    // initialize
    // -------------------------------------------------------------------------
    let program_id = chall_id;

    println!("Program ID: {}\n", program_id);


    let admin_keypair = chall.ctx.payer.insecure_clone();
    let admin = chall.ctx.payer.pubkey();

    let (game, _) = Pubkey::find_program_address(&[b"game"], &program_id);

    // -------------------------------------------------------------------------
    // create game singleton
    // -------------------------------------------------------------------------

    let ix = challenge::instruction::CreateGame {};
    let ix_accounts = challenge::accounts::CreateGame {
        game,
        admin,
        system_program: solana_program::system_program::id(),
    };

    chall
    .run_ixs_full(
        &[Instruction::new_with_bytes(
            program_id,
            &ix.data(),
            ix_accounts.to_account_metas(None),
        )],
        &[&admin_keypair],
        &admin,
    ).await.unwrap_or_else(|error| println!("Error initializing game: {}", error));

    // -------------------------------------------------------------------------
    // User operations (register, gacha, merge, etc.)
    // -------------------------------------------------------------------------

    let user_keypair = Keypair::new();
    let user = user_keypair.pubkey();
    // You only have 8 SOL :(
    chall
        .run_ix(system_instruction::transfer(&admin, &user, 8_000_000_000))
        .await?;

    // provided info
    writeln!(socket, "user: {}", user)?;
    writeln!(socket, "game: {}", game)?;

    let solve_ix = chall.read_instruction(solve_id)?;
    
    let bump_budget = ComputeBudgetInstruction::set_compute_unit_limit(10_000_000);

    chall
        .run_ixs_full(&[bump_budget, solve_ix], &[&user_keypair], &user)
        .await.unwrap_or_else(|error| println!("Error running exploit: {}", error));

    // -------------------------------------------------------------------------
    // generate dungeon
    // -------------------------------------------------------------------------

    let ix = challenge::instruction::GenerateDungeon {
        bosses: vec![
            challenge::CharacterInfo {
                name: "Kael’tharion".to_string(),
                description: "The Infernal Overlord, wreathed in flames and hatred.".to_string(),
                attack: 100,
                defense: 100,
            },
            challenge::CharacterInfo {
                name: "Morvexus".to_string(),
                description: "The Shadow Weaver, who manipulates darkness and traps.".to_string(),
                attack: 200,
                defense: 200,
            },
            challenge::CharacterInfo {
                name: "Drakthar, the Wyrmborn".to_string(),
                description: "A corrupted dragonkin with the power of a thousand storms.".to_string(),
                attack: 500,
                defense: 500,
            },
            challenge::CharacterInfo {
                name: "Veyra, the Bloodbound".to_string(),
                description: "A vampiric queen whose beauty hides her insatiable hunger.".to_string(),
                attack: 800,
                defense: 800,
            },
            challenge::CharacterInfo {
                name: "Zephrak, the Eternal Machine".to_string(),
                description: "A relentless mechanical behemoth powered by an ancient core.".to_string(),
                attack: 1337,
                defense: 1337,
            },
        ],
    };
    let mut ix_accounts = challenge::accounts::GenerateDungeon {
        game,
        admin,
        system_program: solana_program::system_program::id(),
    }.to_account_metas(None);

    let remaining_accounts: Vec<AccountMeta> = (0..5)
        .map(|idx: usize| {
            AccountMeta {
                pubkey: Pubkey::find_program_address(&[b"boss", &idx.to_le_bytes()], &program_id).0,
                is_signer: false,
                is_writable: true,
            }
        }).collect();
    
    ix_accounts.extend(remaining_accounts);


    chall
    .run_ixs_full(
        &[Instruction::new_with_bytes(
            program_id,
            &ix.data(),
            ix_accounts,
        )],
        &[&admin_keypair],
        &admin,
    ).await.unwrap_or_else(|error| println!("Error creating dungeon: {}", error));

    // -------------------------------------------------------------------------
    // boss fight
    // -------------------------------------------------------------------------

    writeln!(socket, "Please choose 3 characters for boss fight: ")?;

    let mut reader = BufReader::new(socket.try_clone()?);
    let mut line = String::new();
    reader.read_line(&mut line)?;

    let character_ids: Vec<usize> = line.trim().split_whitespace().map(|s| s.parse::<usize>().unwrap()).collect();
    let (player, _) = Pubkey::find_program_address(&[b"player", user.as_ref()], &program_id);
    let (character1, _) = Pubkey::find_program_address(&[b"character", user.as_ref(), &character_ids[0].to_le_bytes()], &program_id);
    let (character2, _) = Pubkey::find_program_address(&[b"character", user.as_ref(), &character_ids[1].to_le_bytes()], &program_id);
    let (character3, _) = Pubkey::find_program_address(&[b"character", user.as_ref(), &character_ids[2].to_le_bytes()], &program_id);

    let character1_account = challenge::Character::try_deserialize(&mut chall.ctx.banks_client.get_account(character1).await?.unwrap_or_default().data.as_slice())?;
    let character2_account = challenge::Character::try_deserialize(&mut chall.ctx.banks_client.get_account(character2).await?.unwrap_or_default().data.as_slice())?;
    let character3_account = challenge::Character::try_deserialize(&mut chall.ctx.banks_client.get_account(character3).await?.unwrap_or_default().data.as_slice())?;

    if character1_account.level + character2_account.level + character3_account.level < 10 {
        writeln!(socket, "Please choose 3 characters with total level of at least 10")?;
        return Ok(());
    }

    for idx in 0_usize..5 {
        let (boss, _) = Pubkey::find_program_address(&[b"boss", &idx.to_le_bytes()], &program_id);

        let ix = challenge::instruction::BossFight {};
        let ix_accounts = challenge::accounts::BossFight {
            game,
            boss,
            player,
            character1,
            character2,
            character3,
            user,
            system_program: solana_program::system_program::id(),
        };    
        
        let fight = chall
        .run_ixs_full(
            &[Instruction::new_with_bytes(
                program_id,
                &ix.data(),
                ix_accounts.to_account_metas(None),
            )],
            &[&user_keypair],
            &user,
        ).await;
        
        if fight.is_err() {
            writeln!(socket, "Failed fighting boss {}", idx)?;
            break;
        }
    }

    // -------------------------------------------------------------------------
    // check winning condition
    // -------------------------------------------------------------------------

    for idx in 0_usize..5 {
        let (boss, _) = Pubkey::find_program_address(&[b"boss", &idx.to_le_bytes()], &program_id);
        if !chall.ctx.banks_client.get_account(boss).await?.unwrap_or_default().data.is_empty() {
            writeln!(socket, "Boss {} still alive", idx)?;
            return Ok(());
        }
    }

    writeln!(socket, "Congrats!")?;
    if let Ok(flag) = env::var("FLAG") {
        writeln!(socket, "flag: {:?}", flag)?;
    } else {
        writeln!(socket, "flag not found, please contact admin")?;
    }


    Ok(())
}