import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { PublicKey } from '@solana/web3.js';
import { expect } from 'chai';

describe('solana-scm', () => {
  const program = anchor.workspace.SolanaScm;
  const userAccount = new PublicKey("A5i8uPKdCycDG3nbGCCAUiLzHEc4ddpfeYGQhPEWuaTJ");

  it('Crear un registro y un dispositivo', async () => {
    const registryName = "Registro4";
    const deviceName = "SensorNuevo";
    const descriptionDevice  = "Sensor medico";
    const deviceDatas = ["Humedad", "45%"];
    const deviceMetadata = ["Bateria", "85%"];

    const tx = await program.rpc.createRegistry(registryName, {
      accounts: {
        user: userAccount,
        systemProgram: anchor.web3.SystemProgram.programId,
      },
    });

    const tx2 = await program.rpc.addDevice(registryName, deviceName, descriptionDevice, deviceDatas, deviceMetadata, {
      accounts: {
        user: userAccount,
        systemProgram: anchor.web3.SystemProgram.programId,
      },
    });

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