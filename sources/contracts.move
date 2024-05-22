module contracts::contracts {

    use sui::transfer;

    /// Struct representing the NFT
    public struct NFT has key, store {
        id: UID,
        name: vector<u8>,
        description: vector<u8>,
    }

    /// Function to create an NFT
    public fun create_nft(
        ctx: &mut TxContext,
        name: vector<u8>,
        description: vector<u8>
    ): NFT {
        let nft = NFT {
            id: object::new(ctx),
            name,
            description,
        };
        nft
    }

    /// Function to transfer an NFT to another address
    public fun transfer_nft(
        nft: NFT,
        recipient: address,
    ) {
        transfer::transfer(nft, recipient);
    }

}
