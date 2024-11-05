import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { registry } from '@project-serum/anchor/dist/cjs/utils';
import {PublicKey } from '@solana/web3.js';
import { expect } from 'chai';

describe('solana-scm', () => {
  const program = anchor.workspace.SolanaScm;
  const userAccount = new PublicKey("A5i8uPKdCycDG3nbGCCAUiLzHEc4ddpfeYGQhPEWuaTJ");

  it('Crear un registro y un dispositivo', async () => {
    const registryName = "Registro 1";
    const deviceName = "Sensor";
    const descriptionDevice  = "Sensor oficina";
    const deviceData = ["Humedad", "25%"];
    const deviceMetadata = ["Bateria", "65%"];
//ID dispositivos

    const tx = await program.rpc.createRegistry(registryName , {
      accounts: {
        registry: registry,
        user: userAccount,
        systemProgram: anchor.web3.SystemProgram.programId, 
      },
    });

    const tx2 = await program.rpc.addDevice(registryName, deviceName, descriptionDevice, deviceData, deviceMetadata, {
      accounts: {
        registry: registry,
        user: userAccount,
        systemProgram: anchor.web3.SystemProgram.programId, 
      },
    });

    const registryPublickey = tx.publicKey;

    const registryAccount = await program.account.registry.fetch(registryPublickey);
    expect(registryAccount.name).to.equal(registryName);
    expect(registryAccount.deviceCount).to.equal(1);


  });

});