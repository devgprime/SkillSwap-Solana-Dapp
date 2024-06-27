const Nft = require('../models/Nft');
const Transaction = require('../models/Transaction');

exports.listForSale = async ({ nftId, price, userId }) => {
    const nft = await Nft.findById(nftId);
    if (!nft) {
        throw new Error('NFT not found');
    }

    if (nft.owner.toString() !== userId.toString()) {
        throw new Error('You are not the owner of this NFT');
    }

    nft.price = price;
    nft.is_listed = true;
    await nft.save();

    return { message: 'NFT listed successfully', nft };
};

exports.buyNft = async ({ nftId, buyerId }) => {
    const nft = await Nft.findById(nftId);
    if (!nft) {
        throw new Error('NFT not found');
    }

    if (!nft.is_listed) {
        throw new Error('This NFT is not listed for sale');
    }

    const transaction = new Transaction({
        buyer: buyerId,
        seller: nft.owner,
        nft: nftId,
        price: nft.price,
        status: 'pending'
    });

    await transaction.save();
    nft.is_listed = false;
    await nft.save();

    return { message: 'Transaction initiated successfully', transaction };
};

exports.cancelListing = async ({ nftId, userId }) => {
    const nft = await Nft.findById(nftId);
    if (!nft) {
        throw new Error('NFT not found');
    }

    if (nft.owner.toString() !== userId.toString()) {
        throw new Error('You are not the owner of this NFT');
    }

    if (!nft.is_listed) {
        throw new Error('This NFT is not listed for sale');
    }

    nft.is_listed = false;
    await nft.save();

    return { message: 'Listing canceled successfully' };
};
