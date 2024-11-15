// src/lib.rs

// Importa la biblioteca de Anchor para construir programas en Solana.
use anchor_lang::prelude::*;

// Declara la ID del programa, necesaria para interactuar con la blockchain.
declare_id!("21dr8CYgZFHh8E1DjNqRYXjAUaz76VsziQ3CVr7RgJao");

/// Módulo principal del programa que implementa la lógica de negocio.
#[program]
pub mod solana {
    use super::*;

    /// Función para crear un registro. 
    /// Se asegura de que el nombre del registro no exceda los 64 caracteres y 
    /// inicializa las propiedades principales del registro.
    ///
    /// # Parámetros
    /// - `ctx`: Contexto que contiene las cuentas necesarias para esta operación.
    /// - `registry_name`: El nombre del registro a crear.
    pub fn create_registry(ctx: Context<CreateRegistry>, registry_name: String) -> Result<()> {
        // Valida la longitud del nombre del registro.
        if registry_name.len() > 64 {
            return Err(RegistryError::NameTooLong.into());
        }

        // Inicializa el registro con los datos proporcionados.
        let registry = &mut ctx.accounts.registry;
        registry.name = registry_name;
        registry.device_count = 0; // No hay dispositivos inicialmente.
        registry.device_ids = vec![]; // Lista vacía de dispositivos.

        // Mensajes informativos para el usuario.
        msg!("Registro creado con el nombre: {}", registry.name);
        msg!("Contador iniciado en {}", registry.device_count);

        Ok(())
    }

    /// Función para agregar un dispositivo a un registro existente.
    /// Se guarda la información básica del dispositivo y se actualiza el registro.
    ///
    /// # Parámetros
    /// - `ctx`: Contexto que contiene las cuentas necesarias para esta operación.
    /// - `device_name`: Nombre del dispositivo a agregar.
    /// - `device_description`: Descripción del dispositivo a agregar.
    pub fn add_device(
        ctx: Context<AddDevice>,
        device_name: String,
        device_description: String,
    ) -> Result<()> {
        // Inicializa y guarda la información del dispositivo.
        let device = &mut ctx.accounts.device;
        device.name = device_name;
        device.description = device_description;

        // Actualiza el registro con el nuevo dispositivo.
        let registry = &mut ctx.accounts.registry;
        registry.device_count += 1; // Incrementa el contador de dispositivos.
        registry.device_ids.push(device.key()); // Guarda la clave pública del dispositivo.

        // Mensajes informativos para el usuario.
        msg!("Dispositivo creado: {}", device.name);
        msg!("Descripción: {}", device.description);

        Ok(())
    }
}

/// Estructura que representa un registro en la blockchain.
/// Un registro puede contener múltiples dispositivos.
#[account]
pub struct Registry {
    pub device_count: u64,        // Número de dispositivos registrados.
    pub device_ids: Vec<Pubkey>, // Claves públicas de los dispositivos asociados.
    pub name: String,            // Nombre del registro.
}

/// Estructura que representa un dispositivo individual.
#[account]
pub struct Device {
    pub name: String,        // Nombre del dispositivo.
    pub description: String, // Descripción del dispositivo.
}

/// Contexto necesario para crear un registro.
/// Contiene las cuentas implicadas en esta operación.
#[derive(Accounts)]
pub struct CreateRegistry<'info> {
    #[account(init, payer = user, space = 8 + 8 + 32 * 100)] // Espacio para el registro.
    pub registry: Account<'info, Registry>, // Cuenta que almacenará el registro.
    #[account(mut)]
    pub user: Signer<'info>, // Usuario que paga por la creación del registro.
    pub system_program: Program<'info, System>, // Programa del sistema para inicializar cuentas.
}

/// Contexto necesario para agregar un dispositivo a un registro.
/// Contiene las cuentas implicadas en esta operación.
#[derive(Accounts)]
pub struct AddDevice<'info> {
    #[account(mut)]
    pub registry: Account<'info, Registry>, // Registro al que se añadirá el dispositivo.
    #[account(init, payer = user, space = 8 + 32 + 32)] // Espacio para el dispositivo.
    pub device: Account<'info, Device>, // Cuenta que almacenará el dispositivo.
    #[account(mut)]
    pub user: Signer<'info>, // Usuario que paga por la creación del dispositivo.
    pub system_program: Program<'info, System>, // Programa del sistema para inicializar cuentas.
}

/// Enumeración de errores personalizados del programa.
/// Define los posibles errores que pueden ocurrir durante la ejecución.
#[error_code]
pub enum RegistryError {
    #[msg("Registro no encontrado.")]
    RegistryNotFound, // Error si no se encuentra un registro.
    #[msg("Dispositivo no encontrado.")]
    DeviceNotFound, // Error si no se encuentra un dispositivo.
    #[msg("Acceso no autorizado.")]
    UnauthorizedAccess, // Error si un usuario no está autorizado.
    #[msg("El nombre es demasiado largo.")]
    NameTooLong, // Error si el nombre excede la longitud permitida.
}
