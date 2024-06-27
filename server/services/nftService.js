const Nft = require('../models/Nft');

exports.createNft = async (nftData) => {
    const nft = new Nft(nftData);
    await nft.save();
    return nft;
};

exports.listNfts = async () => {
    return await Nft.find();
};

exports.getNftById = async (id) => {
    return await Nft.findById(id);
};
