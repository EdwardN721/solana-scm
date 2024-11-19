// Importa la biblioteca de Anchor para construir programas en Solana.
use anchor_lang::prelude::*;

// Declara la ID del programa, necesaria para interactuar con la blockchain.
declare_id!("21dr8CYgZFHh8E1DjNqRYXjAUaz76VsziQ3CVr7RgJao");

#[program]
pub mod solana_scm {
    use super::*;

    pub fn create_registry(
        ctx: Context<CreateRegistry>, 
        registry_name: String
    ) -> Result<()> {
        if registry_name.len() > 32 {
            return Err(ErrorCode::NameTooLong.into());
        }
            
        let registry = &mut ctx.accounts.registry;
        registry.name = registry_name;
        registry.device_count = 0;
        registry.devices_ids = Vec::new();

        msg!("Registro creado con el nombre: {}", registry.name);
        msg!("Registro creado con {} dispositivos.", registry.device_count);
        Ok(())
    }

    pub fn add_device(
        ctx: Context<AddDevice>, 
        device_name: String, 
        device_description: String
    ) -> Result<()> {
        let device = &mut ctx.accounts.device;
        device.name = device_name;
        device.description = device_description;

        let registry = &mut ctx.accounts.registry;
        registry.device_count += 1;
        registry.devices_ids.push(device.key());

        msg!("Dispositivo creado con el nombre: {}", device.name);
        msg!("Descripción: {}", device.description);
        msg!("ID registro: {}", registry.key());
        msg!("Nombre registro: {}", registry.name);
        msg!("Dispositivos: {}", registry.device_count);
        msg!("IDs: {:?}", registry.devices_ids);

        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateRegistry<'info> {
    #[account(
        init, 
        payer = signer, 
        space = 8 + 4 + 32 + 8 + (32*10), 
    )]
    pub registry: Account<'info, Registry>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AddDevice<'info>{
    #[account(
        init, 
        payer = signer, 
        space = 8 + 32 + 32,  // Si planeas almacenar solo 5 dispositivos
    )]
    pub device: Account<'info, Device>,
    #[account(mut)]
    pub registry: Account<'info, Registry>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>
}

#[account]
pub struct Registry {
    pub name: String,
    pub device_count: u64,
    pub devices_ids: Vec<Pubkey>,
}

#[account]
pub struct Device {
    pub name: String,
    pub description: String
}

#[error_code]
pub enum ErrorCode {
    #[msg("Nombre del registro muy largo!.")]
    NameTooLong,
}
