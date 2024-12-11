import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SolanaScm } from "../target/types/solana_scm";
import { Keypair, PublicKey, SystemProgram } from "@solana/web3.js";

describe('Solana_scm', () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.SolanaScm as Program<SolanaScm>;

  const newRegistryKp = Keypair.generate();
  const newDeviceKp = Keypair.generate();
  
  const wallet = provider.wallet;

  const registryName = "Registro Test 6";
  const deviceName = "Sensor";
  const deviceDescription = "Sensor para área de agricultura";

  const claveData = "Temperatura";
  const valorData = "15°";

 const claveMetaData = "Bateria";
 const valorMetaData = "85%"; 

  it('Crear registro!', async () => {
    console.log("Creando registro con publicKey:", newRegistryKp.publicKey.toBase58());
    const txHash = await program.methods
      .createRegitry(registryName)
      .accounts({
        registry: newRegistryKp.publicKey,
        user: wallet.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .signers([newRegistryKp])
      .rpc();

    console.log("Firma de tu transacción para crear registro: ", txHash);

    const registryAccount = await program.account.registry.fetch(newRegistryKp.publicKey);
    console.log("Registro inicializado: ", registryAccount);
  });

  // Añadir dispositivo a registro ya añadido
  //const idRegistry = new PublicKey("BUXXwssQ6en5UnaRoD6z4JXQmApuqScK9ns7yg22rAj1");
  
  it('Añadir dispositivo!', async () => {
    const txHash = await program.methods
      .addDevice(deviceName, deviceDescription)
      .accounts({
        device: newDeviceKp.publicKey,
        registry: newRegistryKp.publicKey,
        user: wallet.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .signers([newDeviceKp])
      .rpc();

    console.log("Firma de tu transacción para añadir dispositivo: ", txHash);
  });


  const newDeviceKp2 = Keypair.generate();

  const deviceName2 = "Sensor A45UG3";
  const deviceDescription2 = "Sensor de pasillo chico";

  it('Añadir dispositivo!', async () => {
    const txHash = await program.methods
      .addDevice(deviceName2, deviceDescription2)
      .accounts({
        device: newDeviceKp2.publicKey,
        registry: newRegistryKp.publicKey,
        user: wallet.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .signers([newDeviceKp2])
      .rpc();

    console.log("Firma de tu transacción para añadir dispositivo: ", txHash);
  });



  //Agregar los metadataos y datos
  
  it('Añadir datos al dispositivo!', async () => {
    const txHash = await program.methods
      .setDeviceData(newDeviceKp.publicKey, claveData, valorData)
      .accounts({
        device: newDeviceKp.publicKey,
        registry: newRegistryKp.publicKey,
        user: wallet.publicKey,
      })
      .rpc();

    console.log("Transacción enviada para añadir datos: ", txHash);
  }); 
 
  it('Añadir metadata al dispositivo!', async () => {
    const txHash = await program.methods
      .setDeviceMetadata(newDeviceKp.publicKey, claveMetaData, valorMetaData)
      .accounts({
        device: newDeviceKp.publicKey,
        registry: newRegistryKp.publicKey,
        user: wallet.publicKey,
      })
      .rpc();

    console.log("Transacción enviada para añadir metadata: ", txHash);
  });


  const claveData2 = "Altura";
  const valorData2 = "5M";

  const claveMetaData2 = "Idioma";
  const valorMetaData2 = "Español";

  it('Añadir datos al dispositivo!', async () => {
    const txHash = await program.methods
      .setDeviceData(newDeviceKp.publicKey, claveData2, valorData2)
      .accounts({
        device: newDeviceKp.publicKey,
        registry: newRegistryKp.publicKey,
        user: wallet.publicKey,
      })
      .rpc();

    console.log("Transacción enviada para añadir datos: ", txHash);
  }); 
 
  it('Añadir metadata al dispositivo!', async () => {
    const txHash = await program.methods
      .setDeviceMetadata(newDeviceKp.publicKey, claveMetaData2, valorMetaData2)
      .accounts({
        device: newDeviceKp.publicKey,
        registry: newRegistryKp.publicKey,
        user: wallet.publicKey,
      })
      .rpc();

    console.log("Transacción enviada para añadir metadata: ", txHash);
  });


  console.log("Métodos disponibles:", Object.keys(program.methods));

})