import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SolanaScm } from "../target/types/solana_scm";
import { Keypair, SystemProgram, Connection } from "@solana/web3.js";

// Conexión al clúster
const connection = new Connection("https://api.devnet.solana.com", "confirmed");
connection.getVersion()
  .then(version => console.log("Cluster version: ", version))
  .catch(err => console.error("Error connecting to cluster: ", err));

describe('Solana_scm', () => {
  // Configurar el cliente al cluster
  const provider = anchor.AnchorProvider.env(); // Obtén el proveedor
  anchor.setProvider(provider);                 // Configura el proveedor globalmente

  const program = anchor.workspace.SolanaScm as Program<SolanaScm>;

  const newRegistryKp = Keypair.generate(); // Generar una nueva Keypair para el registro
  const newDeviceKp = Keypair.generate();   // Generar una nueva Keypair para el dispositivo
  const wallet = provider.wallet;          // Obtener la billetera asociada al proveedor

  const registryName = "Registro1";
  const deviceName = "Sensor";
  const deviceDescription = "Sensor de Oficina";

  it('Crear registro!', async () => {
    const txHash = await program.methods
      .createRegistry(registryName)
      .accounts({
        registry: newRegistryKp.publicKey,
        signer: wallet.publicKey, // Dirección de la billetera
        systemProgram: SystemProgram.programId, // Dirección del System Program
      })
      .signers([newRegistryKp]) // Firmantes
      .rpc();

    console.log("Firma de tu transacción para crear registro: ", txHash);
  });

  it('Añadir dispositivo!', async () => {
    const txHash = await program.methods
      .addDevice(deviceName, deviceDescription) // Método `addDevice` del contrato
      .accounts({
        device: newDeviceKp.publicKey,
        registry: newRegistryKp.publicKey,
        signer: wallet.publicKey, // Dirección de la billetera
        systemProgram: SystemProgram.programId, // Dirección del System Program
      })
      .signers([newDeviceKp]) // Firmantes
      .rpc();

    console.log("Firma de tu transacción para añadir dispositivo: ", txHash);
  });
});
 
