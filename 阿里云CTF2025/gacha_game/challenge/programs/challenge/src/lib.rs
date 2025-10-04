use anchor_lang::{
    prelude::*, solana_program::system_program, system_program::{create_account, transfer, CreateAccount, Transfer}, Discriminator
};

declare_id!("39EXycGNJFySJCK61evbVbJ3JWj9wVsqRF7RbJq1fic3");

fn create_character(name: &str, description: &str, attack: u64, defense: u64) -> CharacterInfo {
    CharacterInfo {
        name: name.to_string(),
        description: description.to_string(),
        attack,
        defense,
    }
}

fn generate_random(seed: u64) -> u64 {
    (seed * 1103515245 + 12345) & 0x7fffffff
}

fn close_account(account: AccountInfo, dest: AccountInfo) -> Result<()> {
    // Ref: https://github.com/coral-xyz/anchor/blob/master/lang/src/common.rs
    let user_lamports = dest.lamports();
    **dest.lamports.borrow_mut() = user_lamports.checked_add(account.lamports()).unwrap();
    **account.lamports.borrow_mut() = 0;
    
    account.assign(&system_program::ID);
    account.realloc(0, false)?;

    Ok(())
}

#[program]
pub mod challenge {
    use super::*;

    /// Initialize game
    pub fn create_game(ctx: Context<CreateGame>) -> Result<()> {
        let game = &mut ctx.accounts.game;
        game.price = 1_000_000_000; // 1 SOL
        game.admin = ctx.accounts.admin.key();
        game.characters = [
            create_character(
                "Hiiroya",
                "A cold and ruthless shadow assassin skilled in stealth and assassination. Her left eye is marked with mysterious runes, hinting at a pact with an unknown power.",
                92,
                48,
            ),
            create_character(
                "Shirayuu",
                "A gentle healer who wields the power of life and nature. She is always accompanied by a cute fox spirit.",
                36,
                75,
            ),
            create_character(
                "Aoran",
                "A steadfast swordswoman wielding an ancient greatsword. She is dedicated to protecting the weak and serves as the team's indispensable shield.",
                70,
                89,
            ),
            create_character(
                "Ruri",
                "A ghostly mage who specializes in controlling blue flames. She always carries a mischievous smile, making her motives hard to decipher.",
                85,
                53,
            ),
            create_character(
                "Otoha",
                "A battle songstress whose voice inspires allies and weakens foes. On stage, she shines like a descending star.",
                50,
                62,
            ),
        ];

        Ok(())
    }


    pub fn generate_dungeon<'c: 'info, 'info>(ctx: Context<'_, '_, 'c, 'info, GenerateDungeon<'info>>, bosses: Vec<CharacterInfo>) -> Result<()> {
        let rent = Rent::get()?;

        for (idx, boss) in bosses.iter().enumerate() {
            // PDA check
            let (boss_pda, bump) = Pubkey::find_program_address(&[b"boss", &idx.to_le_bytes()], ctx.program_id);
            require_keys_eq!(
                *ctx.remaining_accounts[idx].key,
                boss_pda
            );

            // generate boss account
            create_account(
                CpiContext::new_with_signer(
                    ctx.accounts.system_program.to_account_info(),
                    CreateAccount {
                        from: ctx.accounts.admin.to_account_info(),
                        to: ctx.remaining_accounts[idx].clone(),
                    },
                    &[&[b"boss", &idx.to_le_bytes(), &[bump]]],
                ),
                rent.minimum_balance(8 + Boss::INIT_SPACE),
                8 + Boss::INIT_SPACE as u64,
                ctx.program_id,
            )?;

            // init character
            let boss_data = Boss {
                info: boss.clone(),
                level: idx as u8,
            }.try_to_vec()?;
            let boss_account = &mut ctx.remaining_accounts[idx].try_borrow_mut_data()?;
            boss_account[..8].copy_from_slice(&Boss::DISCRIMINATOR);
            boss_account[8..boss_data.len()+8].copy_from_slice(&boss_data);
        }

        Ok(())
    }

    /// Initialize player
    pub fn register(ctx: Context<Register>) -> Result<()> {
        let player = &mut ctx.accounts.player;
        player.user = ctx.accounts.user.key();
        player.slot_count = 10;
        player.seed = 42;
        Ok(())
    }

    /// Merge and level up same characters
    pub fn merge(ctx: Context<Merge>, character1: u8, character2: u8) -> Result<()> {
        let player = &mut ctx.accounts.player;
        require_neq!(character1, character2, GameError::InvalidCharacter);

        let c1_key = player.characters[character1 as usize];
        let c2_key = player.characters[character2 as usize];

        require_keys_neq!(c1_key, Pubkey::default(), GameError::InvalidCharacter);
        require_keys_neq!(c2_key, Pubkey::default(), GameError::InvalidCharacter);
        require_keys_neq!(c1_key, c2_key, GameError::InvalidCharacter);

        let character1_account = &mut ctx.accounts.character1;
        let character2_account = &mut ctx.accounts.character2;

        require_eq!(&character1_account.info.name, &character2_account.info.name, GameError::InvalidCharacter);
        
        require_gt!(character1_account.level, 0, GameError::InvalidCharacter);
        require_gt!(10, character2_account.level, GameError::MaxLevel);

        character1_account.level -= 1;
        character1_account.attack -= 20;
        character1_account.defense -= 20;

        character2_account.level += 1;
        character2_account.attack += 20;
        character2_account.defense += 20;

        // Close character1 account if level == 0
        if character1_account.level == 0 {
            close_account(
                ctx.accounts.character1.to_account_info(),
                ctx.accounts.user.to_account_info(),
            )?;
            player.characters[character1 as usize] = Pubkey::default();
        }

        Ok(())
    }

    /// Gacha
    pub fn gacha<'c: 'info, 'info>(ctx: Context<'_, '_, 'c, 'info, Gacha<'info>>) -> Result<()> {
        let player = &mut ctx.accounts.player;

        transfer(
            CpiContext::new(
                ctx.accounts.system_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.user.to_account_info(),
                    to: ctx.accounts.game.to_account_info(),
                },
            ),
            ctx.accounts.game.price,
        )?;

        let random = generate_random(player.seed);
        player.seed = random;
        let character = &ctx.accounts.game.characters[random as usize % 5];

        let rent = Rent::get()?;
        
        let empty_slot = player
            .characters
            .iter()
            .position(|&p| p == Pubkey::default())
            .ok_or_else(|| error!(GameError::NoEmptySlots))?;
        
        // PDA check
        let (character_pda, bump) = Pubkey::find_program_address(&[b"character", player.user.as_ref(), &empty_slot.to_le_bytes()], ctx.program_id);
        require_keys_eq!(
            *ctx.accounts.character.key,
            character_pda
        );

        // generate character account
        create_account(
            CpiContext::new_with_signer(
                ctx.accounts.system_program.to_account_info(),
                CreateAccount {
                    from: ctx.accounts.user.to_account_info(),
                    to: ctx.accounts.character.to_account_info(),
                },
                &[&[b"character", player.user.as_ref(), &empty_slot.to_le_bytes(), &[bump]]],
            ),
            rent.minimum_balance(8 + Character::INIT_SPACE),
            8 + Character::INIT_SPACE as u64,
            ctx.program_id,
        )?;

        // init character
        let character_data = Character {
            info: character.clone(),
            user: player.user,
            level: 1,
            attack: character.attack,
            defense: character.defense,
        }.try_to_vec()?;

        let character_account = &mut ctx.accounts.character.try_borrow_mut_data()?;
        character_account[..8].copy_from_slice(&Character::DISCRIMINATOR);
        character_account[8..character_data.len()+8].copy_from_slice(&character_data);

        player.characters[empty_slot] = ctx.accounts.character.key();
        
        Ok(())
    }


    /// Fight
    pub fn boss_fight<'c: 'info, 'info>(ctx: Context<'_, '_, 'c, 'info, BossFight<'info>>) -> Result<()> {
        let boss = &ctx.accounts.boss;
        let character1 = &ctx.accounts.character1;
        let character2 = &ctx.accounts.character2;
        let character3 = &ctx.accounts.character3;
        
        let total_atk = character1.attack + character2.attack + character3.attack;
        let total_def = character1.defense + character2.defense + character3.defense;

        require_gt!(total_def, boss.info.attack, GameError::UserLose);
        require_gt!(total_atk, boss.info.defense, GameError::UserLose);

        Ok(())
    }
    
}

#[derive(Accounts)]
pub struct CreateGame<'info> {
    #[account(
        init,
        space = 8 + Game::INIT_SPACE,
        seeds = [b"game"],
        bump,
        payer = admin
    )]
    pub game: Account<'info, Game>,
    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct GenerateDungeon<'info> {
    #[account(
        has_one = admin,
    )]
    pub game: Account<'info, Game>,
    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Register<'info> {
    #[account(
        init,
        space = 8 + Player::INIT_SPACE,
        seeds = [b"player", user.key().as_ref()],
        bump,
        payer = user
    )]
    pub player: Account<'info, Player>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Gacha<'info> {
    #[account(mut)]
    pub game: Account<'info, Game>,
    #[account(
        mut,
        seeds = [b"player", user.key().as_ref()],
        bump
    )]
    pub player: Account<'info, Player>,
    #[account(mut)]
    pub user: Signer<'info>,
    /// CHECK: PDA check
    #[account(mut)]
    pub character: UncheckedAccount<'info>, 
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Merge<'info> {
    #[account(
        mut,
        seeds = [b"player", user.key().as_ref()],
        bump
    )]
    pub player: Account<'info, Player>,
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        mut,
        has_one = user,
    )]
    pub character1: Account<'info, Character>,
    #[account(
        mut,
        has_one = user,
    )]
    pub character2: Account<'info, Character>,
    pub system_program: Program<'info, System>,   
}

#[derive(Accounts)]
pub struct BossFight<'info> {
    pub game: Account<'info, Game>,
    #[account(
        mut,
        close = user,
    )]
    pub boss: Account<'info, Boss>,
    #[account(
        has_one = user,
    )]
    pub player: Account<'info, Player>,
    #[account(
        has_one = user,
        constraint = character1.key() != character2.key()
    )]
    pub character1: Account<'info, Character>,
    #[account(
        has_one = user,
        constraint = character2.key() != character3.key()
    )]
    pub character2: Account<'info, Character>,
    #[account(
        has_one = user,
        constraint = character1.key() != character3.key()
    )]
    pub character3: Account<'info, Character>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
#[derive(InitSpace, Debug)]
pub struct Game {
    pub price: u64,
    pub admin: Pubkey,
    pub characters: [CharacterInfo; 5],
}

#[derive(InitSpace, PartialEq, Clone, Debug, AnchorSerialize, AnchorDeserialize)]
pub struct CharacterInfo {
    #[max_len(32)]
    pub name: String,
    #[max_len(256)]
    pub description: String,
    pub attack: u64,
    pub defense: u64,
}

#[account]
#[derive(InitSpace, Debug)]
pub struct Player {
    pub user: Pubkey,
    pub seed: u64,
    pub slot_count: u8,
    pub characters: [Pubkey; 10],
}

#[account]
#[derive(InitSpace, Debug)]
pub struct Character {
    pub info: CharacterInfo,
    pub user: Pubkey,
    pub level: u8,
    pub attack: u64,
    pub defense: u64,
}

#[account]
#[derive(InitSpace, Debug)]
pub struct Boss {
    pub info: CharacterInfo,
    pub level: u8,
}

#[error_code]
pub enum GameError {
    #[msg("No empty slots")]
    NoEmptySlots,
    #[msg("Invalid character")]
    InvalidCharacter,
    #[msg("Maximum level reached")]
    MaxLevel,
    #[msg("Total level should be at least 10 to enter the dungeon")]
    LevelTooLow,
    #[msg("You lose!")]
    UserLose,
}