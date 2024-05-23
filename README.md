# Building PTBs in Sui: A Guide for Rust and TypeScript Developers
Programmable Transaction Blocks(PTB) is a feature on the Sui blockchain that allows multiple transactions(upt to 1024) to be chained together and executed atomically, lowering fees and latency. These operations are executed atomically, meaning either all succeed or none do. Transactions within a PTB can depend on the outputs of previous transactions. This allows for more complex operations to be built up from simpler ones.

The Sui blockchain operates on an object-centric model. Everything on Sui, from coins to NFTs to complex DeFi protocols, is represented as an object. 

## Structure of Programmable Transaction Blocks
The PTB consist of 2 major parts: inputs and commands.
```typescript
{  
	inputs:  [Input],  
	commands:  [Command],  
}
```
- **Inputs**: This section provides the data that the transactions within the block rely on. They can be objects or pure Move values and would be used for the commands
- **Commands**: These are the operations that manipulate the blockchain state.

After executing all transactions within a PTB, each transaction produces a result. These results can be:
-   Return values: Functions called within the PTB can return data, which becomes the result for that specific transaction.
-   New object references: If a transaction creates a new object, the reference ID of the created object becomes the result.
-   Boolean values: Transactions might simply succeed or fail, with a boolean result indicating success.

## Sui Programmable Transaction Blocks with Rust
The Rust [code example](https://github.com/tosynthegeek/moveptb/tree/main/rustptb) is designed to create and transfer an NFT (Non-Fungible Token) using the Sui blockchain. It defines a struct for the NFT, a function to create an NFT object, and a main function to build and execute a transaction that includes creating and transferring the NFT.
### Inputs
The inputs for the transaction are defined as a vector of `CallArg::Pure` items, which are serialized forms of the NFT name and description:
```rust
let inputs = vec![
    CallArg::Pure(bcs::to_bytes(&nft_name).expect("Failed to serialize NFT name")),
    CallArg::Pure(bcs::to_bytes(&nft_description).expect("Failed to serialize NFT description")),
];
```
### Commands
The commands for the transaction are defined as a vector of `TransactionData` items, which include:
-   A command to create a new NFT object:
```rust
commands.push(TransactionData::new_object(
    ObjectFormat::nil(), // New object ID will be generated
    NFT_MODULE_ADDRESS.clone(),
    vec![
        CallArg::Pure(bcs::to_bytes(&MyNFT { name: nft_name.clone(), description: nft_description.clone() }).expect("Failed to serialize NFT")),
    ],
));
```
- A command to transfer the created NFT to the recipient:
```
commands.push(transfer::create_transfer(coin_object_id, recipient, 1));
```
### Tx Construction
The transaction is constructed with the coin object ID, recipient, and gas price. With this we can add the inputs and command to the transaction `tx`:
```rust
let tx = Transaction::new(coin_object_id, recipient, 0);

tx.inputs(inputs);
tx.commands(commands);
```

## Sui Programmable Transaction Blocks with Typescript
The [TypeScript code](https://github.com/tosynthegeek/moveptb/blob/main/scripts/pbt.ts) example mirrors the Rust implementation, focusing on creating and transferring an NFT in a single transaction..

The inputs for the transaction are the `name` and `description` of the NFT, which are added as arguments to the `moveCall`.
```typescript
const createNFTCall = {
  target: `${NFT_MODULE_ADDRESS}::nft::create_nft`,
  arguments: [tx.pure(nft.name), tx.pure(nft.description)],
};
```
The command to create the NFT is added to the transaction block using `tx.moveCall`. The command to transfer the created NFT object to the recipient is added using `tx.transferObject`.
  ```typescript
tx.moveCall(createNFTCall);
tx.transferObject({ recipient, objectId: newObjectRef });
```

The transaction block `tx` is signed and executed by the signer using `signAndExecuteTransactionBlock`.
```typescript
const result = await signer.signAndExecuteTransactionBlock({
  transactionBlock: tx,
});
return result.effects.status;
```

## Summary
This guide demonstrates how to create and transfer an NFT using Programmable Transaction Blocks (PTB) on the Sui blockchain with both Rust and TypeScript. By leveraging PTBs, developers can perform complex operations in a streamlined and efficient manner, ensuring atomic execution of multiple transactions.

Check out the [Sui Docs](https://docs.sui.io/concepts/transactions/prog-txn-blocks) to read more about Programmable Transaction Blocks 
