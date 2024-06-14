// Required imports
const { ApiPromise, WsProvider } = require('@polkadot/api');
const { hexToU8a } = require('@polkadot/util');

const { Keyring } = require('@polkadot/keyring');

async function main () {
    // Initialise the provider to connect to the local node
    //const provider = new WsProvider('wss://rpc.nice.hydration.cloud');
    const provider = new WsProvider('ws://127.0.0.1:9988');

    // Create the API and wait until ready
    const api = await ApiPromise.create({ provider });

    // Retrieve the chain & node information information via rpc calls
    const [chain, nodeName, nodeVersion] = await Promise.all([
        api.rpc.system.chain(),
        api.rpc.system.name(),
        api.rpc.system.version()
    ]);

    console.log(`You are connected to chain ${chain} using ${nodeName} v${nodeVersion}`);

    const keyring = new Keyring({ type: 'sr25519' });
    const alice = keyring.addFromUri('//Alice');

    let balance = await api.query.system.account(alice.publicKey);
    console.log(`Alice's HDX balance before DDOS ${balance.data.free}`);

    const blockNumber = await api.rpc.chain.getBlock();

    await createDcaSchedules(api, alice, blockNumber.block.header.number);

    let balance_after = await api.query.system.account(alice.publicKey);
    console.log(`Alice's HDX balance after DDOS ${balance_after.data.free}`);

    let balance_diff = balance.data.free - balance_after.data.free;

    console.log(`Alice's spent balance ${balance_diff}`);

}

main().catch(console.error).finally(() => process.exit());

async function createDcaSchedules(api, user, block) {
    let counter = 0;
    let prev_block = block;
    let prev_balance = await api.query.system.account(user.publicKey);

    while (1) {
        const blockInfo = await api.rpc.chain.getBlock();
        const blockNumber = blockInfo.block.header.number;

        //Change this `ddos_run_duration` variable to define how long (in blocks) the DDOS should take
        let ddos_run_duration = 1000;
        let block_spent = blockNumber - block;
        if (block_spent == ddos_run_duration) {
            console.log(`The specified blocktime ${block_spent} passed`);
            break;
        }

        //If there is a new block
        if (!(Math.abs(blockNumber - prev_block) < Number.EPSILON)) {
            ///Change this `dcas_per_block` variable to reach different extrinsic weight utilization
            ///10 (10%)
            ///20 (20%)
            ///28 (30%)
            ///38 (40%)
            ///48 (50%)
            ///56 (60%)
            ///64 (70%)
            ///72 (80%)
            ///85 (90%)
            ///100 (100%)
            let dcas_per_block = 100;
            for (let i = 0; i < dcas_per_block; i++) {
                let user_pub_key = user.publicKey;
                const nonce = await api.rpc.system.accountNextIndex(user_pub_key);
                const tip = 1;

                await createDca(user_pub_key, nonce, tip);
            }

            //Print out block feeresult
            let balance = await api.query.system.account(user.publicKey);
            let balance_diff = prev_balance.data.free - balance.data.free;
            const blockWeight = await api.query.system.blockWeight();

            console.log(`${balance_diff} HDX fee spent in block ${blockNumber} with weight ${blockWeight.normal.refTime}`);

            prev_block = blockNumber;
            prev_balance = balance;
        }
    }

    async function createDca(user_pub_key, nonce, tip) {
        try {
            await api.tx.dca
                .schedule(
                    {
                        owner: user_pub_key,
                        period: 1,
                        totalAmount: 1000000000000000,
                        maxRetries: null,
                        stabilityThreshold: null,
                        slippage: null,
                        order: {
                            Sell: {
                                assetIn: 5,
                                assetOut: 2,
                                amountIn: 100000000000000,
                                minAmountOut: 0,
                                route: null
                            }
                        }
                    },
                    null)
                .signAndSend(user, { nonce, tip });
        } catch (error) {
            console.log("Error while sending DCA - Sent transaction counter when signing fails: ", counter);
        }
    }
}