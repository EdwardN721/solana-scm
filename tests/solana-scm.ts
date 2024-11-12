import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { PublicKey, Keypair } from '@solana/web3.js';
import { expect } from 'chai';

describe('solana-scm', () => {
  const program = anchor.workspace.SolanaScm;
  const userAccount = new PublicKey("GzQkzkgiYYDtwtz3bx6MZmAwLvLBn2B2Tn5JMvDJVGgr");

  it('Crear un registro y un dispositivo', async () => {
    const registryName = "RegistroTest5";
    const deviceName = "SensorNuevo3";
    const descriptionDevice  = "Sensor medico";
    const deviceDatas = ["Humedad", "45%"];
    const deviceMetadata = ["Bateria", "85%"];

    // Crear una cuenta para el registro
    const registryAccount = Keypair.generate();

    // Crear el registro
    const tx = await program.rpc.createRegistry({
      name: registryName,  // Aquí pasas los parámetros correctos dentro de un objeto
    }, {
      accounts: {
        registry: registryAccount.publicKey,  // Asegúrate de usar solo la clave pública para el registro
        user: userAccount,
        systemProgram: anchor.web3.SystemProgram.programId,
      },
      signers: [registryAccount],  // Firmamos la transacción con la cuenta generada
    });

    // Añadir el dispositivo al registro
    const tx2 = await program.rpc.addDevice(registryName, deviceName, descriptionDevice, deviceDatas, deviceMetadata, {
      accounts: {
        user: userAccount,
        systemProgram: anchor.web3.SystemProgram.programId,
      },
    });

    // Obtener la clave pública de los registros del programa
    const registryPublickey = await program.provider.connection.getProgramAccounts(program.programId).then(accounts => accounts[0].pubkey);

    // Obtener información del registro (función hipotética)
    const registryData = await program.rpc.getRegistryData(registryName);

    expect(registryData.name).to.equal(registryName);
    expect(registryData.deviceCount).to.equal(1);

    // Obtener información del dispositivo (función hipotética)
    const deviceData = await program.rpc.getDeviceData(registryName, deviceName);
    expect(deviceData.name).to.equal(deviceName);
    // ... otras validaciones de los datos del dispositivo
  });
});
