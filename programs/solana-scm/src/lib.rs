use anchor_lang::prelude::*; // Incluye el prelude para simplificar imports en Anchor

declare_id!("A5i8uPKdCycDG3nbGCCAUiLzHEc4ddpfeYGQhPEWuaTJ"); // Identificador único del programa en Solana

#[program]
pub mod registry_project {
    use super::*;

    pub fn create_registry(ctx: Context<CreateRegistry>, name: String) -> Result<()> {
        // Validar longitud del nombre
        if name.len() > 64 {
            return Err(RegistryError::NameTooLong.into());
        }

        let registry = &mut ctx.accounts.registry;
        registry.name = name;
        registry.owner_id = *ctx.accounts.user.key;
        registry.device_count = 0;
        registry.device_ids = vec![];
        registry.devices = vec![];

        Ok(())
    }

    pub fn add_device(
        ctx: Context<AddDevice>,
        name: String,
        description: String,
        metadata: Vec<(String, String)>,
        data: Vec<(String, String)>
    ) -> Result<()> {
        let registry = &mut ctx.accounts.registry;

        // Validar que el dispositivo no exista
        if registry.devices.iter().any(|(n, _)| *n == name) {
            return Err(RegistryError::DeviceNotFound.into());
        }

        // Crear el nuevo dispositivo y agregarlo al registro
        let device = Device {
            name: name.clone(),
            description,
            metadata,
            data,
        };

        registry.device_count += 1;
        registry.device_ids.push(ctx.accounts.device.key());
        registry.devices.push((name, device));
        Ok(())
    }

    pub fn set_device_metadata(
        ctx: Context<SetDeviceMetadata>,
        name: String,
        metadata: Vec<(String, String)>,
    ) -> Result<()> {
        let registry = &mut ctx.accounts.registry;
        let device = registry.devices.iter_mut()
            .find(|(n, _)| *n == name)
            .ok_or(RegistryError::DeviceNotFound)?;
        device.1.metadata = metadata;
        Ok(())
    }

    pub fn set_device_data(
        ctx: Context<SetDeviceData>,
        name: String,
        data: Vec<(String, String)>,
    ) -> Result<()> {
        let registry = &mut ctx.accounts.registry;
        let device = registry.devices.iter_mut()
            .find(|(n, _)| *n == name)
            .ok_or(RegistryError::DeviceNotFound)?;
        device.1.data = data;
        Ok(())
    }

    pub fn set_device_metadata_param(
        ctx: Context<SetDeviceMetadataParam>,
        registry_name: String,
        device_name: String,
        param: String,
        value: String,
    ) -> Result<()> {
        let contract = &mut ctx.accounts.contract;

        // Buscar el registro y validar que el usuario es el propietario
        let registry = contract.registries.iter_mut()
            .find(|(n, _)| *n == registry_name)
            .ok_or(RegistryError::RegistryNotFound)?;

        if &registry.1.owner_id != ctx.accounts.user.key {
            return Err(RegistryError::UnauthorizedAccess.into());
        }

        // Buscar el dispositivo y actualizar el parámetro de metadata
        let device = registry.1.devices.iter_mut()
            .find(|(n, _)| *n == device_name)
            .ok_or(RegistryError::DeviceNotFound)?;

        device.1.metadata.push((param, value));
        Ok(())
    }
}

#[account]
pub struct Registry {
    pub device_count: u64,
    pub device_ids: Vec<Pubkey>,
    pub name: String,
    pub owner_id: Pubkey,
    pub devices: Vec<(String, Device)>,
}

impl Registry {
    const MAX_DEVICES: usize = 100;
    const MAX_NAME_LENGTH: usize = 64;
    const MAX_DEVICE_ID_SIZE: usize = 32;
    const MAX_METADATA_LENGTH: usize = 256;
    const MAX_DESCRIPTION_LENGTH: usize = 256;

    const LEN: usize = 8 + 8 + (Self::MAX_DEVICES * Self::MAX_DEVICE_ID_SIZE) + (Self::MAX_NAME_LENGTH + 4) + 32 + (Self::MAX_DEVICES * (Self::MAX_NAME_LENGTH + Self::MAX_METADATA_LENGTH + Self::MAX_DESCRIPTION_LENGTH + 4 + 4));
}

#[account]
pub struct Device {
    pub name: String,
    pub metadata: Vec<(String, String)>,
    pub data: Vec<(String, String)>,
    pub description: String,
}

impl Device {
    const MAX_NAME_LENGTH: usize = 64;
    const MAX_METADATA_LENGTH: usize = 256;
    const MAX_DESCRIPTION_LENGTH: usize = 256;

    const LEN: usize = 8 + (Self::MAX_NAME_LENGTH + 4) + (Self::MAX_METADATA_LENGTH * 2 + 4) + (Self::MAX_DESCRIPTION_LENGTH + 4);
}

#[account]
pub struct Contract {
    pub registries: Vec<(String, Registry)>,
}

#[derive(Accounts)]
pub struct CreateRegistry<'info> {
    #[account(init, payer = user, space = Registry::LEN)]
    pub registry: Account<'info, Registry>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AddDevice<'info> {
    #[account(mut)]
    pub registry: Account<'info, Registry>,
    #[account(init, payer = user, space = Device::LEN)]
    pub device: Account<'info, Device>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SetDeviceMetadata<'info> {
    #[account(mut)]
    pub registry: Account<'info, Registry>,
    #[account(mut)]
    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct SetDeviceData<'info> {
    #[account(mut)]
    pub registry: Account<'info, Registry>,
    #[account(mut)]
    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct SetDeviceMetadataParam<'info> {
    #[account(mut)]
    pub contract: Account<'info, Contract>,
    #[account(mut)]
    pub user: Signer<'info>,
}

#[error_code]
pub enum RegistryError {
    #[msg("Registro no encontrado.")]
    RegistryNotFound,
    #[msg("Dispositivo no encontrado.")]
    DeviceNotFound,
    #[msg("Acceso no autorizado.")]
    UnauthorizedAccess,
    #[msg("El nombre es demasiado largo.")]
    NameTooLong,
}

impl Contract {
    pub fn validate_exists_registry(&self, registry_name: &String) -> bool {
        self.registries.iter().any(|(n, _)| *n == *registry_name)
    }

    pub fn validate_owner(&self, registry_name: String, signer_account: &Pubkey) -> bool {
        if let Some((_, registry)) = self.registries.iter().find(|(n, _)| *n == registry_name) {
            &registry.owner_id == signer_account
        } else {
            false
        }
    }
    
    pub fn validate_exists_device(&self, registry: &Registry, device_name: &String) -> bool {
        registry.devices.iter().any(|(n, _)| *n == *device_name)
    }
}