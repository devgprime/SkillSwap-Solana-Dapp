const marketplaceService = require('../services/marketplaceService');

exports.listForSale = async (req, res) => {
    try {
        const listing = await marketplaceService.listForSale(req.body);
        res.status(201).json(listing);
    } catch (error) {
        res.status(400).json({ error: error.message });
    }
};

exports.buyNft = async (req, res) => {
    try {
        const transaction = await marketplaceService.buyNft(req.body);
        res.status(200).json(transaction);
    } catch (error) {
        res.status(400).json({ error: error.message });
    }
};

exports.cancelListing = async (req, res) => {
    try {
        const result = await marketplaceService.cancelListing(req.body);
        res.status(200).json(result);
    } catch (error) {
        res.status(400).json({ error: error.message });
    }
};
