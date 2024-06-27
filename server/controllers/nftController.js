const nftService = require('../services/nftService');

exports.createNft = async (req, res) => {
    try {
        const nft = await nftService.createNft(req.body);
        res.status(201).json(nft);
    } catch (error) {
        res.status(400).json({ error: error.message });
    }
};

exports.listNfts = async (req, res) => {
    try {
        const nfts = await nftService.listNfts();
        res.status(200).json(nfts);
    } catch (error) {
        res.status(400).json({ error: error.message });
    }
};

exports.getNftById = async (req, res) => {
    try {
        const nft = await nftService.getNftById(req.params.id);
        res.status(200).json(nft);
    } catch (error) {
        res.status(400).json({ error: error.message });
    }
};
