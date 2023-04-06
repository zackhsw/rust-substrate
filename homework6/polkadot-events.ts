import { ApiPromise, WsProvider } from '@polkadot/api';

async function main() {
  const provider = new WsProvider('wss://rpc.polkadot.io');
  const api = await ApiPromise.create({ provider });

  // 订阅区块链事件
  api.query.system.events((events) => {
    console.log(`New events: ${JSON.stringify(events)}`);
  });

  // 订阅自定义事件
  const subscription = await api.query.myModule.myEvent((event) => {
    console.log(`My event: ${JSON.stringify(event)}`);
  });
}

main().catch((error) => {
  console.error(error);
});
