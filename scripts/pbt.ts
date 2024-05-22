import {
  JsonRpcProvider,
  Ed25519Keypair,
  RawSigner,
  TransactionBlock,
  SuiObjectRef,
} from "@mysten/sui.js";
import { getFullnodeUrl, SuiClient } from "@mysten/sui.js/client";

// Replace with your actual RPC endpoint
const RPC_ENDPOINT = getFullnodeUrl("devnet");

// Replace with your NFT Move module address
const NFT_MODULE_ADDRESS = "0xYOUR_MODULE_ADDRESS";

// Define your NFT structure
interface MyNFT {
  name: string;
  description: string;
}

/**
 * Function to create and transfer an NFT in a single Sui Programmable Transaction Block (PTB).
 *
 * This function takes the signer, NFT data, and recipient address as input. It creates a PTB
 * that performs the following operations:
 *
 * 1. Calls a Move function within the specified NFT module to create a new NFT object.
 * 2. Registers a placeholder reference for the newly created object within the PTB.
 * 3. Transfers the newly created NFT object to the recipient address.
 * 4. Signs and executes the PTB.
 *
 * @param signer: The RawSigner object used for signing the transaction.
 * @param nft: An object containing the NFT name and description.
 * @param recipient: The recipient address for the transferred NFT.
 * @return A promise that resolves to the execution status of the PTB.
 */
async function createAndTransferNFT(
  signer: RawSigner,
  nft: MyNFT,
  recipient: string
): Promise<string> {
  const tx = new TransactionBlock();

  // 1. Move call to create NFT
  const createNFTCall = {
    target: `${NFT_MODULE_ADDRESS}::nft::create_nft`,
    arguments: [tx.pure(nft.name), tx.pure(nft.description)],
  };
  tx.moveCall(createNFTCall);

  // 2. Register the new object to obtain its reference (placeholder for now)
  const newObjectRef: SuiObjectRef = tx.newObjectRef();

  // 3. Transfer the newly created NFT object
  tx.transferObject({ recipient, objectId: newObjectRef });

  // Sign and execute the PTB
  const result = await signer.signAndExecuteTransactionBlock({
    transactionBlock: tx,
  });
  return result.effects.status;
}

// Main function to demonstrate creating and transferring an NFT
async function main() {
  // Initialize provider and keypair
  const provider = new JsonRpcProvider(RPC_ENDPOINT);
  const keypair = Ed25519Keypair.fromSecretKey(
    Uint8Array.from([
      /* your private key here */
    ])
  );
  const signer = new RawSigner(keypair, provider);

  // Define your NFT
  const nft: MyNFT = {
    name: "My First NFT",
    description: "This is a description of my first NFT",
  };

  // Define the recipient address
  const recipient = "0xRECIPIENT_ADDRESS";

  // Create and transfer NFT in a PTB
  const createAndTransferStatus = await createAndTransferNFT(
    signer,
    nft,
    recipient
  );
  console.log("NFT Create & Transfer Status:", createAndTransferStatus);
}

// Run the main function
main().catch(console.error);
