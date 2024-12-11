const anchor = require('@project-serum/anchor');
const { SystemProgram } = anchor.web3;

const main = async() =>{
    console.log("🚀 Empezando test...");

    const provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);

    const program = anchor.workspace.solana_scm;
    const newRegistryPk = anchor.web3.Keypair.generate();

    const registryName = "Registro Test";

    let txHash = await program.rpc.create_regitry(
        registryName,
        {
        accounts: {
            registry: newRegistryPk.publicKey,
            user: provider.wallet.publicKey,
            systemProgram: SystemProgram.programId 
        },
        signers: [newRegistryPk],
    });
    console.log("🖊 Firma de tu transacción: ", txHash);

    let account = await program.account.registry.fetch(newRegistryPk.publicKey)
    console.log("📑 Contador Dispositivos: ", account.deviceCount.toString());
}

const runMain = async () => {
    try {
        await main();
    } catch (error) {
        console.log(error);
        process.exit(1);
    }
}

runMain();