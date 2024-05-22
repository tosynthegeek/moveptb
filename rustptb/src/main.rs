use sui::coin::Coin;
use sui::object::{Self, ObjectFormat};
use sui::sui::SUI;
use sui::transaction::{CallArg, Transaction, TransactionData};
use sui::transfer;

// Replace with your actual NFT Move module address
const NFT_MODULE_ADDRESS: ObjectFormat = "[NFT_MODULE_ADDRESS_HERE]";

// Struct representing the NFT to be minted
/// @title MyNFT
/// @dev Represents the structure of the NFT
struct MyNFT {
    /// @dev The name of the NFT
    name: String,
    /// @dev The description of the NFT
    description: String,
}

// Function to create a new MyNFT object
/// @notice Creates a new NFT object
/// @param ctx The transaction context
/// @param name The name of the NFT
/// @param description The description of the NFT
/// @return The created NFT object
fn create_nft(ctx: &mut TransactionContext, name: String, description: String) -> Object {
    // Retrieve the NFT module object
    let module = sui::getObject(ctx, &NFT_MODULE_ADDRESS).expect("NFT module not found");

    // Generate a new object ID
    let object_id = object::new(ctx);

    // Create and return the new NFT object
    TransactionData::new_object(
        object_id,
        module,
        vec![
            CallArg::Pure(bcs::to_bytes(&name).expect("Failed to serialize NFT name")),
            CallArg::Pure(bcs::to_bytes(&description).expect("Failed to serialize NFT description")),
        ]
    )
}

/**
 * Main function to create and transfer an NFT in a single Sui transaction.
 *
 * This function takes the coin object ID, recipient address, amount (assuming it's an input),
 * NFT name, and description as input. It builds a Sui transaction with the following steps:
 *
 * 1. Creates a new object containing the NFT data using the `create_nft` function.
 * 2. Defines transaction inputs including serialized NFT name, description, and amount.
 * 3. Defines transaction commands:
 *     - Creating a new NFT object using the NFT module address and the serialized NFT data.
 *     - Transferring the created NFT to the recipient address.
 * 4. Builds the transaction with the coin object ID, recipient address, gas price (set to 0 for simplicity), inputs, and commands.
 *
 * @param coin_object_id: The object ID of the coin used for gas fees (assumed to be an input).
 * @param recipient: The recipient address for the NFT and the transferred Sui.
 * @param amount: The amount of Sui to be transferred (assumed to be an input).
 * @param nft_name: The name of the NFT to be created.
 * @param nft_description: The description of the NFT to be created.
 * @return A Sui transaction object ready for signing and execution.
 */
pub fn main(
    coin_object_id: ObjectFormat, 
    recipient: address::Address, 
    nft_name: String, 
    nft_description: String
) -> Transaction {
    // **Inputs** (Explicit Definition)
    /// @dev Define the inputs for the transaction
    let inputs = vec![
        CallArg::Pure(bcs::to_bytes(&nft_name).expect("Failed to serialize NFT name")),
        CallArg::Pure(bcs::to_bytes(&nft_description).expect("Failed to serialize NFT description")),
    ];

    // **Commands** (Explicit Definition)
    /// @dev Define the commands for the transaction
    let mut commands = vec![];

    // Command to create the new NFT object
    commands.push(TransactionData::new_object(
        ObjectFormat::nil(), // New object ID will be generated
        NFT_MODULE_ADDRESS.clone(),
        vec![
            CallArg::Pure(bcs::to_bytes(&MyNFT { name: nft_name.clone(), description: nft_description.clone() }).expect("Failed to serialize NFT")),
        ],
    ));

    // Command to transfer the created NFT object to the recipient
    commands.push(transfer::create_transfer(coin_object_id, recipient, 1)); // Adjust transfer amount or method accordingly

    // Build the transaction
    /// @dev Construct the transaction with the defined inputs and commands
    let tx = Transaction::new(coin_object_id, recipient, 0); // Gas price set to 0 for simplicity
    tx.inputs(inputs);
    tx.commands(commands);

    tx
}
