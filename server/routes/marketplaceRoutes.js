const express = require('express');
const marketplaceController = require('../controllers/marketplaceController');
const authMiddleware = require('../middlewares/authMiddleware');

const router = express.Router();

router.post('/list', authMiddleware, marketplaceController.listForSale);
router.post('/buy', authMiddleware, marketplaceController.buyNft);
router.post('/cancel', authMiddleware, marketplaceController.cancelListing);

module.exports = router;
