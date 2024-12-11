use anchor_lang::prelude::*;

declare_id!("8fzNJf6G2S2T1tkstn8T5D7EPmvhpW6AuexbGe3ZmrBQ");

#[program]
pub mod solana_scm {
    use super::*;

    pub fn create_regitry(
        ctx: Context<CreateRegistry>,
        registry_name: String
    ) -> Result<()>{
        if registry_name.len() > 32 {
            return Err(ErrorCode::NameTooLong.into());
        }
        let registry = &mut ctx.accounts.registry;
        registry.name = registry_name;
        registry.device_count = 0;

        msg!("Nombre del registro: {}", registry.name);
        msg!("Dispositivos {}", registry.device_count);
        Ok(())
    }

    pub fn add_device(
        ctx: Context<AddDevice>,
        device_name: String,
        device_description: String
    ) -> Result <()> {
        let device = &mut ctx.accounts.device;
        device.name = device_name;
        device.description = device_description;

        let device_struct = DeviceStruct{
            device_id: device.key() 
        };

        let registry = &mut ctx.accounts.registry;

        registry.device_list.push(device_struct);
        registry.device_count += 1;

        msg!("ID del registro: {}", registry.key());
        msg!("Nombre del registro: {}", registry.name);
        msg!("ID del dispositivo: {}", device.key());
        msg!("Nombre del dispositivo: {}", device.name);
        msg!("Descripción: {}", device.description);
        msg!("Lista: {:?}", registry.device_list);
        Ok(())
    }

    pub fn set_device_data(
        ctx: Context<SetDeviceData>,
        device_id: Pubkey,
        clave_data: String,
        valor_data: String,
    ) -> Result<()> {
        let registry = &mut ctx.accounts.registry;
        let device = &mut ctx.accounts.device;
    
        // Validar si el dispositivo existe en el registro
        let device_exists = registry
            .device_list
            .iter()
            .any(|d| d.device_id == device_id);
    
        if !device_exists {
            return Err(ErrorCode::DeviceNotFound.into());
        }
    
        // Crear la estructura de datos
        let data_struct = DataStruct {
            clave: clave_data,
            valor: valor_data,
        };
    
        // Agregar los datos al dispositivo
        device.data.push(data_struct);
    
        msg!("Datos añadidos al dispositivo con ID: {}", device_id);
        msg!("Data: {:?}", device.data);
        
        Ok(())
    }
    
    pub fn set_device_metadata(
        ctx: Context<SetDeviceMetaData>,
        device_id: Pubkey,
        clave_data: String,
        valor_data: String,
    ) -> Result<()> {
        let registry = &mut ctx.accounts.registry;
        let device = &mut ctx.accounts.device;

        // Validar si el dispositivo existe en el registro
        let device_exists = registry
            .device_list
            .iter()
            .any(|d| d.device_id == device_id);

        if !device_exists {
            return Err(ErrorCode::DeviceNotFound.into());
        }

        // Crear la estructura de datos
        let metadata_struct = MetaDataStruct {
            clave: clave_data,
            valor: valor_data,
        };

        // Agregar los datos al dispositivo
        device.metadata.push(metadata_struct);

        msg!("Datos añadidos al dispositivo con ID: {}", device_id);
        msg!("Metadatos: {:?}", device.metadata);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateRegistry<'info>{
    #[account(
        init, 
        payer = user, 
        space = 432)]
    pub registry: Account<'info, Registry>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct AddDevice<'info>{
    #[account(
        init, 
        payer = user, 
        space = 6480)]
    pub device: Account<'info, Device>,
    #[account(mut)]
    pub registry: Account<'info, Registry>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct SetDeviceData<'info>{
    #[account(mut)]
    pub device: Account<'info, Device>,
    #[account(mut)]
    pub registry: Account<'info, Registry>,
    #[account(mut)]
    pub user: Signer<'info>
}

#[derive(Accounts)]
pub struct SetDeviceMetaData<'info>{
    #[account(mut)]
    pub device: Account<'info, Device>,
    #[account(mut)]
    pub registry: Account<'info, Registry>,
    #[account(mut)]
    pub user: Signer<'info>
}

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct DeviceStruct{
    pub device_id: Pubkey
}

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct DataStruct{
    pub clave: String,
    pub valor: String
}

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct MetaDataStruct{
    pub clave: String,
    pub valor: String
}

#[account] 
pub struct Registry{
    pub name: String, 
    pub device_count: u64,
    pub device_list: Vec<DeviceStruct>
}

#[account]
pub struct Device{
    pub device_id: Pubkey,
    pub name: String,
    pub description: String,
    pub data: Vec<DataStruct>,
    pub metadata: Vec<MetaDataStruct>
}

#[error_code]
pub enum ErrorCode{
    #[msg("Nombre del registro muy largo!.")]
    NameTooLong,
    #[msg("Dispositivo no encontrado!.")]
    DeviceNotFound,
}
